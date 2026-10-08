# 06. AML and KYC fundamentals: what the systems do and how they work

Status: done (2026-10-06). Primer; no requirement IDs (the requirements live in the topic files).

Why this file exists: the other research files assume the reader knows the vocabulary. This file explains, in plain
language, what anti-money laundering (AML) and know-your-customer (KYC) controls are for, how a bank's AML system works
from end to end, and where each piece is covered in depth elsewhere. It also gives an English-Arabic glossary, because
AKR-TMS is an Arabic-first product and its users, regulators and documents use the Arabic terms.

Contents:
- 1 The crimes the controls target
- 2 The rule chain: from the FATF to a bank's system
- 3 The risk-based approach
- 4 KYC and customer due diligence over the customer's life
- 5 Sanctions, PEP and adverse-media screening
- 6 Transaction monitoring
- 7 Investigation and case management
- 8 Reporting to the authorities
- 9 Governance: who is responsible for what
- 10 Anatomy of an AML system
- 11 Where AML systems usually fail
- 12 English-Arabic glossary
- 13 What it means for AKR-TMS
- 14 Sources

## 1. The crimes the controls target

**Money laundering** is making criminal proceeds look legitimate. The textbook model has three stages, which often
overlap in practice:

| Stage | What happens | What a bank's system can see |
|---|---|---|
| **Placement** | Criminal cash enters the financial system | Cash deposits just under reporting thresholds, many depositors paying into one account, cash at branches far from the customer's home, currency exchange in bulk |
| **Layering** | Funds are moved to hide their origin | Rapid transfers between accounts and banks, round-tripping, use of exchange companies, front companies and third-party payers, trade invoices that do not match the goods |
| **Integration** | Laundered funds come back as apparently clean money | Property purchases, loans repaid early with funds of unclear origin, company profits that do not fit the business |

In Iraq's 2022 national risk assessment (summarised in the 2024 mutual evaluation), the top proceeds-generating crimes
are corruption (theft of public money, bribery, embezzlement) and drug trafficking, both rated high, followed by oil
and fuel smuggling (file 02). Corruption proceeds are the largest by value: confiscations of about USD 3.16 billion
were mostly corruption-related (file 01 section 5).

**Terrorist financing** is providing funds for terrorist acts, terrorists or terrorist organisations. It differs from
money laundering in two ways that matter for system design: the amounts are often small, and the money can come from
legitimate sources (salaries, donations, small businesses). Detection therefore relies more on who is paid and where
the money goes (high-risk areas, designated persons, charities used as fronts) than on the size of the amounts.
In Iraq the main terrorist financing threat is ISIL remnants (file 02); US designations also target the economic
networks of Iraqi armed groups (file 05 section 1e).

**Proliferation financing** is financing the development or trade of weapons of mass destruction. For banks it shows
up mostly as **sanctions evasion**: trade with or for Iran or North Korea through front companies, vessels, and
dual-use goods. The UN re-imposed its Iran sanctions on 27 September 2025 (the "snapback", file 04 section 4), which
makes this directly relevant for Iraqi banks.

**Sanctions evasion** is any attempt to move funds for or on behalf of a designated person or country. Unlike money
laundering, sanctions breaches do not require suspicion or intent in most regimes: a payment to a listed person is a
breach even if the bank did not know. This is why sanctions screening is list-based and must be complete.

## 2. The rule chain: from the FATF to a bank's system

AML rules flow down a chain. Each layer is covered in its own file:

| Layer | Who | Instrument | File |
|---|---|---|---|
| International standard | FATF (Financial Action Task Force) and the regional body MENAFATF | 40 Recommendations, Interpretive Notes, the mutual evaluation methodology | 03 |
| International sanctions | UN Security Council | Resolutions and the UN consolidated list; asset freezes "without delay" | 04 |
| Peer standards | Basel Committee, Wolfsberg Group, Egmont Group | Banking supervision guidance, industry best practice, FIU cooperation | 04 |
| National law | Iraq's parliament | Anti-Money Laundering and Counter-Terrorism Financing Law No. 39 of 2015 | 01 |
| National bodies | AML/CFT Council; the AML/CFT Office at the CBI (Iraq's financial intelligence unit); the Committee for Freezing Terrorist Funds (CFTF) | Policy, STR reception and analysis, national list and freezing | 01 |
| Supervisor | Central Bank of Iraq (CBI) | Instructions No. 1 of 2023, the 2025 supervisory controls (with an 86-plus scenario catalogue), circulars such as the August 2026 PEP and behavioural-indicator circulars | 01, 27 |
| Foreign regimes | US, EU, UK, Gulf supervisors; correspondent banks | Sanctions, correspondent due diligence, payment transparency | 05 |
| Bank | Board, MLRO, compliance | Policies and procedures, risk assessment, the AML system | 26, 28, 29 |

The FATF checks every country through **mutual evaluations**. A country with serious weaknesses can be put on the
list of "jurisdictions under increased monitoring" (the grey list), as Iraq was again on 19 June 2026 (file 01
section 1). Grey-listing does not change Iraqi law by itself, but it changes how every foreign bank treats Iraqi banks.

## 3. The risk-based approach

The FATF does not ask banks to treat all customers the same. It asks them to **understand their risks and spend
their effort where the risk is**. In practice this has three levels:

1. **Enterprise-wide risk assessment (EWRA):** the bank assesses its exposure by customer type, product, channel and
   geography, and records how its controls reduce that exposure. The CBI requires the four risk elements and a review
   at least every two years (file 01 section 2d).
2. **Customer risk rating:** each customer gets a risk level (the CBI requires High, Medium, Low, and allows up to five
   levels), which decides how much due diligence and monitoring the customer gets (file 26).
3. **Risk-based monitoring and screening:** scenarios, thresholds and review frequencies differ by risk segment, and
   the bank can show that higher-risk segments get more attention (file 05, requirement R-02).

A risk-based approach is harder to run than a uniform one, because the bank must explain and evidence every
difference. That is why documentation, reason codes and audit trails matter so much in an AML system.

## 4. KYC and customer due diligence over the customer's life

**Know your customer (KYC)** is the umbrella term; **customer due diligence (CDD)** is the set of measures required by
law. CDD has four core parts (FATF Recommendation 10, file 03):

1. **Identify and verify** the customer (identity documents, for Iraq the national card or older civil ID with the
   family number, file 01 section 7).
2. **Identify the beneficial owner**: the natural person who ultimately owns or controls the customer. Iraq uses a
   20 percent ownership threshold (file 01).
3. **Understand the purpose and intended nature** of the relationship (expected activity, source of funds).
4. **Conduct ongoing due diligence**: keep information up to date and check that transactions fit what the bank knows.

The level of CDD follows risk:

| Level | When | What it adds or removes |
|---|---|---|
| Simplified (SDD) | Proven low risk; in Iraq for example small accounts below IQD 1 million (file 01) | Fewer documents, less frequent review; never applied where there is suspicion |
| Standard CDD | Most customers | The four core parts above |
| Enhanced (EDD) | High risk: PEPs, high-risk countries, complex ownership, cash-intensive businesses, correspondent banks, unusual activity | Source of wealth and source of funds, senior management approval, more frequent review, closer monitoring |

**The customer life cycle:**

| Stage | What happens | Where it happens for AKR-TMS banks |
|---|---|---|
| Onboarding | Collect and verify data, screen, rate risk, approve or refuse | In the core banking system (CBS); AKR-TMS screens and rates the customer from the replica as soon as the record appears |
| Ongoing monitoring | Transactions checked against the profile; list changes re-screened | AKR-TMS |
| Trigger events | Change of address, ownership, activity; adverse news; an alert or STR | AKR-TMS detects the trigger and raises a review task |
| Periodic review | Information refreshed on a cycle set by risk (the CBI requires KYC refresh at least yearly for the MLRO's review, and risk rating review at least every two years) | AKR-TMS schedules and tracks it; updated data are entered in the CBS |
| Exit | Relationship ended for risk reasons; records retained | Decision recorded in AKR-TMS; account closure done in the CBS |

AKR-TMS does not replace the CBS's onboarding screens. Its job is the risk rating, screening, CDD and EDD records,
periodic reviews and the evidence vault (the scope agreed for this project).

## 5. Sanctions, PEP and adverse-media screening

**Screening** compares names and identifiers against lists. There are four kinds:

| Kind | Compares | When |
|---|---|---|
| **Customer (name) screening** | Every customer, beneficial owner, director and authorised signatory against sanctions and PEP lists | At onboarding, on every list change (delta screening), on every customer data change, and on a full cycle (the CBUAE expects at least daily; file 05) |
| **Payment screening** | Every party, bank, country and free-text field in a payment message | Before the payment is released (requires a hook in the payment flow; phase 2 for AKR-TMS) or after the event (phase 1) |
| **PEP screening** | Customers against lists of politically exposed persons, their family members and close associates | Same as customer screening; the CBI's August 2026 circular adds daily local and regional news screening |
| **Adverse-media screening** | Customers against news about crimes, corruption and investigations | At onboarding, periodic review and on triggers |

**Lists an Iraqi bank must screen against:** the UN consolidated list; the Iraqi national list published by the
AML/CFT Office on the IQTFS platform (Arabic only, with mother's name and birth year, file 01 section 6); and, in
practice, the US OFAC, EU and UK lists, because correspondents require them (the CBI's 2025 controls name UN, OFAC and
EU and require refresh at least every 12 hours).

**Why screening is hard:** names are spelled many ways, especially Arabic names written in Latin letters (Mohammed,
Muhammad, Mohamad), and Iraqi names are long chains of given name, father's name, grandfather's name and tribe. Exact
matching misses real matches; loose matching floods analysts with false ones. Screening engines use **fuzzy
matching** (phonetic codes, edit distance, transliteration rules, name-part logic) and score each potential match.
Files 14 and 22 cover the engines and the Arabic problem.

**When a match is confirmed:** the bank must freeze the funds without delay (the FATF defines this as ideally within
hours of a designation), must not make funds available, and must report to the authorities (in Iraq to the CFTF and the AML/CFT
Office). Everything must be recorded. Because AKR-TMS only reads from the CBS, the freeze itself is an operations
task done in the CBS; AKR-TMS creates the task, then verifies from the replica that the freeze is in place and that no
money leaves the frozen account afterwards, including bank fees (file 05 section 3c).

## 6. Transaction monitoring

**Transaction monitoring** is the automated review of transactions to find activity that is unusual for the customer
or that matches a known money laundering or terrorist financing pattern (a **typology**). The system raises an
**alert**; a person decides whether the activity is suspicious.

**Detection methods**, from oldest to newest (file 21 covers them in depth):

| Method | How it works | Example | Strength and weakness |
|---|---|---|---|
| **Rules (scenarios)** | Fixed logic with thresholds | Cash deposits over IQD 15 million in 7 days; more than 10 incoming transfers from different senders in a month followed by a cash withdrawal | Easy to explain and audit; easy to evade once the threshold is known; noisy if not tuned |
| **Profiles and peer groups** | Compare a customer with their own history and with similar customers | A grocery shop whose turnover triples, or that receives far more transfers than other grocery shops | Catches change in behaviour; needs good segmentation |
| **Supervised machine learning** | A model trained on past outcomes scores how likely an alert or customer is to be truly suspicious | Rank alerts so the most likely ones are worked first; "hibernate" the least likely with sampling | Cuts noise; needs enough labelled history and model governance |
| **Unsupervised anomaly detection** | Finds activity that is statistically unusual without needing past labels | A dormant account that suddenly becomes a hub of transfers | Finds new patterns; harder to explain |
| **Network (graph) analytics** | Links customers through shared counterparties, addresses, phones, devices, employers | A ring of student accounts that all receive and forward money to the same few recipients (money mules) | Sees organised activity that single-account rules miss |

The CBI's 2025 controls require the full CBI scenario catalogue (86 or more scenarios plus product indicators) as a
minimum, real-time monitoring of account movements against the customer's declared activity, and AI or machine-learning
analytics (file 01 section 2d; the catalogue is translated in file 27).

**Batch, near-real-time and real-time.** Most monitoring systems run in daily batches after the event. Screening of
payments must happen before release to be useful. The CBI also expects a transaction that matches a scenario to be
held until the MLRO clears it. A read-only connection can see transactions minutes after they are posted, but it
cannot hold them: this is why AKR-TMS has a phase-2 pre-release screening API for the bank's payment gateway (file 05
section 5f; file 25).

**The false-positive problem.** Most alerts turn out not to be suspicious. Rates of 90 to 95 percent or more are
commonly reported, and a published NICE Actimize case started at 96 percent (file 08). Every false alert costs analyst
time, and large backlogs are themselves a regulatory failure (file 05, FCA and OFSI). Banks manage this by **tuning**:

- **Above-the-line testing:** review the alerts a rule produces and measure how many were productive.
- **Below-the-line testing:** sample activity that fell just under a threshold to see what the bank is missing if it
  raises the threshold.
- **Segmentation:** different thresholds for different customer types (a fuel trader is not a pensioner).
- **Governed change:** every threshold change is documented, tested, approved and reversible (file 29).

**Alert life cycle:** generated, then consolidated per customer, triaged by a first-level analyst, investigated in
depth if needed, escalated to the MLRO, and closed with a decision: no further action, continued watch, customer
review, exit, or a suspicious transaction report.

## 7. Investigation and case management

Investigating an alert means answering one question with evidence: is there a reasonable explanation for this
activity, or is it suspicious? An analyst typically looks at:

- the customer's KYC data, risk rating, declared activity and source of funds;
- the transaction history over months, not just the alerted transactions;
- counterparties, including whether they are customers of the same bank and how they behave;
- earlier alerts and cases, screening results and adverse news;
- information requested from the branch or relationship manager (without tipping off the customer).

A **case management** system organises this work: queues and assignment, service levels and ageing, maker-checker
(four-eyes) review, escalation to the MLRO, evidence attachments, a written narrative, quality-assurance sampling, and
a full audit trail. CBI rules add Iraqi specifics: staff can report suspicions internally and must receive a written
acknowledgment; every branch has a liaison officer with a branch "window" in the AML system; and the MLRO's decisions,
including decisions not to report, must be written down with reasons (file 01 section 2d). Files 28 and 31 cover case
management and analyst experience in depth. Generative AI can draft case summaries and report narratives, but a named
analyst reviews every draft and makes the decision (file 05, R-11).

## 8. Reporting to the authorities

| Report | What triggers it | Iraq |
|---|---|---|
| **Suspicious transaction report (STR)** | The bank suspects, or has reasonable grounds to suspect, that funds are criminal proceeds or linked to terrorist financing | Filed through **goAML** to the AML/CFT Office, regardless of amount and including attempted transactions, with the reasons for suspicion (Law 39/2015; CBI controls 2025; file 01) |
| **Unusual transaction report (UTR)** | Activity that is unusual although suspicion is not formed | Filed only through goAML under the Office's first-edition 2026 guide: a third outcome beside an STR and no report, followed by enhanced monitoring that can end in an STR (files 23, 28) |
| **Cash transaction report (CTR)** | Cash transactions at or above a threshold | USD 10,000 or equivalent in any currency (file 01) |
| **Freeze report** | A true (confirmed) match with a designated person or entity | To the Committee for Freezing Terrorists' Funds (CFTF), copy to the AML/CFT Office: first report within 48 hours of the freeze, second within two months (Regulation No. 6 of 2023; file 01 section 6, files 23 and 28) |
| **Responses to authority requests** | FIU, court or law-enforcement requests for information | Answered from the bank's records; Egmont rules set expectations for FIU-to-FIU requests (file 04 section 3) |
| **Supervisory reports** | Periodic CBI requirements | Monthly account movement reports (customers and staff), transfer register, correspondent account activity, board and audit-committee reports (file 01 section 2d) |

**goAML** is the reporting software that the United Nations Office on Drugs and Crime (UNODC) provides to financial
intelligence units; banks submit reports through a web portal or as structured XML files. File 23 covers goAML and
report formats.

**Tipping off is a crime.** The customer, and anyone not involved, must not learn that a report was made or is being
considered. An AML system enforces this with need-to-know access: branch staff can report suspicions but cannot see
whether an STR was filed.

**Records** must be kept for at least five years after the relationship ends or the transaction is completed, in a
form that lets authorities reconstruct individual transactions (Iraq; file 01). Foreign regimes add longer or shorter
periods (file 05 section 5e).

## 9. Governance: who is responsible for what

| Role | Responsibility |
|---|---|
| **Board of directors** | Approves the AML policy and risk appetite; receives regular reports; is ultimately responsible. CBI rules require reports to the board and audit committee (file 01) |
| **Senior management** | Provides resources, approves high-risk relationships (EDD), acts on findings |
| **MLRO (money laundering reporting officer) and the compliance function** | Runs the AML programme: monitoring, investigations, STR decisions, the AML system's configuration. In Iraq the MLRO must meet CBI conditions (age 30 or more, professional certification such as ACAMS, CGSS or CFCS, CBI approval; file 01) |
| **First line (business and branches)** | Collects KYC, knows the customers, reports suspicions; in Iraq through branch liaison officers |
| **Second line (compliance and risk)** | Sets policy, monitors, investigates, reports, tests the first line |
| **Third line (internal audit)** | Independently tests the whole programme, including the AML system's rules and parameters |
| **Independent validation** | Tests that rules and models work as intended. The CBI requires AML tooling to be verified by a CBI-approved firm (file 01 section 2e); file 29 covers model risk |

Good governance in an AML system means: every rule and threshold has an owner and a documented reason; every change
is approved by a second person and tested; management information shows effectiveness, not just activity; and
everything is auditable.

## 10. Anatomy of an AML system

Every commercial AML suite (files 08-18) is built from the same components. The names differ by vendor; the jobs do
not:

| Component | What it does | In AKR-TMS | Depth in file |
|---|---|---|---|
| **Data integration** | Brings customers, accounts and transactions from the core banking system and other sources into the AML system; checks completeness | Read-only connection to a CBS database replica; mapping layer from each CBS's tables to one AKR-TMS data model; reconciliation of counts and totals on every load | 25 |
| **Customer and entity hub** | One view of each person or company across accounts, branches and roles; links between related parties | Entity resolution across CBS records, family number and ownership links | 25, 26 |
| **Customer risk rating** | Scores each customer's risk from customer, product, channel and geography factors; re-rates on events | Explainable scoring with reason codes and CBI-compliant levels | 26 |
| **Watchlist management and screening** | Loads and versions lists; screens customers and payments; manages matches | UN, Iraqi national, OFAC, EU, UK lists; Arabic-aware matching; delta re-screening | 14, 22 |
| **Transaction monitoring** | Applies rules, profiles and models to transactions; raises alerts | CBI catalogue plus extended scenario library; models on a governed track | 21, 27 |
| **Alert and case management** | Organises investigation work, evidence and decisions | Queues, four-eyes, narrative drafting, branch window, audit trail | 28, 31 |
| **Regulatory reporting** | Produces STRs, CTRs and supervisory reports in the formats the authorities require | goAML XML, CBI reports | 23 |
| **Analytics and management information** | Dashboards, KPIs, effectiveness and tuning analysis | Per-rule MIS, backlog and timeliness, evidence packs | 29, 31 |
| **Configuration and governance** | Rule authoring, testing, change control, validation support | Rule studio with test harness and maker-checker | 29 |
| **Audit, evidence and retention** | Immutable logs, document vault, retention and deletion | Tamper-evident evidence vault, retention engine per record class | 05, 30 |
| **Security and access** | Authentication, roles, need-to-know, encryption | Role model matching the three lines of defence; tipping-off guard | 30 |
| **Pre-release screening interface** (optional) | Lets payment systems ask "may I release this?" before execution | Phase-2 API for the bank's payment or SWIFT gateway | 25 |

**Deployment models:** on-premises in the bank's data centre (the default for Iraqi banks; file 01 section 8 found no
explicit data-localisation rule but strong practical preference), private cloud, or vendor-hosted software as a
service. **Integration patterns:** nightly batch extracts, incremental reads from a database replica, change data
capture from database logs, message feeds, and file drops. AKR-TMS uses only reads from a replica (or a read-only
account on the CBS database); it never writes to the CBS and never calls the CBS's application interfaces.

## 11. Where AML systems usually fail

The enforcement record (file 20) shows the same failures again and again, across countries and vendors:

1. **The system does not see all the data.** Transactions or accounts never reach the monitoring engine because of a
   mapping or feed error, sometimes for years (the UK Metro Bank case, file 20).
2. **The system does not see all the list.** Screening runs against part of a sanctions list (the UK Starling case).
3. **Vendor defaults are never tuned** to the bank's customers, so the system is either blind or flooded (BCBS, file 04).
4. **Backlogs grow** until alerts are reviewed months late (FCA, file 05).
5. **Nobody can explain the logic**: the bank relies on a vendor black box it cannot challenge (FCA 2026, file 05).
6. **Documentation is missing**: rules, thresholds and changes have no recorded rationale (NYDFS, file 05).
7. **People defeat the system**: staff help customers avoid controls, or override alerts without review (file 27,
   know-your-employee scenarios).

AKR-TMS's design answers each of these: reconciliation on every load, list integrity checks, calibration tooling,
backlog controls, white-box logic, generated documentation, and staff monitoring with four-eyes on overrides.

## 12. English-Arabic glossary

| English | Arabic | Note |
|---|---|---|
| Anti-money laundering (AML) | مكافحة غسل الأموال | |
| Money laundering | غسل الأموال | |
| Terrorist financing | تمويل الإرهاب | |
| Proliferation financing | تمويل انتشار التسلح | Iraqi texts often say "منع انتشار التسلح" |
| Know your customer (KYC) | اعرف عميلك | |
| Customer due diligence (CDD) | العناية الواجبة تجاه العملاء | |
| Enhanced due diligence (EDD) | العناية الواجبة المعززة | |
| Simplified due diligence (SDD) | العناية الواجبة المبسطة | |
| Beneficial owner | المستفيد الحقيقي | |
| Politically exposed person (PEP) | الشخص المعرض سياسياً | Plural: الأشخاص المعرضون سياسياً |
| Source of funds / source of wealth | مصدر الأموال / مصدر الثروة | |
| Risk-based approach | النهج القائم على المخاطر | |
| Enterprise-wide risk assessment | التقييم الذاتي للمخاطر على مستوى المؤسسة | |
| National risk assessment | التقييم الوطني للمخاطر | |
| Customer risk rating | تصنيف مخاطر العملاء | |
| Transaction monitoring | مراقبة العمليات | Also مراقبة المعاملات |
| Scenario / rule | سيناريو / قاعدة | The CBI catalogue uses "سيناريوهات" |
| Alert | تنبيه | |
| Case | حالة | |
| False positive | إنذار كاذب | Also نتيجة إيجابية خاطئة |
| True, potential and incorrect match | التطابق الحقيقي، التطابق المحتمل، التطابق غير الصحيح | The AML/CFT Office's screening terms (2026); the CBI's 2023 guidelines say تطابق مؤكد، تطابق محتمل، نتيجة إيجابية خاطئة (file 28 section 3.4) |
| Suspicious transaction report (STR) | تقرير عن معاملة مشبوهة | Reporting duty: الإبلاغ عن العمليات المشبوهة |
| Unusual transaction report (UTR) | تقرير المعاملات غير الاعتيادية | The Office's 2026 guide; goAML only; no suspicion needed |
| Cash transaction report (CTR) | تقرير المعاملات النقدية | |
| Tipping off | تنبيه العميل (الإفصاح) | Prohibited |
| Financial intelligence unit (FIU) | وحدة المعلومات المالية | In Iraq: مكتب مكافحة غسل الأموال وتمويل الإرهاب (the AML/CFT Office) |
| Committee for Freezing Terrorist Funds (CFTF) | لجنة تجميد أموال الإرهابيين | |
| Targeted financial sanctions | العقوبات المالية المستهدفة | |
| Sanctions list | قائمة العقوبات | |
| Asset freeze | تجميد الأموال | |
| Designated person | الشخص المدرج | |
| Screening | الفحص / التدقيق على القوائم | |
| Correspondent banking | علاقات المراسلة المصرفية | |
| Shell bank | بنك صوري | |
| Wire transfer | الحوالة / التحويل البرقي | |
| Structuring | تجزئة المعاملات | |
| Money mule | وسيط نقل الأموال | Informal usage varies |
| Hawala | الحوالة (غير الرسمية) | Informal value transfer |
| Exchange company | شركة صرافة | |
| Adverse media | الأخبار السلبية | |
| Mutual evaluation | التقييم المتبادل | |
| Grey list (jurisdictions under increased monitoring) | القائمة الرمادية (الولايات القضائية الخاضعة للمتابعة المتزايدة) | |
| Money laundering reporting officer (MLRO) | مسؤول الإبلاغ عن غسل الأموال | Iraqi banks: مدير قسم الإبلاغ عن غسل الأموال وتمويل الإرهاب |
| Three lines of defence | خطوط الدفاع الثلاثة | |
| Core banking system (CBS) | النظام المصرفي الشامل | |

Arabic terms should be checked against the wording of Law 39/2015 and the CBI's 2025 controls when the product's
Arabic interface text is written; the CBI's own terms take priority over general usage.

## 13. What it means for AKR-TMS

- **The product mirrors the life cycle.** Data, customer risk, screening, monitoring, cases, reporting and governance
  are the modules, in that order of dependency. The plan should present them the same way.
- **The read-only principle shapes three things:** onboarding stays in the CBS (AKR-TMS rates and screens the record
  once it appears); freezes and holds become tracked operations tasks with verification from the replica; and stopping
  a payment before release needs the phase-2 API.
- **False positives and backlogs are the everyday pain** that bank staff will judge the product by. Explainable
  prioritisation, consolidation per customer and good case tools matter as much as detection.
- **Explainability is not optional.** A risk-based approach only works if every difference in treatment can be
  explained, so reason codes, generated rule descriptions and full audit trails run through every module.
- **Arabic first.** The glossary above is the seed of the product's terminology list. The interface, reports and
  generated explanations use the CBI's Arabic terms, with English available for correspondents and auditors.

## 14. Sources

This file summarises the sources cited in files 01-05 and 08; see those files for the full lists. Key primary sources:
- FATF Recommendations, updated October 2025 (mirror hosted by the Hong Kong Companies Registry):
  https://www.cr.gov.hk/en/compliance/aml_cft/FATF/FATF_Recommendations.pdf (file 03).
- Iraq: Law No. 39 of 2015; CBI Instructions No. 1 of 2023; CBI AML/CFT/CPF supervisory controls 2025; CBI circulars of
  26 August 2026; MENAFATF/FATF mutual evaluation of Iraq 2024 (file 01 and file 02 sources).
- BCBS, Sound management of risks related to money laundering and financing of terrorism (2020):
  https://www.bis.org/bcbs/publ/d505.htm (file 04).
- Wolfsberg Group guidance on monitoring, screening and the risk-based approach (file 04 sources).
- EU, US, UK and Gulf rules (file 05 sources).
- NICE Actimize case-study false-positive figures (file 08 sources).
