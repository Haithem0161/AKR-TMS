# 05. Other regimes that reach Iraqi banks: US, EU, UK, GCC and regional peers

Status: done (2026-10-06). Requirements R-01 to R-61 in section 6.

Why this file exists: Iraqi banks are supervised by the CBI (file 01), but the rules that decide whether they can
move dollars, euros and dirhams are written in Washington, Brussels, London, Abu Dhabi, Riyadh and Amman. These
rules reach an Iraqi bank in three ways:

1. **Directly** through sanctions with extraterritorial reach (OFAC secondary sanctions, FinCEN Section 311).
2. **Through correspondents**: every US, EU, UK and Gulf correspondent applies its own home-country AML rules to its
   respondents (due diligence questionnaires, RFIs, payment-transparency checks, exits).
3. **As the benchmark**: international consultants, auditors and the CBI's quarterly meetings with the US Treasury and
   Federal Reserve (file 01 section 4) measure Iraqi banks against what a US, EU or UK bank would be expected to do.

Each section ends with what the regime means for AKR-TMS. Section 6 holds the derived requirements (R-xx); section 7
holds the sources.

Contents:
- 1 United States (FinCEN programme rule, BSA/AML examination manual, OFAC, Section 311, NYDFS Part 504, model risk)
- 2 European Union (AMLR, AMLD6, AMLA, EBA guidelines, transfer of funds regulation)
- 3 United Kingdom (MLR 2017, JMLSG, OFSI, FCA)
- 4 GCC and regional peers (UAE, Saudi Arabia, Jordan, others)
- 5 What it means for AKR-TMS (cross-regime synthesis)
- 6 Derived requirements R-01..
- 7 Sources

## 1. United States

The US matters more than any other foreign regime for an Iraqi bank. Iraq's oil revenue sits at the Federal Reserve Bank
of New York; dollar clearing for Iraqi banks runs through US banks or through foreign banks that clear dollars in New
York. US rules reach Iraqi banks through OFAC sanctions, FinCEN Section 311 (Al-Huda Bank, 2024), and every US-dollar
correspondent's own Bank Secrecy Act programme. File 01 section 4 covers the 2023-2025 dollar bans on Iraqi banks.

### 1a. FinCEN AML/CFT programme rule: the April 2026 "effectiveness" proposal

**History.** The AML Act of 2020 (section 6101(b)) told FinCEN to modernise the programme rule. FinCEN published a first
proposal in July 2024. On **7 April 2026** FinCEN issued a new notice of proposed rulemaking (published in the Federal
Register **10 April 2026**) that **supersedes and withdraws** the 2024 proposal. The OCC, FDIC and NCUA issued a
concurrent proposal the same day; the Federal Reserve Board had not joined at the time of publication. The comment
period closed **9 June 2026**. Searches on 2026-10-06 found **no final rule yet**. If adopted, the rule takes effect
**12 months after the final rule** (the 2024 proposal had 6 months; commenters "nearly unanimous" in asking for more time
for technology changes).

**What changes**

- **"Effective" programme, defined in two prongs.** An institution must *establish* a programme (design) and *maintain*
  it (implementation "in all material respects"). The stated purpose is to "prevent conflating criticisms of program
  design with criticisms of day-to-day implementation". For banks, enforcement or significant supervisory action for a
  maintenance failure is limited to "a significant or systemic failure"; failure to establish a programme is not limited.
- **Four pillars:** (1) risk-based internal policies, procedures and controls (customer due diligence, formerly the
  "fifth pillar", is folded into this one); (2) independent testing; (3) an AML/CFT officer **located in the United
  States** (staff abroad may still perform functions); (4) ongoing training. Board (or equivalent) approval; the
  programme must be available to FinCEN on request.
- **Risk assessment process is mandatory.** It must evaluate ML/TF risk across business activities, products, services,
  distribution channels, customers and geographies; "review and, as appropriate, incorporate" FinCEN's **AML/CFT
  National Priorities**; and be updated "when the financial institution knows, or should know, there was a significant
  change to those risks". FinCEN "does not prescribe any particular processes or methodologies".
- **Resources follow risk.** Institutions allocate "more attention and resources toward higher-risk customers and
  activities" with "significant flexibility and discretion". Examiners assess whether allocation decisions rest on a
  reasonably designed risk assessment; they should not substitute their "subjective judgment".
- **FinCEN consultation.** Bank supervisors must give FinCEN at least **30 days' written notice** (with workpapers)
  before any "significant supervisory action" on AML/CFT.
- **Technology.** In enforcement decisions FinCEN will consider "whether the bank is employing innovative tools such as
  artificial intelligence that demonstrate the effectiveness of the bank's AML/CFT program", and "demonstrable outputs
  evincing the effectiveness", including "effective use of artificial intelligence, federated learning, and other
  advanced monitoring tools". Principle: "responsible experimentation will not incur additional risk of AML/CFT
  enforcement or significant supervisory action".

**Why this matters to an Iraqi bank.** US correspondents will re-baseline their own programmes on this rule, and they
will expect respondents to talk the same language: a documented risk assessment that drives resource allocation,
evidence of effectiveness (outputs, not activity counts), and a record of why low-risk areas get less attention. It is
the same direction as Wolfsberg's effectiveness papers (file 04 sections 2a-2b) and FATF's 2025 risk-based approach
amendments (file 03).

**AKR-TMS hooks**

- **Risk assessment as a living object** that records which national priorities and typologies it considered and why,
  with a trigger to re-run it when the risk profile changes (new product, new list regime, grey-listing, correspondent
  RFI). Links to the dynamic EWRA in file 04.
- **Resource allocation report:** analyst hours and alert volumes by risk segment, so the bank can show that higher-risk
  segments get more attention (and explain why low-risk ones get less).
- **Effectiveness evidence pack:** outputs such as STRs that led to FIU action, law-enforcement requests answered, true
  positives by typology, time to detect; not just counts of alerts closed.
- **Design vs implementation split in the audit workspace:** findings tagged as "design" (rules, thresholds, coverage)
  or "implementation" (backlog, overdue reviews, quality of investigations), mirroring how the US rule separates the two.
- **Experimentation sandbox with a paper trail:** champion-challenger runs and AI pilots logged as governed experiments,
  so innovation can be shown to supervisors rather than hidden from them.

### 1b. US suspicious activity reporting rules and the October 2025 SAR FAQs

**Base rules (31 CFR 1020.320 for banks):** file a SAR for a transaction conducted or attempted by, at or through the bank
that involves or aggregates **at least USD 5,000** where the bank knows, suspects or has reason to suspect it is illicit,
designed to evade BSA reporting, has no apparent lawful purpose, or uses the bank to facilitate crime. Deadline: **30
calendar days** after initial detection of facts that may constitute a basis for filing; **plus 30 more days** to identify
a suspect if none is known, but never more than **60 days** after initial detection. Currency transaction reports (CTRs)
cover cash in or out of more than **USD 10,000** per business day (the same threshold Iraq uses, file 01). Structuring is
"conducting or attempting to conduct one or more transactions in currency, in any amount, at one or more financial
institutions, on one or more days, in any manner, for the purpose of evading" CTR reporting.

**FAQs of 9 October 2025** (FinCEN jointly with the Federal Reserve, FDIC, NCUA and OCC; 4 pages, read in full). They
"do not alter existing BSA legal or regulatory requirements", but they cut work that had become expected in practice:

| # | Question | Answer | Effect on monitoring design |
|---|---|---|---|
| 1 | Must a SAR be filed for transactions at or near the USD 10,000 CTR threshold? | **No.** "The mere presence of a transaction or series of transactions ... at or near the $10,000 CTR threshold is not information sufficient to require the filing of a SAR." A SAR is required only if the bank knows, suspects or has reason to suspect evasion. Programmes should still be designed to detect structuring, commensurate with risk. | Structuring rules must look for *evasion patterns* (splitting across days, branches, linked people, round amounts just below the line, behaviour change after a CTR), not just "amount between 9,000 and 10,000". |
| 2 | Is a separate review required after a SAR to see whether activity continued? | **No.** The 90-day "rule of thumb" from October 2000 had "become interpreted as a requirement or expectation". Banks may rely on risk-based policies and controls instead. | Continuing-activity review becomes a configurable policy, not a hard-coded queue. |
| 3 | Timeline for banks that choose continuing-activity SARs | Day 0 detection; Day 30 initial SAR; Day 120 end of 90-day period; Day 150 continuing SAR (60/150/180 if no suspect was identified). Item 30 should cover the whole 90-day period. | Built-in clock for the optional continuing-activity cycle. |
| 4 | Must the decision *not* to file be documented? | **No** requirement or expectation. If documented, "a short, concise statement ... will likely suffice", with more for complex cases. | Short closure rationale by default; longer structured rationale only where the case is complex. |

**AKR-TMS hooks**

- **Regulatory clock per jurisdiction**, configurable: Iraq's own STR timelines from the CBI and FIU (file 01) are the
  default; the US 30/60-day and 90/120/150-day clocks are available as reference policies for banks whose
  correspondents ask about them.
- **Structuring scenario designed around evasion intent** (the CBI scenario catalogue in file 27 already has cash
  splitting rules; add evasion indicators such as a pattern that starts after a CTR or after a branch query).
- **Closure rationale sized to the case:** a one-line reason picked from a governed list plus free text for simple
  closures; a structured rationale template (facts considered, typology, why not suspicious) for complex ones. Iraq's
  supervisor still expects decisions to be recorded (file 01 section 2d), so the product keeps the record. The point is
  that the record should be short and useful, not long and defensive.
- **Continuing-activity policy switch:** a risk-based watch list after an STR (heightened monitoring of the subject)
  instead of a mandatory manual re-review every 90 days.

### 1c. US model risk guidance: SR 11-7 replaced in April 2026

For fifteen years, US banks validated their transaction monitoring and screening systems under **SR 11-7 / OCC Bulletin
2011-12** (model risk management, 2011) and, from 2021, the **Interagency Statement on Model Risk Management for Bank
Systems Supporting BSA/AML Compliance** (OCC 2021-19, Fed SR 21-8, FDIC FIL-27-2021). Vendors and consultants exported
this practice worldwide; Gulf and Iraqi banks hear it from their correspondents and auditors as "has your TM system been
independently validated?".

**On 17 April 2026 the OCC, Federal Reserve and FDIC issued "Revised Guidance on Model Risk Management"**, rescinding:
SR 11-7 / OCC 2011-12 / FDIC FIL-22-2017; the **2021 BSA/AML model risk statement** (rescinded with no replacement);
OCC Bulletin 1997-24 on credit scoring models; and the Comptroller's Handbook "Model Risk Management" booklet.

Key points of the new guidance (from the Orrick and Davis Polk summaries):

- **Narrower definition of "model":** "a complex quantitative method, system, or approach that applies statistical,
  economic, or financial theories to process input data into quantitative estimates". It **excludes simple arithmetic,
  deterministic rule-based processes**, and software without statistical, economic or financial theory. For AML this
  means a threshold rule ("cash deposits above X in Y days") is not a model; a machine-learning risk score is.
- **Generative and agentic AI are outside its scope** ("novel and rapidly evolving"); banks manage them under broader
  risk management practices.
- **Size:** "most relevant to" banking organisations with **over USD 30 billion** in assets (previously the practical
  line was about USD 1 billion).
- **Proportionality:** risk is judged as inherent risk (complexity, assumptions, data quality) in the context of
  materiality (exposure and purpose); lower-materiality models get lighter oversight.
- **Validation:** still a core component, usually before deployment, with approaches that "may differ across models".
  Independence is judged by substance: "the quality of the validation process depends on the rigor and effectiveness of
  the review rather than on organizational structure". Effective challenge needs "appropriate expertise, sufficient
  independence to permit objectivity, and organizational standing and influence to effect change".
- **Vendor models:** a new section says "the principles of model risk management remain applicable" even when the bank
  cannot fully validate a vendor model or obtain information from the developer; bank customisations must be documented
  and evaluated.
- **Status:** guidance, not enforceable standards ("non-compliance with this guidance will not result in supervisory
  criticism"), though unsafe practices and violations of law remain actionable.

**What it means for an Iraqi bank and for AKR-TMS**

- The CBI has no stand-alone model risk rule (file 01), so correspondents, auditors and the CBI's consultants will keep
  asking for SR 11-7-style evidence for some years. The shift in the US makes the evidence lighter but more targeted:
  focus validation effort on the components that are real models (scores, anomaly detection, name-match scoring) and
  keep rules governed as configuration (version, owner, rationale, test results, change approval).
- **Two governance tracks in the product:** (1) *rules and thresholds* as governed configuration with change control,
  below-the-line testing and effectiveness metrics; (2) *models* (supervised scores, unsupervised anomaly detection,
  fuzzy-match scoring) with model cards, a validation pack, drift and stability monitoring, and outcome analysis.
- **Generative AI** (investigator summaries, STR narrative drafts) sits on a third track: human review of every output,
  prompt and output logging, no autonomous decisions (file 04 section 2b; file 29).
- **Vendor transparency is a selling point.** The new US guidance accepts that banks often cannot validate vendor
  models. AKR-TMS can remove that problem by shipping full model documentation, feature lists, training-data
  descriptions and a bank-run validation harness, so the bank's validator can test the model on the bank's own data.

### 1d. FFIEC BSA/AML Examination Manual: what US examiners look for in a monitoring system

The FFIEC manual (online "InfoBase" at bsaaml.ffiec.gov; it blocks automated reading, so this section relies on its
well-known text as quoted by search snippets and practitioners) is the examiner's checklist for US banks. Correspondent
banks apply it to themselves and expect respondents' systems to look similar. Its suspicious activity monitoring model
(Appendix S, "Key Suspicious Activity Monitoring Components") has five parts:

1. **Identification of unusual activity** from several sources: employee referrals, law enforcement inquiries, other
   referrals, and output from transaction monitoring reports or automated ("surveillance") monitoring systems, which
   may be rule-based or "intelligent" (learning) systems.
2. **Managing alerts:** referral routes from every business line to the unit that evaluates unusual activity, with
   adequate staffing for the bank's risk profile and transaction volume.
3. **SAR decision-making** (the 2014 text asked banks to document the specific reason for filing or not filing; the
   October 2025 FAQs, section 1b, removed that expectation for no-file decisions).
4. **SAR completion and filing** (timeliness, quality of narrative).
5. **Monitoring and SAR filing on continuing activity** (now optional, section 1b).

On the system itself, the manual's most-quoted lines are:

- Banks must **understand their filtering criteria** (the rules, scenarios and thresholds).
- "Management should **periodically review and test the filtering criteria and thresholds** established to ensure that
  they are still effective."
- "The monitoring system's **programming methodology and effectiveness should be independently validated** to ensure
  that the models are detecting potentially suspicious activity."
- "The independent validation process should also **verify the policies in place** and that management is complying
  with those policies."
- In the examination procedures, examiners check whether systems use "reasonable filtering criteria whose programming
  has been independently verified".

Other manual themes that shape a system design: the BSA/AML risk assessment drives monitoring coverage; records are
kept **five years**; the board is told about SAR filings; law enforcement inquiries and subpoenas are a source of
alerts but not, by themselves, a reason to file.

**AKR-TMS hooks:** a plain-language explanation of every rule and threshold, generated from its configuration (so
the bank "understands its filtering criteria"); periodic threshold review tasks with below-the-line results attached; a
validation pack (rule logic, data lineage from the CBS replica, test cases, results) the bank hands to its validator; a
source tag on every alert (system, employee referral, authority request, RFI) so referral routes are visible; five-year
minimum retention as a configurable floor (Iraq's own retention rules in file 01 apply first).

### 1e. OFAC sanctions: how US sanctions reach an Iraqi bank

**Why an Iraqi bank is exposed even though it is not a "US person"**

- **Dollar clearing.** A USD payment that touches a US bank, or a foreign bank's US branch, brings US jurisdiction. A
  foreign bank that sends a payment involving a blocked person through the US can cause a US bank to violate sanctions
  ("causing" liability). OFAC's 2019 framework lists among the root causes of violations "utilizing the U.S.
  financial system, or processing payments to or through U.S. financial institutions, for commercial transactions
  involving or relating to sanctioned persons" and "facilitating transactions by non-U.S. persons".
- **Secondary sanctions.** Iran programmes (CISADA section 104(c) and later authorities) let Treasury cut off or
  restrict US correspondent accounts of a foreign financial institution that knowingly conducts significant
  transactions for the IRGC or other designated Iran-linked persons. This is the mechanism behind the 2023-2025 dollar
  bans on Iraqi banks (file 01 section 4).
- **Correspondent exit.** Even where no OFAC power is used, a US or non-US dollar correspondent will close the account
  of a respondent it sees as a sanctions risk.

**Programmes that matter in Iraq** (entity and programme names only; no personal names in this file)

| Programme / authority | What it covers | Iraq relevance |
|---|---|---|
| Iraq-related sanctions (E.O. 13315 of 2003 on the former regime; E.O. 13438 of 2007 on threats to stabilisation) | Former regime assets; persons who threaten peace or stability in Iraq | Militia-linked persons and companies |
| Counter-terrorism (E.O. 13224, SDGT) and Foreign Terrorist Organizations (FTO, State Department) | Terrorist groups and supporters; FTO adds material-support criminal liability | Kata'ib Hizballah and Asa'ib Ahl al-Haq (FTOs); on **17 September 2025** State added **Harakat al-Nujaba, Kata'ib Sayyid al-Shuhada, Harakat Ansar Allah al-Awfiya and Kata'ib al-Imam Ali** as FTOs (previously SDGTs) |
| Iran programmes (ITSR, E.O. 13902, IRGC-related) | Iranian government, IRGC, oil and petrochemicals, shipping | Iranian oil "disguised as Iraqi crude"; dollar leakage to Iran |
| Global Magnitsky / corruption | Serious corruption and human rights abuse | Corruption in public contracts |
| FinCEN Section 311 | "Primary money laundering concern" special measures | Al-Huda Bank (final rule 2 August 2024, file 01) |

**2025-2026 OFAC actions with an Iraq nexus** (from Treasury and press reports)

- **3 July 2025 and 2 September 2025:** designations of people, companies and vessels smuggling **Iranian oil
  disguised as Iraqi crude**.
- **9 October 2025:** a **state-owned Iraqi company** said to be controlled by Kata'ib Hizballah, which used
  **sub-contracting to divert funds from Iraqi government contracts** and an agricultural front to conceal weapons
  smuggling ("a miniature PMF-run economy"); and **three Iraqi bankers** accused of using their positions to
  "generate revenue and launder money" for Kata'ib Hizballah, Asa'ib Ahl al-Haq and the IRGC, moving "tens of millions".
  Iraq's government called the company designation "unilateral and unjustified".
- **17 April 2026:** seven commanders of Kata'ib Hizballah, Kata'ib Sayyid al-Shuhada, Harakat al-Nujaba and Asa'ib Ahl
  al-Haq, responsible for attacks on US personnel (reported as part of an "Economic Fury" campaign).

Typologies an Iraqi bank's system must therefore see: state-contract sub-contracting chains that end in militia-linked
companies; insiders (bank staff and officers) who use their position to move funds; front companies in agriculture,
trading and logistics; oil-trade payments where the origin of the crude is doubtful; dollar flows that benefit Iran.

**The 50 percent rule (OFAC guidance, 2014).** An entity owned **50 percent or more, directly or indirectly, in the
aggregate**, by one or more blocked persons is itself blocked, even if it is not on the SDN list. Ownership by several
blocked persons is added together; control without ownership does not by itself trigger the rule under OFAC guidance
(but is a red flag). EU and UK rules differ (more than 50 percent, and control also counts; sections 2 and 3). A
screening system that only matches names against the list misses these entities.

**OFAC's compliance framework (May 2019)**: five components (management commitment, risk assessment, internal
controls, testing and auditing, training) and a list of root causes from enforcement cases, several of them about
systems: screening software or filters that were not updated, were misconfigured, or did not screen relevant
identifiers (such as SWIFT BICs, addresses or vessel identifiers); weak due diligence on customers' ownership and
business; non-standard payment practices (stripping or vague fields); decentralised compliance.

**Record keeping: now 10 years.** The 21st Century Peace through Strength Act (24 April 2024, section 3111) extended
the statute of limitations for IEEPA and TWEA violations from 5 to 10 years. OFAC's final rule (published 21 March
2025) extends the record-keeping requirement for transactions and blocked property to **at least 10 years**. OFAC
said a conflicting law (for example EU rules requiring deletion after five years) does not excuse compliance.

**AKR-TMS hooks**

- **OFAC lists in the unified watchlist** (SDN and consolidated non-SDN lists, with programme tags), plus FinCEN 311
  institutions and FTO lists, alongside UN, Iraqi, EU and UK lists (file 04 section 4d).
- **Ownership-aware screening:** an ownership graph built from the bank's own KYC data (shareholders, beneficial owners,
  directors) and, where licensed, commercial ownership data, that computes aggregated ownership by blocked persons under
  each regime's rule (US 50 percent or more; EU and UK more than 50 percent or control) and raises "derived designation"
  alerts.
- **Identifier screening beyond names:** BICs, IBANs, vessel IMO numbers, addresses, phone numbers and email addresses
  in payment messages and KYC records, since OFAC enforcement cases often turn on missed non-name identifiers.
- **Iraq militia economy typology pack:** state-contract sub-contracting chains, insider-facilitated flows (staff
  accounts and accounts they manage), agricultural and trading front companies, oil-trade payments with doubtful
  origin; combined with the Iran nexus flag from file 04.
- **Retention policy per record class** with a 10-year option for sanctions-related records (alerts, screening
  results, blocked or rejected payments), so banks dealing in USD can meet what their correspondents now expect.

### 1f. NYDFS Part 504: the most system-specific AML rule anywhere

New York's Department of Financial Services (DFS) supervises the New York branches of many foreign banks that clear US
dollars for the Middle East (Gulf, Jordanian, Pakistani and other banks clear USD through New York). Part 504 of the
Superintendent's regulations ("Banking Division Transaction Monitoring and Filtering Program Requirements and
Certifications", 3 NYCRR 504) was finalised on **30 June 2016**, took effect **1 January 2017**, and has required an
annual certification every **15 April since 2018**. It applies to DFS-licensed banks, trust companies, branches and
agencies of foreign banks, money transmitters and cheque cashers (for foreign banks, only the New York office).

It matters to Iraqi banks indirectly but strongly: their USD and regional correspondents, and those correspondents'
New York offices, design their own monitoring to Part 504 and ask the same questions of respondents. It is also the
clearest public specification of what a "good" TM and screening system must contain.

**Transaction Monitoring Program** (post-execution monitoring for BSA violations and suspicious activity), "to the
extent applicable", must:

1. be based on the institution's risk assessment;
2. be reviewed and updated periodically, at risk-based intervals, to reflect changes in laws, regulations and other
   relevant information (for example regulatory warnings and typologies);
3. appropriately match BSA/AML risks to the institution's businesses, products, services and customers;
4. use detection scenarios with threshold values and amounts designed to detect potential money laundering or other
   suspicious activity;
5. document the detection scenarios and their underlying assumptions, parameters and thresholds;
6. include protocols for how alerts are generated, investigated and documented, how decisions to file or not file are
   made, who makes them, and how the decisions are recorded;
7. include "end-to-end, pre- and post-implementation testing" and ongoing analysis of the scenarios, rules, threshold
   values, parameters and assumptions.

**Filtering Program** (manual or automated) reasonably designed to interdict transactions prohibited by OFAC, with:

1. end-to-end, pre- and post-implementation testing, including data matching, whether the OFAC list and threshold
   settings map to the institution's risks, the logic of the matching technology, **model validation**, and data
   input and output;
2. ongoing analysis of the logic and performance of the name and account matching technology, the lists used and the
   threshold settings;
3. documentation of the intent and design of the filtering tools, processes and technology.

**Attributes common to both programs:**

- identification of **all data sources** that contain relevant data;
- validation of the **integrity, accuracy and quality of data** so that accurate and complete data flows through;
- **data extraction and loading processes** that ensure a complete and accurate transfer from source systems to the
  monitoring and filtering systems;
- governance and management oversight, including **change management** so that changes are "defined, managed,
  controlled, reported, and audited";
- vendor selection process where a third party supplies, installs, implements or tests any part;
- funding to design, implement and maintain the programs;
- qualified personnel or consultants for design, implementation, operation, testing, validation, ongoing analysis,
  case management and decision-making;
- periodic training of all stakeholders.

DFS was concerned about banks changing their systems "for purposes of minimizing generated alerts or avoiding AML/CFT
compliance obligations because of insufficient staffing or other resources". The proposed ban was dropped from the
final text, but changes must be documented and available for inspection.

**Annual certification.** By 15 April each year the board (by resolution) or a senior officer ("compliance finding")
certifies compliance, "to the best of" their knowledge, based on documents, reports, certifications and opinions of
staff, vendors and others. Supporting records are kept for **five years** for DFS review. A false certification can
lead to individual liability under other laws.

DFS enforcement against foreign-bank branches that clear dollars for the region has repeatedly cited weak transaction
monitoring and filtering (examples, including Gulf and South Asian banks, belong in file 20).

**AKR-TMS hooks (this is a near-direct specification)**

- **Data source inventory and lineage**: every CBS table and column the mapping layer reads, with owner, refresh time,
  and which scenarios and screening checks depend on it.
- **Data quality and completeness controls** on every load from the read-only replica: row counts and control totals
  reconciled against the CBS, null and default-value checks on fields that scenarios depend on, late-data alarms.
  This is the "coverage reconciliation" requirement already derived from BCBS (file 04, B-01).
- **Scenario documentation generated from configuration**: purpose, typology, assumptions, parameters, thresholds,
  data used, last test, last tuning, owner.
- **Pre- and post-implementation testing harness** for rules and for screening (test data sets, expected hits, results
  stored with the change request).
- **Change management** for rules, thresholds, lists and match settings with maker-checker approval, reason, impact
  analysis (alert volume and coverage before and after) and a flag for changes whose main effect is to cut alert volume.
- **Annual certification pack** (also useful for the CBI's own expectations and for correspondents): an auto-assembled
  file of the year's testing, tuning, data quality, validation, staffing and backlog evidence, which the MLRO and board
  can sign off.

### 1g. FinCEN Advisory FIN-2025-A002 (6 June 2025): Iranian oil smuggling, shadow banking and weapons procurement

Read in full (18 pages). It replaces the 2018 Iran advisory (FIN-2018-A006), supports the "maximum pressure" policy
(NSPM-2, 4 February 2025), and asks US institutions to use the key term **"IRAN-2025-A002"** in SAR field 2 and the
narrative. It came out the same day OFAC designated more than 40 persons in an Iranian "shadow banking" network
(exchange houses in Iran running front companies in the UAE and Hong Kong).

It describes the exact model an Iraqi bank must guard against: **"third-country exchange houses and trading companies
rely on their regional banks, which have correspondent banking relationships with U.S. financial institutions, to
access the U.S. financial system"**, converting currency into dollars "using forged documents that falsify the source
of the funds"; these exchange houses "often are located in jurisdictions in close geographical proximity to Iran and
primarily process commercial transactions rather than personal remittances". Iraq fits that description, which is why
US correspondents scrutinise Iraqi banks' exchange-company and trading-company customers.

**Red flags (condensed) and how AKR-TMS can detect them from bank data**

| Red flag (FinCEN 2025) | Detection idea in AKR-TMS |
|---|---|
| Oil-shipping documents inconsistent with maritime databases (port calls in Iran not disclosed); vessels with recent name or flag changes, reused or scrapped IMO numbers; "Malaysian blend"; ship-to-ship transfers; bills of lading with no consignee | Trade finance module: vessel and IMO screening, document-field checks, optional maritime-data connector (licensed) |
| Transactions moving through **multiple exchange houses or trading companies**, adding fees, in patterns that do not match normal commercial practice | Payment-chain analysis across the bank's exchange-company customers and nostro flows; layering scenario |
| An exchange house or trading company near Iran using forged documents to hide parties and reach USD through regional banks' correspondents | Exchange-company (PSP/MSB) segment with document-consistency checks and counterpart concentration (file 04 section 2r) |
| Wires or deposits with **no or incomplete source-of-funds information**, or not matching the customer's line of business, involving high-risk jurisdictions | Payment-quality monitor (meaningless or missing fields) plus KYC declared-vs-actual activity |
| **UAE free-zone general trading company** with opaque ownership, counterparties mostly in Singapore and Hong Kong, accounts at several UAE banks | Counterparty profiling: free-zone trading counterparties flagged by geography and ownership opacity |
| Hong Kong company banking with a Chinese non-resident account, little web presence, co-located with similar companies, recently incorporated, large payments to UAE general traders | Counterparty risk enrichment: company age, address sharing, name similarity clusters |
| Front-company indicators: opaque ownership, obscure directors, residential or shared addresses, links to previously sanctioned companies | Entity resolution and network graph: shared addresses, phones, directors, names |
| Declared business inconsistent with transactions; receives from commodity traders, pays electronics suppliers; Middle East company receiving from petroleum firms and paying Hong Kong and Chinese electronics firms | "In-flow sector vs out-flow sector" mismatch scenario using merchant category or counterparty sector tags |
| **Several companies incorporated at about the same time** sharing counterparties, addresses, owners or name patterns, with similar transaction profiles and large recurring amounts | Cluster detection over customers and counterparties (similar onboarding dates, shared attributes, similar flow profiles) |

**AKR-TMS hooks:** ship an **Iran shadow-banking typology pack** built from this advisory and OFAC's exchange-house
guidance, tagged so the bank can show correspondents which red flags are covered (coverage matrix, file 04); support
**authority key terms** in STR narratives and goAML exports (Iraq's FIU and CBI may issue their own campaign codes);
treat **new advisories as a governed input**: each one creates a review task to map its red flags to scenarios, with
the result logged in the regulatory obligations register.

## 2. European Union

The EU matters to Iraqi banks through euro clearing and trade finance with European banks, through EU sanctions, and
because the EU's 2024 AML package is the most detailed single rulebook on what obliged entities must do. From July
2027 the same text applies directly in all member states, so every EU correspondent of an Iraqi bank will apply the same
rules.

### 2a. The 2024 AML package and its timeline

| Instrument | What it is | Key dates |
|---|---|---|
| **AMLR**, Regulation (EU) 2024/1624 | Directly applicable "single rulebook" for obliged entities (CDD, beneficial ownership, monitoring, reporting, records) | In force 9 July 2024; **applies from 10 July 2027** |
| **AMLD6**, Directive (EU) 2024/1640 | National supervisors, FIUs (powers incl. suspension), beneficial ownership and bank account registers | Transposition mostly by 10 July 2027 |
| **AMLA Regulation**, (EU) 2024/1620 | Creates the EU Anti-Money Laundering Authority (AMLA) in Frankfurt | AMLA operational from mid-2025; **direct supervision of up to 40 selected high-risk cross-border groups from 2028**; issues guidelines and technical standards from 2025-2027 |
| **Transfer of Funds Regulation (recast)**, (EU) 2023/1113 | Payer and payee information in transfers, extended to crypto-assets (travel rule) | Applies from 30 December 2024 |

### 2b. AMLR articles that specify what a monitoring system must do (read in the EUR-Lex text)

- **Article 20(1)** lists CDD measures including (d) "verifying whether the customer or the beneficial owners are subject
  to targeted financial sanctions, and, in the case of a customer ... who is a legal entity, whether natural or legal
  persons subject to targeted financial sanctions **control** the legal entity or have **more than 50 %** of the
  proprietary rights ... or majority interest in it, **whether individually or collectively**"; and (f) ongoing monitoring
  "to ensure that the transactions being conducted are consistent with the obliged entity's knowledge of the customer,
  the business and risk profile, including where necessary the source of funds". Article 20(4): entities "shall at all
  times be able to demonstrate to their supervisors that the measures taken are appropriate".
- **Article 21(3):** keep records of CDD actions "including records of the decisions taken and the relevant supporting
  documents and justifications".
- **Article 22(1)(a):** identity data for a natural person includes "all names and surnames", "place and full date of
  birth", nationalities and national identification number. **Article 24(2)(a)** recognises "different ways of
  transliteration" as a benign cause of discrepancies with beneficial ownership registers.
- **Article 25:** before entering a relationship, understand purpose, "the estimated amount of the envisaged
  activities", source of funds, destination of funds, and occupation or business (the declared profile that
  monitoring later compares against).
- **Article 26 (ongoing monitoring):**
  - monitor transactions for consistency with knowledge of the customer, business activity and risk profile, and
    "where necessary, with the information about the origin and destination of the funds";
  - where a relationship spans several products, CDD must "cover all those products and services";
  - within a group, take into account the customer's relationships with other group entities;
  - **maximum update intervals for customer information: 1 year for higher-risk customers (enhanced due diligence),
    5 years for all others**, plus event-driven reviews when circumstances change or a relevant fact becomes known;
  - **re-verify sanctions status regularly and, for credit and financial institutions, "upon any new designation"**.
- **Article 27:** for customers subject to **UN financial sanctions** (or owned more than 50 percent or controlled by
  such persons), keep records of funds held at the moment the UN designation is made public, and of transactions
  attempted and carried out, until the EU transposes the designation. This closes the gap between a UN listing and its
  application (relevant to file 04 section 4c on freezing "without delay").
- **Article 69:** report suspicions "regardless of the amount involved", including attempted transactions and
  suspicions arising from failed CDD; answer FIU requests **within 5 working days** (FIUs may shorten this to less than 24
  hours in urgent cases). Suspicion is to be based on, among other things, "the link between several transactions or
  activities, the origin, destination or use of funds" and consistency with the customer's risk profile. AMLA must
  issue guidelines on indicators of suspicious activity by 10 July 2027 and technical standards on the STR format.
- **Article 71:** refrain from carrying out transactions known or suspected to be linked to crime until reported; the
  entity may proceed if the FIU gives no contrary instruction **within 3 working days**.
- **Article 73:** prohibition of disclosure (tipping-off), with defined exceptions (within a group; between obliged
  entities involved in the same transaction).
- **Article 75:** partnerships for information sharing between obliged entities, with safeguards (file 21 covers
  data-sharing technology).
- **Article 76(5): automated decisions and AI.** Entities "may adopt decisions resulting from automated processes,
  including profiling ... or from processes involving AI systems", provided that the data are limited to CDD data,
  that any decision to enter, refuse, maintain or end a relationship, or to increase or decrease CDD, "is subject to
  **meaningful human intervention**", and that "the customer may obtain an explanation of the decision ... and may
  challenge that decision, except in relation to a report" (an STR). **Article 76(2)(c):** no decisions "that would lead
  to biased and discriminatory outcomes" from special-category data.
- **Article 77: record retention.** Keep CDD documents, transaction evidence, and "a record of the assessment ...
  including the information and circumstances considered and the results of such assessment, **whether or not such
  assessment results in a suspicious transaction report**" (contrast the US 2025 FAQ, section 1b), records must not be
  redacted; references instead of copies are allowed only if the information "cannot be modified or altered".
  Retention is **5 years** after the relationship ends, then personal data must be deleted (extendable by up to 5 years
  on authority request). OFAC now requires 10 years for sanctions records (section 1e): a bank dealing in both USD and
  EUR needs per-record-class retention.
- **Article 78:** "systems in place that enable them to respond fully and speedily" to FIU enquiries on whether they had
  a relationship with specified persons in the past five years, through secure, confidential channels.
- **Correspondent banking (Articles 36-39):** EDD for third-country respondents (understand the business, assess AML/CFT
  controls, senior approval, document responsibilities, payable-through accounts). **Article 38 is new and matters to
  Iraqi banks:** AMLA can issue a **recommendation about an individual third-country respondent institution** that is
  in "serious, repeated or systematic breach" or whose controls are "not commensurate" with its risks; EU banks must
  then not open new relationships and must review and terminate existing ones unless mitigation is adequate. AMLA must
  first consult the respondent's home supervisor (two months). An Iraqi bank named in such a recommendation would lose
  EU correspondents across all member states at once.
- **PEPs (Articles 42-46):** senior management approval, source of wealth and funds, enhanced ongoing monitoring;
  measures continue for **at least 12 months** after a person leaves office; family members and close associates are
  covered; member states publish lists of prominent public functions, compiled into an EU list.

**AMLA draft guidelines on ongoing monitoring** (Article 26(5); consultation published **3 June 2026**, closed 3
September 2026, final text expected Q4 2026): they describe three monitoring layers, "pre-transaction monitoring",
"real-time monitoring" at execution, and "post-transaction monitoring, aimed at pattern detection and behavioural
analysis"; cover "activities" as well as transactions; and require that outputs of automated or AI systems "be
explainable and verifiable", with entities remaining "accountable for monitoring decisions" rather than relying on
"algorithmic outcomes". Customer information updates follow "a documented assessment of the circumstances" rather than
automatic re-collection of documents.

### 2c. EBA guidelines on restrictive measures (EBA/GL/2024/14 and /15, apply from 30 December 2025)

Published 14 November 2024 (76-page final report, read). GL/2024/14 applies to all institutions under the EBA's remit
(governance, a **restrictive measures exposure assessment** reviewed at least yearly, a senior staff member responsible,
management body oversight including a yearly assessment of whether technology and staff are adequate). GL/2024/15
applies to all payment service providers, including banks, when executing transfers, and is an explicit screening
system specification:

- **Choice and review of the screening system:** chosen and validated against the exposure assessment; performance
  reviewed **at least once a year** and immediately when there are grounds for concern; the institution must
  "understand and document the screening system's capabilities and limitations" and be able to demonstrate adequacy.
- **List management:** detect when measures are adopted, amended or lifted and update the screened dataset
  "**immediately** after" entry into force.
- **Data to screen (customers):** names "in the original and/or transliteration", date of birth, legal-entity names,
  "any other names, aliases, trade names" on the lists (not screening available aliases must be justified); beneficial
  owners through ownership and through control; persons acting on the customer's behalf.
- **Customer base screening:** regular full re-screening with a risk-based frequency, plus mandatory triggers: any new
  or changed designation, onboarding, significant CDD changes (name, residence, nationality, business), and suspicion of
  circumvention.
- **Transfer screening:** screen transfers **before** making funds available to the payee; screen all parties,
  purpose and relevant free-text fields, and **all PSPs in the chain including intermediaries and correspondents, by
  BIC and other identifiers**; screen information that arrives later as well.
- **Whitelisting (good-guy lists):** allowed to avoid repeated false alerts, with documented reasons, and the list must
  be reviewed immediately when measures change or customer data changes.
- **Calibration:** per regime, set matching parameters "by checking the thresholds of true positive results associated
  with different percentages of matching", neither too sensitive nor too lax; use **fuzzy matching** (spelling,
  pattern or sound); document the rationale; recalibrate periodically.
- **Alert handling:** investigate all potential matches without delay; record decisions; **two-person review for
  higher-exposure situations**; alerts must show which element of the measure was matched; use secondary identifiers
  (other ID data, residence, nationality, management structure, contact details) to resolve; if unresolved, do not
  provide the service until an informed decision is made.
- **Ownership and control:** apply the EU Council's criteria on ownership and control and beneficial ownership
  criteria; use registers; consult the national authority if inconclusive.
- **Sectoral and country measures:** screen nationality, place of birth, residence, countries of transfer, and free text
  about goods, vessels and origin; consider IP geolocation and proxy detection where warranted; check OCR or MRZ tools
  capture data accurately.
- **Circumvention:** follow typologies from authorities, FIUs and partnerships; detect attempts to "omit, delete or
  alter information in payment messages", route through connected persons, structure transfers, hide ownership or use
  forged documents; consider aggregated analysis of flows to sanctioned countries and known circumvention hubs.
- **Freezing and reporting:** suspend without delay any operation that triggers a possible match; on confirmation,
  freeze immediately and stop the transfer; report actions, violations and executed breaching transfers (including
  incidents in the screening system) to the competent authority.

### 2d. Transfer of Funds Regulation and the EBA "travel rule" guidelines (EBA/GL/2024/11)

Applicable from **30 December 2024** (replacing the 2017 joint guidelines on missing information): payment service
providers must detect **missing, incomplete or meaningless** payer and payee information, decide on a risk basis
whether to execute, reject or suspend, and identify and report **repeatedly failing** counterparty PSPs to the
competent authority. This is the EU version of FATF R.16 (file 03) and matches the payment-quality monitor already in
the AKR-TMS feature list.

### 2e. EU high-risk third countries and Iraq

Iraq was on the EU list of high-risk third countries (Delegated Regulation 2016/1675) from 2016 until the Commission
removed it in early 2022, after concluding in Q4 2021 that Iraq had addressed its strategic deficiencies. The latest
amendments found (Delegated Regulations 2026/46 and 2026/83, adopted December 2025, published 9 January 2026) added
Bolivia, the British Virgin Islands and Russia and removed six African countries; they do not concern Iraq. Iraq's
return to the FATF grey list on 19 June 2026 is normally followed by an EU delegated act some months later; **no act
re-listing Iraq was found as of 2026-10-06**. Once Iraq is listed, EU obliged entities must apply enhanced due
diligence to business relationships and transactions involving Iraq, including correspondent relationships with Iraqi
banks. Iraqi banks should plan for that now.

### 2f. EU sanctions relevant to Iraq

The EU applies UN measures through its own regulations (including Council Regulation (EC) No 1210/2003 on the former
Iraqi regime, implementing UN resolution 1483) plus autonomous regimes (counter-terrorism, Iran, Syria, Russia).
Ownership and control are judged under Council guidance: **more than 50 percent** ownership **or** control by a
designated person. Unlike the US rule (50 percent or more, aggregated, ownership only), EU control tests can catch
entities with lower ownership.

### 2g. AKR-TMS hooks from the EU regime

- **Retention engine per record class and jurisdiction:** EU 5 years after relationship end with deletion; OFAC 10
  years; Iraqi rules first (file 01). Conflicts are resolved by policy, with legal holds.
- **Record of every assessment, including "no STR" outcomes** (AMLR Article 77), without redaction, in tamper-evident
  storage (the "cannot be modified or altered" condition fits MinIO object lock).
- **Human-in-the-loop by design for automated decisions** (AMLR Article 76(5)): models and rules can propose risk
  changes, exits or EDD, but a named person decides; the customer-facing explanation is generated from reason codes
  (never for STR-related decisions).
- **Delta re-screening on every new designation** and full re-screening on a risk-based cycle, with the trigger list
  of EBA/GL/2024/15 built in (designation change, onboarding, CDD change, circumvention suspicion).
- **Screening of every party and identifier in payments**, including intermediary and correspondent BICs, purpose
  and free text, with per-regime calibration based on measured true-positive rates at different match scores.
- **Whitelist governance:** reasons, owner, expiry, automatic review on list or customer change (EBA 2024/15 para 13).
- **Two-person review rule** configurable by exposure (EBA 2024/15 para 32(d)).
- **FIU enquiry tool** that answers "did we have a relationship with these persons in the last five years, and what
  was it?" within the 5-working-day deadline (AMLR Articles 69 and 78; also Egmont, file 04 section 3).
- **Correspondent-risk early warning:** the respondent self-view (file 04) should anticipate what an AMLA Article 38
  assessment would look at (repeated breaches, controls not commensurate with risk).
- **UN-designation gap register** (AMLR Article 27 logic) for the window between a UN listing and its national
  application, which in Iraq goes through the CFTF and IQTFS (file 04 section 4c).

## 3. United Kingdom

UK rules reach Iraqi banks through sterling clearing and through UK banks' correspondent and trade relationships. UK
enforcement is also the richest public record of system failures (Metro Bank, Starling, Monzo, Barclays, Nationwide;
detailed in file 20).

### 3a. Money Laundering Regulations 2017 and the 2026 amendment

The Money Laundering, Terrorist Financing and Transfer of Funds (Information on the Payer) Regulations 2017 (MLRs) are
the UK rulebook. HM Treasury's advisory notice of **22 June 2026** (read through the GOV.UK content API) says:

- **Iraq is a "high-risk third country" (HRTC) for UK purposes** since the FATF plenary of June 2026: the MLR definition
  is "a country named on either of the ... lists published by the Financial Action Task Force as they have effect from
  time to time". The notice's list of jurisdictions under increased monitoring includes **Iraq, Kuwait, Lebanon and
  Syria**, all relevant to Iraqi payment corridors; Iran and the DPRK are under a call for action.
- Until 30 June 2026, regulation 33(1)(b) required enhanced CDD and enhanced ongoing monitoring for **every** business
  relationship with a person established in an HRTC (for a bank, "having its principal regulatory authority in that
  country").
- **The Money Laundering and Terrorist Financing (Amendment) Regulations 2026** (laid 25 March 2026, **in force 30 June
  2026**) narrow that mandatory trigger to countries under a FATF **call for action** only. For grey-listed countries
  such as Iraq, UK firms "will continue to be required to take FATF mutual evaluations into account as a relevant
  geographical risk factor" and must apply EDD where they identify high risk.

Effect: a UK correspondent is no longer forced to apply EDD to every Iraqi relationship just because of the grey
listing, but it will still treat Iraq as high risk in its own assessment. The quality of an Iraqi bank's controls, and
the evidence it can show, decides whether the UK bank keeps the relationship.

Other UK elements that matter for system design: the **JMLSG guidance** (industry guidance approved by HM Treasury;
Part I covers CDD and monitoring, Part II has sector chapters including correspondent banking and trade finance), which
UK supervisors and courts treat as the benchmark; the **Economic Crime and Corporate Transparency Act 2023**, whose
information-sharing provisions (in force January 2024) remove civil liability for breach of confidence when regulated
firms share customer information to prevent, detect or investigate economic crime, and whose "failure to prevent
fraud" offence has applied to large organisations since **1 September 2025**.

### 3b. FCA supervision themes, 2024-2026

- **Enforcement on monitoring systems** (details in file 20): Metro Bank (2024, transactions not monitored for years
  because of a data feed set-up), Starling (2024, screening against only part of the sanctions list), Monzo (2025),
  Barclays (2025) and Nationwide (2025). The repeated theme is that a system existed but did not see all the data or
  all the list.
- **FCA multi-firm review of sanctions systems and controls (published 28 May 2026; 150 firms).** Firms "performed well
  on exact matches but poorly on **spelling variants, one-word names, digits, titles and non-Latin characters**", which
  is precisely the Arabic-name problem (file 22). The FCA found errors and omissions in **vendor-supplied lists**
  (poor data quality and data transfer between systems) and delays in updating the UK Sanctions List; it has "little
  patience for failures caused by **backlogs**, particularly if those are due to known resourcing issues"; **35 percent
  of breaches reported in 2025 related to activity from earlier years** (late detection); management information and
  oversight were uneven; and "**vendor reliance does not equate to regulatory assurance.** Firms must ensure that they
  understand and challenge their vendors' logic and document the challenge". Breach reports increasingly involve Iran
  and North Korea, not only Russia.
- An FCA review of alternative asset managers (2025-26, 242 firms) found nearly three in ten had no formal transaction
  monitoring system and weak customer risk assessment documentation.

### 3c. OFSI (HM Treasury's Office of Financial Sanctions Implementation)

UK financial sanctions are strict liability for civil penalties (since 2022). The largest bank case so far: on
**2 September 2026** OFSI fined **Citibank N.A., London Branch GBP 4.73 million** (baseline about GBP 7.89 million,
reduced by 20 percent for voluntary disclosure and 20 percent for settlement) for **970 transactions worth about GBP 19.7
million** involving designated persons under the Russia and Global Anti-Corruption regimes, including delays in
freezing, payments through sanctioned banks, fees deducted from frozen accounts and late reporting of frozen assets.
OFSI attributed the breaches to "foreseeable systems and controls weaknesses, delays in sanctions alert handling,
inadequate escalation procedures and human error", including struggles with alert volumes during rapid regulatory
change and weak assessments of general licences and of ownership and control. OFSI's lessons: robust screening,
**stress-testing frameworks ahead of geopolitical events**, timely self-reporting, careful licence assessment, and
systems that can handle large alert volumes when lists change quickly.

### 3d. AKR-TMS hooks from the UK regime

- **Country risk table fed by FATF list changes** three times a year (February, June, October), with the UK, EU and US
  consequences recorded separately; for Iraqi banks the important rows are Iraq itself, Kuwait, Lebanon, Syria, Iran and
  the UAE (FATF lists change; the table must be governed and dated).
- **Screening quality tests for the FCA's weak spots:** spelling variants, one-word names, digits, titles and honorifics
  (Sayyid, Sheikh, Hajji, Dr), and non-Latin script; run in the screening test harness on every configuration change.
- **List integrity checks on every list load:** record counts and checksums against the publisher's file, field-level
  diff, and an alarm if vendor-supplied data deviate from the official source (UN XML, IQTFS, OFAC, EU, UK).
- **Backlog management as a control:** ageing dashboards, service levels per alert type, surge capacity planning, and an
  escalation when backlog growth is driven by resourcing.
- **Late-detection metric:** share of confirmed matches or STRs whose underlying activity is older than N months, as a
  measure of control timeliness (the FCA's 35 percent figure is the benchmark).
- **Surge mode for list events:** when a large designation package lands (for example the Iran snapback in September
  2025, file 04), the system re-prioritises queues, pre-clears known false positives using the pair memory, and reports
  progress to the MLRO.
- **Frozen-asset ledger with fee blocking:** a register of frozen accounts verified against the CBS replica that flags
  any debit, including bank fees, after the freeze date (OFSI fined Citibank partly for fees deducted from frozen
  accounts).
- **Vendor-challenge record:** where the bank uses third-party lists or data, AKR-TMS documents tests and challenges
  of that vendor's data and logic.

## 4. GCC and regional peers

Since the CBI's electronic platform ended in 2025, Iraqi banks deal directly with correspondents abroad, mostly in
Jordan, the UAE, Kuwait, Lebanon and Turkey (file 01 section 4). Those correspondents apply their own regulators' rules,
and the UAE is also the hub named in US Iran advisories (section 1g). Regional rules therefore shape what Iraqi banks
are asked for every day.

### 4a. United Arab Emirates: the CBUAE guidance on transaction monitoring and sanctions screening (13 September 2021)

The Central Bank of the UAE issued "Guidance for Licensed Financial Institutions on Transaction Monitoring and Sanctions
Screening" on 13 September 2021, effective immediately with one month to demonstrate compliance. It applies to UAE
banks, foreign bank branches, exchange houses and finance companies. It does not create new obligations but sets out
the CBUAE's expectations, and is the most detailed public TM and screening specification in the region (read from the
CBUAE outreach presentation of 14 September 2021, 22 slides; companion documents: CBUAE STR guidance, CBUAE TFS
guidance, and the Executive Office's TFS guidance). It follows the system lifecycle:

**Risk assessment and design**

- The enterprise risk assessment (customers, products and services, channels, geography, plus control strength and
  residual risk) drives TM and SS design.
- Larger institutions are expected to run automated systems. **"All SS systems should be fully automated for the update
  of any changes to the UN Consolidated List and the Local Terrorist List."**
- The institution must be able to demonstrate effectiveness whether processes are automated or manual.

**Data identification and management**

- Document all data sources (customer databases, core banking, SWIFT message data), each with a named owner.
- Extraction and loading must give "a complete, accurate, and fully traceable transfer of data".
- Test and validate data integrity before deployment and then "**at minimum every 12 to 18 months**", including
  reconciliation of transaction codes across core banking and the TM and SS systems.
- Use MIS trends and exception reports to detect "abnormally functioning TM rules or scenarios or SS logic", and
  escalate data issues to senior management.

**Transaction monitoring rules**

- A **typology assessment** drives scenario design, with thresholds tailored to risk profile, product and customer type.
- Risk-based customer and product **segmentation**.
- **Above-the-line and below-the-line testing** for larger institutions.
- Multi-factor scenarios to catch patterns across transactions.
- Documentation of every scenario's assumptions, parameters and thresholds.

**Name screening** covers:

- customers, beneficial owners and related parties;
- **employees**;
- **third-party providers, landlords and tenants**;
- ISINs of assets in custody;
- **recipients of corporate donations and sponsorship**.

It must happen before onboarding or an occasional transaction and "on an ongoing basis (**at least daily**)".

**Transaction screening** covers:

- all payments, before completion, including parties, agents, intermediaries and financial institutions;
- BICs and routing codes;
- free text such as SWIFT field 70;
- ISINs, trade documents, and geographic details (addresses, cities, ports, airports in fields 50 and 59 or from vessel
  tracking).

Screening must happen where the transaction can still be stopped. Points where information could be changed or removed
need particular attention.

**Testing and operation**

- Pre-implementation testing on historical data, with system integration and user acceptance testing. Material
  data-mapping and transaction-coding issues are fixed and retested before go-live.
- Risk-weighted **alert scoring** to prioritise alerts, with high scorers routed to senior or specialist investigators.

**List management**

- Under Article 21.2 of Cabinet Decision No. 74 of 2020, screening lists must include the UN Consolidated List and the
  UAE Local Terrorist List.
- Institutions should also screen for entities "owned or controlled mainly or fully by a listed person". Because such
  "**shadow designated persons**" are not listed, LFIs should keep **internal lists** built from their own due
  diligence.
- List procedures cover selection, sourcing, maintenance, data enhancement, whitelisting, geographic scope, exact versus
  fuzzy matching, and frequency.

**Outcomes analysis and MIS**

- Track outputs per rule.
- Treat sudden changes in rule productivity or match volumes as possible data-quality issues.
- Risk-based suppression of repeatedly cleared patterns is allowed but "should not be applied to higher-risk customer or
  transaction types".
- Report to senior management per rule: alerts, false positives, investigations and STRs.
- Feed results back into the risk assessment and risk appetite.

**Post-implementation testing and validation**

- Periodic and event-driven reassessment, including back-testing of rules.
- Material changes get pre-implementation testing on historical data.
- Validation by people "not responsible for the development or use of the model and [who] do not have a stake in
  whether a model is determined to be valid".

**Governance and vendors**

- Board oversight; separate TM and SS responsibilities.
- Independent testing by auditors with technical expertise.
- The institution remains responsible when using vendors. It should demand "developmental evidence" on design and
  intended use, validate its own use of the product, document and justify its customisations, monitor vendor-model
  outcomes on its own data, and keep "as much knowledge in-house as possible".

**Why this matters for Iraqi banks:** UAE correspondents and exchange houses examine their respondents through this
lens. The UAE is also a frequent counterparty geography in Iraqi flows, and US advisories single out UAE free-zone
general trading companies (section 1g).

### 4b. Saudi Arabia (SAMA)

SAMA's AML/CTF Guide (circular No. 18318/486 of 17 November 2019, in force; SAMA rulebook), **Section 7 "Monitoring of
Transactions and Activities"**:

- "Manual monitoring of transactions alone is not sufficient". Institutions "shall include appropriate technological
  systems that enable it to monitor transactions and activities and detect any unusual or unexpected behavior".
- Detection is expected "in real time of execution or before that".
- Monitoring tools must be tested "periodically (Once a year a maximum)" for effectiveness and adequacy.
- Monitoring is enhanced for high-risk customers and simplified, but never absent, for low-risk ones. It continues for
  all customers until the relationship ends.
- Institutions must "review and update customer information and categorize customers based on the monitoring results".

The Saudi FIU (SAFIU, under the Presidency of State Security) receives STRs on the UNODC-developed platform (goAML).
Secondary sources report a 10-year record retention period under Saudi law (not verified against the law text).

### 4c. Jordan (CBJ)

- Jordan is one of the main correspondent and parent-bank countries for Iraqi banks (file 18 will map ownership).
- Jordan left the FATF grey list in October 2023.
- The bank AML instruction found online is **CBJ Instructions No. 51/2010** (published by the Jordanian FIU, AMLU).
- Later CBJ instructions under the newer AML/CFT law were not retrieved. **Open item:** confirm the current CBJ bank
  instruction and its system requirements.

### 4d. Other regional signals

The HM Treasury notice of June 2026 lists **Kuwait and Lebanon** as jurisdictions under increased monitoring, alongside
Iraq and Syria. Correspondents in two of Iraq's key corridors are therefore under grey-list pressure themselves, which
makes them more demanding of their Iraqi respondents.

### 4e. AKR-TMS hooks from the regional regimes

- **Data integrity testing pack every 12-18 months** (CBUAE). Reconciliation of transaction codes and control totals
  between the CBS replica and AKR-TMS runs continuously, with a formal report on that cycle.
- **Daily re-screening** of the customer base (CBUAE "at least daily"), plus immediate delta re-screening on list
  change (EU, file 04).
- **Non-customer screening** (CBUAE): employees, vendors, landlords, tenants and donation recipients. These come from
  uploaded HR and procurement files, since the CBS does not hold them.
- **"Shadow designated persons" list** (CBUAE): the derived-designation list produced by the ownership graph
  (section 1e).
- **Annual effectiveness test of monitoring tools** (SAMA; EBA's yearly screening review) as a scheduled, evidenced
  task.
- **Real-time expectation** (SAMA "before execution"; CBUAE screening "where a transaction can be stopped") is met by
  the phase-2 pre-release API.
- **Per-rule MIS** (CBUAE): alerts, false positives, investigations and STRs per rule. It feeds the risk assessment.
- **Suppression never applies to high-risk customers or transaction types** (CBUAE), as a hard rule in the suppression
  engine.
- **Vendor transparency** (CBUAE "developmental evidence", "as much knowledge in-house as possible"): full model and
  rule documentation shipped with the product, plus training for bank staff.


## 5. What it means for AKR-TMS (cross-regime synthesis)

### 5a. The correspondent is the second supervisor

Iraq returned to the FATF grey list on 19 June 2026 (file 01). Every foreign regime above reacts to that through the
correspondent banks that give Iraqi banks access to dollars, euros, pounds and dirhams:

| Regime | How it reaches an Iraqi bank after June 2026 | What the correspondent will ask for |
|---|---|---|
| **US** | Dollar clearing; OFAC (direct and secondary sanctions, causing liability); FinCEN Section 311 (Al-Huda Bank precedent, 2024); FinCEN's Iran advisory names regional banks' USD correspondents as a channel | Proof of sanctions screening against OFAC lists with ownership logic, Iran and militia-economy typology coverage, payment transparency, 10-year sanctions records |
| **EU** | EU correspondents apply AMLR from 10 July 2027; Iraq's re-listing as a high-risk third country is expected but not yet adopted (section 2e); AMLR Article 38 lets AMLA recommend measures against an individual third-country respondent | Evidence of controls "commensurate with risk"; EDD information once Iraq is re-listed; record of every assessment; answers to FIU enquiries within days |
| **UK** | Iraq is a high-risk third country for UK purposes; mandatory EDD for every Iraqi relationship ended on 30 June 2026, but Iraq stays a risk factor (section 3a) | The quality of the Iraqi bank's own controls decides whether EDD is "high risk" in practice: screening quality, backlog, timeliness, MI |
| **UAE / GCC** | UAE banks follow the CBUAE 2021 guidance for their own monitoring and screening; Kuwaiti and Lebanese correspondents are grey-listed themselves | Data integrity, daily re-screening, documented tuning and validation; Kuwait and Lebanon banks will pass their own pressure down to Iraqi respondents |
| **Jordan** | Parent and correspondent country for several Iraqi banks (file 18) | Group-wide policies and MIS that a Jordanian parent can consolidate |

The practical conclusion: **an Iraqi bank's controls have to be visible and provable to a foreign reader.** Building
good controls is not enough. The bank must be able to show them to someone who cannot inspect its systems and does not
read Arabic. AKR-TMS should keep one body of evidence and produce several views of it:

| Audience | What they ask for | What AKR-TMS hands over |
|---|---|---|
| CBI and the AML/CFT Office (FIU) | STRs and CTRs on time, compliance with CBI instructions, inspection access | goAML exports and regulatory reports (file 23), inspection pack, audit trail |
| Correspondent banks (RFIs, Wolfsberg CBDDQ) | Lists used and update speed, screening coverage, scenarios, testing, independent review, staffing, backlog | **Respondent evidence pack** (English and Arabic), generated from live configuration and metrics |
| External auditor or independent validator | Rule logic, data lineage, test results, model documentation | **Validation pack** (rules, models, data lineage, test cases, results) |
| Board and MLRO | Effectiveness, resources, backlog, open findings | **Annual certification pack** (NYDFS 504 pattern) and board dashboard |

### 5b. Screening: the regimes now agree on a specification

The NYDFS rule (2016), the CBUAE guidance (2021), the EBA guidelines (2024, applying since 30 December 2025), the FCA
review (May 2026) and OFSI's Citibank case (September 2026) were written separately, years apart, yet they describe
almost the same screening system. Building to the union of them is the safest design:

| Requirement | Where it comes from |
|---|---|
| Lists complete and current; immediate update on publication; fully automated for UN and national terrorist lists | NYDFS 504 (current watch lists); EBA 2024/15 (immediate update); CBUAE (automated UN and local lists); FCA (delays updating the UK list) |
| List integrity checked against the official publisher's file; vendor data challenged and documented | FCA ("vendor reliance does not equate to regulatory assurance"); CBUAE (developmental evidence) |
| Every party and identifier screened: customers, beneficial owners, directors, payment parties, intermediary and correspondent BICs, purpose and free text, non-name identifiers | EBA 2024/15; OFAC enforcement practice (section 1e); NYDFS 504 (names and accounts) |
| Original script and transliteration, aliases, spelling variants, one-word names, titles, digits | EBA 2024/15; FCA review (the weak spots); file 22 for Arabic |
| Matching logic and thresholds tested and calibrated on measured outcomes, before and after every change | NYDFS 504 (pre- and post-implementation testing, threshold mapping to risk); EBA 2024/15 (true-positive rates by score); CBUAE |
| Ownership and control: aggregated 50 percent (US), more than 50 percent or control (EU, UK); derived "shadow" designations | OFAC 50 percent rule; AMLR Article 20(1)(d); EBA 2024/15; CBUAE (shadow designated persons); OFSI (weak ownership assessment) |
| Re-screening on every list change and daily across the customer base, plus event triggers (onboarding, CDD change, circumvention suspicion) | EBA 2024/15; CBUAE (at least daily); AMLR Article 26 |
| Whitelists (good guys) with reason, owner, expiry and review on change | EBA 2024/15 para 13 |
| Two-person review for higher-exposure decisions | EBA 2024/15 para 32(d) |
| Yearly review of the whole screening system | EBA 2024/14 and 2024/15; SAMA (yearly tool testing) |
| Backlog under control; surge capacity when large list packages land; stress tests before geopolitical events | FCA (backlogs); OFSI (alert volumes during rapid change; stress-testing) |
| Freeze immediately on a confirmed match, report, and keep the frozen funds frozen (no fees, no leakage) | EBA 2024/15; OFSI Citibank (fees deducted from frozen accounts, delays in freezing, late reporting) |
| Non-customers screened too: employees, vendors, landlords, donation recipients | CBUAE |

For AKR-TMS this table is close to a product specification for the screening module. File 14 compares how screening
vendors meet it; file 22 covers the Arabic-name part, which the FCA found to be the weakest point across 150 UK firms.

### 5c. Transaction monitoring: the same convergence

The same exercise for monitoring (NYDFS 504 transaction-monitoring attributes, FFIEC manual, CBUAE, SAMA section 7,
the FinCEN 2026 proposal and the AMLA June 2026 draft, plus BCBS and Wolfsberg in file 04) gives eight common
expectations:

1. **Scenarios follow the risk assessment** and are mapped to typologies, so coverage gaps can be shown (NYDFS,
   FinCEN NPRM, CBUAE, AMLA draft).
2. **Complete, reconciled data**: every account and every transaction, with counts and totals reconciled to the source
   (NYDFS data attributes, CBUAE data integrity testing every 12-18 months, BCBS, and the UK cases in file 20 where a
   data feed silently dropped transactions).
3. **Documented, tested and tuned**: above- and below-the-line testing, threshold reviews, documented rationale
   (NYDFS, FFIEC, CBUAE).
4. **Governed change**: maker-checker approval, impact analysis, and extra scrutiny when a change mainly cuts alert
   volume (NYDFS, CBUAE).
5. **Independent validation** of the monitoring system; under the April 2026 US guidance this means full model
   validation only for real models, and configuration governance for rules (FFIEC, CBUAE, US MRM 2026).
6. **Speed where it matters**: detection before execution where the transaction can still be stopped (SAMA, CBUAE,
   EBA, AMLA's pre-transaction, real-time and post-transaction layers).
7. **Management information per rule and outcomes**: alerts, false positives, cases and STRs per rule; outcomes such as
   FIU feedback and law-enforcement use (CBUAE, FinCEN NPRM effectiveness standard).
8. **Decisions recorded**, including decisions not to report (AMLR Article 77; CBI, file 01). The US relaxed this in
   October 2025, but Iraqi and EU rules still need the record, so the product keeps it short rather than dropping it.

### 5d. Three governance tracks, white-box by default

- **Rules and thresholds are governed configuration**: version, owner, rationale, test results, approval. The April
  2026 US guidance confirms that deterministic rules are not "models", which removes the heaviest validation burden
  from the rule library.
- **Models are validated**: risk scores, anomaly detection and name-match scoring get model cards, a validation pack,
  drift and stability monitoring, and outcome analysis.
- **Generative AI drafts, people decide**: case summaries and STR narrative drafts are reviewed by a named analyst, and
  prompts and outputs are logged. The US guidance leaves GenAI out of scope, and AMLR Article 76(5) requires meaningful
  human intervention in automated decisions, so the conservative design is the right one for every regime.

Three regulators say the same thing about transparency in different words: AMLA (outputs must be explainable and
verifiable), CBUAE (vendor "developmental evidence", keep "as much knowledge in-house as possible") and the FCA (firms
must understand and challenge their vendor's logic and document the challenge). **An Iraqi bank must be able to tell
a correspondent why an alert fired, and why one did not.** AKR-TMS should therefore be white-box by default: reason
codes on every alert, feature contributions on every score, a plain-language description generated from every rule's
configuration, and full documentation shipped with the product. Most black-box competitors cannot match this (files
08-13).

### 5e. Records: one retention engine, several clocks

| Rule | Period | Notes |
|---|---|---|
| Iraq (AML/CFT Law and CBI instructions, file 01) | At least 5 years after the transaction or the end of the relationship | The default for every Iraqi bank; records must allow reconstruction of individual transactions |
| US BSA (FFIEC manual) | 5 years | Reference policy for banks whose correspondents ask |
| OFAC (statute of 24 April 2024; rule of 21 March 2025) | **10 years** | Sanctions-related records: screening results, blocked and rejected transactions, licences |
| EU AMLR Article 77(3) | 5 years from the end of the relationship, the occasional transaction or a refusal to onboard; **then delete personal data** | Authorities may require longer retention case by case; records must not be redacted; references allowed instead of copies only if they cannot be modified or altered |
| NYDFS Part 504 | 5 years for the evidence behind each annual certification | Relevant to banks with New York correspondents that apply the same standard to respondents |

The rules conflict: EU says delete after 5 years, OFAC says keep for 10. AKR-TMS needs a **retention engine per record
class and jurisdiction**, with the bank's policy deciding conflicts, a recorded legal basis for each class, legal holds
that override deletion, and a deletion log. Tamper-evident storage (MinIO object lock) meets the AMLR condition that
retained information "cannot be modified or altered". Refused-onboarding records belong in the retention model too
(AMLR Article 77(3) starts their clock on the date of refusal); since onboarding stays in the CBS, AKR-TMS keeps the
screening and risk evidence for refusals that passed through it.

### 5f. Speed: post-event now, pre-release later

SAMA wants detection "in real time of execution or before that", the EBA wants transfers screened before funds are made
available, the CBUAE wants screening where a transaction can still be stopped, AMLA's draft describes pre-transaction,
real-time and post-transaction layers, and AMLR Article 71 assumes an institution can refrain from executing. Phase 1 of
AKR-TMS reads a read-only replica after the event, so it cannot stop a payment. The plan must say this plainly:

- **Phase 1** delivers detection, daily and delta re-screening, investigation, reporting and evidence. It also
  re-screens every executed payment after the fact, which catches what the bank's existing payment filter missed (the
  FCA found 35 percent of 2025 breach reports concerned earlier activity).
- **Phase 2** adds the pre-release screening API that the bank's payment or SWIFT gateway calls before release, which
  closes the real-time gap for payments.
- Until phase 2 is live, the bank's existing pre-release filter, where it has one, remains its pre-release control, and
  AKR-TMS records that dependency in the bank's control inventory so the gap is documented, not hidden.

### 5g. Regime packs: build to the union, switch on per bank

Iraqi banks differ in which foreign regimes reach them: a state bank with large dollar flows, a private bank owned by a
Jordanian group, an Islamic bank with UAE correspondents, a bank with Lebanese or Turkish shareholders. AKR-TMS should
ship **regime packs** that a bank switches on: Iraq (always on), US/OFAC, EU, UK, UAE, Saudi Arabia, Jordan. Each
pack sets the lists, the ownership rule (50 percent or more aggregated versus more than 50 percent or control),
retention defaults, regulatory clocks, typology packs, country-risk consequences and evidence-pack templates. The
bank's profile is the combination it switches on; where packs conflict, the stricter rule applies unless the bank's
policy records a different choice and its legal basis.

### 5h. What the foreign regimes do not require (avoid over-engineering)

- The US SAR FAQs (October 2025) dropped the expectation to document every no-SAR decision, the mandatory 90-day
  continuing-activity review and the idea that near-threshold transactions alone are suspicious. Iraqi and EU rules
  still need the decision record, so AKR-TMS keeps it, but short.
- The April 2026 US model risk guidance takes deterministic rules and GenAI out of model validation. Rules need
  governance, not model validation.
- The UK no longer forces EDD on every grey-listed relationship (30 June 2026). Risk-based beats blanket.

These relaxations matter because the most common complaint about incumbent systems is paperwork that buries analysts
(files 08-13). The product should meet every documentation duty with the least effort per case: picked reasons,
templates, auto-filled facts, and evidence assembled by the system instead of typed by people.

## 6. Derived requirements for AKR-TMS (from this file)

Prefix R. Iraqi law and CBI instructions (file 01) always come first; these requirements make an Iraqi bank's controls
defensible to US, EU, UK and Gulf correspondents and to international auditors.

| ID | Requirement | Source |
|---|---|---|
| R-01 | **Risk assessment as a living object**: records which national priorities, typologies and regimes it considered and why; re-run triggers for new products, new list regimes, grey-listing changes and correspondent RFIs | FinCEN programme NPRM 2026 (1a); file 04 dynamic EWRA |
| R-02 | **Resource allocation report**: analyst hours, alert volumes and backlog by risk segment, so the bank can show resources follow risk; feeds the board's yearly review of human and technical resources | FinCEN NPRM 2026; EBA/GL/2024/14 (management body review) |
| R-03 | **Effectiveness evidence**: outcome metrics (STRs used by the FIU, law-enforcement requests answered, true positives by typology, time to detect, late-detection share), not only alert counts | FinCEN NPRM 2026; AMLA draft 2026; FCA review 2026 |
| R-04 | Audit and validation findings tagged **design** or **implementation** | FinCEN NPRM 2026 (establish vs maintain) |
| R-05 | **Governed experimentation sandbox**: champion-challenger runs and AI pilots logged as experiments with scope, data, results and decision | FinCEN NPRM 2026 (innovation) |
| R-06 | **Regulatory clocks per jurisdiction**, Iraq as default; US 30/60-day and continuing-activity timelines and EU timelines (FIU requests in 5 working days; postponement orders) as reference policies | SAR FAQs 2025; AMLR Arts 69, 71 |
| R-07 | Structuring scenario built on **evasion indicators** (activity after a cash report, after a branch query, split across branches or people), not on near-threshold amounts alone | SAR FAQs 2025 (1b); file 27 |
| R-08 | **Closure rationale sized to the case**: governed reason list plus free text for simple closures; structured template for complex ones; every decision kept | SAR FAQs 2025; AMLR Art 77; CBI (file 01) |
| R-09 | **Post-STR watch policy**: risk-based heightened monitoring of the subject instead of a fixed 90-day manual re-review; configurable per bank | SAR FAQs 2025 |
| R-10 | **Two governance tracks**: rules and thresholds as governed configuration (version, owner, rationale, tests, approval); models (scores, anomaly detection, match scoring) with model cards, validation pack, drift and stability monitoring, outcome analysis | US revised model risk guidance, April 2026 (1c) |
| R-11 | **GenAI as drafts only**: every output reviewed by a named person, prompts and outputs logged, no autonomous decision | US MRM 2026 (GenAI excluded); AMLR Art 76(5) |
| R-12 | **Vendor transparency pack** shipped with the product: model and rule documentation, feature lists, training-data descriptions, and a validation harness the bank runs on its own data | US MRM 2026 (vendor models); CBUAE 2021 (developmental evidence); FCA 2026 |
| R-13 | **Plain-language description of every rule and threshold**, generated from its configuration (English and Arabic) | FFIEC manual (1d) |
| R-14 | **Periodic threshold reviews** as scheduled tasks with below-the-line results attached | FFIEC manual; CBUAE 2021 |
| R-15 | **Validation pack**: rule logic, data lineage from the CBS replica, test cases and results, for the bank's independent validator | FFIEC manual; NYDFS 504; CBUAE 2021 |
| R-16 | **Alert source tag** on every alert and case: system, employee referral, authority request, correspondent RFI, adverse media | FFIEC manual |
| R-17 | **Unified watchlist** with OFAC SDN and non-SDN lists (programme tags), FinCEN 311 institutions and FTO list, alongside UN, Iraqi, EU and UK lists | OFAC (1e); file 04 section 4d |
| R-18 | **Ownership-aware screening**: ownership graph from KYC data (and licensed data where available) computing aggregated ownership per regime rule (US 50 percent or more; EU and UK more than 50 percent or control); derived-designation alerts form the **shadow designated persons** list | OFAC 50 percent rule; AMLR Art 20(1)(d); EBA/GL/2024/15; CBUAE 2021 |
| R-19 | **Identifier screening beyond names**: BICs, IBANs, account numbers, vessel IMO numbers, addresses, phone numbers and email addresses in payments and KYC records | OFAC enforcement practice; EBA/GL/2024/15 |
| R-20 | **Iraq militia-economy typology pack**: state-contract sub-contracting chains, insider-facilitated flows, agricultural and trading fronts, oil-trade payments of doubtful origin | OFAC Iraq-nexus designations 2025-2026 (1e) |
| R-21 | **Iran shadow-banking typology pack** with a coverage matrix showing which advisory red flags each scenario covers | FinCEN FIN-2025-A002 (1g) |
| R-22 | **Authority key terms and campaign codes** supported in STR narratives and goAML exports | FinCEN advisory practice (1g); file 23 |
| R-23 | **Advisories and typology papers as governed inputs**: each creates a review task to map its red flags to scenarios, logged in the regulatory obligations register | FinCEN FIN-2025-A002; FinCEN NPRM 2026 |
| R-24 | **Retention engine per record class and jurisdiction**: Iraq first (at least 5 years); OFAC 10 years for sanctions records; EU 5 years then deletion; US BSA 5 years; conflicts resolved by bank policy with recorded legal basis; legal holds; deletion log | file 01; OFAC rule 21 March 2025; AMLR Art 77(3); FFIEC |
| R-25 | **Data source inventory and lineage**: every CBS table and column read by the mapping layer, with owner, refresh time and dependent scenarios and screening checks | NYDFS 504 (1f) |
| R-26 | **Data quality controls on every load**: row counts and control totals reconciled to the CBS, null and default-value checks on fields scenarios depend on, late-data alarms | NYDFS 504; CBUAE 2021; file 04 B-01 |
| R-27 | **Formal data integrity test report** every 12-18 months, built on the continuous reconciliation results | CBUAE 2021 |
| R-28 | **Scenario documentation generated from configuration**: purpose, typology, assumptions, parameters, thresholds, data used, last test, last tuning, owner | NYDFS 504 |
| R-29 | **Testing harness** for rules and screening, run before and after every change, with test data, expected hits and results stored with the change request | NYDFS 504; EBA/GL/2024/15 |
| R-30 | **Change management** for rules, thresholds, lists and match settings: maker-checker, reason, impact analysis (alert volume and coverage before and after), flag on changes that mainly cut alert volume | NYDFS 504; CBUAE 2021 |
| R-31 | **Annual certification pack** auto-assembled from the year's testing, tuning, data quality, validation, staffing and backlog evidence, for MLRO and board sign-off | NYDFS 504 |
| R-32 | **Respondent evidence pack** for correspondents (English and Arabic): lists and update times, screening coverage, scenario coverage, testing, validation, staffing, backlog and timeliness metrics, generated from live data | Section 5a; FinCEN, AMLR Art 38, UK MLR 2026, CBUAE |
| R-33 | **Record of every assessment**, including "no STR" outcomes, unredacted, in tamper-evident storage | AMLR Art 77(1)(b) and 77(2) |
| R-34 | **Meaningful human intervention** in every automated decision on customers (risk changes, EDD, exits); customer-facing explanation generated from reason codes, never for STR-related decisions | AMLR Art 76(5) |
| R-35 | **Re-screening**: delta re-screening on every list change, full re-screening of the customer base at least daily, and event triggers (onboarding, CDD change, circumvention suspicion) | EBA/GL/2024/15; CBUAE 2021; AMLR Art 26 |
| R-36 | **Payment screening of every party and field**: originator, beneficiary, intermediary and correspondent BICs, purpose and free text, where the data are in the replica (phase 1) or the gateway message (phase 2) | EBA/GL/2024/15; NYDFS 504 |
| R-37 | **Calibration on measured outcomes**: per list and regime, match thresholds set from measured true-positive rates at different scores, re-measured on every change | EBA/GL/2024/15; NYDFS 504 |
| R-38 | **Whitelist governance**: reason, owner, expiry, automatic review on list or customer change | EBA/GL/2024/15 para 13 |
| R-39 | **Two-person review** configurable by exposure (amount, list, regime, customer risk) | EBA/GL/2024/15 para 32(d) |
| R-40 | **FIU enquiry tool**: "did we have a relationship with these persons in the last five years, and what was it?", answerable within 5 working days | AMLR Arts 69, 78; Egmont (file 04) |
| R-41 | **Correspondent-risk early warning** in the respondent self-view, tracking what an AMLA Article 38 assessment would look at (repeated breaches, controls not commensurate with risk) | AMLR Art 38 |
| R-42 | **UN-designation gap register** for the window between a UN listing and its national application in Iraq | AMLR Art 27 logic; file 04 section 4c |
| R-43 | **Payment transparency monitor**: missing, incomplete or meaningless payer and payee information; repeatedly failing counterparty banks tracked and reported | EBA/GL/2024/11; FATF R.16 (file 03) |
| R-44 | **Former-PEP handling**: PEP measures kept for a configurable minimum after leaving office (EU at least 12 months), then risk-based | AMLR Arts 42-46 |
| R-45 | **Periodic review cycles** with EU defaults (at least yearly for high risk, at least every 5 years for others) available as a reference policy; Iraq's CBI cycle first | AMLR Art 26; file 26 |
| R-46 | **Country risk table fed by FATF list changes** (February, June, October), with US, EU and UK consequences recorded separately, dated and approved | HMT notice June 2026; MLR amendment 2026 |
| R-47 | **Screening quality tests for known weak spots**: spelling variants, one-word names, digits, titles and honorifics (Sayyid, Sheikh, Hajji, Dr), non-Latin script; run on every configuration change | FCA review 2026; file 22 |
| R-48 | **List integrity checks on every load**: record counts and checksums against the publisher's file, field-level diff, alarm on vendor deviation from the official source | FCA review 2026 |
| R-49 | **Backlog management as a control**: ageing dashboards, service levels per alert type, surge planning, escalation when backlog growth comes from resourcing | FCA review 2026; OFSI 2026 |
| R-50 | **Late-detection metric**: share of confirmed matches or STRs whose underlying activity is older than N months | FCA review 2026 |
| R-51 | **Surge mode for list events**: queue re-prioritisation, pre-clearing of known false positives from pair memory, progress reporting to the MLRO; stress tests ahead of expected events | OFSI 2026; FCA review 2026 |
| R-52 | **Frozen-asset ledger with fee blocking**: frozen accounts verified against the CBS replica, any debit after the freeze date (including fees) flagged at once; freeze and report tasks tracked to completion | OFSI Citibank penalty 2026; EBA/GL/2024/15 |
| R-53 | **Vendor-challenge record** for third-party lists, data and logic: tests run, issues raised, vendor responses | FCA review 2026; CBUAE 2021 |
| R-54 | **Non-customer screening** of employees, vendors, landlords, tenants and donation recipients from uploaded HR and procurement files | CBUAE 2021 |
| R-55 | **Annual effectiveness test** of monitoring and screening tools as a scheduled, evidenced task | SAMA guide s.7; EBA/GL/2024/14-15; CBUAE 2021 |
| R-56 | **Pre-release layer**: phase-2 screening API for the payment gateway; until it is live, the dependency on the bank's existing filter is recorded in the control inventory | SAMA s.7; CBUAE 2021; EBA/GL/2024/15; AMLA draft 2026 |
| R-57 | **Per-rule MIS**: alerts, false positives, cases and STRs per rule, feeding the risk assessment and tuning | CBUAE 2021; NYDFS 504 |
| R-58 | **Suppression never applies to high-risk customers or high-risk transaction types** (hard rule in the suppression engine) | CBUAE 2021 |
| R-59 | **Training material** for bank staff shipped with the product, so knowledge stays in-house | CBUAE 2021; NYDFS 504 (training) |
| R-60 | **Sanctions exposure assessment** (EBA "restrictive measures exposure assessment") as a module, kept separate from or combined with the AML risk assessment, reviewed at least yearly | EBA/GL/2024/14 |
| R-61 | **Regime packs** (Iraq always on; US/OFAC, EU, UK, UAE, Saudi Arabia, Jordan optional) setting lists, ownership rule, retention, clocks, typologies, country-risk consequences and evidence templates; stricter rule wins unless bank policy records otherwise | Section 5g |

## 7. Sources

Accessed 2026-10-06 unless stated. Where a primary site blocked automated access (FFIEC, FATF, some law-firm and
Iran Watch pages), the content was confirmed through search-result extracts and secondary summaries, as noted.

United States
- FinCEN AML/CFT programme proposal (April 2026), summaries:
  Debevoise & Plimpton, https://www.debevoise.com/insights/publications/2026/04/from-check-the-box-to-effectiveness-fincen-propose ;
  Gibson Dunn, https://www.gibsondunn.com/fincen-proposes-rule-to-fundamentally-reform-financial-institution-programs-designed-to-fight-illicit-finance/
- FinCEN, SAR FAQs (9 October 2025), read in full: https://www.fincen.gov/system/files/2025-10/SAR-FAQs-October-2025.pdf ;
  Mayer Brown summary, https://www.mayerbrown.com/en/insights/publications/2025/11/fincen-publishes-faqs-to-reduce-certain-compliance-burdens-associated-with-sar-filings ;
  Davis Polk, https://www.davispolk.com/insights/client-update/fincen-and-banking-agencies-release-updated-sar-guidance
- Revised model risk management guidance (17 April 2026), summaries:
  Orrick, https://www.orrick.com/en/Insights/2026/04/Agencies-Overhaul-Model-Risk-Management-Guidance-for-Banks-Heres-What-Changed ;
  Davis Polk visual memo, https://www.davispolk.com/insights/client-update/visual-memo-key-changes-under-federal-banking-agencies-revised-model-risk
- FFIEC BSA/AML InfoBase, "Assessing Compliance with BSA Regulatory Requirements" (site blocks automated access;
  quotations confirmed via search extracts): https://bsaaml.ffiec.gov/manual/AssessingComplianceWithBSARegulatoryRequirements/04 and
  https://bsaaml.ffiec.gov/manual/AssessingComplianceWithBSARegulatoryRequirements/04_ep
- FinCEN Section 311 final rule on Al-Huda Bank (2024; details in file 01):
  https://www.fincen.gov/news/news-releases/fincen-finalizes-financial-measure-against-iraq-based-al-huda-bank-combat ;
  https://www.govinfo.gov/content/pkg/FR-2024-07-03/html/2024-14415.htm
- OFAC actions with an Iraq nexus, 2025-2026 (Treasury and State press releases as reported):
  Long War Journal on the four FTO designations of 17 September 2025,
  https://www.longwarjournal.org/archives/2025/09/us-designates-4-iran-backed-iraqi-militias-as-foreign-terrorist-organizations.php ;
  Rudaw, https://www.rudaw.net/english/middleeast/iraq/17092025 , https://www.rudaw.net/english/middleeast/iraq/10102025 ,
  https://www.rudaw.net/english/middleeast/iraq/12102025 , https://www.rudaw.net/english/middleeast/iraq/18042026 ;
  Peregraf, https://peregraf.com/en/news/9927 ;
  Kurdistan24, https://www.kurdistan24.net/en/story/868790/iraq-condemns-us-sanctions-on-state-owned-firm-as-unilateral-and-unjustified ;
  US Treasury press release (17 April 2026), https://home.treasury.gov/news/press-releases/sb0458 ;
  WION, https://wionews.com/world/us-sanctions-7-iraqi-militia-commanders-linked-to-iran-backed-groups-under-economic-fury-who-all-are-on-the-list-1776456245548 ;
  Iran Watch (search extracts; direct fetch blocked),
  https://www.iranwatch.org/library/governments/united-states/executive-branch/department-state/terrorist-designations-four-iran-aligned-militia-groups ,
  https://www.iranwatch.org/library/governments/united-states/executive-branch/department-treasury/economic-fury-targets-iran-backed-iraqi-militia-commanders
- OFAC record-keeping extended to 10 years (final rule, 21 March 2025): https://www.govinfo.gov/content/pkg/FR-2025-03-21/pdf/2025-04864.pdf ;
  Troutman Pepper summary, https://www.troutman.com/insights/ofac-recordkeeping-requirement-extended-to-10-years.html
- NYDFS 3 NYCRR Part 504 (final regulation, 2016), Clifford Chance briefing with the regulation's attributes:
  https://www.cliffordchance.com/content/dam/cliffordchance/briefings/2016/07/the-new-york-department-of-financial-services-finalizes-regulation-clarifying-transaction-monitoring-and-filtering-pro.pdf
- FinCEN Advisory FIN-2025-A002 (6 June 2025), read in full: https://www.fincen.gov/system/files/FinCEN-Advisory-Illicit-Oil-Smuggling-508.pdf

European Union
- Regulation (EU) 2024/1624 (AMLR), full text read on EUR-Lex: https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=OJ:L_202401624
- AMLA draft guidelines on ongoing monitoring (Article 26(5) AMLR; consultation 3 June 2026):
  PayTechLaw, https://paytechlaw.com/en/amla-consults-on-guidelines-for-the-ongoing-monitoring-of-business-relationships/ ;
  CMS (search extract; page returned 404 on fetch), https://cms.law/en/gbr/legal-updates/amla-publishes-draft-guidelines-on-ongoing-monitoring-under-article-26-5-amlr
- EBA, Final Report, Guidelines on restrictive measures (EBA/GL/2024/14 and EBA/GL/2024/15), read in full:
  https://www.eba.europa.eu/sites/default/files/2024-11/eaeae49d-81a5-4154-8af9-5014f6ee8881/Final%20Report%20Guidelines%20restrictive%20measures%20.pdf ;
  Hengeler Mueller newsletter (November 2024), https://hengeler-news.com/assets/articles/2024_11_eba-1732179840.pdf
- EBA travel rule guidelines (EBA/GL/2024/11): https://www.eba.europa.eu/sites/default/files/2024-07/6de6e9b9-0ed9-49cd-985d-c0834b5b4356/Travel%20Rule%20Guidelines.pdf ;
  FIAU Malta, https://fiaumalta.org/news/the-eba-issues-new-travel-rule-guidelines/
- EU high-risk third countries: Council document on Iraq's removal (2022), https://data.consilium.europa.eu/doc/document/ST-5174-2022-INIT/en/pdf ;
  FIAU Malta on the 2026 delegated acts, https://fiaumalta.org/news/eu-commission-delegated-acts-on-high-risk-third-countries/ ;
  Stibbe, https://www.stibbe.com/publications-and-insights/european-commission-updates-high-risk-third-countries-list

United Kingdom
- HM Treasury, Money laundering advisory notice: high-risk third countries (22 June 2026), read through the GOV.UK
  content API: https://www.gov.uk/api/content/government/publications/money-laundering-advisory-notice-high-risk-third-countries--2/money-laundering-advisory-notice-high-risk-third-countries--2
  (page: https://www.gov.uk/government/publications/money-laundering-advisory-notice-high-risk-third-countries--2/money-laundering-advisory-notice-june-2026)
- Regulatory Policy Committee opinion on the Money Laundering and Terrorist Financing (Amendment) Regulations 2026:
  https://www.gov.uk/government/publications/rpc-opinion-the-money-laundering-and-terrorist-financing-amendment-regulations-2026
- FCA multi-firm review of sanctions systems and controls (28 May 2026), WilmerHale summary:
  https://www.wilmerhale.com/en/insights/client-alerts/20260603-uk-financial-conduct-authority-review-of-firms-sanctions-systems-and-controls
- OFSI penalty on Citibank N.A., London Branch (2 September 2026), DWF summary:
  https://dwfgroup.com/en/news-and-insights/insights/2026/9/ofsi-imposes-penalty-on-citibank-london-branch
- FCA enforcement cases (Metro Bank, Starling, Monzo, Barclays, Nationwide) and the alternative asset manager review:
  see file 20 for case sources.

GCC and regional
- CBUAE, Guidance for licensed financial institutions on transaction monitoring and sanctions screening (13 September
  2021): https://www.centralbank.ae/media/j3ogg1ql/cbuae-issues-new-guidance-on-amlcft-for-licensed-financial-institutions-on-transaction-monitoring-and-sanctions-screening-en.pdf ;
  CBUAE outreach presentation (14 September 2021), read in full:
  https://www.centralbank.ae/media/rdzlhqse/outreach-presentation-cbuae-amlcft-guidance-for-lfis-on-transaction-monitoring-and-sanctions-screening-14-sept-2021.pdf
- SAMA, Anti-Money Laundering and Counter-Terrorist Financing Guide (circular 18318/486, 17 November 2019), section 7,
  SAMA rulebook: https://rulebook.sama.gov.sa/node/3657
- Central Bank of Jordan, AML/CFT Instructions No. 51/2010 for banks (published by AMLU):
  https://www.amlu.gov.jo/ebv4.0/root_storage/en/eb_list_page/anti_money_laundering_and_counter_terrorist_financing_instruction_no._51-2010_of_banks.pdf
- FATF grey list of 19 June 2026 (Iraq, Kuwait, Lebanon): see file 01 sources; confirmed in the HM Treasury notice above.
