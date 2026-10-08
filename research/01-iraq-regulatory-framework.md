# 01 — Iraq: AML/CFT legal and regulatory framework

Living document. Findings are appended as they are confirmed. Sources at the end of each section.

## Summary: what an Iraqi bank's AML system must do (as of October 2026)

The rules are mostly in place; what fails is **execution**. The 2024 MER rated bank preventive measures (IO4)
Moderate and PF sanctions (IO11) Low, and FATF grey-listed Iraq again in June 2026. Under US pressure, Iraqi banks
were cut off from dollar transactions in waves (14 in July 2023, 8 in February 2024, 5 more in February 2025), and
one was blacklisted outright (Al-Huda, FinCEN Section 311, 2024). Since the CBI's dollar platform ended
(end-2024), each bank must satisfy its own foreign correspondents. A credible AML system is now a condition of
doing dollar business at all.

Hard requirements found in CBI rules (with section):

| Requirement | Source | Section |
|---|---|---|
| Real-time monitoring of all account movements vs KYC-declared flows and the AML/CFT Office's scenarios | CBI controls 2025, ch. 5 art. 5 | 2d |
| Load the full CBI scenario catalogue (86+ scenarios plus product indicators) as a minimum; banks may add their own with the Office | CBI controls 2025, ch. 6 | 2d, file 27 |
| Customer risk rating High/Medium/Low (5 levels allowed) on customer, product, channel, geography; dynamic re-rating; review at least every 2 years; KYC refresh at least yearly (MLRO duty) | CBI controls 2025, ch. 2 art. 2; ch. 3 | 2d |
| Direct link to the core banking system and to the screening platform ("Automatic Integration") | CBI controls 2025 | 2d |
| AI / machine-learning analytics to find suspicious transactions | CBI controls 2025, ch. 5 art. 5 | 2d |
| Hold a transaction that matches a scenario until the MLRO clears it | CBI controls 2025, ch. 5 art. 5 | 2d |
| Screening: UN, OFAC, EU (UK in the diagram) and the Iraqi national list (CFTF / IQTFS); all parties incl. BOs; lists refreshed at least every 12 hours; Arabic and English search | CBI controls 2025 | 2d, 6 |
| Wire transfers: complete originator/beneficiary data incl. purpose; detect incomplete incoming wires; reject if not fixed | CBI controls 2025; Instructions 1/2023 | 2b, 2d |
| PEPs: relatives to 2nd degree and associates; daily local/regional news screening; status persists after office; KPI report a-g | CBI PEP circular 26 Aug 2026 | 2c |
| Behavioural indicators from front-line staff combined with automated monitoring; 7 SAR escalation criteria; annual staff training and tests | CBI circular 26 Aug 2026 | 2c |
| Branch liaison officer in every branch with a branch "window" in the AML system; confidential reports straight to the MLRO; monthly report by the 10th | CBI controls 2025, ch. 11 | 2d |
| Internal reports: written acknowledgment to the employee, tipping-off reminder, written decision | CBI controls 2025, ch. 2 art. 3 | 2d |
| STRs via goAML (incl. attempted transactions, any amount), with reasons; MLRO closures must state reasons | Law 39/2015; CBI controls 2025 | 2, 2d |
| Records 5+ years from end of relationship, transactions reconstructable, off-site backup, fast retrieval | Law 39/2015; Instructions 1/2023; CBI controls 2025 | 2b, 2d |
| Standard reports: monthly account movements (customers and staff), transfers register, correspondent account activity; board/audit-committee reports | CBI controls 2025, ch. 3 and 15 | 2d |
| Dormant accounts frozen after 6 months; reactivation signed by branch manager and liaison officer | CBI controls 2025, ch. 15 | 2d |
| Know-your-employee: staff indicators, mandatory 2-week leave, rotation | CBI controls 2025, ch. 6 art. 14 | file 27 |
| AML tooling verified by a CBI-approved firm; three lines of defence | CBI standards handbook 2025 | 2e |

Iraq-specific data facts: CTR threshold USD 10,000; occasional-customer CDD at USD 10,000 incl. linked
transactions; SDD below IQD 1 million; BO at 20%; the national list is Arabic-only with mother's name and birth
year; Iraqi names are long patronymic chains with tribe names; Kurdish spellings differ; the family number links
relatives; bank STRs went from 82 (2022) to 672 (2023) and 451 (2024).

---

## 1. Headline: Iraq is back on the FATF grey list (19 June 2026)

- At the FATF plenary of 17-19 June 2026, Iraq (and Bosnia and Herzegovina) were added to the list of
  "jurisdictions under increased monitoring" (the grey list). Algeria and Namibia were removed. The list
  then stood at 22 jurisdictions.
- Iraq was previously under FATF monitoring from 2013 to 2018 (exited June 2018 after its first action plan).
- FATF President Elisa de Anda Madrazo cited the need to address risks of Iraq's cash-intensive economy,
  to increase money laundering investigations and prosecutions, and to make better use of financial
  intelligence.
- Iraq gave a high-level political commitment to work with FATF and MENAFATF to fix the deficiencies
  within agreed timeframes.

### Action plan items (as reported by Al Jazeera, 19 June 2026; FATF's own page blocks automated reading)

1. Strengthen understanding of ML/TF risks.
2. Detect informal money transfer activity (unlicensed hawala / underground banking).
3. Create a legislative framework regulating virtual assets.
4. Increase the volume AND quality of suspicious transaction reports (STRs).
5. Expand the use of financial intelligence in judicial and security investigations.
6. Accelerate ML/TF investigations and prosecutions.
7. Strengthen oversight of beneficial ownership of companies.
8. Develop mechanisms to prevent evasion of targeted financial sanctions related to proliferation financing.

Other secondary sources add: inadequate risk-based supervision of financial institutions and DNFBPs,
limited asset recovery capacity, and shortcomings in targeting TF and PF risks.

### What it means for AKR-TMS (design implications)

- Items 2, 4, 7 and 8 land directly on banks' monitoring systems: better-quality STRs (not just more),
  detection of hawala-like behaviour in accounts, beneficial-ownership data quality, and PF sanctions
  screening (UN 1718 / DPRK and Iran-related lists).
- Foreign correspondent banks will apply enhanced due diligence to Iraqi banks. An Iraqi bank that can
  show a modern, explainable, auditable monitoring system has a commercial advantage in keeping
  correspondent (USD) relationships.
- Peer guidance (e.g., UAE) says grey-listing should drive enhanced monitoring, not blanket de-risking,
  so Iraqi banks will be asked by counterparties to evidence their controls (Wolfsberg CBDDQ etc.).
- FATF in the same plenary: public consultation on Recommendation 16 (payment transparency) guidance
  (from week of 22 June 2026); R.6 updated with humanitarian carve-outs (UNSC 2664 and 2761); upcoming
  work on hawala/underground banking risk indicators (Sept 2026) and public-private information sharing.

Sources:
- [FATF — Jurisdictions under Increased Monitoring, 19 June 2026](https://www.fatf-gafi.org/en/publications/High-risk-and-other-monitored-jurisdictions/increased-monitoring-june-2026.html)
- [FATF — Outcomes FATF Plenary, 17-19 June 2026](https://www.fatf-gafi.org/en/publications/Fatfgeneral/outcomes-fatf-plenary-june-2026.html)
- [Al Jazeera (Arabic) — Iraq on the FATF grey list, 19 June 2026](https://www.aljazeera.net/ebusiness/2026/6/19/%D8%A7%D9%84%D8%B9%D8%B1%D8%A7%D9%82-%D8%B9%D9%84%D9%89-%D8%A7%D9%84%D9%82%D8%A7%D8%A6%D9%85%D8%A9-%D8%A7%D9%84%D8%B1%D9%85%D8%A7%D8%AF%D9%8A%D8%A9-%D9%84%D9%85%D8%AC%D9%85%D9%88%D8%B9%D8%A9)
- [ComplyAdvantage — FATF plenary June 2026 outcomes](https://complyadvantage.com/insights/fatf-plenary-june-2026-outcomes/)
- [Sanction Scanner — FATF plenary June 2026](https://www.sanctionscanner.com/blog/fatf-plenary-june-2026-grey-list-changes-and-key-outcomes-1391)
- [Compliance7 — FATF grey list June 2026](https://blog.compliance7.com/2026/06/25/fatf-grey-list-june-2026/)
- [AML UAE — grey list update 19 June 2026](https://amluae.com/fatf-grey-list-countries-update-19th-june-2026/)

---

## 2. MENAFATF Mutual Evaluation of Iraq (on-site August 2023, published 2024/Jan 2025)

- On-site visit: 6-23 August 2023. Report dated May 2024 (PDF created Dec 2024; FATF announced
  publication in January 2025). 345 pages. Second-round MENAFATF evaluation.
- Technical compliance: Compliant on 14 and Largely Compliant on 13 of the 40 Recommendations.
- Effectiveness (11 Immediate Outcomes): IO1 Moderate, IO2 Substantial, IO3 Moderate, IO4 Moderate,
  IO5 Moderate, IO6 Moderate, IO7 Low, IO8 Substantial, IO9 Moderate, IO10 Moderate, IO11 Low.
  (IO4 = preventive measures at FIs, which is what bank AML systems serve; IO6 = use of financial
  intelligence; IO11 = proliferation financing sanctions.)

### Key findings from the executive summary (verbatim points, paraphrased lightly)

- **Banks are the core of the system.** Banking = about 96.2% of total assets of the formal financial
  sector. Cash economy estimated at about 42% of GDP in 2017 (about USD 73 billion).
- **National Risk Assessment (NRA), completed October 2022.** ML threat by sector: real estate and
  exchange companies "high"; banking "moderate high"; e-payment companies "moderate"; dealers in
  precious metals and stones "moderate low"; securities, insurance, lawyers, accountants "low".
  TF: banking "moderate"; other sectors not assessed for TF.
- **Main threats:** corruption (major proceeds generator), drug trafficking, smuggling of oil and oil
  derivatives, trafficking in national antiquities, cross-border organised crime. TF: ISIL remnants
  (now funded by extortion, theft, drug smuggling, foreign funding), unlicensed money transfer
  companies moving funds between governorates and abroad, cash couriers across borders.
- **Weakness that lands on banks (IO4):** "there are several clear deficiencies in the practical
  application of AML/CFT obligations by all FIs, including banks and exchange companies engaged in
  foreign remittances, namely when it comes to monitoring transactions and reporting suspicions."
- Banks apply CDD, record-keeping and EDD (correspondent banking, higher-risk countries, PEPs) but
  "without having a clear mechanism in place to verify the source of their wealth and that of their
  relatives" (PEPs).
- Banks and exchange companies file most STRs; they often get no useful feedback from the FIU, which
  hurts STR quality. Exchange-company STRs are of lower quality than bank STRs.
- STRs received by the Office are weak in number; many sectors file none.
- **CTR regime:** the Office receives Cash Transaction Reports for cash transactions equal to or above
  USD 10,000 (or equivalent in any currency). It also receives cross-border currency declarations
  from the General Authority of Customs (GAC).
- **Targeted financial sanctions (TFS):** Iraq applies UN TFS "without delay" only to a limited extent
  because of deficiencies in disseminating sanctions lists in a timely manner. PF-related TFS are
  implemented "in a weak manner" because of delays in disseminating lists and updates to the private
  sector. Most FIs (except banks and insurers) have weak understanding of PF TFS.
- **Beneficial ownership:** the Companies Registration Department (CRD) has no mechanism to verify the
  accuracy of BO information filed by legal persons. BO information quality is "good among banks".
- **Supervision:** CBI applies a risk-based approach to supervision of FIs it supervises (not yet
  e-payment companies). CBI sanctions (warnings, fines, administrative) are mostly dissuasive.

### Legal instruments named in the MER

- Law No. 39 of 2015 on AML/CFT — Official Gazette issue 4387, 16 November 2015. TF criminalised in
  art. 1/tenth. Gap noted: does not criminalise financing travel of foreign terrorist fighters.
- Art. 8: the AML/CFT Office established at the CBI as a Public Department with legal personality and
  financial and administrative autonomy. Art. 9: receives and analyses reports, requests additional
  information from reporting entities, disseminates to law enforcement / Public Prosecution.
- Council of Ministers Decision No. 5 of 2016: regulation for freezing terrorists' funds; creates the
  **Committee for Freezing Terrorist Funds (CFTF)** that applies UN TFS. Amended by Decision No. 4 of
  2022 (adds PF TFS) and Decision No. 6 of 2023 (fixes R.6/R.7 technical gaps).
- **CBI Instructions No. 1 of 2017 on CDD rules** for FIs (banks, money transfer, e-payment,
  securities, insurance) — **cancelled and replaced by Instructions No. 1 of 2023**, applicable to all
  FIs and DNFBPs. (This is the key CDD rulebook AKR-TMS must implement.)
- CBI Controls No. 1 of 2017: declaration of cross-border movement of funds (inbound/outbound), all
  modes of transport.
- Strategic plan 2023-2025 (confiscation as a policy objective).

### Technical compliance ratings (2024 MER) — the ones that matter for a bank AML system

| Rec | Topic | Rating |
|---|---|---|
| R.1 | Assessing risks, risk-based approach | LC |
| R.6 | TFS — terrorism/TF | LC |
| R.7 | TFS — proliferation financing | LC |
| R.8 | NPOs | PC |
| R.10 | Customer due diligence | C |
| R.11 | Record keeping | C |
| R.12 | PEPs | C |
| R.13 | Correspondent banking | C |
| R.14 | Money or value transfer services | PC |
| R.15 | New technologies (incl. virtual assets) | **NC** |
| R.16 | Wire transfers | C |
| R.17 | Reliance on third parties | C |
| R.18 | Internal controls, foreign branches | C |
| R.19 | Higher-risk countries | LC |
| R.20 | Reporting suspicious transactions | C |
| R.21 | Tipping-off and confidentiality | LC |
| R.24 | BO of legal persons | PC |
| R.25 | BO of legal arrangements | PC |
| R.26 | Regulation and supervision of FIs | PC |
| R.29 | FIU | LC |
| R.34 | Guidance and feedback | LC |
| R.35 | Sanctions | PC |

Reading: the LAW is largely in place for banks (R.10-R.13, R.16, R.20 all Compliant). The problem is
EFFECTIVENESS — banks' practical transaction monitoring and suspicion reporting (IO4 Moderate),
use of financial intelligence (IO6 Moderate), PF sanctions (IO11 Low). That is exactly the gap a
monitoring system closes: the rules exist, execution is weak.

### Priority actions (MER) that a bank system can directly support

- (b) Raise the number AND quality of STRs; FIU should give feedback.
- (f) Implement TFS without delay; **"complete the subscription of all entities concerned with the
  implementation to the electronic notification program as soon as possible to receive updates made to
  the sanctions lists in a timely manner"** — Iraq has an electronic notification programme for
  sanctions-list updates. AKR-TMS should ingest national list updates automatically and re-screen the
  whole customer base the same day.
- (g) Circulate listing decisions without delay; reduce PF risk; enhanced monitoring of trade with
  **Iran** in particular (dual-use goods).
- (h) FIs "rely mainly on the customer's statement" for beneficial ownership; need proper BO
  identification procedures and "activating ongoing monitoring of their customers' transactions".
- (a) More network analysis of ML linked to corruption by officials and PEPs, drug trafficking,
  cross-border cash smuggling, including in the Kurdistan Region.
- MVTS: no effective mechanism to identify unlicensed VASPs; sanctions on unlicensed MVTS not dissuasive.

### IO4 (preventive measures) — how Iraqi banks actually work today (MER chapter 5, paras 523-563)

Key findings for IO4, verbatim where it matters:

- "some FIs have insufficient human resources to effectively monitor and investigate alerts for STRs
  within a reasonable timeframe, especially considering the high number of alerts." -> ALERT FATIGUE IS
  AN IRAQI PROBLEM TOO. A system that cuts false positives directly fixes a named MER weakness.
- Identifying relatives and close associates of PEPs "relies on the experience and knowledge of financial
  institution employees or directly asking the client"; banks recognise "the family name and its
  connection to political figures (primarily Iraqi)". No criteria for degree of kinship. -> A local Iraqi
  PEP + relatives/associates register with family/tribal-name linking is a real gap.
- Feedback from the FIU to reporting entities is "considered inappropriate in all cases" per the
  institutions interviewed.

Practices observed at banks (useful as AKR-TMS defaults):

- **CBI-issued monitoring scenarios.** Banks run continuous monitoring "based on RBA scenarios for the
  sector, although these institutions had no role in developing these scenarios, they were developed by
  the CBI based on the sectoral assessment of banks and required banks to implement them."
  -> AKR-TMS must ship the CBI scenario set out of the box (need the actual list — research item).
- **Periodic KYC refresh in practice:** high-risk customers every year, medium every two years, low every
  three years.
- Some banks require Compliance Department approval to open a relationship, and sometimes per
  transaction (e.g., NPOs, commercial transactions).
- Field visits to company premises/warehouses to verify legal entities exist and do what they declare.
- ID documents used for verification: national ID cards, residence cards, ration cards. No reliance on
  third parties.
- **Beneficial ownership:** natural persons self-declare on the KYC form. For corporates, banks and
  exchange companies check the Companies Registration Department (CRD) for owners with **10% or higher**
  stakes; managers are verified too. Banks also use independent sources, MOIs, shareholder info, annual
  financial statements, at onboarding and at each update; an update can change the risk rating and
  therefore the automated monitoring.
- **PEPs:** domestic and foreign PEPs both high risk, no differentiation in practice; domestic PEPs seen
  as the bigger concern given Iraq's political situation. Banks screen against **commercial databases**
  to find PEPs; senior management approval before onboarding a PEP or when a customer becomes one;
  source of funds documented, but "no information available on how banks verify the source of wealth".
- **Correspondent banking:** Iraqi banks use CBDDQ-type questionnaires; they do not act as correspondents
  for respondent banks (no nested or payable-through accounts).
- **New products:** CBI approval required, based on a risk assessment, before launching new products,
  channels, mobile payments, e-wallets.
- **Targeted financial sanctions:** names screened before onboarding; **daily** checks and list updates via
  electronic systems and the local list on the AML/CFT Office website; banks also screen against other
  international lists. An **electronic notification platform** for list updates was introduced in **2022**.
  On a positive match: freeze the funds and file an STR with the Office. (Interviewed FIs claimed no
  positive matches and no false positives — implausible, suggests weak screening/record-keeping.)
- **Wire transfers:** parties screened against local and international lists; CBI found "violations related
  to many banks executing suspicious transfers where all the required information and data for the
  transfer were not taken into account", and **banned several banks on more than one occasion from
  external transfers or foreign exchange.**
- **Record keeping:** at least **5 years** after completion of the transaction or end of the relationship.
- **Supervisory fines:** CBI reported 300 fines in 2018-2022 (MER figures internally inconsistent: "about
  180 billion IQD" total vs "average 600,000 IQD per fine"). CDD violations 2018-2022: 361 found on-site,
  1,318 off-site.
- **STRs filed because CDD could not be completed:** banks 132 (97 refused relationship, 35 refused
  transaction); exchange companies 167 (123 / 44).

Other sectors (context):

- Exchange companies (categories A and B) do domestic and international transfers; they lowered their
  foreign-transfer threshold to about USD 3,000; verify powers of attorney via a notary barcode system;
  use electronic monitoring systems. Several were used for TF and had licences revoked by the CBI.
- 17 e-payment companies; their clients' transactions run through bank accounts.
- Real estate: cash payments for property above about **USD 380,000** must be deposited in a bank in Iraq.
- DPMS apply CDD above **USD 10,000** (stricter than FATF's USD 15,000), per AML/CFT Council instruction.
- Securities brokers cap deposits/withdrawals at USD 10,000 per day.
- No licensed VASPs; CBI warned against using cards/e-wallets to trade crypto. Trading crypto is not
  prohibited by law (as of 2023), and there is no regulatory framework (now an action-plan item).
- About 17% of bank branches are in the Kurdistan Region; all banks are licensed by the CBI in Baghdad.
- Guidance with suspicion indicators exists for human trafficking (banks) and real estate brokers (2023).

### Monitoring systems, alert workflow and STRs at Iraqi banks (MER paras 563-582)

- **Every CBI-supervised FI (banks, exchange companies, e-payment) runs an automated monitoring system**
  using **standardised scenarios developed and disseminated by the CBI**. For banks the set grew from
  **34 to 56 scenarios** after the CBI's sectoral assessment. **No interviewed institution had built any
  scenario of its own** — the MER reads this as a lack of expertise to tailor detection to their own
  risk. -> AKR-TMS: ship the CBI 56-scenario pack as the baseline, plus a guided way for each bank to add,
  test and tune its own scenarios (a named MER gap).
  *Update 2026-10-06 (file 18, checked by the lead in the Trans Iraq Bank 2022 report, p.70):* external auditors at two
  banks (Ameen Al-Iraq 2021-2024, Trans Iraq Bank 2022-2024) still describe the CBI minimum as **25 scenarios**, against
  56 in the MER and 86 numbered scenarios in the 2025 controls (file 27). Trans Iraq Bank added 81 (2023), then 96 (2024)
  scenarios through a vendor (Pio-Tech), so some banks do extend the set, through vendors rather than in-house. The pack
  must be a versioned register with a per-bank coverage report (VD-39).
- **Two-tier alert workflow already in place:** each branch has a designated, trained liaison officer
  who does first review of alerts; the central AML/CFT department then reviews those decisions (close for
  insufficient grounds, or escalate). Large numbers of alerts are archived daily against very few STRs;
  the MER reads some banks as deliberately avoiding defensive filing. -> AKR-TMS must model
  branch-level first review + central second review, with reason codes, and measure alert-to-STR
  conversion.
- **CBI transfer-monitoring system (2023):** after repeated cases, the CBI built a system so that no
  transfer can be processed unless all required data is complete. MER: banks still must verify the
  ACCURACY of transfer data, not just completeness. All external transfers go through the CBI.
- **Electronic reporting system:** STRs are filed electronically; bank STRs rose after banks were
  connected (MER text says banks reached 430 STRs in 2022, but its own table shows 82 bank ML STRs in
  2022 and 428 from money transfer companies — internal inconsistency). Electronic reporting gives an
  automatic receipt; FIs say feedback is only an acknowledgement, with no outcome.
- **Tipping-off:** if asking the customer for more information could tip them off, institutions stop
  asking and report immediately with what they have.
- Banks: compliance officer at management level, staff screening, independent audit unit, training.
  Many AML staff are well qualified (some with more than 10 years' experience).

#### The FIU's reporting platform is goAML (confirmed)

- MER para 179: at the end of 2022 the AML/CFT Office launched (pilot) "an advanced electronic reporting
  system to receive STRs from the reporting entities, as well as to receive cash transactions reports",
  linking several banks, FIs, DNFBPs, the Ministry of Interior and the Companies Registration Department.
- MER para 259: "after adopting the **GoAML** late 2022, the classification process became automated".
  MER para 278: correspondence and information exchange "takes place through the GoAML system".
- -> AKR-TMS must produce goAML-compatible reports (STR, SAR/TFR, CTR) — XML per the UNODC goAML schema,
  validated before upload. (goAML schema research: see 23-regulatory-reporting-goaml.md.)

#### How the FIU grades STR quality (build this check INTO the STR form)

- Since 2017 the Office grades each STR (good / moderate / acceptable / poor) against a form. Criteria
  include whether the STR contains **real indicators of suspicion** — e.g., value or frequency of
  deposits/withdrawals/transfers not in line with the customer's known activity and income, or equal
  amounts transferred to several individuals in different countries — and whether it is supported by
  **all documents and the measures the reporting entity took**.
- Good/moderate/acceptable STRs get priority. Poor STRs go back to the reporter to complete, AND the
  supervisor (CBI) is notified. Automated since goAML.
- -> AKR-TMS: a pre-submission "STR quality check" that mirrors these criteria (indicator present? profile
  mismatch quantified? documents attached? actions taken recorded?) protects the bank from "poor" grades
  that are reported to the CBI.

#### Other facts from the MER

- **CBI External Transfer Controls No. 5/11/1083 dated 19 February 2023** (to all banks) — automation and
  control of external transfers, long the system's major vulnerability.
- Real estate purchases above **IQD 500 million (about USD 380,000)** must be completed through banks.
- The AML/CFT Office joined the **Egmont Group in July 2023**.
- Box 3.1 lists FIU databases: AML/CFT database (STRs, TF reports, CTRs, FIU requests), Companies
  Registry (shareholders, founders, board, managing director), real estate records, tax register, and
  "Customer data of banks: CBS system linked with banks allows direct access to customer information,
  accounts associated with them, companies, their shareholders and managers without the need to refer to
  banks" (a CBI-side system linked to banks — to clarify).
- Federal Commission of Integrity (FCOI) holds financial disclosures of officials (family, companies,
  property, funds) — used by the FIU via goAML. Relevant for PEP source-of-wealth checks.
- Third national AML/CFT strategy: 13 objectives, incl. (7) strengthening FIs' reporting and RBA,
  (11) implementing UNSC resolutions immediately, (12) financial inclusion and fighting the informal sector.
- NPOs: annual audit if income or spending ≥ IQD 75 million (about USD 57,250); NPOs are obliged entities
  that must report suspicions; high-risk NPOs inspected 2-3 times a year.

#### STR statistics 2018-2022 (MER table 5.2.1)

| Reporter | 2018 | 2019 | 2020 | 2021 | 2022 | Total |
|---|---|---|---|---|---|---|
| Banks — ML/predicate | 158 | 143 | 110 | 135 | 82 | 628 |
| Money transfer companies — ML | 5 | 3 | 437 | 397 | 428 | 1,270 |
| E-payment companies — ML | 1 | 1 | 13 | 59 | 38 | 112 |
| Currency exchange companies — ML | - | - | - | - | 14 | 14 |
| Banks — TF | 9 | 7 | 5 | 4 | 2 | 27 |
| Money transfer companies — TF | - | - | 31 | 21 | 9 | 61 |

- Population of reporters: **74 banks, 560 exchange companies, 17 e-payment companies.**
- 628 ML STRs from 74 banks over five years = roughly **1.7 STRs per bank per year**, and falling. The MER
  calls the number "extremely low". Total STRs received: 2,114 (2,026 ML/predicate + 88 TF).
- By predicate (all sources, 2018-2022, total 2,354): generic "suspicious transactions" (deposits vs
  income, activity/profession mismatch, invoice values, transfers, no justification, false info) 1,911;
  TF 115; corruption 94; fraud 92; forgery 88; cheque endorsement 16; extortion 10; tax evasion 6;
  virtual currencies 5; customs evasion 4; gambling 3; antiquities 2; drug trafficking 2.
- CBI inspection findings — "suitable systems to detect and update unusual transactions": banks 4
  violations (2018-2021); exchange companies 38 (2021-2022).

#### Two MER case studies (good material for the plan)

1. **Two passports (exchange company):** a customer sent USD 7,400 to Turkey with one Iraqi passport, then
   tried to send IQD 10,032,000 with a second, different Iraqi passport. Both passports were genuinely
   issued. The second transfer was cancelled; the customer had also received three transfers from abroad.
   Referred to law enforcement. -> Lesson: entity resolution across identity documents.
2. **USD 21 million "chemical imports" (bank):** a company and its authorised director made external
   transfers totalling USD 21 million over two years, supposedly to import chemicals. The company could
   not prove its source of funds; the bank filed an STR. The FIU matched the director to a corruption
   investigation; part of the funds came from cheques issued by a state department; customs permits did
   not match the volume transferred. The FIU suspended the account under **Article 9/First/C9 of Law
   39/2015**, got a court freeze, sent an MLA request to the UAE, and properties worth IQD 8.2 billion
   (five upscale properties in Baghdad owned by the director and family) were frozen.
   -> Lesson: trade-based ML (transfers vs customs data), PEP/corruption links, state cheques as source.

Sources:
- [FATF — Iraq's measures to combat ML/TF (MER 2024)](https://www.fatf-gafi.org/en/publications/Mutualevaluations/MER-Iraq-2024.html)
- [MENAFATF — Iraq MER (English PDF)](https://menafatf.org/sites/default/files/MER_Iraq_English.pdf)
- [MENAFATF — Iraq 9th follow-up report / exit from first round](https://menafatf.org/sites/default/files/Newsletter/Iraq%20Exit%20FUR_En.pdf)

---

## 2b. The CDD rulebook: AML/CFT Council Instructions No. 1 of 2023 (Customer Due Diligence Rules)

Issued by the Chairman of the AML/CFT Council under Article 10/Fourth of Law 39/2015; applies to all FIs
and DNFBPs; replaced CBI Instructions No. 1 of 2017. Article-level content as cited in the 2024 MER
technical-compliance annex (R.10-R.21). These are the rules AKR-TMS must encode as configuration:

| Topic | Rule | Source |
|---|---|---|
| Anonymous/fictitious accounts | Prohibited | Law art. 12/Second |
| CDD timing | Before/during account opening or relationship | Law art. 10/Second/A |
| Occasional customer threshold | CDD when a transaction **≥ USD 10,000** (or IQD/FX equivalent), single or **several linked transactions** | Instr. art. 2/4 |
| Wire transfer, occasional customer | CDD **regardless of value** | Instr. art. 3/4 |
| Suspicion / doubtful data | CDD on suspicion of ML/TF, or doubts about earlier ID data | Law art. 10/Second D-E; Instr. 4/4, 4/5 |
| Legal persons | Name, legal form, principal address, registration docs, MOA, by-laws, signatories, senior managers | Instr. art. 5 |
| Persons acting on behalf | Verify identity + official power of attorney | Law 10/First/B; Instr. art. 6 |
| **Beneficial owner threshold** | Natural person owning **20% or more**, directly or indirectly; else control by other means (personal ties, **close family relationships or historical ties**); else senior managing official | Instr. art. 7/First/B |
| BO guidance | AML/CFT Council BO guidance 2023 (enforceable): indirect ownership via chains | — |
| Legal arrangements | Settlor, trustee, protector, beneficiaries, controllers; equivalent roles for **waqf (endowments)** | Instr. art. 7/First/C |
| Ongoing monitoring | Examine transactions vs customer info, activity, risk pattern, source of funds | Law 10/First/E |
| Keeping data current | Review records, especially higher-risk customers | Instr. art. 9/2 |
| Delayed verification | Allowed only as soon as practicable, to avoid interrupting business, with risk controls: limits/ceilings on number, type, amount of transactions; monitor large/complex transactions | Law 10/Third; Instr. arts. 10, 2/10 |
| Existing customers | CDD on materiality and risk basis | Instr. art. 27 |
| Risk assessment & RBA | Chapter Three; low-risk factors art. 19; RBA art. 21 | Instr. arts. 19-21 |
| New products/tech | Risk-assess before launch | Instr. art. 20 |
| EDD | Higher-risk relationships: more and deeper monitoring | Instr. art. 22/2 |
| SDD | Only with national risk analysis, low risk, per supervisor guidance | Instr. arts. 19, 21 |
| Correspondent banking | Info on respondent, assess controls, senior mgmt approval, document roles; no shell banks | Instr. art. 23 |
| Higher-risk countries | EDD for FATF-called countries; countermeasures circulated by Council | Instr. art. 24 |
| **PEPs** | Foreign PEPs: risk systems to detect, **senior management approval** (new and existing), **source of wealth AND source of funds**, enhanced ongoing monitoring. Domestic PEPs and intl-org officials: same when higher risk. **Family members and close associates included** | Instr. art. 25 |
| Wire transfers (ordering) | Originator name, account no. (or unique reference), address or ID no. or customer no., date and place of birth — **verified**; beneficiary name and account/reference; batch files; **no de-minimis threshold**; domestic = international; do not execute if incomplete | Instr. art. 28 |
| Wire transfers (intermediary) | Keep info with transfer; keep 5 years if technical limits; STP-compatible detection of missing info; risk-based execute/reject/suspend | Instr. art. 29 |
| Wire transfers (beneficiary) | Detect missing info (post-event or real-time); verify beneficiary; risk-based policy | Instr. art. 30 |
| MVTS agents | Keep agent list; include agents in AML programme | Instr. art. 31 |
| Record keeping | **≥ 5 years** after transaction or end of relationship; records must allow **reconstruction of individual transactions**; available swiftly to authorities | Law art. 11; Instr. art. 32 |
| AML programme | Compliance officer/unit, staff screening, ongoing training, **independent audit** of systems | Instr. art. 33 |
| Group-wide programme | Information sharing within group with confidentiality and anti-tipping-off safeguards | Instr. art. 35 |
| Third-party reliance | With supervisor approval; FI keeps responsibility | Instr. art. 13 |
| **STR obligation** | **"Promptly" notify the Office of any suspected ML/TF transaction, whether executed or not (attempts), any amount.** Predicate offence = every felony or misdemeanour in Iraqi law | Law arts. 1/18, 12/Fifth/A; Instr. art. 36 |
| Tipping-off during CDD | Stop CDD if it would tip off; file STR immediately | Instr. art. 15 |
| Cannot complete CDD | Do not open/transact; terminate; notify the Office | Law art. 10/Fifth |
| Tipping-off | FI must not disclose to customer or others (gap: law does not name staff explicitly) | Law art. 12/Fourth |
| Safe harbour | No liability for good-faith reporting | Law art. 48 |

Penalty references in the MER: Law art. 39/First fines IQD 25-250 million; other administrative fines
IQD 1-25 million; CBI Law 56/2004 art. 62 fines up to IQD 10 million per violation for unlicensed
activity; Banking Law 94/2004 art. 3 (licence needed), art. 27 (transfers are banking business), art. 57.

Other Iraqi laws referenced: Banking Law No. 94 of 2004; CBI Law No. 56 of 2004; Companies Law No. 21 of
1997 (amended 2004); CBI controls for mobile e-payment providers (2020); controls for primary/secondary
agents of foreign remittance providers (Western Union, MoneyGram agents). CBI warning Nov 2022 and
decision March 2023 banning use of cards/e-wallets for crypto speculation via banks and non-bank FIs.

## 2c. CBI circulars of 26 August 2026 (issued after the grey-listing) — read from the scanned originals

Both signed by Prof. Dr. Ammar Hamad Khalaf, Deputy Governor (acting), Banking Supervision Department,
dated 2026-08-26, published on cbi.iq page 139 ("instructions and controls for banking supervision").

### A. "Behavioural indicators for transactions suspected of ML or TF" (to all licensed banks)

Source: https://cbi.iq/static/uploads/up/file-178963291760044.pdf (6 pages, scanned)

Preamble: based on FATF Recommendations, Law 39/2015, and **the AML/CFT Office's 2025 guidance manual on
reporting suspicious transactions and activities**, the CBI issued modern behavioural indicators and
scenarios to strengthen SAR monitoring. They focus on BEHAVIOUR rather than amount thresholds, "given the
inadequacy of amount thresholds in detecting some ML/TF patterns". They strengthen front-line staff as
the first line of defence.

Instructions to banks:
1. SARs to the AML/CFT Office go **through goAML**; they must be clear, accurate, specific, with all
   data, information and documents on the persons/entities involved.
2. On noticing an indicator: do not alert the customer; continue normal procedures; **document the
   observation precisely**; the head of the bank's AML/CFT department (or deputy) files the SAR via goAML
   through approved channels, preserving confidentiality.
3. The SAR must include a **coherent, detailed narrative** consistent with Law 39 and the Office's 2025
   guidance manual.
4. Missing information is acceptable if the maximum available is given and what is unknown is stated.
5. **Behavioural indicators depend on human observation and many cannot be detected by AML systems unless
   front-line staff document them. Banks must therefore develop automated monitoring scenarios in their
   AML systems for the automatable patterns — transaction restructuring, splitting (structuring) and
   repeated patterns — so automated monitoring and behavioural monitoring reinforce each other.**
6. Senior management must prioritise training front-line staff; an **annual training plan** approved by
   senior management; **competency tests** for front-line staff; keep training records for the CBI.
7. Update scenarios and internal indicators using the NRA, sectoral assessments, customer types,
   products/services and channels.

The indicators below are the MINIMUM front-line staff must be trained on; banks may add scenarios and
indicators. Indicators are not proof; they require review and follow-up.

**1. Attempts to avoid due diligence**
- Explicit questions about reporting limits (amounts that trigger reporting, extra forms, or notifying
  supervisors).
- Restructuring a transaction: lowering or splitting the amount on learning it exceeds a limit.
- Inconsistent or unverifiable information (altered ID data/addresses, answers that change, simple facts
  that cannot be verified).
- Unusual substitute documents (secondary documents instead of required ones; unofficial issuers).
- Evasive answers on source of funds ("personal money", "savings", "general trade") with no details or
  documents, plus pressure to finish quickly.
- Weak knowledge of the customer's own business.
- Excessive, unrequested justification (long stories to convince the clerk).
- Unusual objection or irritation when asked for CDD information or documents.

**2. Behaviour during the transaction**
- Avoiding identification (insists on wearing something that prevents identity verification, refuses to
  remove it).
- Change in behaviour or tone when documents or details are requested; changing the subject.
- Changing explanations of the same transaction without reason.

**3. Physical indicators on cash**
- Unusual traces: strong chemical smells, mould/rot from long storage, unknown residues.
- Contaminated or dyed notes.
- Non-bank packaging: bags, envelopes, containers, heat-shrink wrap.
- Denominations that do not fit the activity: large amounts in small, disordered notes.

**4. Customer taking instructions / transaction run by someone else**
- Reading written answers or checking the phone before answering.
- Relying on notes with beneficiary names, amounts, purposes; cannot explain beyond them.
- A companion hands the customer the cash inside the branch or just before the transaction.
- **The same person accompanies several customers at different times for similar transactions** (a
  money-mule "herder" pattern).

**5. Bribery or manipulation of staff**
- Building an unjustified personal relationship with the employee.
- Offering a benefit to skip or speed up procedures, skip CDD, or exceed transfer limits.
- Invoking relationships with officials or influence to change a decision.
- Exploiting the employee's emotions with unverifiable emergencies.
- Offering a commission or a percentage of the transfer / of future volume.

**6. After the transaction**
- Requests to delete or change information after completion without clear error.
- Refusing receipts or notices, or asking that they not be kept (only meaningful with other indicators).
- Cancelling or reversing a transaction shortly after execution without clear reason.
- Sudden change in demeanour after completion.
- Asking that the beneficiary not be notified, or that some details be hidden.
- Repeatedly returning to change non-material descriptive details (rewriting the justification).

**7. Terrorist financing**
- Reliable negative information, or requests/notices from competent authorities, linking the customer,
  beneficiary or related party to terrorism or TF.
- Requests to transact for persons/entities suspected of TF links, even if not executed.
- Using personal accounts to receive donations for "charitable or humanitarian" purposes without a licence
  or institutional framework.
- Using intermediaries or third parties to hide the true beneficiary or purpose.
- Refusing CDD or source-of-funds information after being told of compliance requirements, then
  cancelling the request.

**8. Criteria to escalate to a SAR** (front-line staff document and refer to the reporting unit when one
or more applies)
1. More than one behavioural, operational or financial indicator in one transaction or a chain of linked
   transactions.
2. Incoherent or contradictory explanations of source of funds, purpose, or relationship with the other
   party.
3. A link to a high-risk customer, beneficiary, third party, activity or country, or any element that
   raises risk under the bank's assessment.
4. A clear attempt to avoid CDD or controls (restructuring, refusing documents).
5. Signs the customer takes instructions from someone else or does not understand the transaction.
6. The same indicators recurring for the customer across transactions or within short periods.
7. Other information held by the bank, or from supervisors or law enforcement, that strengthens suspicion.

The circular says the behavioural indicators complement — not replace — CDD/EDD, and complement "all
scenarios previously issued by this bank and the AML/CFT Office"; staff must act neutrally without
harming good-faith customers.

**AKR-TMS implications (strong differentiator):**
- A **branch observation log**: a fast form (web/phone) where tellers tick the CBI indicators above for a
  customer, attach notes, and route to the AML unit. Observations become first-class signals that join
  automated alerts on the same customer timeline ("hybrid alert": observation + transaction pattern).
- The 8 escalation criteria become a **built-in SAR decision checklist** inside each case.
- Training records and competency-test tracking for front-line staff (CBI asks for them on inspection).
- Automated scenarios for the automatable indicators: structuring after a limit enquiry, quick
  reversals/cancellations, repeated data edits after execution, one person linked to many customers'
  transactions (shared phone/address/introducer), donations into personal accounts.

### B. "Due diligence towards politically exposed persons (PEPs)" (to all licensed banks and non-bank FIs)

Source: https://cbi.iq/static/uploads/up/file-178963295727758.pdf (4 pages, scanned). Explicitly issued
"in response to the FATF action plan".

- Apply EDD on a risk basis to PEPs, high-risk position holders **and their relatives up to the second
  degree**. PEP status is not a presumption of wrongdoing; it is an indicator that requires extra controls.
- References **"Controls for due diligence towards high-risk senior position holders for FIs and DNFBPs
  No. (2) of 2023"** (the detailed PEP controls; earlier version 2017-08-31).

Requirements:
1. Risk-based assessment of PEP customers considering position type, level of influence, activity,
   related countries, transaction pattern and other indicators; written, updated PEP policy (standalone or
   within AML policy) with classification and risk-management mechanisms, reviewed periodically by the
   compliance officer; copy sent to the CBI within compliance reports.
2. Verify whether the applicant, **beneficial owner, authorised signatory or any related person** is a PEP
   (local or foreign or senior position holder) **before** the relationship and **throughout** it.
3. Keep search systems and databases continuously updated to identify **PEPs, family members and close
   associates**; update **immediately** when office holders change; also check they are not on sanctions
   lists; **do not rely only on the politician names inside commercial search systems** — also search
   **reliable local and regional news and media sources as part of daily screening** to catch accounts or
   transfers tied to a PEP's inner circle used to pass corruption proceeds.
4. Examine company structures; extra care where PEPs or relatives hold shares or indirect control; EDD on
   those companies; detailed analysis of complex structures to reach the ultimate BO.
5. On unjustified suspicion indicators about PEP transactions, their BOs or associates: file a **SAR via
   goAML** without informing the customer.
6. **Senior management approval** to deal with PEPs / open accounts; periodic effectiveness review; ongoing
   training of compliance, account-opening, front-line and supervisory staff on FATF R.12.
7. If a PEP did not disclose PEP status on the KYC form despite signing a declaration, and the bank's
   searches confirm it, the customer bears legal consequences; the bank takes **immediate precautionary
   measures on the account** and considers a SAR.
8. **EDD continues after the PEP leaves office.** Leaving office, retirement or end of term does not end the
   classification automatically; it can be ended or downgraded only after a **documented risk assessment**.
9. Banks must measure PEP-EDD effectiveness and keep statistics (retained per Law art. 11), available to
   the CBI on request — at minimum:
   a. number of PEP customers by category;
   b. PEPs found by screening systems vs PEPs who self-disclosed;
   c. relationships needing senior approval; number refused, terminated or restricted for high risk;
   d. number of data updates and PEP risk re-assessments;
   e. number of SARs on PEPs and associates;
   f. compliance/internal-audit test results on these procedures and % of findings remediated;
   g. % of relevant staff trained on PEPs.
10. Minimum PEP red flags (banks may add scenarios):
   a. wealth/source of funds inconsistent with monthly income or known position as a government employee;
   b. unusual or high-value transactions inconsistent with activity, during or after tenure;
   c. transfers with high-risk countries/regions without economic rationale;
   d. third parties, companies or complex structures hiding the BO or source of funds;
   e. unjustified transactions with family members, close associates or related entities;
   f. reliable negative news about corruption, bribery, fraud or financial crime;
   g. insufficient or inconsistent source-of-funds/wealth documents;
   h. sudden unjustified changes in transaction pattern or asset size.
- CBI inspection teams will check compliance with these indicators and factor it into the rating of the
  bank's AML/CFT system effectiveness.

**AKR-TMS implications:** an Iraqi PEP register with relatives to 2nd degree and close associates, updated
the day an office changes hands; adverse-media screening of local/regional Arabic news; PEP status that
persists after office until a documented downgrade; the CBI's PEP effectiveness KPIs (a-g) as a ready
report; PEP-specific scenarios (a-h) in the scenario library; detection of undisclosed PEPs (screening hit
vs self-declaration mismatch triggers precautionary action and SAR review).

### C. Other CBI documents on the same page (to read next)

- AML/CFT/CPF supervisory controls, 2025-01-08: file-173666439114818.pdf (76 pages, text PDF)
- AML/CFT controls for e-payment service providers, 2025-04-20: file-174643326199402.pdf
- Risk-management controls for Islamic banks / commercial banks, 2026-07-13
- Digital bank licensing controls + standards handbook 2026 (2026-04-14)
- AML/CFT controls for banks and non-bank FIs, 2016-09-19: file-150079544562212.pdf
- Islamic finance instruments controls, 2018-08-01
- Procedures to verify source of funds of FX buy/sell applicants, 2016-07-31
- PEP controls 2017-08-31 (superseded by Controls No. 2 of 2023)

## 2d. CBI AML/CFT/CPF supervisory controls for banks (8 January 2025) — THE AML SYSTEM REQUIREMENTS

Source: "الضوابط الرقابية الخاصة بمكافحة غسل الأموال وتمويل الإرهاب ومنع انتشار التسلح" (Supervisory controls
for AML, CFT and counter-proliferation), CBI, dated 2025-01-08, 76 pages (74 numbered), applies to all banks
licensed by the CBI operating in Iraq. File: https://cbi.iq/static/uploads/up/file-173666439114818.pdf
(text layer partly garbled — key pages were read from rendered images).

Structure (chapters): 1 definitions, ML stages, TF, PF concepts; 2 principles (KYC, risk-based approach,
reporting, investigation and training); 3 governance (board, MLRO / head of the AML reporting department,
external auditors' extra duties, compliance officer, risk management); 4 internal control systems and record
keeping; 5 CDD, EDD, SDD, ongoing monitoring, **AML systems and screening systems**; 6 indicators by product
(cash, transfers, general ML/TF scenarios, LCs, LGs, credit, digital banking, cards, FX, safe deposit lockers,
securities, customer behaviour, other indicators, employee behaviour and know-your-employee); 7-15 further
chapters (to skim).

Definition 24 (p.9): "AML systems: a set of systems that provide software solutions for receiving customer
movements and deposits, processing and recording them, to find out how consistent these movements and cash
deposits are with the customer's flows and cash income." Definition 26: goAML = the UNODC digital reporting
system used by the AML/CFT Office to receive STRs in electronic form.

### Chapter 5, Article 5 — "AML/CFT systems and search-and-investigation (screening) systems" (doc pp. 47-48)

**First — the AML/CFT system.** All banks must operate a system that contains AT LEAST:

1. **Instant, real-time follow-up** ("متابعة آنية وفورية") of all movements on customer accounts, measuring and
   analysing amounts withdrawn or deposited against (a) the cash flows the customer declared under KYC and in
   the account-opening form, and (b) **the ML/TF scenarios approved by the AML/CFT Office**, raising alerts as
   they happen.
2. **Customer risk classification** based on each customer's cash flows, nature of business, geographic
   location, and the products and delivery channels used — at minimum **High / Medium / Low**; a **five-level**
   scale is allowed.
3. **Direct link to the core banking system** on one side and to the **sanctions/ban-list screening platform**
   on the other ("Automatic Integration").
4. **Uses AI and machine-learning algorithms** to analyse data and identify possibly suspicious transactions;
   on that basis the MLRO sends STRs to the AML/CFT Office **via goAML**.
5. Analyses the patterns of customers' financial operations and **compares actual volumes with what the
   customer disclosed** (expected activity vs actual).
6. **Stops any transaction that matches one or more suspicion scenarios until the MLRO verifies it is sound**,
   or stops movements on an account whose risk the bank cannot bear, as decided by the AML reporting,
   compliance, risk, audit and IT departments. (Real-time interdiction requirement.)
7. **Identifies PEPs** so they are handled according to their risk and the required approvals are obtained
   before the relationship starts.

**Second — screening systems for local and international ban and sanctions lists.**

1. Digital systems to search individuals and entities under local or international sanctions, including:
   UN Security Council list; US Treasury **OFAC** list; **EU** list; and the **Committee for Freezing Terrorist
   Funds** (a committee in the General Secretariat of the Council of Ministers that freezes funds of
   terrorists designated by the UN Security Council committee) — i.e. the Iraqi national list.
2. Screening is done **through the World-Check system** (named explicitly) for all individuals and entities
   wishing to deal with banks and other FIs; it must also cover **all parties dealt with, directly or
   indirectly**, and **identify the beneficial owners** of entities and legal arrangements.
3. The screening systems must support **updating the local and international lists every 12 hours at least**
   ("كل (12) ساعة خلال اليوم الواحد في أقل تقدير").
4. The screening platform must support **search in Arabic and English**.

### Chapter 5, Article 4 — "Ongoing monitoring of transactions" (doc pp. 45-47)

- CDD on identity when a **transfer exceeds USD 10,000** (or equivalent in IQD / other currencies).
- **Outgoing transfers:** the ordering bank enters full originator data in the transfer form: name; account
  number or unique reference if no account; address; **purpose of transfer**; beneficiary name, address,
  account number if available. Walk-in originators: collect personal data and keep a certified copy of the
  ID card / passport. Verify before executing. Batch transfers: may carry account number / unique ID if the
  bank keeps full originator data and can **provide it to the receiving bank within 3 working days**, and
  responds immediately to orders from competent authorities. Non-routine transfers must not be batched where
  that raises ML/TF risk.
- **Incoming transfers:** effective procedures and systems to **detect transfers missing complete originator
  information**; treat it as a suspicion factor and report to the AML reporting department; request the
  missing data from the ordering institution and, if not supplied, take risk-based measures **including
  rejecting the transfer**. Beneficiary without an account: collect data and keep a certified copy of the
  national ID card, **family card** or passport. Intermediary banks keep all accompanying data; if technical
  limits prevent it, keep whatever is available **for 5 years**; an intermediary that receives incomplete
  originator data must inform the receiving bank, which must refuse the transfer. Exemptions: card
  transactions where the card number travels with the transfer; bank-to-bank transfers for own account.
- An **internal system for ongoing monitoring** of customer transactions against what the bank knows about
  the customer; monitoring intensity follows the customer's risk level, nature and size of activity,
  nationality and links abroad.
- **Periodic review** (or when needed), especially of high-risk customers, and **dynamic update of customer
  risk records** (with each customer interaction) **through the customer risk management system linked to
  the AML system**.
- Special attention to complex or unusually large transactions and unusual patterns with no apparent economic
  or lawful purpose (large vs the relationship, above set limits, inconsistent with balance, outside the usual
  pattern); examine background and purpose, **record the findings in writing, keep them for at least 5 years**
  for supervisors and external auditors.
- Special attention to internal reports on large, complex and suspicious operations; an **integrated system**
  that defines how these reports flow, who owns them and how they are analysed, **reviewed daily by the head
  of the AML department**.

### Chapter 5, Article 3 — simplified due diligence (doc p. 45)

SDD allowed where risk is low: ministries and government bodies; FIs subject to AML/CFT rules consistent with
the law; occasional transactions **below IQD 1 million**; walk-in digital transfers **below IQD 1 million**.
The bank keeps the evidence supporting the low-risk classification. SDD is NOT allowed when ML/TF is suspected,
when acting for another person suspected of ML/TF, or when **linked transactions appear designed to stay
under the threshold**. Prohibited (Art. 2 end, p. 45): anonymous persons or fictitious names; **shell banks**;
anyone providing virtual-asset services without a licence/registration.

### Chapter 2, Article 2 — risk-based approach (doc pp. 15-19)

Based on FATF Recommendation 1. Banks must:

- **Risk assessment:** develop a risk-based monitoring approach proportionate to business size, number of
  customers and transaction types; classify customers AND products by ML/TF risk; give special attention to
  high-risk situations; apply measures proportionate to the risk level; classify risk as **High / Medium /
  Low**.
- **Risk identification:** (1) **review each customer's risk classification at least every two years**, or
  sooner when changes occur; (2) acquire an AML system that measures and identifies risk across **customers,
  products, delivery channels and geography**, **directly linked to the core banking system and to the
  sanctions-screening platform ("Automatic Integration")** — the document draws it as a chain:
  `Core banking system -> AUTOMATIC INTEGRATION -> AML system -> OFAC / UN / EU / UK`; (3) prepare a **detailed
  risk matrix** that the system relies on, covering high-risk customer types (e.g. DNFBPs, PEPs, NPOs,
  non-residents); (4) **load the AML/CFT Office's scenarios into the AML system and keep the system updated with
  any new scenarios the Office issues**.
- **Applying measures:** the risk-management system must cover identification, assessment, monitoring,
  management and reporting of risk across all risk areas. Customer risk must consider at least **four
  elements: customer, product, delivery channel, geography**.
  - **Customer risk factors listed:** BO hard to identify (complex ownership of legal persons); customers with
    tarnished reputation or past dealings; non-residents; PEPs and their associates; foreign customers;
    holders of other nationalities who are non-resident; real-estate dealers; precious-metals dealers; car
    dealers; any DNFBP. EDD and ongoing monitoring when a customer may be an NPO linked to terrorism, is
    sanctioned, appears on lists circulated to banks, or is a PEP. **No relationship with NPOs or EDD
    customers without senior executive management approval after EDD is complete.**
  - **Transaction risk factors:** inconsistent with stated purpose; services out of line with the customer;
    complex or large without reason; branch far from home or work; several accounts in the same area without
    purpose; large cash where the activity is not cash-intensive; sudden pattern change or adverse
    information; unexplained intermediaries; excessive secrecy; **non-face-to-face and technology-enabled
    transactions**.
  - **Sector risk:** cash-intensive activities, money transfer and exchange companies, charities and NPOs,
    precious metals and gems, antiques and art, real-estate brokers and companies.
  - **Product risk:** savings accounts, transfer products, correspondent payable-through accounts, digital
    transfers; new or innovative products that limit identity disclosure; international products (network
    services, stored-value cards, international digital transfers). Monitor at least: number of outgoing and
    incoming external transfers, number of accounts opened per year, cash-transaction risk, complexity of
    correspondent accounts, volume of gold bullion / precious-metals products, products linked to DNFBPs.
  - **Delivery-channel risk:** assess and document risks of digital channels at onboarding and during the
    relationship; channel-specific EDD; customer classification by channel.
- **Geographic risk (doc p. 19):** high-risk geographies include sanctioned/embargoed countries (UN or similar);
  countries without adequate AML/CFT law or not applying FATF Recommendations; countries funding or supporting
  terrorism; countries with high corruption or illicit activity (drugs, arms smuggling). Consider the
  customer's **nationality, country of residence, country of work, country of registration (legal persons)**,
  and **customers of branches in border areas**, plus transactions for customers in high-risk countries. FATF
  black list and grey list are named explicitly. To judge another country's regime: legal framework, sanctions
  and supervision, international cooperation.

- **Legal arrangements / waqf (Art. 1 items 9-11, p. 15):** know all parties (settlor/endower, trustee/mutawalli,
  beneficiaries, protector, agents, anyone with effective control); **screen all parties** against local and
  international lists; verify source of funds; check whether any party is a PEP; follow the AML/CFT Office's BO
  guidance for legal arrangements.

### Chapter 2, Article 3 — reporting (doc pp. 20-21)

- Report **all suspicious transactions including attempts, regardless of amount**, to the AML/CFT Office;
  the report must give **detailed reasons, facts and circumstances**; use the Office's form and its completion
  instructions, attaching data and copies of all related documents.
- Internal reporting: a **short reporting line**; every official and employee with reasonable grounds must
  quickly file an **internal report to the head of the AML reporting department**, including details of the
  customer's subsequent transactions. The head **documents the report, gives the employee a written
  acknowledgment of receipt, reminds them of the confidentiality / tipping-off rules**, examines it, decides
  whether it is suspicious, and **notifies the employee of the decision in writing**.
- Tipping-off prohibited (customer, beneficiary, any non-competent party). Safe harbour: Law 39/2015 Art. 48.
  Law Art. 12 (Fifth/A): notify the Office **immediately** of any suspected transaction, executed or not.
- Senior management must make all relevant staff report, and explain the penalties for failing to report
  (Law Art. 39 Second/A).

### Chapter 2, Article 4 — vetting and continuous training (doc pp. 21-22)

Fit-and-proper hiring checks; **at least annual** training plans for all officials and staff including AML
staff; periodic training-needs review (skills, roles, bank size, risk rating, past results) considered by the
board; programmes planned with the AML/CFT Office and the CBI; all units covered; compliance officer
coordinates nominations.

**AKR-TMS implications (reporting/training):** the internal-report workflow is prescribed almost step by step —
employee files an internal report (branch observation log), the system issues a **written acknowledgment** with
the tipping-off reminder, the AML head records a decision, and the employee gets a **written decision notice**.
All timestamps must be kept for examiners. A training register (who was trained on what, when, test score)
supports both this Article and the 2026 behavioural-indicator circular.

### Chapter 3 — governance: board, MLRO, records (doc pp. 22-26)

**Board duties (Art. 1, items 15-17):** ensure CDD is applied; ensure records are kept **5 years from the end
of the relationship, account closure or occasional transaction, whichever is longest**, and can be provided to
competent authorities quickly. Records include at least all CDD records (ID documents of customers and BOs,
account files, business correspondence) and **all local and international transactions, executed or
attempted, detailed enough to reconstruct each transaction step by step**. The AML programme must include a
risk assessment, internal policies and controls, staff integrity standards, continuous training, independent
audit, and a system "with the efficiency and effectiveness needed to measure risks related to customers, the
nature of their business and their geographic location, using the latest methods".

**MLRO (head of the AML/CFT reporting department) — appointment rules (Art. 2):** at least **30 years old**,
**Iraqi**, university degree (law, business administration, finance, accounting, banking, economics or
statistics; deputy may hold a diploma); training of at least **50 hours (MLRO) and 75 hours (deputy)** in AML/CFT
inside or outside Iraq, and the bank must get the MLRO and deputy certified within one year (**ACAMS, CGSS or
CFCS**) or the approval lapses; **5+ years** banking experience with good English (deputy 3+ years); resident in
Iraq, full-time, not employed by another FI; no convictions; **appointed with CBI approval after passing a CBI
test, and cannot be dismissed without CBI approval**. Any change must be notified to the AML/CFT Office and
the CBI.

**MLRO guarantees:** independence; no conflicting tasks; **right to access all information, records and
documents** needed to examine unusual-transaction reports and STRs, and to contact any employee; right to report
to the board via the audit committee; **full confidentiality** of incoming unusual-transaction reports and STRs;
**coordinates with compliance, risk management and IT on activating the AML scenarios** required by the CBI and
the AML/CFT Office, and on additions or changes when supervisors issue instructions.

**MLRO duties (selection):** (a) examine the unusual transactions the bank's internal systems surface and the
suspicious transactions reported by staff with reasons, or from any other source; (b) file STRs with the AML/CFT
Office on its forms; (c) **decide to close ("keep") a case where no suspicion is found — the decision must state
its reasons**; (d) propose AML policy updates; (e) supervise all branches' compliance, desk-based and on site;
(g) **field visits and site inspections of high-risk customers**; (i) ensure the department is staffed in
proportion to branch business, and **supervise the liaison officers in the branches**; (j) **review and audit the
monthly reports of the branch liaison officers** — they must list the cases reported and the latest action on
each, and be available to inspectors; (k) **detailed periodic reports to the board via the audit committee** on
unusual and suspicious transactions and actions taken, with copies kept for CBI inspection; (l) analyse
customers' financial data and check consistency of transaction volume with account movement; (m) verify all
account-opening documents and the evidence of the customer's activity (official papers, incorporation contracts,
professional licences, import invoices); (n) **update the KYC form periodically, at least once a year**;
(o) vetting at hiring; (p) a **correspondent-bank policy** listing correspondents, their ratings from
international rating agencies, their ML/TF risk and the due diligence done.

### Chapter 4 — internal controls and record keeping (doc pp. 35-37)

- Internal controls: board-approved AML policy (regional manager for foreign branches); detailed written
  procedures; **internal systems able to detect unusual transactions or transactions with suspicious customers
  and put them before the MLRO**; a mechanism to check compliance; internal audit tests the systems' efficiency
  and effectiveness and proposes fixes.
- **Records to keep:** (a) customer and BO records incl. account-opening applications, ID copies,
  correspondence; (b) transaction records sufficient to identify each operation; (c) **unusual-transaction
  reports and the results of reviewing them**; (d) STR records incl. copies of reports sent to the AML/CFT Office
  with data and documents; (e) **records of cases the MLRO decided to close**; (f) **training records** (programme,
  trainees, departments, content, duration, provider).
- **Conditions:** stored securely **with backup copies in another location**; retrievable easily and quickly so
  any data requested is provided in full without delay.
- **Retention: at least 5 years** — customer/BO records and transaction records from account closure (or end of
  the occasional transaction); unusual-transaction reports from the report date; others as specified.

**AKR-TMS implications (governance):** an MLRO workspace that shows every system-surfaced unusual transaction
and staff report; **case closure requires a written reason** (stored and reportable); a **liaison-officer monthly
report** generated per branch (cases reported, latest action) that the MLRO reviews and signs off; a **board /
audit-committee report pack**; a **correspondent-bank register** with ratings and DD status; KYC refresh tracking
with an **annual** default per the MLRO duty (configurable by risk tier); evidence vault with off-site backup,
fast retrieval and 5-year-plus retention clocks that start at account closure; full transaction reconstruction
(executed and attempted).

### Chapter 8 — high-risk countries (doc pp. 64-65)

Based on FATF R.19 and Law 39/2015: EDD on business relationships and transactions with natural and legal
persons and FIs from FATF-identified high-risk countries, proportionate countermeasures when FATF calls for
them. The controls describe four list colours: green (no issues), **grey** (cooperating, with strategic
deficiencies and an action plan), red (non-cooperative), **black** (non-cooperative, subject to
countermeasures / UNSC sanctions). Required measures: special attention to relationships and transactions to and
from these countries; extra information on the customer and transactions; **review correspondent relationships in
those countries**; verify nature and purpose; **source of funds and wealth**; periodic KYC refresh for foreign
customers; **senior management approval to continue the relationship**; enhanced transaction monitoring.

### Chapter 9 — counter-proliferation financing (doc pp. 65-68)

Banks must apply **freezing without delay** for any transaction suspected of supporting proliferation, and keep
policies updated. Typologies: front/shell companies with complex ownership in low-transparency jurisdictions;
hiding behind residents of other (often neighbouring, low-risk-looking) countries; foreign FIs with correspondent
accounts acting for sanctioned states; exploiting neighbours' trade and shipping networks.
**PF indicators** (to encode as scenarios / screening rules):
- *Customer:* data similar to a listed person or entity; involved in supplying or buying **dual-use, strategic or
  military goods** for high-risk countries; address similar to a listed party or with an export-control violation
  record; linked to a proliferation-concern country (e.g. dual national handling complex equipment without
  technical background); military or research body in a high-risk jurisdiction.
- *Activity:* party in a high-PF-risk country; shared directors, owners or addresses between trading companies
  (residential address used as company address); complex shipping route and indirect payments without reason;
  wire pattern tied to dual-use or sensitive goods; sudden change in transfer behaviour; unusual use of products;
  payment to third parties named in LG or other documents; shipment off normal trade patterns or **vessel on a
  sanctions list**; goods from a country other than the declared end-user's; sudden repeated changes of board
  members or signatories; contradictions between trade documents and financial flows.
- *Geography:* countries linked to PF; weak AML/CFT or export-control regimes; receiving country produces
  dual-use or military goods; known diversion hubs.
- *Trade documents / shipping:* declared value below shipping cost; mismatches between documents and goods;
  unexplained third parties; consignee is a shipping company or freight forwarder; forged shipping, customs or
  payment documents; goods out of line with the destination's technical level (e.g. semiconductor equipment to a
  country without that industry); circuitous routing.

**AKR-TMS implications (high-risk countries and PF):** a country-risk table with FATF status (black / grey, with
dates), sanctions programmes, CPI-style corruption score and border-area flags, versioned so every rating can be
traced to the list in force that day; geography factored into customer risk (nationality, residence, work
country, registration country, border branch). PF needs **trade-finance screening**: goods descriptions against a
dual-use keyword list, vessel and port screening (IMO numbers), and price checks against shipping cost. The MER
rated Iraq **IO11 (PF sanctions) Low** and the FATF action plan names PF sanctions evasion, so this is a
selling point, not a nice-to-have.

### Chapter 10 — correspondent banking (doc pp. 68-70)

Minimum measures: check staff compliance and audit scope over correspondent relationships; policies to identify
the **beneficial owner** of correspondent accounts and EDD for customers with direct access (payable-through);
a **comprehensive risk assessment of each correspondent**, updated periodically; adequate approval process;
**monitoring of transactions on correspondent accounts**; detect transactions unusual for the expected volume and
report them; for any transaction with correspondents, investigate the BO via official correspondence
**documented exclusively over SWIFT**. Due diligence before opening: understand the respondent's business,
reputation and supervision; assess its AML/CFT controls; **senior management approval**; document each side's
AML/CFT responsibilities; payable-through accounts only if the respondent does CDD on direct-access customers;
**no shell banks** and no FIs that let shell banks use their accounts; a written AML questionnaire (Wolfsberg-style);
terminate relationships with non-compliant correspondents; understand nested relationships and payable-through
risk; know the respondent's products, ownership, board and management, AML controls, sanctions history and home
jurisdiction.
To win correspondent relationships abroad, Iraqi banks must have: 3 years of financial statements audited by a
top-10 global firm; an **independent AML/CFT and compliance assessment by an international firm**; a credit
rating from an agency recognised by the CBI (**S&P, Fitch, Moody's, Capital Intelligence, Islamic International
Rating Agency**); an up-to-date **Bankers Almanac** profile; and certified staff in transfers, compliance and AML.

### Chapter 11 — branch liaison officers ("موظف الارتباط") (doc pp. 70-71)

Every bank appoints a liaison officer in **every branch** by administrative order, with a named alternate. The
liaison officer:
1. is the **MLRO's assistant in the branch**;
2. gets **"a digital window in the AML system specific to their branch"** through which they examine all the
   branch's transactions and check that all documents are attached;
3. **signs the account-opening (KYC) form** on behalf of the MLRO (in the head-office branch only the MLRO or
   deputy signs);
4. verifies customer documents and IDs against originals;
5. **checks the customer is not on local or international lists**, and gives the branch the CFTF lists published
   on the AML/CFT Office's official website (or obtains them through the department head);
6. **sends suspicion reports to the MLRO only — branch management has no part in it**;
7. works full-time on compliance and AML only;
8. reports **technically to the MLRO only**, administratively to the branch manager;
9. has a **dedicated phone line and e-mail** for confidential contact with the MLRO;
10. submits a **monthly report to the MLRO no later than the 10th of the following month**, listing the cases
    reported and the latest actions taken, available to inspectors on request;
11. attends the AML department's training.

### Chapter 12 — importing cash (doc pp. 71-72)

Based on FATF R.32 and CBI cash-import instructions No. 4 of 2011: banks need **prior CBI approval** for each
foreign-cash import programme (source institution, counterparty, contract date and duration, number and value of
shipments, total); the foreign seller must be licensed and AML-supervised; the bank must be financially sound;
cash arrives **by air through Civil Aviation Authority airports only**; imported cash serves customers
(companies, organisations, individuals) receiving transfers from abroad, with full CDD; licensed global carriers
(e.g. DHL Global Forwarding, GardaWorld, Brink's).

**AKR-TMS implications (correspondents and liaison officers):**
- Chapter 11 describes AKR-TMS's branch workspace almost word for word: a **branch-scoped "window"** for each
  liaison officer showing every branch transaction, document-completeness flags and screening results; a
  **confidential internal-report channel straight to the MLRO** that branch management cannot see; and an
  **auto-drafted monthly liaison report due by the 10th**, with MLRO review and sign-off. Role-based access must
  enforce "branch sees only its own branch" and "branch manager cannot see suspicion reports".
- A correspondent-bank module: register, risk assessment, questionnaire tracking (Wolfsberg CBDDQ), approvals,
  periodic review, and monitoring of nested / payable-through flows in correspondent (nostro/vostro) accounts.
- The bank's own **evidence pack for foreign correspondents** (independent AML assessment, programme description,
  statistics) is a selling point: US correspondent banks decide whether Iraqi banks keep dollar access.

### Chapters 13-15 — secrecy, penalties, general guidance (doc pp. 72-74)

- **Banking secrecy** (Banking Law 94/2004 Art. 49) must not obstruct AML/CFT controls.
- **Penalties:** sanctions under Banking Law 94/2004, AML/CFT Law 39/2015 and Companies Law 21/1997 (amended)
  apply to violations of these controls.
- **General guidance:** use every available means to monitor suspicious transactions, focusing on those passing
  through countries on the non-cooperative list and on internationally wanted persons and entities; follow
  typologies from FATF, IMF, World Bank, Basel Committee; know the source of funds at account opening,
  especially large cash deposits.
- **Dormant accounts:** comply with Banking Law Art. 37 and CBI dormant-account instructions No. 1 of 2009
  (balances transfer to the CBI after a set period). **Current accounts unused for 6 months are frozen** and may
  only be reactivated when the holder or legal agent attends in person with ID, signs a reactivation request,
  and the reactivation is audited and **signed by the branch manager AND the branch liaison officer**; internal
  control audits reactivations in branches **monthly**. Applies to individual and company current accounts (not
  savings, term deposits, government accounts or card-based accounts). Operations on such accounts only through
  the branch manager or deputy; withdrawals by cheque only.
- **Reports the automated system must produce** ("program the automated system to prepare reports that improve
  internal AML/CFT controls"): (a) **monthly current-account movements and balances** for all customer AND
  employee accounts — all movements, month-end balance, average balance, number of operations — to spot unusual
  activity; (b) **transfer reports** — all incoming and outgoing, domestic and foreign transfers with amount,
  currency and payment method (cash or cheque) per customer; (c) **foreign correspondent account reports** —
  all transfers by any means, amount, currency, beneficiary bank, number and volume of dealings with each foreign
  bank, and changes.

Signed: Ali Mohsen Ismail, Governor of the Central Bank of Iraq.

**AKR-TMS implications (chapter 15):** dormant-account control is a ready-made scenario family — reactivation
without the two signatures, activity on a frozen account, large inflow right after reactivation (also CBI
general scenario 1 and cash scenario 6). The three standard reports (monthly account movements incl. staff
accounts, transfers register, correspondent account activity) should ship as built-in regulatory reports.

### What this means for AKR-TMS (design implications — high priority)

- The CBI explicitly expects **real-time monitoring** AND the power to **hold a transaction that hits a
  scenario until the MLRO clears it**. A read-only DB link cannot stop a payment. So the optional
  **pre-release screening/scoring API** for the payment/SWIFT gateway (and ideally a CBS-side hook the bank
  configures) is not optional for full compliance in the long run — it is how a bank meets item 6. Phase 1
  covers near-real-time detection from the DB (minutes); phase 2 adds the hold-and-release API.
- **Expected vs actual activity** is a named requirement: the KYC profile must store declared monthly
  turnover / cash flows / purpose, and the system must compare continuously.
- **Customer risk rating**: 3 levels minimum, 5 allowed; factors are customer cash flows, business, geography,
  products, channels; **dynamic** re-rating on events, tied to the AML system.
- **AI/ML is written into the rule**, so the product needs explainable ML (risk scoring, alert prioritisation,
  anomaly detection) on top of the CBI scenarios — and must document it for examiners.
- **Screening**: UN, OFAC, EU and Iraqi CFTF lists at minimum; list refresh at least every 12 hours (AKR-TMS:
  continuous ingestion + full re-screen of the customer base on every list change); bilingual Arabic/English
  fuzzy search; screen all parties incl. counterparties and beneficial owners. **World-Check is named**, so
  AKR-TMS must either integrate World-Check (bank's licence) or show equivalence — design a pluggable
  data-source layer (World-Check / Dow Jones / own lists).
- **Wire transfer completeness checks** (missing originator/beneficiary data, purpose of transfer) are
  explicit system duties for incoming and outgoing transfers.
- **Written rationale for unusual-transaction reviews**, kept 5+ years, and a daily review queue for the
  head of AML.

## 2e. Other CBI rulebooks that touch AML systems (skimmed 2026-10-06)

**Digital bank licensing controls (CBI, April 2026, 12 pages)** — "ضوابط ترخيص المصارف الرقمية في العراق":
- Item 11: the licence file must include a governance manual with an information-security policy, risk policy,
  compliance policy, **AML/CFT policy**, anti-fraud/corruption/bribery policy.
- Item 12: the CBI may require the applicant to appoint one or more independent specialist firms to assess the
  IT infrastructure and security, governance and operations, and the **overall risk level including AML/CFT
  and cybersecurity risk**.
- **Item 13: "Digital banks must provide specialised systems for screening against international and local
  sanctions lists, and specialised AML/CFT systems with daily transaction monitoring, linked to the bank's core
  banking system."**
- Ownership: max qualifying holding 9.99% per person or company (incl. related parties), exceptions for
  "qualified institutional investors" (FIs/fintechs not in FATF grey or black-listed jurisdictions, 3+ years
  serving customers, IQD 30bn+ revenue, 100k+ active users; or funds with IQD 100bn+ AUM) up to 60%.
- Related parties defined to the 4th degree of kinship (1st: parents, spouse, children; 2nd: grandparents,
  siblings, grandchildren; 3rd: uncles/aunts; 4th: cousins).
- Market note: new digital-bank licences in Iraq are a greenfield segment for AKR-TMS (cloud-native, API-first,
  no legacy AML system).

**CBI standards handbook for private commercial and Islamic banks (2025, 17 pages)** — "كتيب معايير":
- Standard D2 (AML/CFT/sanctions): every bank must have AML/CFT and sanctions policies, processes and controls
  and **use methodological tools**, in line with CBI requirements set out in the assessment guide; **compliance
  must be verified by a specialist firm from the CBI's approved list**.
- D3: accounts audited under IFRS (e.g. IFRS 9) by **at least two independent auditors**, one from the CBI's
  pre-approved list; rotate auditors every 5 years.
- D4: internal controls built on the **three lines of defence**, assessed by a CBI-approved firm.
- Implication: banks will be independently assessed on their AML tooling; AKR-TMS should produce the evidence
  an assessor asks for (coverage matrix, tuning records, model documentation, QA results, audit trail).

**CBI risk-management controls for Islamic banks / commercial banks (13 July 2026, ~100 pages each)** — the AML
section is short and generic: risk understanding across customer base, products/services, delivery channels
and countries, using internal data and external sources; board-approved policies covering customer acceptance
(esp. PEPs), CDD incl. BO, identification of persons on CBI lists and UNSC resolutions, **ongoing monitoring with
appropriate systems that detect unusual or suspicious transactions and identify prohibited ones**, retention of
CDD data **at least 5 years after the relationship ends**, timely reporting, and special procedures for
correspondents, exchange offices and e-money transfer firms. The IT-risk section refers to the CBI's **IT
governance controls** and **cyber-resilience controls for the financial sector**, and requires at least
**annual testing including penetration tests** (feeds file 30). The Islamic version also restates Sharia rules
for istisna'a, musharaka and mudaraba contracts (feeds file 24).

**CBI AML/CFT controls for e-payment service providers (20 April 2025, 48 pages, scanned/garbled)** — mentions
goAML, KYC, AML systems; not read in detail because the product targets banks first. Revisit if AKR-TMS is
offered to PSPs / e-wallets later.

## 3. Core law: Anti-Money Laundering and Counter-Terrorism Financing Law No. 39 of 2015

- Governs ML and TF in Iraq; replaced the 2004 CPA Order 93 regime.
- Created the AML/CFT Council and the AML/CFT Office (the FIU) inside the Central Bank of Iraq, replacing
  the old Money Laundering Reporting Office.
- Reporting entities: financial institutions and DNFBPs.
- CDD: identify and verify customer and beneficial owner using reliable, independent source documents,
  data or information; apply CDD before or during account opening / establishing a relationship, and
  before an occasional transaction or wire transfer above a threshold.
- Record keeping: accurate and complete records of transactions and customer interactions.
- STR trigger (as summarised by US State Dept INCSR): funds derived from illegal activities or ML,
  intended to finance crime (including terrorism), over which a criminal organisation has disposal power,
  or transactions to evade any law or with no apparent business or lawful purpose.
- The AML/CFT Office can suspend a transaction (reported as up to one week / 7 days).
- (Article-level details — thresholds, retention period, penalties, tipping-off, PEPs, wire transfers —
  to be added from the law text below.)

Sources:
- [Law No. 39 of 2015 — English translation (aml.iq)](https://www.aml.iq/wp-content/uploads/2017/09/AMLCFT-Law-of-Iraq.pdf)
- [Law No. 39 of 2015 — Ministry of Justice (Arabic)](https://moj.gov.iq/upload/pdf/anti.pdf)
- [CBI — law text](https://cbi.iq/static/uploads/up/file-150199913642139.pdf)
- [US State Dept INCSR 2016 — Iraq](https://2009-2017.state.gov/j/inl/rls/nrcrpt/2016/vol2/253408.htm)
- [Sanction Scanner — AML in Iraq](https://www.sanctionscanner.com/aml-guide/anti-money-laundering-aml-in-iraq-720)
## 4. US dollar access: Treasury / Federal Reserve pressure on Iraqi banks (2023-2026)

Why it matters: Iraq's oil revenue is held at the Federal Reserve Bank of New York, so US authorities control
whether Iraqi banks can obtain and move dollars. Losing dollar access is an existential threat for an Iraqi
bank, far more immediate than a CBI fine. AML/sanctions quality is the condition for keeping it.

- **July 2023:** US barred **14 Iraqi banks** from dollar transactions (reported by The National, 20 Jul 2023).
- **4 February 2024:** after US accusations of breaching Iran sanctions, the CBI barred **8 banks** from its
  dollar auctions / dollar transactions: Ashur International Bank for Investment; Investment Bank of Iraq; Union
  Bank of Iraq; Kurdistan International Islamic Bank for Investment and Development; **Al-Huda Bank**; Al-Janoob
  Islamic Bank for Investment and Finance; Arabia Islamic Bank; Hammurabi Commercial Bank.
- **Al-Huda Bank — FinCEN Section 311:** FinCEN found Al-Huda Bank to be of "primary money laundering concern"
  (NPRM January 2024); **final rule published 3 July 2024, effective 2 August 2024**. Covered US institutions may
  not open or maintain correspondent accounts for Al-Huda and must apply special due diligence so that foreign
  correspondent accounts are not used for transactions involving it. FinCEN's findings: Al-Huda exploited its
  dollar access to support the **IRGC, IRGC-Quds Force, Kata'ib Hizballah and Asa'ib Ahl al-Haq**; its chairman
  was complicit, laundering through **front companies**; it had no direct US correspondent but reached the US
  system **indirectly through dollar correspondent accounts at six foreign banks** (nested correspondent risk).
- **February 2025:** after quarterly meetings in Dubai with the US Treasury and Federal Reserve, the CBI barred
  **5 more banks** from dollar transactions — Al-Mashreq Al-Arabi Islamic Bank, United Bank for Investment,
  Al-Sanam Islamic Bank, Misk Islamic Bank, Amin Iraq for Islamic Investment and Finance — plus three payment
  firms (Amwal, Al-Saqi Payment, Aqsa Payment). Note the high share of **Islamic banks** among those cut off.
- The CBI holds **quarterly meetings with the US Treasury and the Federal Reserve** (2025 meetings in Dubai,
  "with participation from international auditing and consulting firms"). US Treasury statement: CBI steps have
  led to "legitimate Iraqi banks achieving international connectivity through correspondent banking
  relationships".
- **End of the dollar auction and the electronic platform (2023-2025):** the CBI's "electronic platform" for
  external transfers was launched in early 2023 (under US pressure) to pre-vet every dollar transfer and its
  ultimate beneficiary. In November 2023 the CBI stepped back from acting as the "correspondent" itself and
  handed that role to several foreign banks. After PM al-Sudani's Washington visit (April 2024) the Iraqi and US
  governments backed ending the auction platform by end-2024. CBI (Sept 2024): **by end-2024, 95% of
  foreign-currency transactions had moved from the platform to direct banking channels**; from 2025 the
  platform was terminated and **local banks deal directly with correspondent banks abroad** (Jordan, Kuwait,
  UAE and others cited), with personal transfers via e-payment systems. The PM's adviser warned that "not all
  banks will be capable of establishing correspondent relationships abroad; only the stronger banks will manage
  to do so". Experts warned that weaker oversight after the platform could worsen ML and tax evasion. Effect:
  **the AML burden moved from the CBI's central pre-vetting to each bank and its foreign correspondent**. A bank
  without a credible AML programme cannot win or keep a correspondent, so it cannot do dollar business at all.

- **Update checked by the lead on 2026-10-06 against the CBI originals.** (1) The CBI statement of 22 February 2026
  and the list PDF dated 26 February 2026 name **28 "USD Restricted Banks"** (CBI codes BIQxxx/2) and **4
  "Blacklisted Banks"**. Restriction dates on the list: 30 Oct 2019 (1), 24 Dec 2020 (1), 3 Nov 2022 (4), **20 Jul
  2023 (14)**, **29 Jan 2024 (8)**. 17 of the 28 are Islamic by name. Blacklisted: Al Ataa Islamic (formerly
  Al-Bilad), Islamic Cooperation Bank for Investment, and the Iraqi branches of Bank Melli Iran and Parsian Bank.
  The five banks reported barred in February 2025 are **not** on the February 2026 list (cleared or never listed
  by the CBI: not verified). Listed banks "cannot participate in foreign currency windows or deal in US dollars,
  whether directly or indirectly" until reform is complete. Lead counted the 28 BIQ codes and 4 blacklisted rows in
  the PDF text. (2) **CBI statement of 18 July 2026** (cbi.iq news 3263, read in Arabic): after the Governor's
  meetings with the US Treasury, a joint understanding returns restricted banks to **external correspondent
  banking channels not linked to the US dollar** once they meet the approved compliance and governance standards
  and complete the first phase of the CBI's reform and re-licensing programme; banks completing reform and
  re-licensing later regain eligibility for dollar transactions. The Governor said **seven banks** currently
  qualify for the non-dollar channel. The CBI repeated it may restrict or suspend access to international channels
  or revoke licences of banks below standard. Sources: [CBI statement 22 Feb 2026](https://cbi.iq/news/view/3135);
  [CBI restricted and blacklisted banks list (PDF, 26 Feb 2026)](https://cbi.iq/static/uploads/up/file-177208376739661.pdf);
  [CBI statement 18 Jul 2026](https://cbi.iq/news/view/3263). See also file 24 section 7d.

**AKR-TMS implications:** the buyer's real fear is losing USD correspondent access. The product must (1) screen
against **OFAC SDN and FinCEN 311 lists** in real time, including indirect exposure (counterparties of
counterparties, front companies, shared directors/addresses); (2) detect **Iran-sanctions evasion and
militia-linked flows** (front companies, dollar smuggling patterns, trade-based ML); (3) produce an **evidence pack
for US correspondents and auditors** (programme description, screening coverage, alert statistics, tuning,
independent validation) — the "international auditing and consulting firms" in the Treasury meetings ask for
exactly this; (4) monitor **nested correspondent** activity.

Sources:
- [Mishcon de Reya — ban on five Iraqi banks from US dollar transactions](https://www.mishcon.com/news/the-ban-on-five-iraqi-banks-from-us-dollar-transactions)
- [AGBI — Iraq bans five local banks from US dollar deals (Feb 2025)](https://www.agbi.com/banking-finance/2025/02/iraq-bans-five-local-banks-from-us-dollar-deals/)
- [Iran International — Iraq bans 8 banks from dollar trade (Feb 2024)](https://www.iranintl.com/en/202402042786)
- [The National — US bars 14 Iraqi banks (Jul 2023)](https://thenational-the-national-prod.cdn.arcpublishing.com/mena/2023/07/20/us-bars-14-iraqi-banks-from-dollar-transactions-says-regulator)
- [FinCEN — final measure against Al-Huda Bank](https://www.fincen.gov/news/news-releases/fincen-finalizes-financial-measure-against-iraq-based-al-huda-bank-combat)
- [Federal Register 2024-14415 (3 Jul 2024)](https://www.govinfo.gov/content/pkg/FR-2024-07-03/html/2024-14415.htm)
- [Central Banking — Iraqi central bank blocks dollar access to eight lenders](https://www.centralbanking.com/central-banks/governance/7960785/iraqi-central-bank-blocks-dollar-access-to-eight-lenders)
- [Iraq Business News — CBI meets US Treasury and Federal Reserve](https://www.iraq-businessnews.com/?p=230456)
- [BNN Bloomberg — Iraq in talks with US Treasury over banks' dollar restrictions (Jul 2024)](https://bnnbloomberg.ca/business/international/2024/07/13/iraq-in-talks-with-us-treasury-over-banks-dollar-restrictions)
- [IraqiNews — Iraq ends dollar auction platform](https://www.iraqinews.com/business/iraq-ends-dollar-auction-platform-economic-impact/)
- [The New Region — action taken to close currency sale window (Jun 2024)](https://thenewregion.com/posts/581)


## 5. STR statistics 2016-2024 (AML/CFT Office annual reports + MER), via University of Baghdad study (Dec 2025)

Source: Dr. Qaid Hadi Dahash (College of Law, University of Baghdad), "How effective is the suspicious transaction
reporting system in Iraq?", Journal of Legal Sciences vol. 40 no. 2, 25 Dec 2025, DOI 10.35246/ank2mv33
(https://jols.uobaghdad.edu.iq/index.php/jols/article/view/1190/version/1151). Built from the AML/CFT Office
annual reports 2016-2024 and the May 2024 MER.

**ML STRs received by the Office (selected rows):**

| Reporter | 2018 | 2019 | 2020 | 2021 | 2022 | 2023 | 2024 | Total 2016-24 |
|---|---|---|---|---|---|---|---|---|
| Banks | 158 | 143 | 110 | 135 | 82 | **672** | **451** | 1,751 |
| Exchange cos (cat. a+b) | 5 | 3 | 437 | 397 | 428 | - | - | 1,270 |
| E-payment cos | 1 | 1 | 13 | 59 | 38 | - | - | 112 |
| Central Bank of Iraq (as supervisor) | 31 | 30 | 24 | 24 | 5 | 11 | 62 | 198 |
| Natural persons | 4 | 11 | 8 | 7 | 8 | 53 | - | 91 |
| All reporters | 217 | 203 | 605 | 628 | 586 | | | **5,152** |

- **Bank STRs jumped from 82 (2022) to 672 (2023) and 451 (2024)** — the first full years of goAML and the CBI
  electronic platform. Even so, 451 STRs across ~76 banks is about 6 per bank per year.
- **CBI deck "Strategic Achievements 2020-2025" (Governor's Office, 18 May 2026), slide 51 "AML/CFT in numbers"**
  (read by the lead 2026-10-06): goAML received **7,020 reports (2023-2024)**, **415 entities** connected to goAML;
  **242,884 CTRs received 2023-2025**; **503 "SAR" reports received 2023-2025** (goAML's suspicious *activity* report
  type, not the STR type, so it does not conflict with the bank STR counts above); 37,286 beneficial-ownership
  statements received on the BO platform; 738 cash seizure records and 995 currency declaration forms from the
  General Customs Authority (2023-2024); Egmont: 443 requests sent, 93 received, 28 spontaneous disclosures
  (2023-2025). [Deck PDF](https://cbi.iq/static/uploads/up/file-177935854666319.pdf).
- **FIU quality grading of bank STRs (2018-2022):** "good" share rose from 51% (2018) to 56%, 59%, 64% and **90%
  (2022)**; poor 2-5%. Non-bank FIs: good only 20-24%, poor 6-17%. Bank STRs are fewer but better.
- **TF STRs 2018-2022:** 115 in total (banks 27, exchange companies 61, CBI 16, Cabinet NGO department 11);
  42 TF files to the Public Prosecution, 10 convictions, 32 pending; only **36 bank accounts** frozen
  prudentially.
- **National sanctions list:** **4,497 persons** listed under UNSCR 1373 (as of the MER period).
- **Disseminations 2018-2022:** 1,536 (684 to law enforcement, 763 to supervisors, 89 to designated bodies,
  6 spontaneously to foreign FIUs).
- **Referrals to the Public Prosecution:** 2016 8; 2017 15; 2018 25; 2019 34; 2020 24; 2021 39; 2022 59;
  **2023 209; 2024 172**; total 585. "Kept" (closed) files 1,228; in progress 1,884.
- Of 2,239 ML STRs (2018-2022), about 11% led to referrals; 54 cases reached first-instance court, 20 convictions.
  Bank STRs led to 11 ML convictions vs 3 from exchange-company STRs.
- **Only 27 of 76 banks offer money-transfer services and deal in USD**; among those that do not, one institution
  holds **85% of banking-sector assets** (state banks). Financial inclusion 33.5% (2021) vs 20.8% (2017).
- **Confiscations 2018-2022: USD 3.16 billion**, ~85% (~USD 2.7bn) from corruption, then customs evasion
  (USD 91m, incl. oil-derivatives smuggling). Of 62 ML convictions, 17 included confiscation; 25 were
  stand-alone ML.
- Context: Transparency International CPI 2024 ranks Iraq **140 of 180**; former president (2021): at least
  **USD 150 billion** stolen and smuggled out since 2003.
- Author's conclusion: the effectiveness of the STR regime "is questionable, given the relatively low number of
  STRs reported in light of the country's large exposure to high levels of corruption, drug trafficking, fraud,
  human trafficking and other unlawful activities"; reliable statistics are scarce.

**AKR-TMS implications:** the market problem is not "too many STRs" but too few, with weak links to predicate
crimes (corruption above all). The product should raise STR volume AND keep quality "good" by FIU grading:
corruption/PEP typologies, a quality checklist before filing, and statistics the bank can show the CBI (STRs per
alert, per branch, by typology, FIU feedback). State banks hold most assets and are a hard but large segment.

## 6. The Iraqi national sanctions list and the IQTFS platform (AML/CFT Office) — checked 2026-10-06

The AML/CFT Office runs **IQTFS — "منصة العراق لعقوبات مكافحة غسل الأموال وتمويل الإرهاب" (Iraq platform for AML/CFT
sanctions)**, a web app at https://aml.iq plus mobile apps (App Store id6751137202 "IQTFS"; Google Play
"iqtfs.app"), Arabic / English / Kurdish. goAML production is at https://goaml.aml.iq/PROD/Home. The site is a
React SPA backed by a JSON API at https://api.aml.iq/api/.

**Services (from the app's own text and API routes):**
- Search the **local sanctions list** (`localsanctionslist/getall`, `getbyid`) — "browse the local sanctions lists
  of individuals and entities in Iraq".
- **UN Security Council consolidated list** (`globalindividual`, `globalentity`) and **sanctions lists issued by
  foreign states** (`foreignsanctionslist`).
- **Terrorist asset-freeze decisions** (`terroristassetfreezeorder`): official decisions freezing the funds of
  listed terrorists, with freeze decision number, date and amount.
- **Report name/entity matches against freeze decisions** and report the **volume of frozen assets and funds**
  (`frozenassetsandfundsvolume` POST) — i.e. a bank that hits a listed name reports the match and the frozen
  amount through this platform.
- **Appeals and delisting requests** (`appeal`), with tracking.
- A **watchlist** ("قائمة المراقبة"): "check a person, entity, account or payment instrument in the AML/CFT Office's
  watchlist" — access granted by the Office, with a **search quota** (`watchlist/search`, `search-fields`,
  `quota-status`).
- **Company lookup** across registries (`companylookup/search`, `companylookup/registries`).
- **Personal notifications** (`notification/getpersonalnotifications`), news, regulatory documents and guidance
  ("الأدلة الإرشادية"), a "black list" page, and the goAML system page with attachments.
- Registration with mobile login and verification codes (`auth/register`, `auth/mobile-login`).

**Local list data model (public endpoint, observed 2026-10-06):** `rowCount` **6,280** entries (628 pages × 10),
up from 4,497 at the MER. Fields: `name` (full Arabic name, typically **5-8 tokens** — given name, father,
grandfather, sometimes great-grandfather, plus tribe/family name), `type` (individual/entity, labels in Arabic,
English, Kurdish), `alias`, **`birthYear`** (year only), **`decisionNumber` + `decisionYear`** (the freezing
committee decision), **`motherName`** (full name of the mother), `isExcluded` + `excludeAttachment` (delisting),
`id`, `createdAt`. Sample of 50 rows: all individuals; mother's name present in 50/50; birth year in 48/50;
aliases in 0/50; decision years 2020, 2021, 2023, 2026 (new decisions keep arriving: decisions 54-55 of 2026 were
added on 2026-09-29).

**AKR-TMS implications (screening, Iraq-specific):**
- The national list is **Arabic-only** (no Latin transliteration) and identifies people the Iraqi way: long
  patronymic chains, tribe names, **mother's name** and **birth year**. Western screening tools that match on
  "first name + last name + full date of birth" in Latin script will miss or flood. AKR-TMS must match natively in
  Arabic on the patronymic chain (token alignment, partial chains, tribe optional) and use **mother's name and
  birth year as strong secondary identifiers** — the CBS usually stores mother's name because Iraqi ID documents
  carry it.
- Ingest the list from the IQTFS API on a schedule (at least every 12 hours per the 2025 controls; ideally hourly)
  and diff it: new decisions trigger an **immediate re-screen of the whole customer base**; delistings
  (`isExcluded`) close matches with an audit note.
- Close the loop: a confirmed match produces the freeze record and the **report to IQTFS** (match + frozen amount),
  with evidence kept in the case.
- If the Office grants watchlist access to the bank, use the watchlist API (quota-aware) as an extra check.

Sources: https://aml.iq (IQTFS web app); https://api.aml.iq/api/localsanctionslist/getall/1 (public JSON);
https://apps.apple.com/iq/app/iqtfs/id6751137202

## 7. Iraqi identity documents (shape the KYC data model and name matching)

- **Iraq National Card (البطاقة الوطنية الموحدة):** biometric card issued by the Ministry of Interior (Directorate of
  the National Card) since **1 January 2016**, compulsory, **10-year validity**, chip + ICAO-standard MRZ, backed by
  a Unified National Database of citizens and legal residents. Trade press reports **~40 million cards issued**
  (IDTechWire headline; date not verified).
- **Fields on the card:** given name, **father's name, paternal grandfather's name**, surname / tribe (optional),
  **mother's name, maternal grandfather's name**, gender, blood type, place of birth, date of birth, **family number
  (18 alphanumeric characters)**, document number, personal number, card access number; names in Arabic.
  ID-verification vendors (e.g. Uqudo's IRQ_ID) extract all of these, give transliterations, and provide
  **"Kurdish to standard Arabic" normalised name variants**. Residents' cards carry passport number, sponsor and
  residency type.
- **Legacy documents still in circulation and accepted by banks:** civil status ID (هوية الأحوال المدنية),
  nationality certificate (شهادة الجنسية العراقية), **residence card (بطاقة السكن)**, **ration card (البطاقة
  التموينية)**, family card (البطاقة العائلية) and passport. The MER notes banks accept the national ID,
  residence card and ration card; the 2025 CBI controls name national ID card, family card or passport for
  walk-in transfer beneficiaries.
- **Kurdistan Region:** Kurdish (Sorani) is written in an Arabic-based script with extra letters (e.g. پ چ ژ گ ڤ
  ڕ ڵ ۆ ێ ە); the same name can be spelt differently in Kurdish and Arabic documents.

**AKR-TMS implications:** the customer model must store the Iraqi name structure as separate parts (given name,
father, paternal grandfather, great-grandfather, tribe/family, mother's name, maternal grandfather), the
national card number and **family number** (useful to link relatives — e.g. PEP family members to the 2nd degree
share the family number), and legacy document types with expiry. Matching must normalise Kurdish letters to
Arabic equivalents and treat mother's name + birth year as strong identifiers (same fields as the national
sanctions list, section 6).

Sources:
- [Wikipedia — Iraq National Card](https://en.wikipedia.org/wiki/Iraq_National_Card)
- [Uqudo docs — IRQ_ID (Iraqi ID) fields](https://docs.uqudo.com/docs/kyc/uqudo-api/scan/country-specific-ids/irq_id-iraqi-id)
- [IDTechWire — Iraq issues 40 million electronic ID cards](https://idtechwire.com/iraq-issues-40-million-electronic-id-cards-in-major-digital-identity-milestone)
- [UK Home Office — Iraq knowledge base profile](https://www.gov.uk/government/publications/iraq-knowledge-base-profile/iraq-knowledge-base-profile)

## 8. Hosting, cloud and data protection rules (affects deployment model)

- **CBI ICT governance:** CBI Decision 14/611 of 2019 on governance and institutional management of ICT in the
  banking industry sets standards banks must follow when dealing with **cloud service providers**; the CBI also
  expects ISO 27001-aligned cybersecurity for institutions processing electronic transactions, and has issued
  cyber-resilience controls for the financial sector (referenced in the 2026 risk-management controls).
- **No explicit data-localisation law found:** market research notes that "data localisation requirements remain
  undefined", which makes foreign cloud providers hold back investment. The CMC published a **Draft Cloud
  Computing Policy (20 June 2022)** requiring cloud providers to register / be licensed by the CMC.
- **CMC Framework Regulations for Digital Platforms and Services (17 Feb 2025):** cover fintech and digital banking
  platforms serving users in Iraq; licence (domestic, large user base), registration (foreign-based, e.g.
  fintech) or notification; data protection by "international best practices" and current/forthcoming Iraqi data
  protection law; **no explicit Iraq-hosting mandate**; cloud and AI rules "anticipated"; implementing
  instructions from the CMC or CBI pending.
- Iraq has **no comprehensive personal data protection law in force** (draft stage) — so banking secrecy (Banking
  Law 94/2004) and CBI rules are the effective constraint.

**AKR-TMS implications:** default to **on-premises deployment in the bank's own data centre** (or an Iraqi data
centre the bank chooses), which matches the digital-banking-system stack (K3s on two servers, primary + standby)
and avoids the open questions on cloud and data residency. Customer data never leaves the bank; vendor support
happens through controlled remote access. A hosted/SaaS option for small banks and digital banks can come later,
inside Iraq, once CBI cloud rules are clear. External data feeds (sanctions lists, PEP data) are pulled in; no
customer data is pushed out (screening against vendor APIs should use on-prem list copies where licences allow).

Sources:
- [Mondaq — New regulatory framework for digital platforms and services in Iraq (Feb 2025)](https://www.mondaq.com/financial-services/1684654/new-regulatory-framework-for-digital-platforms-and-services-in-iraq)
- [Access Partnership — Introducing Iraq's cloud computing policy](https://accesspartnership.com/access-alert-introducing-iraqs-cloud-computing-policy)
- [MarkWide Research — Iraq ICT market](https://markwideresearch.com/iraq-ict-market)

## 9. Kurdistan Region specifics and salary localisation (Tawtin)

- The CBI supervises banks in the Kurdistan Region too (it has an Erbil branch); the same AML law (39/2015) and CBI
  controls apply. KRG has no separate AML regulator.
- **MyAccount (KRG, launched 2023):** public-sector salaries paid into personal bank accounts instead of cash. Under a
  February 2025 Baghdad-Erbil agreement, all public employees had to open bank accounts by end-2025; **cash salary
  payments ended at the start of 2026**. 800,000+ beneficiaries registered (about 400,000 in Erbil, then Sulaimani,
  Duhok); target 1m+; **USD 3bn+** moved through participating banks.
- **Federal salary localisation ("Tawtin", توطين الرواتب):** the CBI pushes payment of public salaries through bank
  accounts and cards to cut cash handling and widen banking; financial inclusion was 33.5% in 2021.
- One of the banks cut off from dollars in Feb 2024 was Kurdistan International Islamic Bank (section 4).

**AKR-TMS implications:** millions of newly banked salary customers bring huge volumes of low-value, predictable
transactions (salary in, ATM cash out). Monitoring must handle volume cheaply and learn "normal" for salary
segments (peer-group baselines), so alerts focus on deviations: salary accounts used as pass-through or mule
accounts, many salary accounts controlled by one person (shared phone/device/address — "ghost employees" is a
known Iraqi corruption typology), third parties collecting cards. Kurdish-script names must be normalised for
screening.

Sources:
- [Rudaw — MyAccount and salary payments (2025)](https://rudaw.net/english/business/28072025)
- [Rudaw — Kurdistan banking coverage](https://rudaw.net/english/kurdistan/231120253)
- [Kurdistan Chronicle — MyAccount](https://kurdistanchronicle.com/babat/4186)
