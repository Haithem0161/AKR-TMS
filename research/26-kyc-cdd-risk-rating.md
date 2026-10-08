# 26. KYC/CDD and customer risk rating

Status: done (2026-10-06)

ID prefix for derived requirements: K (K-01 to K-42)

Contents:

1. Scope, cross-references and sources read
2. Customer risk rating methods (families, static versus dynamic, choices)
3. Risk factors (customer, geography and country-risk sources, products, channel, behaviour; what the CBS can supply)
4. What FATF, Wolfsberg, Basel and the EU say
5. What Iraq requires (and the gaps the product must handle)
6. Review cycles and event-driven triggers (trigger catalogue TR-01 to TR-26)
7. Enhanced due diligence, source of wealth and source of funds
8. Beneficial ownership: thresholds and the state of registers in Iraq
9. PEP handling (sub-tiers P1-P3, lifecycle, register)
10. Overrides and their governance
11. Calibration, validation, inflation, explainability and data quality
12. How vendors do it (Fenergo, NICE Actimize CDD-X, Oracle KYC, Moody's)
13. What regulators criticised (FSA 2011, HSBC, US Bancorp, Habib, TD Bank, Starling, Barclays, Nationwide)
14. Proposed AKR-TMS customer risk rating model for Iraqi banks
15. What it means for AKR-TMS
16. Derived requirements (K-xx)
17. Sources

## Findings in brief

1. **The rating's only job is to decide controls**, and every enforcement case read here is a rating that did not change what the bank did: Starling (restriction on high-risk onboarding breached 54,359 times), Barclays (customer rated low for six years through police raids and production orders), Nationwide (default "standard" and 44,957 accounts never fed to the tool).
2. **Triggers beat calendars.** The CBI wants dynamic updates "with each customer interaction", two-year classification reviews and a yearly KYC form update (a written conflict to resolve with the CBI); banks practise 1, 2 and 3 years. EU rules cap High at one year and others at five from July 2027; the US only requires event-driven updating.
3. **Beneficial ownership is the weakest data.** Iraq's company registry only began collecting BO declarations in July 2023 (20% threshold), has no verification yet and a definition that misses chains and front persons; banks are the de facto register. The CBI deck counts 37,286 BO statements received.
4. **Iraqi PEPs are higher risk by the FCA's own test** (CPI 28, rank 136 of 182; weak registries). The CBI wants relatives to the second degree, persistence after office, daily local news and seven KPIs; no public Iraqi PEP list exists.
5. **Country indices do not discriminate inside Iraq**: Iraq is 5.90 on the Basel AML Index 2025 (rank 48 of 177, medium band), 28 on CPI 2025, grey-listed since 19 June 2026, not on the EU list as of January 2026. Per-customer geography should measure the foreign nexus and observed corridors.
6. **A hybrid, white-box model is proposed**: 27 factors in six blocks, floors for CBI and legal certainties, a peak term against dilution, five levels mapped to High, Medium, Low, a controls matrix and a full explanation panel; worked examples are computed. All numbers are starting values for a shadow-run calibration.
7. **Vendors sell configurable, event-driven risk but not Iraqi content or transparent logic**, and assume a write path to their own system of record; AKR-TMS is read-only, so ratings live in AKR-TMS with a one-way change feed.

---

## 1. Scope, cross-references and sources read

This file covers customer risk rating (CRR) and the CDD controls that hang on it, and ends with a proposed AKR-TMS rating model for Iraqi banks (section 14).
It does not repeat what files 01, 03, 04, 05 and 20 already hold; it cross-references them and adds primary text read on 2026-10-06.

| Topic | Already covered in | Added here |
|---|---|---|
| FATF R.1, R.10, R.12, R.24-25, INR.10; banking RBA guidance 2014 | File 03 (sections 3, 5b, requirements F-02, F-03, F-07, F-09, F-10, F-14) | Verbatim paras 58-69 of the 2014 banking guidance on risk profiles and monitoring (section 4) |
| BCBS 2020 paras 32-50; Wolfsberg RBA 2026, country risk 2024, PEP 2017, SoW/SoF 2020, digital lifecycle 2022, CBDDQ | File 04 (sections 1c-1d, 2c, 2h, 2j, 2l, 2n, 2q; requirements W-15, W-17, W-19, W-21, W-24) | CBDDQ guidance Q67-Q75; Wolfsberg RBA 2026 customer variables (section 4) |
| Iraq: Instructions 1/2023, CBI PEP circular of 26 Aug 2026, CBI controls 2025 | File 01 (sections 2b, 2c-B, 2d) | MER 2024 beneficial-ownership regime (paras 685-730), CBI May 2026 deck facts, and the unresolved annual versus two-year review conflict (sections 5, 6, 8) |
| US, EU, UK regimes | File 05 (sections 1d, 2a-2e, 3a-3b) | EU AMLR articles 20, 26, 34, 42, 45, 52 and Annexes I and III; EBA ML/TF Risk Factors Guidelines (EBA/GL/2021/02) Guidelines 1-4; FinCEN CDD FAQ (April 2018); UK FCA PEP guidance FG25/3 (July 2025) |
| Enforcement lessons | File 20 (sections 2.2 US Bancorp and 2.4 HSBC carry CRR findings) | FSA June 2011 thematic review; FCA final notices to Starling (27 Sep 2024), Barclays Bank plc (14 Jul 2025) and Nationwide (11 Dec 2025) |
| Model governance, explainability | File 29; file 05 section 1c | How the rating fits the two governance tracks (section 11) |
| Vendors | Files 08 (NICE Actimize), 09 (Oracle FCCM), 16 (KYC/CLM, being written) | A rating-focused read of Fenergo, NICE CDD-X, Oracle KYC and Moody's (section 12) |

Not reachable in this session: the FATF website (blocked), the FFIEC manual (blocked, see file 05), the AMLA draft RTS on CDD, and further web searches (the session's search budget ran out after the Islamic-banking research).
Items resting on memory rather than a document read are marked "unverified".

---

## 2. Customer risk rating methods

### 2a. What a rating is for

Every standard read for this file gives the rating the same job: it **decides how much due diligence is done, how closely the account is watched, how often it is reviewed and who must approve**.
- EBA: the extent of CDD, the level of monitoring and the review date follow from the risk assessment (EBA/GL/2021/02 paras 1.19, 4.7, 4.71).
- FATF 2014 banking guidance para 60: from a holistic view of CDD information "banks should be able to prepare a customer risk profile. This will determine the level and type of ongoing monitoring and support the bank's decision whether to enter into, continue or terminate, the business relationship." Para 63: "banks therefore have to draw up, and periodically update, customer risk profiles".
- FSA 2011 para 80: the category "usually determined the level of CDD required; the frequency at which the relationship was reviewed; and the level of seniority required to give approval for entering into the relationship and for signing off account reviews".
- CBI controls 2025 (file 01): the AML system must classify customers by cash flows, nature of business, geography, products and channels, High, Medium or Low (five levels allowed), and must update the classification dynamically at each interaction.
- FinCEN (April 2018 FAQ Q35): the US rule requires a customer risk profile as a baseline for monitoring, but "The profile may, but need not, include a system of risk ratings or categories of customers". The US does not mandate a rating; Iraq, the EU and the UK expect one.

### 2b. Method families

| Family | How it works | Strengths | Weaknesses and regulator signals |
|---|---|---|---|
| **Rule-based categories and knock-outs** | A customer type or fact forces a tier (PEP is high; listed company is low) | Simple, explainable, matches legal "always high" cases | Cannot rank within a tier; blunt. EBA 3.6: "provisions... regarding situations that always present a high money laundering risk cannot be over-ruled by the firm's weighting" |
| **Weighted additive scoring** | Each factor gets points, points are weighted and summed, bands give tiers. This is what most banks use. FSA 2011 para 79: "The overall risk score was usually generated by adding together scores (or taking an average) for different AML risk factors" | Transparent, tunable, auditable | Arbitrary weights; dilution (many low factors hide one serious one); gaming; weights equal when some factors "appeared to give rise to more risk" (FSA para 83) |
| **Risk matrices** | Cross-tabulation of customer type by product by geography to a tier | Easy to communicate; the CBI asks for "a detailed risk matrix" covering high-risk customer types (file 01) | Matrix cells set by judgement; can allocate "inappropriate low-risk scores to high-risk jurisdictions" (FSA 2011 executive summary para 11) |
| **Hybrid: knock-outs, floors, weighted score, overrides** | Hard rules set minimum tiers; a score sets the rest; reviewed overrides adjust | Meets EBA 3.6 on all points; recommended | Needs governance of three layers |
| **Statistical and machine learning** | Supervised models trained on outcomes (STR filed, exit for ML reasons, confirmed alerts), unsupervised segmentation and anomaly scores for behaviour | Can find patterns rules miss; peer groups; NICE CDD-X and Oracle KYC market "AI-driven segmentation" and "deep learning" (section 12) | Labels are scarce and biased by past monitoring; Iraqi banks filed 628 ML STRs in five years (file 01 section 5), too few to train a supervised customer model; explainability duty (EBA 3.7; Wolfsberg AI/ML principles 2022, file 04 section 2e); under the April 2026 US guidance a scoring model is a "model" while a deterministic rule set is not (file 05 section 1c) |

### 2c. Static versus dynamic

- **Static at onboarding** answers "who is this and what do they say they will do". It is what Nationwide's interim system did: unless a customer "fell into certain limited categories... they were automatically classed as standard risk" (FCA Final Notice, 11 Dec 2025, para 2.7(a)).
- **Dynamic** adds what the customer does and what the bank learns. The FATF 2014 guidance (para 66) describes monitoring as "identifying changes to the customer profile (for example, their behaviour, use of products and the amount of money involved), and keeping it up to date, which may require the application of new, or additional, CDD measures".
  Wolfsberg (2022 digital lifecycle, file 04 section 2q) asks for a "dynamic, holistic customer risk profile" and trigger-based refresh. The CBI wants dynamic updating "with each customer interaction" (file 01 section 2d).
- **The failure the regulators keep finding is a dynamic design that does not react**: Barclays rated Stunt & Co "low" for most of 2015-2021 and "failed to update it in light of relevant information" including police raids, production orders, a law-enforcement request and a frozen account (FCA Final Notice 14 Jul 2025 paras 2.9(a)-(h); section 13).

### 2d. Choices the plan should state plainly

1. **Hybrid method**: knock-outs and floors for legal and regulatory certainties, a transparent weighted score for everything else, a governed override layer (section 14).
2. **Rating decides controls**: tier maps to CDD depth, approval level, review interval, monitoring intensity and product limits, and those mappings are tested daily (the Starling lesson: controls tied to risk status failed silently, 54,359 accounts opened for 49,183 high or higher-risk customers in breach of a restriction, FCA Final Notice 27 Sep 2024).
3. **Machine learning stays out of the customer rating at first.** It supplies behavioural inputs (anomaly score, peer-group deviation) into one block, never the final tier, until the bank has enough labelled outcomes and a validation function.
4. **Unknown is not low.** Missing factors count as higher risk and cap the best achievable tier (section 11).

### AKR-TMS hooks (section 2)

- A rating engine with three layers (knock-outs and floors, weighted factor blocks, override layer), each configurable, versioned and testable.
- A **rating-to-controls matrix** (tier to CDD depth, approver, review interval, monitoring profile, product limits) with a daily conformance test against the CBS data (for example, a Level 4 customer without a current senior approval is a daily exception).
- A **model register** with the CRR configuration as governed rules (change control, rationale, test results) and any ML component as a model with model card and validation pack (file 29).

---

## 3. Risk factors

Four families appear in every standard: customer, geography, product or service, delivery channel (CBI controls 2025 name "customer, product, delivery channel, geography"; EBA Guideline 2; AMLR Annex I; FATF INR.10; Wolfsberg RBA 2026).
The newer texts add a fifth family that the CBI also asks for: **behaviour over time** (CBI "dynamic update"; Wolfsberg RBA 2026 "What the customer does"; NICE CDD-X "customer behavior").

### 3a. Customer factors: who the customer is

| Factor | What the sources say | Iraq note |
|---|---|---|
| **Customer type and structure** | EBA: is the structure "transparent and does it make sense"; bearer shares, nominee shareholders, "asset-holding vehicle"; AMLR Annex III(1): personal asset-holding vehicles, nominee shareholders or bearer shares, "unusual or excessively complex" structure, entity with "no real economic activity"; Wolfsberg RBA 2026: consider "the entity's formation date, as newly established entities may pose higher risk than those that are well-established"; listed companies carry "reduced financial crime risk" | MER 2024 (para 695, file 02): LLCs and foreign-company branches are the medium-risk legal forms; residual TF risk highest for LLCs and foreign branches. 105,241 of 118,639 registered companies are LLCs (end-2022). No bearer shares exist for lack of a legal text, and the Securities Commission barred dealing in bearer negotiable instruments in 2020 (para 702). |
| **Business or occupation** | EBA: sectors "commonly associated with higher corruption risk, such as construction, pharmaceuticals and healthcare, the arms trade and defence, the extractive industries or public procurement"; cash-intensive sectors; "certain Money Service Businesses, casinos or dealers in precious metals". AMLR Annex III(2)(e): "oil, arms, precious metals or stones, tobacco products, cultural artefacts". Wolfsberg RBA 2026: "cash intensive retail and personal-service businesses, money service businesses, and third-party payment processors" | Iraqi NRA (MER para 695): the legal persons most at risk are **import companies, companies with public-sector contracts, real estate investment companies, customs clearance companies, consultancies, oil companies, and recreational businesses (malls, restaurants, cafes, gas stations)**. CBI controls 2025 add real-estate dealers, precious-metals dealers, car dealers, any DNFBP, NPOs, cash-intensive activity, money transfer and exchange companies, antiques and art. Oil-derivative smuggling and antiquities trafficking are named predicate threats (file 02). |
| **PEP status** (customer, beneficial owner, signatory, relatives, associates) | EBA 2.x: political connections "of the customer or beneficial owner", directors who are PEPs; AMLR art. 20(1)(g) and 42; FATF R.12 | CBI circular 26 Aug 2026: relatives to the second degree, associates, persists after office (file 01 section 2c-B); MER: banks rely on staff knowledge of "the family name and its connection to political figures" (para 523-563 summary, file 01) |
| **Reputation** | EBA: adverse media (with a credibility test), frozen assets, "subject of a suspicious transactions report in the past", in-house integrity information; AMLR Annex I(a)(ii) | Arabic and Kurdish local media; FIU and court requests; CBI circular requires daily local and regional news screening for PEP circles |
| **Nature and behaviour at onboarding** | EBA: reluctance to share CDD information, unreasonable secrecy, seeking one-off transactions to avoid a relationship, doubts on identity | Same as CBI Art. 12 customer-behaviour indicators (file 27) |
| **Source of wealth and funds plausibility** | EBA: "Can the customer's... source of wealth or source of funds be easily explained... Is the explanation plausible?" | MER: banks have "no clear mechanism to verify" PEP source of wealth (file 01) |
| **Identity assurance** | FATF digital ID guidance (file 03 F-22); Wolfsberg 2022: strength of the initial authentication event | National card with chip versus legacy ration or residence card; family number; branch with original document versus remote (file 01 section 7) |
| **Expected activity declared** | CBI: compare actual with KYC-declared flows (file 01); FinCEN FAQ Q36: build "a baseline against which customer activity... can be assessed" | Declared monthly income, expected cash, countries; often free text or missing |

### 3b. Geography: the sources of country risk and what Iraq scores

| Source | What it gives | Iraq reading (2026-10-06) |
|---|---|---|
| **FATF lists** (call for action; increased monitoring) | A status with dates; EBA 2.x and AMLR Annex III(3)(a) treat it as a higher-risk factor | Iraq added to the increased-monitoring list on **19 June 2026** (file 01 section 1). Kuwait, Lebanon and Syria are on the same list; Iran and DPRK under call for action (file 05 section 3a) |
| **EU list of high-risk third countries** (Delegated Regulation 2016/1675 as amended) | Mandatory EDD for EU obliged entities | Iraq was listed from 2016 and removed in early 2022; the Commission page last updated 9 January 2026 shows the Dec 2025 amendments (2026/46, 2026/83) and no Iraq entry; no re-listing found as of 2026-10-06 (file 05 section 2e). The CBI deck lists the 2021 UK and EU exits as achievements |
| **UK high-risk third countries** | UK MLR; grey-listed states are a "relevant geographical risk factor", mandatory EDD narrowed to call-for-action states from 30 June 2026 | Iraq counts as a UK HRTC under the definition since June 2026 (file 05 section 3a) |
| **Basel AML Index 2025 Public Edition** (Basel Institute on Governance; data collected to 10 Nov 2025) | Score 0 (low) to 10 (high risk) from 17 indicators in 5 domains: AML/CFT/CPF framework 50% (of which FATF MERs and follow-ups 35%), corruption and fraud 17.5%, financial transparency and standards 17.5%, public transparency 5%, political and legal risks 10%. Expert Edition bands: lower below 4.70, medium 4.70-6.08, higher above 6.08 | **Iraq 5.90, rank 48 of 177** (medium band, just under the 6.08 higher-risk line). Neighbours: Kuwait 6.13 (39th), UAE 6.11 (40th), Lebanon 5.93 (47th), Saudi Arabia 5.87 (50th), Türkiye 5.65 (62nd), Jordan 4.85 (112th). Iran and Syria are not in the public ranking. The Index itself warns that "jurisdiction risk models cannot be used as a solid basis for prediction" and relies partly on perception indicators |
| **Transparency International CPI 2025** | Score 0-100 (100 clean), rank of 182 | **Iraq 28, rank 136**, up 2 on 2024 |
| **UN, OFAC, EU sanctions programmes** | Programme-level country risk and nexus (file 04 section 4; file 05 section 1e) | Iran restored UN measures 27 Sept 2025; OFAC Iraq-nexus actions 2025-26; 1518 regime (file 04 U-07) |
| **CBI and AML/CFT Office lists** | Local circulars of high-risk countries (file 01 section 2d chapter 8: green, grey, red, black) | Needs a maintained table per bank |
| **MERs and NRAs** (EBA lists FATF and FSRB MERs, IMF assessments, FSAPs) | Quality of the regime | Iraq MER 2024: technical compliance strong, effectiveness Moderate or Low (file 01) |

How Wolfsberg says to use them (Country Risk FAQs 2024, file 04 section 2l): inputs normalised "into a numeric point scale" and combined as "a weighted combination of inputs", ratings updated "no less frequently than annually" and on triggers, exceptions given "an expiration date", and "unknown information is a priori a risk factor" (Q11).
The FSA found the opposite practice: "One bank considered several higher risk countries as 'low risk' because they had 'lots of dealings' with them" (para 83).

**The Iraq-specific problem.** Nearly every customer of an Iraqi bank is an Iraqi resident, so "Iraq is grey-listed" adds the same points to everyone and discriminates nothing. It belongs in the bank's **calibration and enterprise risk assessment**, not in per-customer scores.
Per-customer geography should measure the **foreign nexus**: residence or nationality outside Iraq, foreign ownership, registration abroad, and the **corridors actually used** (counterparty countries in the last 12 months), plus the CBI's own named factors: nationality, residence, work country, registration country, and "customers of branches in border areas" (file 01 section 2d).
Sub-national residence (for example former ISIL areas) should not be a customer-level factor because it penalises residents, not behaviour; activity toward conflict-area counterparties is the risk-based version.

### 3c. Products, services and transactions

- **Sources**: EBA Guideline 2 (transparency, complexity, value or size), AMLR Annex I(b) (purpose of an account, regularity or duration, level of assets, transparency, complexity, value), Annex III(2) (private banking, products favouring anonymity, payments from unknown third parties, new products, oil and arms and precious-metal transactions).
- **CBI product list** (file 01 section 2d): savings accounts, transfer products, correspondent payable-through accounts, digital transfers, stored-value cards, products linked to DNFBPs, gold bullion; monitor external-transfer counts, accounts opened per year, cash risk and correspondent complexity.
- **Iraqi specifics**: public-payroll salary-localisation accounts (low value, rule-bound), the national electronic card and e-wallets (file 01 section 9), safe-deposit lockers, LC and LG trade finance, FX sales, remittance agents, and Islamic products (file 24: import murabaha, commodity murabaha, qard hasan).

### 3d. Delivery channel

EBA: "the extent to which the business relationship is conducted on a non-face-to-face basis" and introducers or intermediaries; the CBI lists non-face-to-face and technology-enabled transactions as a transaction risk factor.
Wolfsberg 2022 (file 04 section 2q) says non-face-to-face can be "a standard or even lower-risk channel" with strong authentication. The factor is therefore **onboarding assurance level** (branch with original chip card; agent; remote with liveness; remote without), not "remote equals high".

### 3e. Behaviour and experience (dynamic block)

Wolfsberg RBA 2026: "repeated and/or unusual amendments to key customer profile information, particularly where there is no clear business rationale, may be an indicator of risk, including evasive or obstructive conduct, unexplained delays in responding to customer outreach"; unusual product use; corridors "that do not align with the customer's known profile".
EBA para 4.77: capture "an apparent change in the source of the customer's funds, the customer's ownership structure, or behaviour that is consistently out of line".
The rest of the dynamic inputs come from AKR-TMS itself: alerts and their outcomes, STRs, requests from authorities, shared identifiers (graph), expected-versus-actual deviation.

### 3f. What the CBS can supply (read-only), and what it usually cannot

| Factor | Usually in the CBS? | Gap handling |
|---|---|---|
| Customer type, ID type and number, nationality, residence, date of birth, family number | Yes | Validate Iraqi formats (file 01 section 7) |
| Occupation and business activity | Free text or coded loosely | Map to a taxonomy; show coverage |
| PEP flag | Often a manual flag, often wrong or empty | Bank-supplied register plus screening (section 9) |
| Beneficial owners and shareholders | Partial; often a scanned form | BO model with verification status (section 8) |
| Product holdings, balances, turnover, channels | Yes | Cross-business-line view (US Bancorp lesson, file 20 section 2.2) |
| Declared income and expected activity | Weak | Capture at refresh; mark "not declared" explicitly |
| Source of wealth documents | Scans in a document system | Evidence vault link, corroboration status |
| Adverse media, sanctions and PEP screening results | External or screening system | Screening module (file 04, file 22) |
| STR history, authority requests, branch liaison observations | Not in the CBS; in the case system | AKR-TMS case data |

### AKR-TMS hooks (section 3)

- **Factor catalogue as configuration**: each factor has an ID, value list, points, weight, data source (CBS table and column, screening, case system, external), freshness limit and legal basis (CBI, EBA, AMLR, Wolfsberg, MER).
- **Data-readiness view per bank**: for each factor, the share of customers with a usable value, so leadership sees which factors are blind (the CBI May 2026 deck counts 380 ACAMS-certified and 22 CGSS-certified specialists working in banks, so people are being trained; whether the data supports them is the open question).
- **Country risk table** (W-19) with FATF status, CBI lists, sanctions programmes, Basel AML Index, CPI and nexus flags, versioned with effective dates; **Iraq itself excluded from per-customer points** and handled in calibration.
- **Fairness rule**: names, tribe, religion, sect and region of origin are never rating factors; Iraqi name parts are used only for matching (file 22).
- **Iraqi at-risk activity list** from MER para 695 and CBI controls as a reference table the occupation taxonomy maps into.

---

## 4. What FATF, Wolfsberg, Basel and the EU say about customer risk rating

Details are in files 03 and 04; this section keeps only the statements that shape the rating design, with the new verbatim text read for this file.

| Point | FATF | Wolfsberg | Basel (BCBS 2020) | EU (EBA 2021, AMLR 2024) |
|---|---|---|---|---|
| **Purpose of a rating** | 2014 banking guidance para 60: the profile "will determine the level and type of ongoing monitoring"; profiles can apply per customer or to "groups of customers [that] display homogenous characteristics" (retail) | CBDDQ guidance Q67: "Failure to assign a risk rating to customers appropriately could lead to a portfolio of higher risk customers that exceeds your Entity risk tolerance" | Para 38: profiles per customer or category; changes in behaviour must update the assessment | EBA 3.1-3.9: holistic view; AMLR art. 20(2): extent of CDD set by "an individual analysis of the risks" |
| **Factors** | INR.10: customer, country, product, channel (file 03) | RBA 2026: "who the customer is" and "what the customer does" | Para 32: background, occupation, source of income and wealth, country, products, linked accounts | EBA Guideline 2; AMLR Annex I variables, Annex III higher-risk factors |
| **Weighting** | Silent on weights | Country risk FAQs: weighted average of normalised inputs, documented | Silent | EBA 3.6: not "unduly influenced by just one factor"; "economic or profit considerations do not influence the risk rating"; weighting must not make it "impossible for any business relationship to be classified as high risk"; legal always-high cases cannot be overruled; automated scores can be overridden with documented rationale |
| **Bought systems** | New technologies report: understand and verify (file 03 F-20) | AI/ML principles 2022 (file 04) | Para 29: calibrate vendor defaults (file 04 B-02) | EBA 3.7: if bought, "understand how the system works and how it combines or weights risk factors... and be able to demonstrate this to the competent authority" |
| **Segmentation** | 2014 para 69: "document and state clearly the criteria and parameters used for customer segmentation and for the allocation of a risk level for each of the clusters" | Same, via Demonstrating Effectiveness | Para 29: category profiles | EBA Guideline 4: monitoring intensity by risk |
| **Review and triggers** | INR.10 para 10 and 23 (file 03 F-07) | CBDDQ Q69: a trigger event "may cause an immediate review... a change in ownership, adverse media/negative news event, a sanctions or transaction monitoring investigation, material FC risk, a change in the products and services used by the customer or the customer moving domicile"; Q74 asks if KYC is renewed "at defined frequencies based on risk rating"; Q75 asks for metrics on periodic and trigger reviews | Para 49: MI shows stale CDD and behaviour changes | AMLR art. 26(2)-(3): update interval depends on risk and never exceeds **1 year for higher-risk (art. 34 EDD) customers and 5 years for all others**; reviews also on "a change in the relevant circumstances", a legal duty to contact the customer, or becoming "aware of a relevant fact" |
| **Inclusion** | 2014 Box 4: "financial exclusion in itself is not an indicator of low ML/TF risk" | RBA 2026: risk appetite must take financial inclusion into account | Para 33: policy "not so restrictive that it denies the general public access" | EBA 4.9-4.10: de-risking whole categories is not required; basic products as mitigation |
| **PEPs** | R.12; 2013 guidance (file 03) | 2017: risk-based for all PEPs; declassification documented (file 04 section 2n) | Para 34: EDD and senior approval | AMLR art. 42, 45 (minimum 12 months after leaving office), 46 (family and associates) |

Four further EU facts relevant to Iraqi banks that correspondents will apply:
- **AMLR applies from 10 July 2027** (art. 90) and AMLA must issue guidelines on risk variables and factors by 10 July 2026 (art. 20(3)) and on ongoing monitoring by 10 July 2026 (art. 26(5)); the AMLA guidelines and RTS were not reached in this session (gap).
- **Beneficial ownership**: ownership of "25 % or more" direct or indirect, calculated by multiplying along the chain, tested **in parallel** with control by other means; the Commission may set a lower threshold, "in general not... more than 15 %", for higher-risk categories (art. 52, recital 107).
- **AMLA draft CDD technical standards** (second consultation closed 8 May 2026; as summarised by NICE Actimize, a secondary source; the AMLA text was not read): simplified due diligence only after a risk assessment that checks "a suite of high-risk factors"; **complex ownership structures defined as three or more layers between customer and beneficial owner plus at least one further condition**; country of birth as a mandatory data field; identification of ownership or control "via indirect or non-traditional means"; both periodic and event-triggered reviews; existing customers to comply by their next periodic review. The three-layer test is reflected in factor A3 (section 14d); place of birth is on the Iraqi national card.
- **Enhanced due diligence list** (art. 34(4)): additional information on the customer and beneficial owners, on the nature of the relationship, on source of funds and wealth, on reasons for transactions, senior management approval, enhanced monitoring, and first payment from an account in the customer's name at a bank under comparable standards; plus special measures for relationships above EUR 5 million with customers holding EUR 50 million or more (art. 34(5)).

### AKR-TMS hooks (section 4)

- A **requirements-to-features matrix** per regime (CBI, FATF, EBA/AMLR, Wolfsberg) so a bank or a correspondent can see where each rule lands in the rating engine (supports W-06, W-17).
- A **segmentation register**: documented criteria for each customer segment and its tier allocation (FATF 2014 para 69).
- **EU-style intervals as a selectable profile**: higher risk one year maximum, others five years, for banks with EU correspondents.

---

## 5. What Iraq requires

Read from file 01 (sections 2b, 2c-B, 2d); no new Iraqi primary text beyond the MER and the CBI May 2026 deck.

| Rule | Content | Source |
|---|---|---|
| Risk classification | Customers and products classified **High, Medium, Low** (five levels allowed) on **customer, product, delivery channel, geography**, from each customer's cash flows, nature of business, location, products and channels | CBI controls 2025 ch. 2 art. 2; ch. 5 art. 5 |
| Risk matrix | A "detailed risk matrix that the system relies on, covering high-risk customer types (e.g. DNFBPs, PEPs, NPOs, non-residents)" | CBI controls 2025 ch. 2 art. 2 (3) |
| Customer risk factors | Beneficial owner hard to identify (complex ownership), tarnished reputation or past dealings, non-residents, PEPs and associates, foreign customers, non-resident holders of other nationalities, real-estate dealers, precious-metals dealers, car dealers, any DNFBP; EDD where a customer may be an NPO linked to terrorism, is sanctioned or on lists circulated to banks, or is a PEP | same |
| Approval | "No relationship with NPOs or EDD customers without senior executive management approval after EDD is complete"; PEPs need senior approval before the relationship starts | same; Instructions 1/2023 art. 25 |
| Link to the CBS | The AML system must be directly linked to the CBS ("Automatic Integration") and to the screening platform; updates the **customer risk records "with each customer interaction"** | CBI controls 2025 ch. 5 arts. 4-5 |
| Review of classification | Review each customer's risk classification **at least every two years**, or sooner on change | CBI controls 2025 ch. 2 art. 2 (1) |
| KYC form update | MLRO duty (n): "update the KYC form periodically, **at least once a year**" | CBI controls 2025 ch. 3 (MLRO duties) |
| Practice at banks | High-risk customers refreshed yearly, medium every two years, low every three years | MER 2024 (IO4, file 01) |
| Declared-versus-actual | Compare actual volumes with what the customer declared at KYC | CBI controls 2025 ch. 5 art. 5 (5) |
| Simplified due diligence | Ministries, government bodies, FIs under comparable rules, occasional or digital transfers **below IQD 1 million**; never where ML/TF is suspected or linked transactions seem built to stay under the line; the bank keeps evidence of low risk | CBI controls 2025 ch. 5 art. 3; Instructions 1/2023 arts. 19, 21 |
| Beneficial owner | **20%** direct or indirect, else control by other means (personal ties, close family relationships, historical ties), else senior managing official; banks in practice look at **10%** holders at the registry | Instructions 1/2023 art. 7; MER IO4 |
| PEPs | CBI circular of 26 Aug 2026: EDD for PEPs, high-risk position holders and their **relatives to the second degree**; check applicant, beneficial owner, signatory and related persons before and throughout; daily local and regional news screening; senior approval; classification **persists after office** until a documented assessment; effectiveness KPIs a-g; red flags a-h | file 01 section 2c-B |
| New products | Risk-assess before launch; CBI approval for new products and channels | Instructions 1/2023 art. 20; MER |
| High-risk countries | EDD, review of correspondents, source of funds and wealth, periodic refresh for foreign customers, senior approval, enhanced monitoring, using green, grey, red and black list colours | CBI controls 2025 ch. 8 |
| Independent check | AML tools verified by a CBI-approved firm | CBI standards handbook 2025 (file 01 section 2e) |

### 5a. Gaps and conflicts that the product must handle, not hide

1. **Annual versus two-year review.** The MLRO duty says KYC form updates "at least once a year"; the classification review says "at least every two years"; banks apply 1, 2 and 3 years by tier. The product needs two clocks per customer: **classification review** (default: 1 year for High, 2 years for Medium and Low, because the written rule caps every customer at 2 years, so the 3-year practice for low-risk customers would breach it) and **KYC data refresh** (annual for High and Very High; for others a targeted refresh driven by staleness and triggers, with the bank's chosen interpretation recorded). The bank should ask the CBI for a written reading of the annual duty; until then the interpretation is a documented policy decision (item for the plan, not for the engine).
2. **Threshold mismatch**: the law says 20%, practice looks at 10%, FATF examples say 25%, the EU says 25% with lower thresholds up to 15%. Configure the threshold per customer type and risk tier; default to the legal 20% and let banks go lower for higher risk (section 8).
3. **No Iraqi PEP list**: the circular requires a register but publishes none; the PEP Controls No. 2 of 2023 are not public (file 01). The product ships a **bank-maintained PEP register** with lifecycle states (section 9).
4. **CBI-named customer types as "high-risk"** (DNFBPs, PEPs, NPOs, non-residents) versus FATF R.8 and the proportionality revision of February 2025, which say not every NPO is high risk. The model sets the CBI-named types as defaults at High with a governed exception route backed by a written assessment (section 14).
5. **Rating can be written to the CBS in other markets, not here.** Vendors push scores back (NICE's deployment runbook shows a nightly delta file that a T24 job picks up: "the extracted scores are stored into a table... compare with the previously assigned score. If there is a change... the delta entries are persisted onto a file... T24 will then pick this delta file and update"). AKR-TMS never writes to the CBS, so the rating lives in AKR-TMS and the bank decides whether to feed it into its own system (section 14).

### AKR-TMS hooks (section 5)

- **Regulatory profile "Iraq-CBI"** that sets default tiers, intervals, SDD ceiling (IQD 1 million), BO threshold (20%), PEP scope (second degree, persists), review clocks, and the named high-risk customer types, all editable by the bank with a recorded rationale.
- **Two clocks** per customer: classification review and KYC refresh, each with due date, owner and status; "CBI rule referenced" shown on every due item.
- **Divergence view** between the CBS risk field (if any) and the AKR-TMS rating, to support the CBI inspection and the bank's own reconciliation, with a rating-change feed the bank's integration team can consume (no write-back by AKR-TMS).

---

## 6. Review cycles and event-driven triggers

### 6a. Calendar cycles by tier: what each regime says

| Regime | Rule | Note |
|---|---|---|
| Iraq (CBI controls 2025) | Classification review at least every **two** years; MLRO duty to update the KYC form at least **yearly**; "periodic review... especially of high-risk customers" | Conflict explained in section 5a |
| Iraqi bank practice (MER IO4) | High yearly, medium every two years, low every three years | Matches the two-year written rule only for high and medium |
| EU AMLR art. 26(2) | Update interval "dependent on the risk" and never more than **1 year** for higher-risk customers under the EDD section, **5 years** for all others; plus reviews on change of circumstances, a legal duty to contact the customer, or awareness of a relevant fact | Applies from 10 July 2027 |
| FinCEN CDD rule (FAQ Q14, April 2018) | "Covered financial institutions do not have an obligation to solicit or update beneficial ownership information as a matter of course during regular or periodic reviews, absent specific risk-based concerns"; updating is **event-triggered** | The US relies on events, not calendars |
| Wolfsberg CBDDQ Q74-Q75 | KYC "renewed at defined frequencies based on risk rating"; metrics on periodic and trigger reviews so senior management sees "delays, back logs" | Correspondents will ask Iraqi banks for these metrics |
| Barclays policy (FCA notice 2025 para 4.44) | Had the customer been rated high at onboarding, "the relationship would have been subject to annual KYC reviews under Barclays' policies" | A rating that is too low also turns off the calendar |

### 6b. Review quality: what went wrong in practice

- FSA June 2011 executive summary para 14: "Nearly half the banks in our sample failed to review high-risk or PEP relationships regularly. Relevant review forms often contained recycled information year after year, indicating that these banks may not have been taking their obligation to conduct enhanced monitoring of PEP relationships seriously enough." Correspondent reviews "were often clearly copied and pasted year after year with no apparent challenge" (executive summary para 16).
- FCA, Barclays (para 2.9(f)): "in January 2017, Barclays completed a KYC refresh... and certified that all necessary due diligence had been conducted. This was despite Barclays not having received responses to outstanding queries regarding Stunt & Co, Fowler Oldfield and the source of [the director's] wealth".
- FCA, Nationwide (para 2.7(b)): "Aside from customers already assessed as high risk (which in mid-2020 appears to have amounted to only some 2,000 customers)... Nationwide had no process for undertaking either periodic or event-driven reviews of a substantial proportion of its customer relationships", in breach of its own policies; its uplift of CDD reached only 888,618 customers (5% of the base) and "no refresh planned for back-book customers" (para 4.21-4.23).
- FCA, Starling (paras 4.20-4.21): a control that blocks re-onboarding of customers exited for financial crime reasons "had not updated correctly", so 294 such customers opened new accounts, 161 of whom had been the subject of a SAR.

### 6c. Event-driven refresh: the method and the evidence

Wolfsberg digital lifecycle 2022 (file 04 section 2q) gives the structure: break each risk factor into observable variables, map each to a data source, set a threshold, define a proportionate action; high-confidence triggers can refresh data without contacting the customer.
CBDDQ guidance Q69 lists trigger events. AMLR art. 26(3) lists legal ones. Fenergo describes event-driven reviews that "process non-material updates automatically based on policy" and trigger a review journey for material change; NICE CDD-X "reassesses the customer risk score when a material event is identified" with a periodic model as "a compliance backstop" (section 12).

**Catalogue for an Iraqi bank (what AKR-TMS can observe through the read-only link)**

| ID | Trigger | Data source | Default action |
|---|---|---|---|
| **Identity and profile** | | | |
| TR-01 | Change of shareholders, directors, signatories or beneficial owners | CBS change history; CRD and BO platform extract if the bank supplies it | Re-test BO and structure; refresh task within 30 days; re-score now |
| TR-02 | Repeated or unexplained changes to address, phone, email, employer or occupation (Wolfsberg RBA 2026) | CBS change history | Targeted refresh; flag as profile-amendment signal |
| TR-03 | Residence or nationality changes to a higher-risk country | CBS | Re-score; EDD if foreign nexus rises |
| TR-04 | ID document expired or invalid | CBS | Refresh task; restrict per bank policy after grace period |
| TR-05 | New product or channel in a higher-risk class (trade finance, FX, remittance, Islamic financing) | CBS product table | Re-score; product-specific EDD |
| TR-06 | Personal account used for business (turnover profile or counterparties of a trader) | CBS, behaviour | Re-classify as business; business CDD (Nationwide, para 2.10) |
| **Screening and media** | | | |
| TR-07 | New or changed PEP match on customer, BO, signatory, relative or associate; change of office (elections, cabinet change) | Screening, bank PEP register | PEP workflow: EDD, source of wealth, senior approval (section 9) |
| TR-08 | Sanctions near-match | Screening | Hold and escalate; freeze workflow if confirmed |
| TR-09 | Credible adverse media (ML, corruption, terrorism, fraud) | Media module (file 04 W-16) | EDD review within 10 working days |
| TR-10 | Country-risk table change (FATF plenary in February, June or October; new sanctions; CBI circular) | Country table | Re-score all affected customers; list exposure (W-19) |
| **Activity against profile** | | | |
| TR-11 | Declared-versus-actual breach (cash, turnover, countries) for N consecutive periods | AKR-TMS behaviour | Targeted refresh of expected activity; re-score |
| TR-12 | First transfer to or from a new higher-risk country or corridor | Transactions | Re-score; EDD if material |
| TR-13 | Turnover crosses the SDD ceiling (IQD 1 million) or the tier's expected band | Transactions | Remove SDD; full CDD (file 04 W-24) |
| TR-14 | Dormant account reactivated | CBS status | Re-screen; CBI rule: reactivation signed by branch manager and liaison officer (file 01) |
| TR-15 | Alert escalated, confirmed unusual, or STR filed | AKR-TMS cases | Rating review within 5 working days; STR floor (section 14) |
| TR-16 | Third-party funding or source-of-funds mismatch | Transactions, SoF record | Refresh SoF; corroborate |
| **Authorities and internal intelligence** | | | |
| TR-17 | FIU, court or prosecutor request, production order, suspension or freeze order | Case system | **Immediate** re-rating review and link to the related alerts and to all linked customers (Barclays: the monitoring team "were not made aware of the law enforcement request") |
| TR-18 | Branch liaison officer or staff internal report | Case system | Review; confidentiality and tipping-off controls (file 01 ch. 11) |
| TR-19 | Negative field-visit result | Visit log (MLRO duty g) | Re-rate; consider exit |
| TR-20 | Customer unresponsive or evasive to outreach | Task log | Raise risk; escalate on second failure (Wolfsberg RBA 2026) |
| **Network and data** | | | |
| TR-21 | Shared identifier (phone, email, device, address, family number) with a high-risk customer or an STR-linked party | Entity resolution graph | Review both parties; propagate flag |
| TR-22 | Link to a party newly rated High or Very High | Graph | Review of the linked customer |
| TR-23 | Mandatory factor missing or stale beyond its limit (for example source of wealth on a High customer older than 12 months) | Data-quality counters | Refresh task; best achievable tier capped (section 11) |
| TR-24 | Customer exited previously for financial-crime reasons opens a new account | Exit register against CBS new-account event | Block and escalate; EDD if exception approved (Starling) |
| TR-25 | Employee-linked account or insider signal (KYE) | HR link, CBS user logs | Compliance-head review (file 27 Art. 14) |
| TR-26 | Islamic-product signals (supplier is the customer, NPO flow breach) | IS pack (file 24) | Review; contract-level EDD |

### 6d. Perpetual KYC and the calendar

Vendors market "perpetual KYC" (Fenergo: "continuous monitoring of the client profile - identifying changes across relevant data sources, assessing their risk impact, and applying materiality"). The regulators keep the calendar as a floor: NICE keeps "a periodic risk model" as a backstop and AMLR sets a hard 1-year and 5-year ceiling.
For Iraq the sensible design is **triggers pull reviews forward, the calendar remains the backstop, and quiet low-risk customers get a light automated check** (ID valid, no profile change, activity inside the declared band, no hits) so analysts spend time where change happened (Wolfsberg 2022: "putting resources on priority risks in real time instead of fixed review cycles").

### 6e. Measures that show the cycle works

- Review timeliness by tier and branch, backlog age, share of reviews with new evidence versus "no change" (recycled-content test), trigger yield (share of triggers that changed the rating), time from trigger to review, and the CBDDQ Q75 metrics.
- **Analyst time**: NICE cites a Datos Insights 2023 report that "80% of analysts' time is spent gathering data and only 20% spent performing reviews" (the report concerns fraud and financial-crime case management; vendor-cited, not read). A review that opens with the new and changed facts already assembled is the main lever on review cost.
- **Daily conformance tests** (Starling lesson): every High customer has a current senior approval; no restricted-tier product opened for a restricted customer; exit-register checks fired; re-rating triggers closed within SLA.

### AKR-TMS hooks (section 6)

- A **governed trigger library** (TR-01 to TR-26) where each trigger records factor, variable, source, threshold, action, owner, SLA, version and calibration date (W-24).
- **Review workbench** that pre-fills what changed since the last review, blocks sign-off when mandatory evidence is missing, flags text copied from the previous review, and records the approver.
- **Two-clock scheduling** (classification review, KYC refresh) plus trigger-pulled dates, with ageing reports for the board and the CBDDQ evidence pack.
- **Exit register** of customers closed for financial-crime reasons, checked against new-account events (read-only, from CBS account-open data).

---

## 7. Enhanced due diligence, source of wealth and source of funds

### 7a. When EDD is mandatory or expected

- **Iraq**: PEPs and their relatives; customers that may be NPOs linked to terrorism, sanctioned persons, persons on circulated lists; business with FATF-listed countries; correspondent relationships; relationships the bank classifies as high risk; unusual or complex transactions (file 01, CBI controls 2025; Instructions 1/2023 arts. 22, 24, 25).
  Senior executive approval is required before an NPO or any EDD customer relationship goes ahead.
- **EBA 2021 (paras 4.45-4.47)**: always higher risk and always EDD: PEPs, correspondent relationships with third-country institutions, high-risk third countries, and "all transactions that are complex; unusually large; conducted in an unusual pattern; or without obvious economic or lawful purpose". EDD "cannot be substituted for regular CDD measures but must be applied in addition".

### 7b. The EDD toolbox (merged from FATF 2014 Box 3, EBA 4.45-4.51, AMLR art. 34(4), Wolfsberg)

| Measure | Source | Use in Iraq |
|---|---|---|
| Additional identifying information from "a wider variety or more robust sources" | FATF 2014 Box 3 | National card chip read plus family card; CRD extract for entities |
| Additional searches, verifiable adverse media | FATF Box 3; Wolfsberg negative-news FAQs | Arabic and Kurdish media (file 04 W-16) |
| Intelligence report on the customer or beneficial owner | FATF Box 3 | External provider or FIU channel where available |
| Verify source of funds and wealth | FATF Box 3; EBA 4.50; AMLR 34(4)(c) | See 7c |
| More information on purpose and intended nature | FATF Box 3; AMLR 34(4)(b) | Expected activity by corridor and counterparty |
| Information on reasons for intended or performed transactions | AMLR 34(4)(d) | Case notes |
| Senior management approval | EBA 4.50; AMLR 34(4)(e) | Level and name recorded |
| Enhanced monitoring: more controls, more often | EBA 4.50; AMLR 34(4)(f) | Tighter thresholds and a monitoring profile per tier |
| First payment through an account in the customer's name at a bank under comparable standards | AMLR 34(4)(g) | For foreign customers |
| Site or field visit | MLRO duty (g) in CBI controls; Wolfsberg CBDDQ Q68 (for high-risk non-individual customers; purpose "verifying that the customer does in fact have location and an operational business") | The MER records bank visits to company premises and warehouses |
| Limits and product restrictions | EBA 4.10 (mitigation by basic products) | Per-tier product matrix |

### 7c. Source of wealth and source of funds

Definitions (Wolfsberg SoW/SoF FAQs 2020, file 04 section 2h): **source of wealth** is how the customer's overall net worth was built (salaries, business profits, inheritance, property sales); **source of funds** is where the money in this relationship comes from. The aim is "legitimate and plausible", not an exact net worth, and corroboration comes from independent reputable sources.
What the supervisors found: FSA 2011 para 10, "Three quarters of the banks in our sample failed to take adequate measures to establish the legitimacy of the source of wealth and source of funds"; MER 2024, banks apply EDD to PEPs "without having a clear mechanism in place to verify the source of their wealth and that of their relatives" (file 01).
FCA guidance FG25/3 sets proportion: lower-risk PEP relationships may rely on "information already available to the institution (such as transaction records or publicly available information)"; higher-risk ones need "more intrusive and exhaustive steps".

**Iraqi corroboration sources to support in the evidence vault** (items 1 to 4 are in file 04 section 2h; the rest come from the MER, CBI documents and general practice):
1. Real estate registry (Tabu) extracts; 2. Company registry (CRD) extracts, including the BO declaration since July 2023 (section 8); 3. Salary letters from ministries (large public payroll); 4. Inheritance and personal-status court certificates; 5. Sale deeds and contracts; 6. Audited financial statements (the CBI lists "unaudited financial statements" as a credit-risk indicator);
7. Customs declarations and tax clearance for traders; 8. Statements from other banks; 9. Deferred-dowry (mahr) documents, which the CBI catalogue accepts as one justification for cash (file 27 Art. 3 no. 49); 10. For officials, asset declarations to the Federal Commission of Integrity (held by the FIU via goAML per the MER, not directly accessible to banks).

**Plausibility tests the product can run on bank data**: declared occupation and income against inflows; PEP wealth against the known position (CBI PEP red flag a: wealth "inconsistent with monthly income or known position as a government employee"); balance build-up rate; cash share; third-party funding.

### 7d. Conflicts that undermine EDD

FSA 2011 para 12: "At more than a quarter of banks visited, RMs appeared to be too close to the customer to take an objective view of the business relationship and many were primarily rewarded on the basis of profit and new business, regardless of their AML performance." Paras 7-8: around a third of banks "appeared willing to accept very high levels of money-laundering risk", over half "failed to apply meaningful enhanced due diligence"; a third "dismissed serious allegations about their customers without adequate review". EBA 3.6 says "economic or profit considerations do not influence the risk rating".
Barclays (2025) shows the same pattern: the director and sole shareholder was also a personal banking customer and the corporate relationship was rated low although his source of wealth was in question.

### AKR-TMS hooks (section 7)

- **EDD playbooks by customer type** (PEP, NPO and waqf, import and trading company, real-estate dealer, precious-metals dealer, car dealer, exchange and e-payment company, foreign-owned LLC, non-resident) with required evidence slots, approver level and review interval.
- **SoW/SoF record** per customer (W-15): categories, estimated net worth, expected annual inflows by source, evidence links, corroboration status (none, self-declared, document, independent), last verified date, and mismatch flags from actual flows.
- **Four-eyes rule**: the relationship owner cannot approve or lower the rating of their own customer; approvals visible by level; rating and EDD independent of commercial data (no revenue field is ever an input).
- **Field-visit log** (MLRO duty (g)) with photos and findings stored in the vault.
- **Public-sector pay-scale tables** (to be supplied by the bank or the Ministry of Finance) as a reference for plausibility tests on officials (hook; table not found).

---

## 8. Beneficial ownership: thresholds and the state of registers in Iraq

### 8a. Thresholds in the regimes an Iraqi bank meets

| Regime | Rule | Source |
|---|---|---|
| **Iraq, Instructions 1/2023 art. 7/First/B** | Natural person owning **20% or more**, directly or indirectly; failing that, control by other means ("personal ties, close family relationships or historical ties"); failing that, the senior managing official | File 01 section 2b |
| **Iraq, CRD Controls for declaring beneficial ownership No. 1 of 2023, art. 11** | **20%**, direct or indirect; companies declare to the Companies Registration Department from **July 2023** | MER 2024 paras 711-712 |
| **Iraqi bank practice** | Banks check the registry for holders of **10% or more**, and verify managers | MER IO4 (file 01) |
| **FATF** | A cascade: ownership, then control by other means, then senior managing official; the percentage is for the country (examples at 25%) | File 03 F-09 |
| **EU AMLR art. 52, recital 107** | **25% or more**, direct or indirect, "calculated by multiplying the shares... along the chain", tested **in parallel** with control by other means (not only when ownership fails); a lower threshold, "in general not... more than 15 %", for higher-risk categories | AMLR text read |
| **US CDD rule** | Each individual owning **25% or more** of the equity, plus one individual with significant control ("the ownership/equity and control prongs... are independent") | FinCEN CDD FAQ, April 2018 |
| **Wolfsberg CBDDQ Q66** | Asks the "lowest" ownership threshold the bank applies, defined as the one "applied to the highest risk customer rating"; "the higher the FC risk posed by a legal Entity customer, the more detailed will be the requirements necessary for understanding the legal Entity's ownership" | CBDDQ guidance v2.0 |

### 8b. What Iraq's registers can and cannot supply (MER 2024 paras 685-730; CBI deck of 18 May 2026)

- **Companies**: 118,639 national companies and foreign branches registered by end-2022 (105,241 LLCs, 432 joint-stock, 7,611 foreign branches, 3,911 sole proprietorships, 742 simple partnerships, 702 solidarity); 6,403 foreign persons were shareholders (para 696); 860 companies with foreign shareholders were formed in 2018-2022 (para 708).
- **Before 2023** companies were not required to keep BO information, and the CRD held none before 2022: "BO information was only available before 2022 through FIs and DNFBPs" (para 730). Banks were the register.
- **From July 2023**: CRD Controls No. 1 of 2023 require companies to identify and verify their beneficial owners and file a declaration; "The license application, amendment, or renewal cannot be accepted and processed unless it is accompanied by a declaration of the BO" (para 711). The AML/CFT Council published a BO guidance in 2023. The CRD must build a BO database, verify accuracy and update it; misleading information gets "a period not exceeding 15 days to correct", after which the declaration is invalid and Companies Law penalties apply (para 713). **No penalty had been applied** and "the AT was unable to test the effectiveness" (paras 712-713, 724).
- **Weaknesses recorded**: the CRD "does not include cases where ownership or control is through a chain of ownership or control by other means, that is, indirectly through a group of persons or front persons (strawmen)" (para 725); "the absence of a mechanism to verify the accuracy of BO information submitted by legal persons to the CRD may affect the effectiveness" (para 700); CRD verification "is not based on risks" (para 701); the NRA and sector assessments are silent on how legal persons are actually misused for ML (para 698); the MER rates R.24 and R.25 Partially Compliant and the FATF June 2026 action plan item 7 is "Strengthen oversight of beneficial ownership of companies" (file 01).
- **What the CRD publishes**: since August 2023 its Baghdad portal shows company name, capital, legal form, shareholder names and nationalities, number of shares each holds, authorised director and address; BO information and other documents are available to reporting entities **for a fee of about USD 4 within one to three days** (paras 719-720).
- **The bank's own role**: companies must open a bank account for start-up capital before registration completes and the bank notifies the CRD, "ensur[ing] that information related to the company's BOs is available to the bank" (para 714); banks verify the founders' funds. Authorities mostly get BO data from banks: LEAs obtain CDD information held by banks in an average of 8 days, "Banks maintain accurate and up-to-date BO information" (paras 728-729).
- **CBI platform**: the May 2026 deck lists "launch of the beneficial owner platform to identify the beneficial owners of legal persons established in Iraq" (2023) and **37,286 beneficial-owner statements received**. Against 118,639 companies registered at end-2022 that is about a third of that base, but the base has changed and the deck does not say how many companies are in scope, so it is not a coverage rate. Whether banks can query the platform directly was not found (gap).
- **Bank practice** (MER IO4): BO identified by self-declaration, then registry check at 10%, shareholder documents, financial statements; MER priority action (h) says banks "rely mainly on the customer's statement"; banks showed good understanding of control "through a group of people or front persons" and of doubt triggers: "repeated transactions for the benefit of natural persons or... distribution of profits for the benefit [of] persons who are not shareholders" (para 729).
- **Legal arrangements**: Iraq has no trusts; the only legal arrangements are the endowments (3,555 at end-2022), whose parties are the endower, trustee and beneficiaries (Instructions art. 7/First/C; file 24 section 7g).

### 8c. A BO process that fits these facts

1. **Declared**: signed declaration by the customer (with the family number and ID of each BO).
2. **Registry-matched**: shareholders and directors against the CRD portal; BO declaration requested for companies registered after July 2023 or amended since (fee, 1-3 days).
3. **Document-verified**: constitutional documents, share registers, board minutes, powers of attorney (what banks already do, MER para 729).
4. **Behaviour-tested**: payments and profit distributions to non-shareholders, signatories and agents who appear across unrelated companies, lawyers as repeat agents, shared addresses, phones and devices.
5. **Independently verified** (high risk): commercial ownership data (section 12), site visit, press.
Each beneficial owner carries a **verification status** (declared, registry-matched, document-verified, independently verified, unverifiable) and a last-confirmed date. "Unverifiable after reasonable steps" is a CDD failure under Law 39 art. 10/Fifth: do not open or continue, consider an STR (file 01 section 2b).

### 8d. Rating effects

BO identified and verified (or not), chain depth, foreign owners, PEP among owners, suspected nominees, frequency of ownership change, mismatch between declared BO and bank-observed behaviour, and presence of a CRD BO declaration (post-July 2023) are rating factors (block A, section 14).
Existing customers: the backlog of companies onboarded on a bare self-declaration is a **remediation population** with its own dashboard, because the register is new and unverified.

### AKR-TMS hooks (section 8)

- **BO and control model** with ownership percentages by route, calculation along chains (multiplication, aggregation by natural person), control by other means (family links through family number and name chain), senior managing official, verification status and last-confirmed date; threshold configurable per customer type and tier (default 20%, optional lower for High tiers, optional parallel-test mode for EU correspondents).
- **BO coverage and quality dashboard** by branch and segment: share of legal-person customers with all BOs identified, registry-matched, document-verified, and with a CRD declaration on file.
- **Hidden-ownership scenarios**: profit distribution to non-shareholders; recurring agents, signatories or lawyers across unrelated companies; shared identifiers; sudden ownership change before a large transaction.
- **Import path for registry data** (CSV or API from the CRD or the CBI platform if the bank gets access) with source, date and fee tracking; manual entry always possible.
- **Waqf and NPO parties** (trustee, endower, beneficiaries, controllers) in the same model (file 24 section 4).

---

## 9. PEP handling

### 9a. Definitions and duties compared

| Regime | Who | Duties | Post-office |
|---|---|---|---|
| FATF R.12 (file 03) | Foreign PEPs always higher risk; domestic and international-organisation PEPs risk-based; family and close associates | Risk systems, senior approval, source of wealth and funds, enhanced monitoring | Risk-based |
| **Iraq**: Instructions 1/2023 art. 25; CBI circular 26 Aug 2026; CBI PEP Controls No. 2 of 2023 (text not public) | PEPs and "high-risk position holders" and **relatives up to the second degree**, close associates; domestic and foreign treated alike by banks; check customer, **beneficial owner, authorised signatory and related persons**, before and throughout | Senior approval before the relationship; source of wealth and funds; enhanced monitoring; daily local and regional news screening; KPIs a-g; red flags a-h; undeclared PEP leads to precautionary measures and SAR review | **Does not end automatically**; ends or is downgraded only after a documented risk assessment |
| EU AMLR art. 42, 45, 46 | PEPs, family members and known close associates; Member States list prominent functions (art. 43) | Senior approval, source of wealth and funds, enhanced ongoing monitoring | EDD measures "in any case for not less than 12 months" after leaving office (art. 45(2)) |
| UK FG25/3 (FCA, 7 July 2025) | Only truly prominent roles; domestic PEPs, relatives and associates start as **lower risk** (Regulation 35(3A)); family includes spouse, children and spouses, parents, siblings; a legal entity "should not be treated as a PEP unless it is clear that the PEP exercises significant control" | Senior management defined by knowledge and authority, lower seniority for lower-risk PEPs; MLRO need not sign but must be aware | At least 12 months, longer only if assessed higher risk and documented; family of a former PEP are ordinary customers unless other risk |
| Wolfsberg PEP guidance 2017 (file 04 section 2n) | Risk-based for all PEPs; extend family where ethno-cultural or religious links justify; entities created for the PEP treated like the PEP | Pair-keyed false-positive memory, delta screening both ways | Documented declassification, flag retained |

**What does not transfer from the UK to Iraq.** The UK treats domestic PEPs as lower risk because of "strong state institutions", "a free press with a track record for probing official misconduct", "an independent judiciary", "well-developed registries for ownership of land, companies and equities".
The same FCA text lists as higher-risk country traits: "associated with high levels of corruption", "political stability" absent, "armed conflict", "a political economy dominated by a small number of people/entities with close links to the state", "weaknesses in the transparency of registries of ownership for companies, land and equities". Iraq scores on several of these (CPI 28, rank 136 of 182; MER on registries; file 02 on corruption as the first-ranked threat), so **Iraqi PEPs are higher risk by the FCA's own test**, which matches the CBI circular and the MER finding that banks treat domestic and foreign PEPs alike. The UK's lower-risk indicators (rigorous disclosure registers, no executive decision-making power) can still tier PEPs within Iraq.

### 9b. What supervisors found going wrong

FSA June 2011, high-risk customers and PEPs:
- "More than a third of banks visited failed to put in place effective measures to identify customers as PEPs. Some banks exclusively relied on commercial PEPs databases, even when there were doubts about their effectiveness or coverage. Some small banks unrealistically claimed their relationship managers... knew all PEPs in the countries they dealt with." (para 9). The CBI circular says the same in Iraqi terms: do not rely only on commercial lists; add daily local news.
- "A third of banks visited failed to give due consideration to certain political connections (eg wider family) which meant that, although certain customers might not fall within the Money Laundering Regulations definition of a PEP, they might still have need to be treated as high risk" (para 73); "a quarter of banks... appeared more concerned that a PEP might be involved in a public corruption scandal than that they might be corrupt" (para 74); "Informal, undocumented processes for identifying, classifying and declassifying customers as PEPs" is listed as poor practice.
- File 20: HSBC USA "did not appropriately designate customers as 'high-risk'... even where a customer's association with politically-exposed persons could have harmed the Bank's reputation" (section 2.4).
MER 2024: identifying relatives and associates "relies on the experience and knowledge of financial institution employees or directly asking the client"; no criteria for degree of kinship (file 01).

### 9c. PEP in the rating: factor, floor and sub-tier

A PEP is a **floor, not a refusal**: all four regimes say being a PEP is not wrongdoing (CBI: "not a presumption of wrongdoing"; FCA: "not... stigmatising"). The rating should therefore (1) set a minimum tier, (2) sub-tier PEPs by the FCA/Wolfsberg indicators, (3) drive senior approval and SoW/SoF, (4) feed monitoring.

| PEP sub-tier (proposal) | Examples | Floor | Notes |
|---|---|---|---|
| **P1** | Head of state, prime minister, ministers, members of parliament with executive or committee power over contracts, governors, judges of the highest courts, central bank board, heads of independent commissions, senior security and army commanders, heads of state-affiliated armed formations, heads of major state enterprises (oil, banks), party leaders | Very High (5) | Approval at the top level; enhanced monitoring review every 6 months |
| **P2** | Deputy ministers, directors general, provincial council members, mayors, senior officers, board members of SOEs | High (4) | Annual EDD |
| **P3: relatives and close associates** (second degree for P1 and P2; business partners, joint beneficial owners, persons with sole BO of an entity set up for the PEP) | | High (4); may be set one level below the principal only with a documented assessment | CBI: EDD applies to relatives and associates; FCA: family are not PEPs themselves |
| **Former PEP** | | Keeps the floor until a documented assessment ends or lowers it | CBI circular item 8: leaving office does not end the classification automatically; AMLR: not less than 12 months |
| **Entity with a PEP beneficial owner** | PEP holds 20% or more, or controls | High (4); a PEP owner below the threshold adds points and a flag | UK: not a PEP entity unless significant control; CBI: EDD on companies where PEPs or relatives hold shares or indirect control |
| **Position-based risk modifiers** (add points, can lift P2 to P1 treatment) | Authority over procurement, licences, concessions (FCA higher-risk personal indicators), unexplained wealth, credible allegations, products capable of "large-scale corruption" | n/a | From FG25/3 higher-risk indicators |
| **Lower-risk modifiers** | Rigorous public disclosure, no executive decision-making, low-value product | n/a | Can lower within P2 and P3 only; never below High for P1 |

The degrees of kinship: the CBI PEP circular says "second degree" without defining degrees. The CBI's digital-bank controls define degrees for related parties (first: parents, spouse, children; second: grandparents, siblings, grandchildren; third: uncles and aunts; fourth: cousins; file 01 section 2e). Treat that as the working interpretation and ask the CBI.

### 9d. PEP lifecycle and the register

States: **candidate** (match awaiting decision), **confirmed** (position, category, country of exposure, appointment date), **former** (left-office date), **downgrade proposed**, **downgraded or declassified** (with documented assessment, approver, date, prior flag retained for investigations), **deceased**.
Sources: customer self-declaration (signed, per the circular), bank-maintained Iraqi register (Presidency, Council of Ministers, Parliament, courts, governors, independent commissions, security leadership; file 04 section 2n lists the positions), commercial PEP data (Moody's states 7M+ PEPs across 200+ countries; Iraqi coverage must be tested), daily Arabic and Kurdish media, branch reports, and CBS family links (family number, name chain).
Both directions of screening (new PEP records against all customers; new customers against all PEPs) and pair-keyed discount memory (Wolfsberg 2017). When screening finds a PEP the front office missed, record why (FSA, Wolfsberg).
Undeclared PEP: precautionary measures on the account and SAR review (circular item 7).

**CBI KPI report (circular item 9, a-g)**: PEP customers by category; PEPs found by screening versus self-disclosed; relationships needing senior approval and number refused, terminated or restricted; number of data updates and re-assessments; SARs on PEPs and associates; internal-audit test results and remediation percentage; share of staff trained. AKR-TMS can produce all seven from its own data.

### AKR-TMS hooks (section 9)

- **PEP register module** with the lifecycle above, maker-checker, effective dates, position tiers P1-P3, and links to relatives (family number, name chain, declared relationships) with human confirmation (W-21).
- **PEP floors and modifiers** in the rating engine, with the sub-tier shown in the analyst's explanation.
- **Event link**: cabinet changes, elections and appointments (a calendar of expected office changes) trigger re-screening and downgrade proposals (TR-07).
- **CBI PEP KPI pack (a-g)** and a PEP portfolio dashboard for the board (counts, tier, products, balances, overdue approvals and EDD).
- **Undeclared-PEP workflow**: mismatch between self-declaration and screening opens a case with precautionary-measure options.

---

## 10. Overrides and their governance

### 10a. What the sources say

| Source | Statement |
|---|---|
| EBA/GL/2021/02 para 3.6 | Firms must "be able to over-ride any automatically generated risk scores where necessary. The rationale for the decision to over-ride such scores should be documented appropriately"; legal always-high cases "cannot be over-ruled by the firm's weighting" |
| FSA June 2011, para 83 and the poor-practice boxes 3.1.3 and 3.2.3 | "At some banks, RMs were able to override the risk score generated by the risk-assessment process without sufficient evidence to support their decision. At one bank, a customer who was the subject of allegations of corruption was classified as low risk simply because he came from a low risk country." Poor practice: "Allowing RMs to override customer risk scores without sufficient evidence to support their decision"; "Giving waivers from AML policies without good reason" |
| FSA 2011, para 83 | "Some banks' CDD files did not contain adequate documentary evidence... to show why customers were rated a high, medium or low risk. At one bank, we reviewed 13 retail accounts classified as high risk but there was no explanation." "One bank with many high-risk customer relationships changed the status of many relationships from low risk to high risk one month before our visit." |
| Wolfsberg Country Risk FAQs 2024, Q12 (on country-rating overrides; the logic carries over) | Manual overrides "should be strictly limited in number and subject to a stringent justification and governance process"; "Recurring overrides are typically an indication that, in some respect, the methodology is not performing as intended"; "The more discretion an FI exercises in overriding and the greater the number of employees/management permitted to exercise override authority, the greater the interference"; "Whenever there are high-risk factors present, the higher and more independent the override approval obtained for the decision, the better"; "Approved exceptions should be assigned an expiration date and be required to be reviewed at least annually" |
| CBI controls 2025 (file 01) | MLRO decisions to close a case must state reasons; no NPO or EDD customer without senior executive approval; the CBI inspects the system and the matrix |
| Barclays (FCA 2025), Nationwide (FCA 2025) | Ratings that were not revisited or defaulted to "standard" are the same failure from the other side: no one owned the question "is this rating still right?" |

### 10b. Override rules for AKR-TMS

1. **Floors cannot be overridden.** Legal and CBI-mandated minimums (section 14: PEP, named high-risk types, call-for-action nexus, sanctions near-match) can be changed only by changing the governed configuration, not per customer.
2. **Upward overrides are easy and fast** (any analyst, relationship manager, MLRO, branch liaison officer): reason code and note required, effective at once, reviewed at the next review.
3. **Downward overrides are hard**: made by compliance, never by the relationship owner of that customer; evidence attached; maker-checker; **expiry within 12 months** with automatic reversion and a task before expiry; limited to one tier per decision. Very High to High needs the MLRO and a senior executive; High to Medium the MLRO; Medium to Low a compliance officer plus a second approver.
4. **Reason codes** (examples): documented source of wealth corroborated independently; listed or regulated entity with public disclosure; government body; adverse media judged not credible with the source and date; country link explained and evidenced; false-positive identity resolved. "Relationship value", "senior introducer" and "customer complained" are not valid codes (EBA 3.6: no economic or profit considerations).
5. **Waivers** (for example accepting an older document) are a separate register with owner, expiry, compensating control and board visibility; a waiver never changes a rating.
6. **Bulk re-rating events** (calibration change, new country table) carry their own record and rationale and are shown separately from manual overrides (the FSA found a bank re-rating many relationships a month before the visit).
7. **Four-eyes on the approver**: the approver cannot be in the same reporting line as the requester for High and Very High.

### 10c. Metrics that show override health

Override rate by user, branch, segment and tier; share downward; average age; overrides expired and reverted; **overrides followed by an alert, STR or authority request within 12 months** (quality test); recurring overrides on the same factor (signals a bad weight, per Wolfsberg); share of High customers without a current approval; bulk re-rating events per quarter.
Quarterly report to the board and to internal audit, with the MLRO's commentary.

### AKR-TMS hooks (section 10)

- **Override register and workflow** with the rules above, reason-code taxonomy, evidence links and automatic expiry.
- **Override analytics**: the metrics in 10c as standing reports; thresholds configurable, breaches routed to the MLRO.
- **Conflict control**: the system knows the relationship owner and branch of each customer and blocks that user from lowering that rating.
- **Immutable history**: every rating state with score, tier, model version, configuration version, factors, overrides in force and approvers, so any past rating can be reproduced (section 11).

---

## 11. Calibration, validation, inflation, explainability and data quality

### 11a. Calibration and validation plan

Principles from the sources: weights must not make "high" impossible nor be dominated by one factor (EBA 3.6; FSA 2011 poor practice: "Inappropriate customer classification systems which make it almost impossible for a customer to be classified as high risk"); a bought or built system must be understood and demonstrable to the supervisor (EBA 3.7); vendor defaults must be calibrated to the bank (BCBS, file 04 B-02); the validator cannot be the owner (US Bancorp, file 20 section 2.2); a deterministic additive rule set is governed as configuration, any statistical score as a model (file 05 section 1c).

| Step | What is done | Output |
|---|---|---|
| 1. Specification | State what the rating is for (allocate controls by exposure) and the error costs: missing a High customer versus over-rating a Low one | Signed specification |
| 2. Initial configuration | Weights and points from regulatory defaults and expert judgement (section 14), every number with a rationale and a source | Configuration version 1 |
| 3. Shadow run | Rate the whole customer base in parallel with the bank's current ratings, no change to controls | Distribution by tier and segment; disagreement list |
| 4. Expert review of disagreements | Sample the cases where AKR-TMS and the bank differ by two tiers or more, plus a **golden set** of customers rated by experts (including the MER cases: the USD 21 million "chemical imports" company with state-cheque funding and a corruption link; the two-passports customer; file 01) | Adjusted configuration; documented decisions |
| 5. Outcome back-test | For customers who later produced confirmed alerts, STRs, FIU requests, freezes, exits or list matches, which tier were they in at the time? Report capture (share of outcome customers in High or Very High), lift and the share of Low-tier customers with outcomes ("escapes") | Back-test report. Labels are scarce (system-wide 82 to 672 bank STRs a year, file 01 section 5), so this is a **sanity check, not training data** |
| 6. Sensitivity | Move weights by plus or minus 20% and thresholds by 5 points; report tier migration | Stability report |
| 7. Independent validation | By a function other than the configuration owner (or a CBI-approved firm per the 2025 handbook, file 01 section 2e) | Validation pack |
| 8. Ongoing | Annual re-validation and after triggers (new typologies, NRA update, FATF outcomes, CBI circulars); distribution and escape monitoring monthly | Governance record |

### 11b. Inflation and deflation

- **Why ratings inflate**: correlated factors counted twice (PEP plus sector plus country); "when in doubt, High" under audit pressure (Wolfsberg RBA 2026: "A focus on everything is a focus on nothing"); unknown values scored high; triggers that never decay; reacting to a country listing for every customer (Iraq's grey listing); copying vendor defaults.
  Cost of inflation: wasted EDD, de-risking, alert fatigue (the MER already records "insufficient human resources to effectively monitor and investigate alerts").
- **Why ratings deflate**: default "standard" (Nationwide), sectors rated low because "regulated" (FSA 2011: extractive industries and pharmaceuticals rated low because regulated, but "not regulated for AML purposes"), weights equal when "some factors appeared to give rise to more risk", low scores for high-risk countries "to avoid having to conduct enhanced due diligence on much of their business".
- **Controls**:
  1. **Correlated-factor clusters** score the maximum in the cluster, not the sum.
  2. **Decay rules** for triggers: STR floor 24 months, credible adverse media 12 to 36 months by severity, address-change flags 12 months, each configurable and logged.
  3. **Distribution sanity bands, not quotas**: when High plus Very High falls below 1% or rises above 20% of customers, or Low-tier escapes rise, the model goes to review. Capacity is never used to shrink High silently (the US Bancorp alert-cap lesson); instead tiers 4 and 5 prioritise work.
  4. **Financial-inclusion lane**: Level 1 for SDD-eligible customers (government payroll, below-threshold accounts) so inclusion products are not over-rated (BCBS para 33; EBA 4.9; Wolfsberg RBA 2026).
  5. **Workload view**: show the analyst hours the current distribution needs next to staffing, so the board sees the cost of risk appetite.

### 11c. Explainability: the analyst screen

Every rating carries: score, tier and **model and configuration version**; contribution by block; the **top contributing factors** with values and points; **floors fired** (rule ID, legal basis, for example "CBI circular 26 Aug 2026 item 1: PEP relative, second degree"); **data completeness and freshness** (which factors are unknown or stale); **change since last rating** (diff); **triggers** that fired; overrides in force with reason, approver and expiry; and the **actions** that would lower it ("provide audited statements", "document source of wealth").
Regulators ask for this directly: EBA 3.7 ("understand how the system works and how it combines or weights risk factors"), Oracle describes "explainable scoring models documented for internal audit and regulator review", NICE logs "changes to risk models" (section 12), the CBI requires records that let each transaction step be reconstructed for 5 years (file 01).
Language: Arabic first with English; reason codes translated; explanation text generated from rule IDs, not free generation.
**Point-in-time reconstruction**: from the stored configuration version and data snapshot, AKR-TMS can reproduce any past rating for an inspector.

### 11d. Data quality

| Control | Detail | Evidence |
|---|---|---|
| **Completeness per factor** | Share of customers with a usable value per factor and segment; mandatory factors missing cap the best achievable tier (a customer with unknown BO, SoW or occupation cannot be Level 1 or 2) | Wolfsberg Country Risk FAQs Q11: "unknown information is a priori a risk factor"; Nationwide: gaps meant "it was not possible to complete a customer risk assessment fully... to be confident that all high risk customers had been identified" |
| **Reconciliation** | Daily counts and sums of customers and accounts ingested versus the CBS; customers not rated shown as a control failure | Nationwide: 44,957 accounts "not fed into Nationwide's customer risk assessment tool" (para 4.20); B-01 (file 04) |
| **Staleness** | Each factor has a freshness limit; stale values lose weight and create refresh tasks | AMLR art. 26; EBA 4.76 |
| **Consistency** | CBS fields versus KYC form and documents; duplicates across branches (one person, several customer numbers); family-number sanity | US Bancorp: rating "did not look across business lines" (file 20) |
| **Lineage** | Each factor value shows its source table and column and extraction time | Model governance (file 29) |
| **Iraqi data traps** | Free-text Arabic occupations needing a taxonomy; legacy ID types; Kurdish and Arabic spellings; Hijri versus Gregorian dates; placeholder dates in CBS; customers migrated from older systems with no KYC fields; accounts opened through agents or powers of attorney | Files 01 section 7, 22 |
| **Enhanced data must reach the score** | Nationwide gathered enhanced CDD for 5% of its base, kept it in a side database, and it "did not inform the RRP customer risk assessment or transaction monitoring systems" (para 4.22) | One canonical customer record feeding rating, screening and monitoring |

### AKR-TMS hooks (section 11)

- **Calibration workbench**: shadow mode, disagreement lists, golden set manager, tier migration matrices, back-test and escape reports, sensitivity runs, all stored with versions.
- **Distribution and escape monitor** with sanity bands and a board view of EDD workload.
- **Explanation service** shared by the analyst screen, the CBI report pack and the inspector's reconstruction view.
- **Data-quality board**: factor completeness, staleness, reconciliation, duplicates, with named owners and trend.

---

## 12. How vendors do customer risk rating (Fenergo, NICE Actimize CDD-X, Oracle KYC, Moody's)

Evidence: vendor pages and data sheets fetched or read on 2026-10-06 (marketing material, not independent validation); NICE and Oracle detail also in files 08 and 09, Fenergo and Moody's Passfort in file 16 (being written). Grades: V = vendor statement.

| Aspect | Fenergo | NICE Actimize CDD-X | Oracle FCCM KYC (cloud) | Moody's |
|---|---|---|---|---|
| **What it is** | Client lifecycle management (CLM) and KYC platform; API-first SaaS; "Fen-X" legal entity system of record (V) | Customer due diligence module of X-Sight, used with screening and monitoring (V) | KYC service in the FCCM suite; cloud-native, API-first, modular (V, data sheet v1.0, 2025) | Data and workflow: Orbis (635M+ entities, 170+ sources, "nearly two billion ownership links"), 30M+ risk profiles incl. 7M+ PEPs, 200+ countries; Moody's for Compliance workflow orchestration (V, figures as of July 2026; Passfort is covered in file 16) |
| **Rating method** | "Risk Engine dynamically responds to inputs to a client's KYC profile and automatically calculates client risk assessment, driving appropriate levels of due diligence, including enhanced due diligence"; **multiple risk models** chosen by "dynamic risk scoping rules, as defined by the business"; "single, centralized source of risk model management" (V) | **30+ out-of-the-box risk factors** across three areas: **customer identity** (name, screening results, geography, occupation or business type), **relationships** (high-risk relationships, ownership structures, UBOs), **behaviour** (channels, services, transactional activity "even if... not considered suspicious", SARs filed); **configurable risk coefficients**; custom factors; factors split into **segments with different scales** (V) | "Real-time entity scoring engine"; "intelligent, multi-dimensional risk scoring w/ actionable risk assessments", automated data discovery including beneficial owner information, 360-degree entity view; graph-based entity resolution; "ready to incorporate deep learning" (V). Cloud API for real-time risk assessment from release 26.5.1 (file 09) | Not a scoring engine as such: supplies inputs (ownership and control, shell-company indicators, PEPs and sanctions, adverse media) and configurable workflows for risk-based decisions (V). **Shell Company Indicator**: seven typology-based indicators (directorship patterns, mass registration, jurisdictional risk disparity, circular ownership, inconsistent age data, dormancy, financial anomalies) |
| **Dynamic and perpetual** | **Perpetual KYC**: "continuous monitoring of the client profile - identifying changes across relevant data sources, assessing their risk impact, and applying materiality"; event-driven reviews "process non-material updates automatically based on policy" | **Perpetual CDD**: reassesses "when a material event is identified"; a **periodic model stays as a backstop**; event-driven risk monitoring with AI and machine learning | "Perpetual KYC"; "extract risk signals from multiple dimensions, including events"; straight-through processing for low-risk customers, CDD/EDD routing for others (file 09) | "AI-enhanced screening and perpetual monitoring"; Chartis FCC50 category winner for perpetual KYC (V) |
| **Explainability and governance** | Policy-driven assessments; audit trail of every action; "Agents execute. Humans stay in the loop for judgement, exceptions, approvals, overrides and governance" (KYRA agentic AI) (V) | Model performance dashboards, governance documentation, **full logging of changes to risk models**, downloadable audit information; **simulation** of risk-factor changes "using production data before implementation" (V) | "Explainable scoring models documented for internal audit and regulator review" (V) | Source-data provenance (registers); not a model governance product |
| **Configuration burden** | No-code configuration of journeys, policies, risk models, data rules | Coefficients and segments set by the bank | Modular configuration (V) | Workflow configuration |
| **Strength for AKR-TMS to learn from** | Policy rules per jurisdiction (120+) feeding the rating; related-party manager for UBO chains | Three-block factor structure and simulation before change; periodic backstop | Entity scoring tied to graph resolution; explainable-model documentation | Shell-company typologies and ownership depth as rating inputs |
| **Limits for an Iraqi bank** | Built as the system of record for client data with write access to workflow; Iraqi jurisdiction pack, Arabic naming and CBI rules not shown on the page; SaaS default | Deployed with write-back to the core by file drop (the NICE ETL runbook: "T24 will then pick this delta file and update"); US/EU factor library; no Iraqi factor set shown | Cloud-first licence metering by "KYC checks created" (file 09); the data sheet does not show Iraqi content or on-premises deployment terms | Iraqi company coverage in Orbis is **not verified**; data licences priced per entity or search; a data source, not a rating engine |

**Observations for the plan**
1. All four sell **dynamic, event-driven risk** with a periodic backstop. The market has converged on the design in section 6 and Wolfsberg 2022.
2. The scoring logic of the incumbents is **configurable, not transparent by default**: customers get "coefficients" and "dashboards", not the logic. EBA 3.7 puts the burden of understanding on the bank. A **white-box rating** with reason codes is a real differentiator, particularly for a CBI-approved assessor (file 01 section 2e).
3. The vendors assume a **system of record they can write to**. AKR-TMS reads the CBS read-only, so its rating, KYC tasks and evidence live in AKR-TMS, with a one-way change feed. That is a design constraint and, for banks nervous about outside systems touching the core, a selling point.
4. **Iraqi content is the gap** none of them advertises: CBI rules as a regulatory profile, Iraqi PEP positions, family-number and name-chain relationship detection, the NRA at-risk activity list, Arabic occupation taxonomy, CRD and BO platform data, Hijri calendar. Orbis and similar data are inputs worth testing on Iraqi companies before relying on them.
5. **Simulation before change** (NICE) and **no-code configuration** (Fenergo) are features to match; the first is also a model-governance control (file 29).

### AKR-TMS hooks (section 12)

- Match: simulation of a rating-configuration change on production data before release; per-segment scales; periodic backstop; materiality filter on refresh triggers.
- Beat: white-box explanation per rating; Iraqi regulatory profile; read-only integration with a rating-change feed; on-premises default.
- Test before buying data: pilot an ownership and PEP data provider on a sample of 200 Iraqi companies and 100 officials and report the hit rate.

---

## 13. What regulators criticised about customer risk rating

The cases below are the ones where the supervisor's own text names a rating or risk-assessment failure. Other cases in file 20 are about monitoring coverage and screening; where they bear on rating they are cross-referenced.

| # | Case (authority, date) | What was criticised | Design lesson |
|---|---|---|---|
| 1 | **UK FSA thematic review "Banks' management of high money-laundering risk situations"** (June 2011; 35 visits to 27 banking groups, fieldwork May 2010 to February 2011) | Risk matrices "allocating inappropriate low-risk scores to high-risk jurisdictions" (para 11); "a third of banks... failed to review their risk assessment regularly"; "inappropriately low risk weightings for certain high risk factors, apparently - and sometimes overtly - to avoid having to conduct enhanced due diligence on much of their business"; sectors rated low because "regulated" (extractive, pharmaceuticals); "almost impossible for a relationship to be classified as high risk" (equal weights, staff who "did not understand how the risk score was generated"); overrides without evidence; no evidence on file for why a rating was given; ratings not updated during monitoring; RMs rewarded on profit; reviews with recycled text (paras 11-14, 83) | Weights documented and reasoned; reachable High; override control; audit trail; staff training on the score; recycled-text detection |
| 2 | **HSBC Bank USA** (OCC, DOJ, Senate subcommittee, 2012; file 20 section 2.4) | Country rated Mexico "standard" (lowest AML risk) so over USD 670 billion of wires from HSBC Mexico went unmonitored; a rule "assigning an overall risk rating to its non-SCC customers based solely on the risk rating that the bank has given the country" called "unacceptable"; the country score had "inadvertently failed to take into account a 2006 FinCEN advisory"; PEP-linked customers not designated high-risk | A country rating must not set the customer rating alone, must never set monitoring coverage, and must absorb regulator advisories (TR-10) |
| 3 | **U.S. Bank (US Bancorp)** (FinCEN, OCC, DOJ, 15 Feb 2018; file 20 section 2.2) | Customer risk rating "failed to include important information about its clients in its risk-rating analysis, such as a customer's country of citizenship and occupation" and did not look across business lines; a look-back produced more than 136 late SARs | Show which inputs each rating used and the share of customers missing each input; one customer view across products |
| 4 | **Habib Bank Limited, New York** (NYDFS, 7 Sep 2017; file 20 section 3.4, grade B) | 2016 examination rating of "5" because of "inadequate customer risk ratings" among other weaknesses, for a branch clearing for high-risk markets | Tailor factors to the business model (trade and correspondent flows) |
| 5 | **TD Bank** (DOJ, FinCEN, OCC, Fed, 10 Oct 2024; file 20 section 2.1) | Only a subset of high-risk countries (not updated after 2013) was monitored; a 2020 procedure told stores no further unusual-activity reports were needed for specific customers unless activity changed or ran past 60 days | Country table with effective dates; no customer-level suppression that outlives its reason |
| 6 | **Starling Bank** (FCA Final Notice, 27 Sep 2024, GBP 28,959,426 after 30% discount) | Agreed restriction on opening accounts for high or higher-risk customers; **54,359 accounts opened for 49,183 such customers**; no formal monitoring of compliance with the restriction; a control did not update so **294 customers previously exited for financial crime reasons opened new accounts** (161 had been subject to a SAR) (paras 2.3, 4.20-4.23) | Controls that depend on a rating need daily conformance tests and an exit register |
| 7 | **Barclays Bank plc** (FCA Final Notice, 14 Jul 2025, GBP 39,314,700 after 30% discount; customer Stunt & Co) | Corporate customer rated **low** (medium only March 2019 to May 2020) without sufficient information on business, source of wealth and source of funds; rating **not revisited** after: a joint venture (8 Oct 2015) changing business model and turnover; a law-enforcement request (Aug 2016); police raids and production orders (Sep to Nov 2016); account freeze (Aug 2018); charge of the owner (May 2020); GBP 46.8 million of receipts from a customer under investigation; the KYC refresh of Jan 2017 certified complete with queries unanswered; the investigations team "were not made aware of the law enforcement request"; the **Risk Scoring Tool's methodology was unchanged from 1 Dec 2014 to 31 Dec 2020** (four components: product, geography, industry, entity); had the customer been rated high "the relationship would have been subject to annual KYC reviews" (paras 2.5, 2.9, 4.16, 4.44) | Authority requests and adverse media are rating triggers visible to every team; refresh cannot be certified with open queries; scoring methodology needs scheduled review |
| 8 | **Barclays Bank UK PLC** (FCA Final Notice, 14 Jul 2025, GBP 3,093,600; customer WealthTek) | Policies did not require information on the purpose of a client account, the types of clients whose money it would hold, expected activity, or a check of the permissions register; eligibility judged by an internal industry code (paras 2.5-2.7) | Product-specific CDD: accounts holding third-party money carry their own factor and checklist |
| 9 | **Nationwide Building Society** (FCA Final Notice, 11 Dec 2025, GBP 44,078,500 after 30% discount; period 1 Oct 2016 to 1 Jul 2021) | "An unsophisticated, interim solution": customers outside a few categories "automatically classed as standard risk"; the new risk-profiling system fully operational only in early 2019; **44,957 accounts not fed into the risk assessment tool**; enhanced CDD gathered for **5% of the base** and not fed into scoring until April 2021; no periodic or event-driven reviews for a large share of customers apart from about 2,000 high-risk ones; personal accounts used for business without business CDD; internal audit found data-quality controls needed "significant improvement" (paras 2.7, 2.10, 4.19-4.23) | Default is never "standard": unknown is capped; reconcile feeds; one data path into rating and monitoring |

**Patterns across the cases**
1. **A rating that does not change when the facts change** (Barclays, Nationwide, FSA 2011).
2. **A rating that decides controls and is not tested** (Starling, HSBC).
3. **Country-only or sector-only shortcuts** (HSBC, FSA "regulated sectors", TD).
4. **Data that never reaches the score** (US Bancorp, Nationwide).
5. **Overrides and weights that make High unreachable** (FSA 2011).
6. **Teams that do not share intelligence** (Barclays: law-enforcement request not seen by the investigations team).
7. **Commercial incentives inside the rating** (FSA 2011 para 12; EBA 3.6).

### AKR-TMS hooks (section 13)

- Map each pattern to a standing control: rating-change-on-trigger test; rating-to-controls conformance test; no single-factor coverage decisions; input coverage report; override health report; shared authority-request register visible to monitoring and CDD teams; profit-blind rating.
- Use cases 1, 6, 7 and 9 as **scenario tests** for the calibration golden set: each should end in the right tier in the shadow run.

---

## 14. Proposed AKR-TMS customer risk rating model for Iraqi banks

Status of the numbers: every weight, point value, band and interval below is a **starting configuration for calibration** (section 11), chosen so that the model satisfies the rules read in this file and gives sensible answers on worked examples (checked with a script, scratchpad `crr_model.py`).
None of them has been validated on Iraqi bank data; the shadow run in section 11a is the first test.

### 14a. Design principles (each traced to a source)

1. **Hybrid**: floors for legal and CBI certainties, a transparent weighted score for the rest, a governed override layer (EBA 3.6; FSA 2011).
2. **Reachable High, no single-factor rule**: weights and floors tested so High and Very High can occur, and no one block decides the result (EBA 3.6; FSA 2011 poor practice).
3. **Unknown is not low**: unknown factors score 60 and count against completeness; mandatory unknowns cap the best tier (Nationwide; Wolfsberg country FAQs).
4. **Rating decides controls**, and the mapping is tested daily (Starling; HSBC).
5. **Dynamic**: triggers re-score at once; the calendar is the backstop (Wolfsberg 2022; AMLR art. 26; Barclays).
6. **Foreign nexus, not Iraq itself**, in per-customer geography (section 3b).
7. **No commercial data and no protected attributes** (names, tribe, religion, sect, region of origin) in the rating (EBA 3.6; fairness rule, section 3).
8. **Inclusion lane**: Level 1 exists for SDD-eligible customers (BCBS para 33; FATF 2014 Box 4; Wolfsberg RBA 2026).
9. **White-box**: every rating reproducible and explainable in Arabic and English (EBA 3.7; section 11c).
10. **Read-only**: the rating lives in AKR-TMS; a change feed exists for the bank to use; AKR-TMS never writes to the CBS.

### 14b. Architecture

```
CBS read-only replica + screening + case data + registry imports
        |
  Factor extraction (27 factors, 6 blocks)  ->  unknown-value and freshness checks
        |
  Block scores (weighted average of applicable factors; unknown values count as 60)
        |
  Inherent score = (1 - p) x weighted blocks + p x highest block      (p = 0.35, configurable)
        |
  Floors (legal and CBI rules) -> tier = max(score tier, highest floor) -> completeness cap
        |
  Override layer (section 10)  ->  Final tier (5 levels, mapped to High, Medium, Low)
        |
  Controls matrix (CDD depth, approver, monitoring profile, review clocks, limits)  +  Explanation
```

Runs nightly over all customers and **immediately** for any trigger (section 6). Phase 2: the same engine answers a payment-time lookup for the optional pre-release API (rating, floors fired, open reviews).
The **peak term** (p) answers the dilution problem: a customer with one block at 61.5 and the rest low would otherwise average to Level 2. With p = 0.35, an import company that uses trade finance lands at Level 3 (worked example below). p below 0.5 keeps any single block from dominating (EBA 3.6).

### 14c. Levels and mapping to the CBI labels

| Level | Score band | CBI label | Typical customers | Hypothesis share (sanity band only) |
|---|---|---|---|---|
| 1 | 0 to 14.9 | Low | Public-sector payroll accounts, pensioners, students, government bodies, low-balance savers (SDD lane) | 25 to 40% |
| 2 | 15 to 34.9 | Low | Salaried private-sector, small self-employed, small domestic companies with simple structure | 35 to 45% |
| 3 | 35 to 54.9 | Medium | Trading and import companies, contractors, foreign-nexus individuals, medium cash use | 12 to 25% |
| 4 | 55 to 74.9 (or a floor) | High | PEP relatives and associates, DNFBPs, NPOs, exchange and e-payment companies, complex structures, credible adverse media, STR within 24 months | 3 to 10% |
| 5 | 75 to 100 (or a floor) | High (top) | P1 PEPs, call-for-action nexus, previously exited customers, multiple serious signals | 0.2 to 1.5% |

The bands for Level 1 and 2 give "Low" in the CBI sense, and the CBI's five-level option is used. The share column is a **hypothesis for the sanity monitor in section 11b**, not a quota.

### 14d. Factor blocks, within-block weights and scoring anchors

Scoring: each factor takes a value from 0 to 100 using the anchors below (intermediate values allowed). A block score is the weighted average of the factors that apply and are known; unknown applicable factors score 60. Block weights: **A customer and structure 25%, B business and wealth 15%, C geography and foreign nexus 20%, D products 15%, E channel 5%, F behaviour and experience 20%.**

**Block A: customer and structure (25%)**

| ID | Factor (weight in block) | Anchors | Source |
|---|---|---|---|
| A1 | Customer type (30%) | Individual employee, pensioner, student 10; self-employed 30; sole proprietor 35; LLC, simple or solidarity partnership 45; unlisted joint-stock 40; listed on the securities exchange 15; government body 5; state enterprise 35; waqf under court supervision 30; registered domestic NPO 55; unregistered association 70; foreign branch 60; DNFBP 75; exchange, money-transfer or e-payment company 85 | EBA; AMLR Annex III; MER para 695; CBI controls |
| A2 | Entity age (5%) | Over 5 years 10; 2 to 5 years 25; 1 to 2 years 50; under 1 year 75; under 90 days 100 | Wolfsberg RBA 2026 (newly formed entities) |
| A3 | Ownership transparency (25%) | Single layer, natural persons identified and verified 10; up to two layers verified 30; three or more layers or a foreign layer, verified 60; declared only 75; nominee or front-person indicators, or unknown after reasonable steps 100 | EBA; AMLR Annex III; MER para 725 |
| A4 | Identity assurance (10%) | Chip national card read at a branch 10; legacy documents with originals at a branch 30; remote with liveness 40; remote without liveness 70; expired or unverifiable 90 | FATF digital ID (file 03 F-22); Wolfsberg 2022 |
| A5 | Reputation (20%) | None 0; adverse media of low credibility 40; STR filed 70; credible adverse media 90; previously exited for financial crime 100 | EBA Guideline 2 |
| A6 | Registration and disclosure (10%) | CRD record valid and BO declaration (post-July 2023) matched 10; valid, no BO declaration 40; registration not verified 80 | MER paras 711-720 |

**Block B: business, occupation and wealth (15%)**

| ID | Factor (weight) | Anchors | Source |
|---|---|---|---|
| B1 | Industry or occupation class (45%) | Low class (public employee, pensioner, salaried, student, health and education) 10; standard 30; elevated (construction, wholesale trade, transport, hospitality, non-sensitive import-export) 50; high class (cash-intensive restaurants, cafes, malls, fuel stations; exchange; jewellery and precious metals; car dealers; real estate; import companies; customs clearance; consultancies; oil sector; public contractors; NPOs; arms and defence; antiquities) 80 | EBA; Wolfsberg RBA 2026; MER para 695; CBI controls; AMLR Annex III(2)(e) |
| B2 | Public-sector exposure (15%) | None 0; public employee without procurement role 10; supplier to government 60; role with procurement, licensing or concession authority 80 | EBA (public procurement); FCA FG25/3 |
| B3 | Source of wealth and funds (25%) | Corroborated independently 10; documented and consistent 30; self-declared and plausible 55; unexplained or inconsistent 90 | Wolfsberg SoW/SoF; EBA |
| B4 | Expected activity profile (15%) | Declared and reasonable 10; partial 40; not declared 70 | CBI declared-versus-actual; FinCEN Q36 |

**Block C: geography and foreign nexus (20%)** (Iraq itself scores low for everyone)

| ID | Factor (weight) | Anchors | Source |
|---|---|---|---|
| C1 | Nationality and residence (35%) | Iraqi resident 10; Iraqi non-resident 40; foreign national resident 55; foreign national non-resident 70; dual national or residence in a high-risk country 85 | CBI controls (nationality, residence) |
| C2 | Registration and ownership countries (20%) | Iraq only 10; foreign registration by country-table score; offshore or secrecy jurisdiction 90 | EBA; AMLR Annex III(3)(f) |
| C3 | Linked-country risk (35%) | Highest country-table score among nationality, residence, registration, BO residence: low 10, medium 40, high 70, very high 100 | Country table (W-19) |
| C4 | Branch location (10%) | Border-area branch 40, otherwise 10 | CBI controls (border branches) |

**Block D: products, services and value (15%)**

| ID | Factor (weight) | Anchors | Source |
|---|---|---|---|
| D1 | Highest-risk product held (50%) | Payroll or basic account 10; term deposit or prepaid card 25; domestic transfers or e-wallet 35; FX or remittance 60; safe-deposit locker 55; trade finance (LC, LG, import murabaha) 75; private banking, client-money account or gold product 80 | CBI product indicators (file 27); EBA; file 24 |
| D2 | Number and complexity of products (15%) | One product 10; several 25; complex or multi-party 60 | EBA complexity |
| D3 | Expected monthly value (20%) | Up to IQD 1m 10; up to 20m 25; up to 100m 40; up to 500m 60; over 500m 80 | CBI SDD ceiling; bank calibration |
| D4 | Cash and third-party features (15%) | None 10; some cash or third-party deposits 40; heavy 80 | EBA; CBI cash scenarios |

**Block E: channel (5%)**

| ID | Factor (weight) | Anchors |
|---|---|---|
| E1 | Onboarding channel (60%) | Branch with chip card 10; agent 40; remote with strong authentication 30; remote with weak authentication 70 |
| E2 | Intermediaries and powers of attorney (40%) | None 10; power of attorney or acting agent 50; introduced by an intermediary 60 |

**Block F: behaviour and experience (20%, dynamic)**

| ID | Factor (weight) | Anchors | Source |
|---|---|---|---|
| F1 | Declared versus actual activity (30%) | Within band 0; moderate deviation 40; sustained large deviation 90 | CBI; FinCEN Q36 |
| F2 | Observed corridors and counterparties (20%) | Share of value with high-risk countries under 5% 10; 5 to 20% 50; over 20% 90 (and new corridors add points) | Wolfsberg RBA 2026 |
| F3 | Alert and case history (20%) | None 0; alerts closed without concern 10; confirmed unusual 60; escalated or STR filed 90 | EBA; NICE CDD-X (SARs as a factor) |
| F4 | Profile change velocity (10%) | Stable 0; three or more key changes in 90 days 60 | Wolfsberg RBA 2026 |
| F5 | Network risk (10%) | Linked to a High party 60; shared identifier with an STR-linked party 90 | Graph (file 04 W-03) |
| F6 | Responsiveness (5%) | Responsive 0; late 40; evasive 90 | Wolfsberg RBA 2026 |
| F7 | Authority requests (5%) | None 0; any in 36 months 90 | Barclays (FCA 2025) |

### 14e. Worked examples (computed with the configuration above)

| Customer | Block scores A / B / C / D / E / F | Weighted | Peak | Inherent score | Score level | Floor | **Final** |
|---|---|---|---|---|---|---|---|
| Teacher with a public payroll account | 6.7 / 15.0 / 10.0 / 10.0 / 10.0 / 0.0 | 7.9 | 15.0 | 10.4 | 1 | none | **1** |
| Import LLC using letters of credit | 19.3 / 49.5 / 20.5 / 61.5 / 10.0 / 18.0 | 29.7 | 61.5 | 40.8 | 3 | none | **3** |
| Relative (second degree) of a deputy minister, government contractor | 16.7 / 64.8 / 10.0 / 39.2 / 10.0 / 14.0 | 25.1 | 64.8 | 39.0 | 3 | FL-03 (4) | **4** |
| Registered domestic charity with Gulf donations | 24.0 / 57.2 / 20.5 / 38.2 / 10.0 / 6.0 | 26.1 | 57.2 | 37.0 | 3 | FL-05 (4) | **4** |
| Exchange company with foreign corridors and an escalated alert | 35.5 / 55.8 / 34.0 / 65.5 / 10.0 / 44.0 | 43.2 | 65.5 | 51.0 | 3 | FL-05 (4) | **4** |

### 14f. Floors (minimum level), with the legal or supervisory basis

A floor sets the **minimum** level; the final level is the higher of the score level and the highest floor that applies. Floors are configuration, versioned, and cannot be overridden per customer (section 10b).

| ID | Condition | Floor | Basis |
|---|---|---|---|
| FL-01 | PEP sub-tier **P1** (customer, beneficial owner at or above threshold, or signatory with control) | 5 | CBI PEP circular 26 Aug 2026; FATF R.12; section 9c |
| FL-02 | PEP sub-tier **P2** | 4 | same |
| FL-03 | **P3** (relative to the second degree or close associate) and former PEPs until a documented assessment | 4 | CBI circular items 1, 8; AMLR art. 45-46 |
| FL-04 | Entity whose beneficial owner at or above the threshold is a PEP, or who controls it | 4 | CBI circular item 4; FCA FG25/3 |
| FL-05 | **CBI-named high-risk customer types**: DNFBPs (real-estate dealers, precious-metals dealers, car dealers, lawyers and accountants acting as gatekeepers), NPOs, non-residents, exchange, money-transfer and e-payment companies | 4 (a governed exception can lower a documented sub-segment, for example a registered domestic NPO with local donors, to 3 with EDD and senior approval retained) | CBI controls 2025 ch. 2 art. 2 (risk matrix, customer factors, NPO approval); FATF R.8 (file 03) |
| FL-06 | Nexus to a FATF call-for-action or comprehensively sanctioned jurisdiction (residence, registration, BO residence, or material flows) | 5 | FATF R.19; CBI ch. 8; Iran PF pack (file 04 U-08) |
| FL-07 | Credible adverse media on money laundering, corruption, terrorism or organised crime | 4 | EBA Guideline 2; CBI circular item 3 |
| FL-08 | STR filed on the customer within 24 months | 4 | EBA (subject of a past STR); NICE factor |
| FL-09 | FIU, court or prosecutor request, production order, freeze or suspension within 36 months | 4, with an immediate review | Barclays (FCA 2025) |
| FL-10 | Complex structure without commercial rationale, nominee shareholders, bearer instruments | 4 | EBA; AMLR Annex III(1)(d), (f) |
| FL-11 | Customer previously exited for financial-crime reasons | 5, and block at account opening unless an exception is approved | Starling (FCA 2024) |
| FL-12 | Beneficial owner not identified or verified after reasonable steps | Not accepted; existing customer 4 with a remediation task and STR consideration | Law 39 art. 10/Fifth (file 01) |
| FL-13 | More than 20% of value to or from high-risk countries in the last 12 months | 4 | Wolfsberg RBA 2026 (corridors) |
| FL-14 | Three or more of seven shell-company indicators (data provider or bank-derived) | 4 | Moody's indicator set; AMLR Annex III(1)(h) |
| FL-15 | Foreign national non-resident with no Iraqi economic nexus | 3 | EBA (could needs be better served elsewhere) |
| (process) | Sanctions near-match | Hold and freeze workflow, not a level | File 04 U-06 |

**Completeness cap**: a customer with any mandatory factor unknown (individuals: occupation, residence, identity assurance; entities: type, ownership, activity, registration) cannot be below Level 3; with two or more mandatory unknowns the review is escalated and the customer cannot be rated before the data is supplied.

### 14g. Controls matrix: what each level means in practice

| Level | CDD depth | Approver for opening and continuing | Monitoring profile | Classification review | KYC data refresh | Light automated check |
|---|---|---|---|---|---|---|
| 1 | SDD where the CBI rules allow (government bodies, accounts below IQD 1 million); standard CDD otherwise | Branch officer | Baseline thresholds, SDD lane scenarios | 24 months (the CBI's written maximum) | By trigger, ID expiry or 36 months | Yearly: ID valid, no profile change, activity in band, no screening hit |
| 2 | Standard CDD | Branch manager | Baseline | 24 months | 36 months or trigger | Yearly |
| 3 | Standard CDD plus targeted source-of-funds and activity evidence | Compliance officer | Tighter thresholds on relevant scenarios; quarterly peer-group review | 24 months (practice for medium) | 24 months | Six-monthly |
| 4 | Full EDD: source of wealth and funds corroborated, ownership verified, adverse media, site visit for entities | Senior management (department head and MLRO); a named executive for NPOs (CBI) | Lowest thresholds; daily review of alerts; enhanced scenarios (file 27, file 24 packs) | 12 months | 12 months | Quarterly |
| 5 | Level 4 plus independent verification of wealth, enhanced screening of related parties, documented rationale for the relationship | Senior executive management or board committee | Real-time hold on matching scenarios until the MLRO clears (CBI ch. 5 art. 5); product limits | 6 months (full EDD yearly) | 12 months | Monthly |

The **KYC data refresh** column is the bank's interpretation of the CBI's "update the KYC form... at least once a year" until the CBI clarifies (section 5a); the light automated check keeps the yearly rhythm for every customer without sending everyone to the branch.
Banks with EU correspondents can switch on the AMLR profile: levels 4 and 5 refreshed at most every 12 months, all others at most every 5 years.

### 14h. Re-rating rules

- **Immediate re-score** on any trigger TR-01 to TR-26 (section 6c); floors from FL-07 to FL-11 apply from the moment the fact is recorded.
- **Decay**: FL-08 expires after 24 months, FL-09 after 36, FL-07 after 12 to 36 months by severity; expiry creates a review, not an automatic downgrade.
- **Stickiness**: a rise in level is immediate; a fall needs the next review (or an approved downward override) and cannot cross a floor.
- **Country table change** (FATF plenary outcomes, sanctions, CBI circulars): all affected customers re-scored the same day and listed with the effective date (W-19).

### 14i. Override limits

Applied as section 10b: floors cannot be overridden; upward easy; downward by compliance with expiry within 12 months, one level per decision, maker-checker, reason code and evidence; relationship owner excluded.

### 14j. What the analyst sees: the explanation panel

Illustration (relative of a P2 PEP, government contractor; Arabic first in production, English shown):

```
CUSTOMER 4410-xxxx            Level 4 (High)      Score 39.0 (Level 3)   Floor FL-03 raises to 4
Rated 2026-10-06 03:12   Model v1.3  Configuration 2026-09-30   Next classification review 2027-10-06
-----------------------------------------------------------------------------------------------
WHY LEVEL 4
 1. Floor FL-03: relative (second degree) of a confirmed P2 PEP
    Basis: CBI PEP circular 26 Aug 2026, items 1 and 8. Evidence: PEP record 7731, family link 2 (confirmed by a compliance officer 2026-09-21)
 2. Highest block: Business and wealth 64.8
      B1 Industry: government contractor 80 | B2 Public-sector exposure 60 | B3 Source of wealth: self-declared 55 | B4 Expected activity: partial 40
 3. Score drivers (contribution to the 39.0): Business 29.0 (incl. peak term), Products 3.8, Customer 2.7, Behaviour 1.8, Geography 1.3, Channel 0.3
DATA GAPS (3)
 - Source of wealth not corroborated (B3, mandatory for PEP relatives)
 - Declared monthly income missing since account opening
 - ID card chip read not recorded (legacy ration card on file)
CHANGES SINCE LAST RATING (2026-03-02)
 - New: P3 link created; contractor registration verified; two incoming transfers above declared band (TR-11)
TRIGGERS OPEN: TR-07 PEP link (review due 2026-10-20) | TR-11 activity above band (targeted refresh task 2026-10-13)
OVERRIDES IN FORCE: none
WHAT WOULD LOWER IT: documented and corroborated source of wealth; written assessment that the principal PEP is lower risk (floor stays 4 for a P3, minimum two reviews)
REQUIRED NOW: senior management approval (not yet recorded) | EDD checklist 6 of 9 complete | next KYC refresh 2027-10-06
```
(Names and numbers are placeholders; no real customer data.)

### 14k. Iraqi specifics built into the model

- **Salary-localisation and payroll accounts** (public-sector payroll): Level 1 lane with SDD and light checks (file 01 section 9).
- **NRA at-risk activity list** embedded in B1; **CBI-named customer types** as floors in FL-05.
- **Name structure**: given name, father, grandfather, great-grandfather, mother, tribe or family name stored as parts for matching and for PEP-relative candidates; never used as rating factors.
- **Family number and national card number** for relative detection (testing needed on bank data).
- **Border-area and Kurdistan branches** as small, explicit factors (C4), with Kurdish name variants in matching.
- **Hijri-aware** time series (file 24) and Arabic reason codes.
- **Islamic products** (file 24) enter D1 as product classes (import murabaha counts as trade finance), and the IS scenario alerts feed F3.

### 14l. Delivery plan and open decisions

**Phases.**
1. *Shadow* (8 to 12 weeks per bank): ingest, rate, compare with the bank's ratings, disagreement review, golden set, back-test, no change to controls.
2. *Live for ratings and reviews*: tasks, approvals, explanation, triggers, CBI KPI pack; the bank decides whether to feed the rating into its CBS.
3. *Optional payment-time lookup* through the pre-release API (Phase 2 of the product).

**Decisions for bank leadership and the CBI.**
1. Reading of the annual "update the KYC form" duty (all customers, or high-risk only).
2. Treatment of NPOs: default High with exception route, versus a lower default (FATF R.8 proportionality).
3. Whether domestic PEPs are all High (the CBI and MER say yes in practice) and the definition of "high-risk position holders" and the degrees of kinship.
4. Which customers qualify for SDD above and below the IQD 1 million line, and for Level 1.
5. BO threshold per tier (default 20%).
6. Data investments: occupation taxonomy, PEP register, BO and registry feeds, source-of-wealth capture.
7. Whether to buy third-party data (ownership, PEP, adverse media) after the pilot on Iraqi samples (section 12).

### AKR-TMS hooks (section 14)

- All of 14b to 14l as configuration objects with versions, owners, rationale and test results; explanation and CBI report packs generated from them.

---

## 15. What it means for AKR-TMS

1. **One rating engine, three layers, Iraq-CBI profile.** Floors for what the law and the CBI make certain, a transparent weighted score for the rest, a governed override layer. Defaults follow the CBI (High, Medium, Low with five levels, named high-risk customer types, PEPs to the second degree, SDD ceiling IQD 1 million, BO 20%) and can be switched to an EU-style profile for banks with European correspondents.
2. **The rating must drive controls, and the product must prove the link every day.** The recurring supervisory failure is a rating that did not change what the bank did (Starling, HSBC, Nationwide). A daily conformance test between level and approvals, reviews, monitoring profile and product limits is the cheapest protection a bank can buy.
3. **Triggers matter more than the calendar.** Barclays rated a customer low for six years while police raids, production orders, a law-enforcement request and a frozen account went by. For an Iraqi bank, the equivalent events are FIU requests, court orders, freeze and suspension orders, STRs and branch liaison reports; they must reach the rating immediately and be visible to monitoring and CDD teams alike.
4. **Beneficial ownership is the weakest data and the product should say so.** Iraq's registry is a few years old, unverified and incomplete in definition; banks are the de facto register. The BO model, verification statuses and coverage dashboard turn that into a managed remediation programme, with the CRD and CBI platform feeds as imports when the bank has access.
5. **PEPs are floors and sub-tiers, not refusals.** Iraqi PEPs are higher risk by the FCA's own test; the CBI wants relatives to the second degree, persistence after office, daily local news, and seven KPIs. A bank-maintained register with lifecycle states, relative detection with human confirmation and the KPI pack is core.
6. **Data quality decides whether any of this works.** Nationwide had a rating system and still could not be confident it knew its high-risk customers, because 44,957 accounts never reached it and enhanced CDD sat in a side database. Reconciliation, completeness caps, "unknown is not low", freshness and one canonical customer record are first-order features.
7. **White-box scoring is the differentiator against vendors**, and **read-only integration is a constraint with a benefit**. The four vendors read in section 12 sell configurable, dynamic, event-driven risk; none shows Iraqi content, and they assume a write path into their own system of record. AKR-TMS keeps ratings, KYC tasks and evidence itself, offers a one-way change feed, and shows the reasoning in Arabic.
8. **Do not over-rate and do not under-rate.** The FSA found both, and Iraq has an alert-fatigue problem already. Correlated-factor clusters, decay rules, sanity bands, an inclusion lane and a workload view keep High reachable and affordable.
9. **Calibration is a project, not a switch.** A shadow run of eight to twelve weeks per bank, a golden set built from the MER cases, outcome back-tests used as sanity checks (labels are scarce), sensitivity analysis and independent validation.
10. **Leadership decisions are needed** before build (section 14l): the annual KYC duty, NPO default, PEP scope and kinship degrees, SDD scope, BO threshold per tier, data investments, third-party data pilot.
11. **Links to the rest of the research**: file 24 (Islamic product classes and NPO and waqf customers feed D1, F3 and FL-05), file 27 (behaviour scenarios feed F1 to F3), file 28 (case management for EDD tasks and approvals), file 29 (governance of rating configuration and any ML component), file 22 (name matching for PEP relatives), file 25 (CBS fields for factors), file 16 (KYC/CLM vendors).

### Gaps and items not verified (2026-10-06)

- No Iraqi bank's actual rating practice beyond the MER's description (review cycles 1, 2, 3 years; 10% BO check) was found; the weights in section 14 are untested.
- The CBI PEP Controls No. 2 of 2023 and the detailed Instructions 1/2023 text (arts. 19-22) were not read; the article-level content comes from the MER annex (file 01).
- The AMLA draft CDD technical standards and the AMLA guidelines on risk factors and monitoring (due 10 July 2026) were not read; the draft is known only through NICE Actimize's summary.
- Whether Iraqi banks may query the CBI beneficial-owner platform, and what share of companies it covers, were not found.
- Oracle's older on-premises KYC risk-assessment configuration, Pega, Encompass and Passfort details are not covered here (see files 09 and 16).
- The FATF 2013 PEP guidance and INR.10 text were not re-read; they come through file 03. The FFIEC examination manual could not be fetched.
- Barclays Bank UK, Metro Bank and Nationwide notices were read for rating-related findings only; the monitoring findings are in file 20 or section 13 as noted. The Monzo notice could not be located by URL.
- Basel AML Index and CPI figures were read from the live sites on 2026-10-06 and will change with the next editions.

---

## 16. Derived requirements (K-xx)

| ID | Requirement | Source |
|---|---|---|
| K-01 | Hybrid rating engine: floors, weighted factor blocks and a governed override layer, all configurable, versioned and testable | EBA 3.6; FSA 2011 paras 79-83; CBI risk matrix |
| K-02 | Five levels mapped to the CBI labels High, Medium, Low, with the mapping shown on every rating | CBI controls 2025 ch. 2 art. 2, ch. 5 art. 5 |
| K-03 | Factor catalogue (27 starting factors in six blocks) with ID, value anchors, data source, freshness limit and legal basis; data-readiness view per bank | EBA Guideline 2; AMLR Annexes I and III; FATF INR.10 |
| K-04 | Unknown is not low: unknown applicable factors score 60, mandatory unknowns cap the best level (Level 3), unrated customers shown as a control failure; daily reconciliation of rated customers to the CBS | Nationwide (FCA 2025) paras 4.20-4.22; Wolfsberg country FAQs; B-01 |
| K-05 | Country-risk table from FATF lists, EU list, sanctions programmes, CPI, Basel AML Index and CBI lists with effective dates; per-customer geography measures foreign nexus and observed corridors, not Iraq itself | EBA 2.x; Basel AML Index 2025; TI CPI 2025; W-19 |
| K-06 | Block weights, within-block weights, anchors and the peak term configurable; shadow run and simulation of any configuration change on production data before release | EBA 3.6-3.7; NICE CDD-X simulation; section 11a |
| K-07 | Floors FL-01 to FL-15 as governed configuration with legal basis, not overridable per customer | EBA 3.6; CBI circular 26 Aug 2026; CBI controls |
| K-08 | Rating-to-controls matrix (CDD depth, approver, monitoring profile, review clocks, product limits) with a daily conformance test against CBS data | Starling (FCA 2024); HSBC (file 20) |
| K-09 | Two clocks per customer, classification review and KYC refresh, with selectable regulatory profiles (Iraq-CBI, EU-AMLR) and the CBI rule referenced on each due item | CBI controls 2025; AMLR art. 26(2); MER IO4 |
| K-10 | Light automated yearly check for every customer (ID validity, profile change, activity in band, screening hits) so yearly rhythm does not require branch visits | Wolfsberg 2022; section 6d |
| K-11 | Governed trigger library TR-01 to TR-26 (factor, variable, source, threshold, action, owner, SLA, version, calibration date) with trigger-effectiveness metrics | Wolfsberg 2022; CBDDQ Q69, Q75; AMLR art. 26(3); W-24 |
| K-12 | Authority-request register (FIU, court, prosecutor, production and freeze orders) that re-rates the customer and is visible to monitoring and CDD teams | Barclays (FCA 2025) paras 2.9(d)-(e), (g) |
| K-13 | Exit register of customers closed for financial-crime reasons, checked against new-account events | Starling (FCA 2024) paras 4.20-4.21 |
| K-14 | Review workbench pre-filled with new and changed facts, mandatory-evidence gate, recycled-text detection, no sign-off with open queries | FSA 2011 paras 14, 16; Barclays 2.9(f) |
| K-15 | EDD playbooks by customer type with evidence slots, approver level and intervals | CBI controls; EBA 4.45-4.51; AMLR art. 34(4) |
| K-16 | Source-of-wealth and source-of-funds record with corroboration status, estimated net worth, expected inflows and actual-versus-declared monitoring | Wolfsberg SoW/SoF 2020; W-15 |
| K-17 | Conflict controls: relationship owner cannot approve or lower their own customer's rating; no commercial data is a rating input | EBA 3.6; FSA 2011 para 12 |
| K-18 | BO and control model: percentages by route, chain calculation, aggregation by natural person, control by other means including family links, verification status, last-confirmed date, threshold configurable per type and tier | Instructions 1/2023 art. 7; AMLR art. 52; MER paras 711-730 |
| K-19 | BO coverage and quality dashboard; import path for CRD and BO-platform data with source, date and fee tracking | MER paras 714-720; CBI deck (May 2026) |
| K-20 | Hidden-ownership scenarios: profit distribution to non-shareholders, recurring agents and signatories across unrelated companies, shared identifiers, ownership change before large transactions | MER para 729 |
| K-21 | PEP register with lifecycle states, tiers P1-P3, relative detection with human confirmation, pair-keyed memory and both-way delta screening | CBI circular; Wolfsberg 2017; W-21 |
| K-22 | PEP floors and modifiers; configurable kinship degrees; undeclared-PEP workflow | CBI circular items 2, 7, 8 |
| K-23 | CBI PEP KPI pack items a-g generated from system data | CBI circular item 9 |
| K-24 | Override register and workflow: upward easy, downward by compliance with expiry within 12 months, one level per decision, maker-checker, reason codes, evidence; waivers separate | EBA 3.6; FSA 2011; Wolfsberg country FAQs |
| K-25 | Override and rating analytics: rate by user and branch, downward share, expired overrides, overrides followed by alerts or STRs, bulk re-rating events | FSA 2011; Wolfsberg |
| K-26 | Immutable rating history and point-in-time reconstruction from model version, configuration version and data snapshot | EBA 3.7; CBI records (file 01) |
| K-27 | Explanation panel in Arabic and English: level, score, model and configuration version, block contributions, top factors, floors fired with basis, data gaps, changes since last rating, open triggers, overrides, what would lower the rating | EBA 3.7; section 11c |
| K-28 | Calibration workbench: shadow mode, disagreement lists, golden set manager, tier migration, back-test and escape reports, sensitivity runs | EBA 3.7; file 05 section 1c; file 29 |
| K-29 | Distribution and escape monitor with sanity bands (High plus Very High below 1% or above 20% triggers review) and a workload view; capacity never suppresses detection | FSA 2011; US Bancorp (file 20) |
| K-30 | Decay and stickiness rules: STR floor 24 months, authority-request floor 36 months, media floor 12 to 36 months; rises immediate, falls at review | Section 14h |
| K-31 | Data-quality board: factor completeness, staleness, reconciliation, duplicates, lineage, owners | Nationwide (FCA 2025); NYDFS Part 504 (file 05) |
| K-32 | Iraqi data handling: occupation taxonomy, name parts, family number, legacy ID types, Kurdish and Arabic variants, Hijri dates, migrated-record placeholders | File 01 section 7; file 22 |
| K-33 | Fairness: no protected attributes (names, tribe, religion, sect, region of origin) as factors; periodic check of level distribution by branch and region for unexplained skew | Section 3 fairness rule; EBA 3.6 |
| K-34 | Governance tracks: rating configuration as governed rules; any ML component as a model with model card and validation pack; validator independent of owner | File 05 section 1c; file 29; US Bancorp (file 20) |
| K-35 | Read-only rule: ratings, KYC tasks and evidence live in AKR-TMS; one-way rating-change feed; divergence view against the CBS risk field | CBI controls (Automatic Integration); NICE ETL runbook pattern |
| K-36 | Inclusion lane: Level 1 and SDD-eligible customers with compensating monitoring and automatic upgrade on crossing thresholds | BCBS para 33; FATF 2014 Box 4; EBA 4.9; Wolfsberg RBA 2026 |
| K-37 | Selectable regulatory profiles (Iraq-CBI, EU-AMLR with 1-year and 5-year ceilings) | CBI controls; AMLR art. 26 |
| K-38 | Management and inspector reports: rating distribution, overrides, reviews and backlog, CBDDQ Q67-Q75 metrics, CBI inspection pack | Wolfsberg CBDDQ; CBI controls |
| K-39 | Islamic-product and NPO inputs to D1, F3 and FL-05 from the IS pack and charity module | File 24 (I-12, I-20, I-28) |
| K-40 | Optional payment-time rating lookup (rating, floors fired, open reviews) in the pre-release API | Product brief (Phase 2) |
| K-41 | Pilot protocol for third-party ownership, PEP and adverse-media data on Iraqi samples (200 companies, 100 officials) before purchase; adapters for chosen providers | Section 12 |
| K-42 | Country-table change handling: same-day re-score of affected customers, exposure list, optional retrospective lookback | Wolfsberg country FAQs; W-19 |

---

## 17. Sources

Primary and official
- [EBA, Final Report on Guidelines on ML/TF risk factors (EBA/GL/2021/02), 2021](https://www.eba.europa.eu/sites/default/files/document_library/Publications/Guidelines/2021/963637/Final%20Report%20on%20Guidelines%20on%20revised%20ML%20TF%20Risk%20Factors.pdf)
- [Regulation (EU) 2024/1624 (AMLR), EUR-Lex](https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=OJ:L_202401624)
- [European Commission, high-risk third countries (page last updated 9 Jan 2026)](https://finance.ec.europa.eu/financial-crime/high-risk-third-countries-and-international-context-content-anti-money-laundering-and-countering_en)
- [FSA, Banks' management of high money-laundering risk situations (June 2011)](https://www.fca.org.uk/publication/corporate/fsa-aml-final-report.pdf)
- [FCA, FG25/3 The treatment of politically exposed persons (7 Jul 2025)](https://www.fca.org.uk/publication/finalised-guidance/fg25-3.pdf)
- [FCA Final Notice, Starling Bank Limited (27 Sep 2024)](https://www.fca.org.uk/publication/final-notices/starling-bank-limited-2024.pdf)
- [FCA Final Notice, Barclays Bank Plc (14 Jul 2025)](https://www.fca.org.uk/publication/final-notices/barclays-bank-plc-2025.pdf)
- [FCA Final Notice, Barclays Bank UK PLC (14 Jul 2025)](https://www.fca.org.uk/publication/final-notices/barclays-bank-uk-plc-2025.pdf)
- [FCA Final Notice, Nationwide Building Society (11 Dec 2025)](https://www.fca.org.uk/publication/final-notices/nationwide-building-society-2025.pdf)
- [FinCEN, Frequently Asked Questions regarding Customer Due Diligence requirements (April 2018)](https://www.fincen.gov/sites/default/files/2018-04/FinCEN_Guidance_CDD_FAQ_FINAL_508_2.pdf)
- [Basel AML Index 2025 Public Edition: ranking](https://index.baselgovernance.org/ranking) and [methodology](https://index.baselgovernance.org/methodology)
- [Transparency International, Iraq, Corruption Perceptions Index 2025](https://www.transparency.org/en/countries/iraq)
- [Wolfsberg CBDDQ Guidance v2.0 (2023)](https://db.wolfsberg-group.org/assets/09bafec3-4b77-428b-9113-f4203d290a2f/EN_CBDDQ%20Guidance%20(2023).pdf); [Country Risk FAQs (2024)](https://db.wolfsberg-group.org/assets/6a3513cd-b486-4d7e-aa2f-698b18ed05fb/Wolfsberg%20Group%20Country%20Risk%20FAQs%20(2024).pdf); [Digital Customer Lifecycle (2022)](https://db.wolfsberg-group.org/assets/d51130c7-0262-4604-978e-ac3b85aea142/Wolfsberg%20Guidance%20on%20Digital%20Customer%20Lifecycle%20Management%20(2022).pdf); [SoW and SoF FAQs (2020)](https://db.wolfsberg-group.org/assets/a27f9bf6-b4a8-41d2-a390-6f1aaf797241/Wolfsberg%20SoW%20and%20SoF%20FAQs%20August%202020%20(FFP)_1.pdf); [Guidance on PEPs (2017)](https://db.wolfsberg-group.org/assets/9c6630de-69a8-4f55-9289-4db938938e34/Wolfsberg%20Guidance%20on%20PEPs.pdf); Wolfsberg Guidance on the Risk-Based Approach (June 2026) via file 04 section 2j
- FATF, Risk-Based Approach Guidance for the Banking Sector (October 2014), local copy used in file 03
- [MENAFATF, Mutual Evaluation Report of Iraq (May 2024)](https://menafatf.org/sites/default/files/MER_Iraq_English.pdf), paras 685-730 and IO4
- [CBI, AML/CFT/CPF supervisory controls (8 Jan 2025)](https://cbi.iq/static/uploads/up/file-173666439114818.pdf); [CBI PEP circular (26 Aug 2026)](https://cbi.iq/static/uploads/up/file-178963295727758.pdf); [CBI, Strategic Achievements 2020-2025 (18 May 2026)](https://cbi.iq/static/uploads/up/file-177935854666319.pdf)

Vendor material (marketing; marked V in the text)
- [Fenergo, KYC](https://www.fenergo.com/know-your-customer-kyc-solution); [Client Lifecycle Management](https://www.fenergo.com/client-lifecycle-management); [AI agents (KYRA)](https://www.fenergo.com/ai-agents)
- [NICE Actimize, Customer Due Diligence (CDD-X)](https://www.niceactimize.com/anti-money-laundering/customer-due-diligence); [CDD-X risk models datasheet](https://resources.niceactimize.com/wp-content/uploads/2025/09/AML_CDD_Models_Datasheet.pdf); [CDD-X simulation datasheet](https://resources.niceactimize.com/wp-content/uploads/2025/09/AML_CDD_Datasheet_Simulation.pdf); [risk rating and assessments eBook](https://resources.niceactimize.com/wp-content/uploads/2025/06/aml-improve-customer-risk-reviews-with-robust-risk-rating-and-assessments-ebook.pdf); [Response to AMLA's CDD consultation (May 2026)](https://resources.niceactimize.com/wp-content/uploads/2026/05/NICE-Actimize-Response-to-AMLAs-CDD-Consultation_Insights-Article.pdf); [NICE Actimize ETL runbook, CDD feedback loop (bank deployment)](https://nice-actimize-etl.readthedocs.io/en/latest/cdd-feedback-loop.html)
- Oracle Financial Services Know Your Customer data sheet, version 1.0, 2025 (oracle.com, via file 09)
- [Moody's KYC](https://www.moodys.com/web/en/us/kyc.html); [Shell Company Indicator](https://www.moodys.com/web/en/us/kyc/products/shell-company-indicator.html); [Orbis](https://www.moodys.com/web/en/us/kyc/products/orbis-compliance.html); [CDD for banks](https://www.moodys.com/web/en/us/kyc/sectors/banking.html)

Cross-references in this research set: files 01, 02, 03, 04, 05, 09, 16, 20, 22, 24, 25, 27, 29. Script used for the worked examples: scratchpad `crr_model.py`.
