# 28. Case management and the investigation workflow

Status: done (2026-10-07)

Prefix for derived requirements: M (M-01, M-02, ...). Written by the research agent for files 28 and 31. Cross-references use "file NN" for the other files in this folder; file 31 (analyst user experience) owns the screens, file 23 (goAML reporting, written in parallel) owns the report and XML design, file 27 (scenarios, written in parallel) owns the alert sources. Anything not verified is marked "NOT VERIFIED". Evidence grades: A = primary source (law, regulator text, official documentation, peer-reviewed paper) read in full or in the relevant part; B = primary source read through a secondary summary; C = vendor or practitioner statement; D = inference by the author.

## Contents

1. Scope, method and what this file adds to files 01, 06, 13, 20, 21, 29 and 30
2. The Iraqi chain as written: branch liaison officer, central AML department, MLRO, the Office (CBI controls 2025, MER, Office guidance 2025-2026)
3. Alert intake and the case model: sources, grouping into customer and network cases, merge versus suppress
4. Triage and prioritisation: the score, the legal order of attention, routing and tiers
5. Statuses, transitions, clocks and ageing (the state machine)
6. The decision: STR, UTR or no report, with recorded reasons; hand-off to the goAML builder
7. Post-decision actions: watch, review, exit, and operations tasks that AKR-TMS cannot execute
8. Closure, hibernation, re-opening and links between cases
9. Requests for information: branches, relationship managers, correspondents, the Office and courts
10. Quality assurance: sampling, four-eyes, scoring, feedback and calibration
11. Workload and backlog management: capacity model, queues, alarms, what regulators did, metrics
12. Evidence and records: the case file, immutability, the vault, retention, legal hold, export
13. Access and confidentiality: who sees which case, branch scoping, conflicts, tipping-off
14. GenAI in investigations
15. Workflow engine options for the Rust stack
16. A concrete design for AKR-TMS case management
17. What it means for AKR-TMS
18. Derived requirements (M-xx)
19. Sources

(Sections are appended as they are researched; each topic section ends with "AKR-TMS hooks".)

## 1. Scope, method and what this file adds

**Question.** How should AKR-TMS organise the work between "an alert fired" and "the bank has recorded a defensible decision, acted on it and can prove it years later", for Iraqi banks that already run a two-tier branch plus central workflow, that must record the MLRO's decisions with reasons (including decisions not to report), and whose evidence will be tested by a CBI-approved external auditor (standard D2: rule logic, alert generation, the communication and reporting paths between the parties concerned, the review and improvement mechanism, user permissions, audit-trail integrity, data traceability; section 2.5)?

**What the other files already settle (not repeated here).**

| Topic | Where | What this file takes from it |
|---|---|---|
| Iraqi alert and reporting rules | file 01 sections 2d, 2c, 4; file 18 section 1 | The chain (liaison officer, central department, MLRO), the ten-day monthly report, written decisions, 5-year records |
| Basics of investigation and case management | file 06 section 7 | Vocabulary: alert, case, narrative, QA |
| Incumbent and challenger case tools | files 08-11 (NICE ActOne, Oracle Investigation Hub, SAS Visual Investigator, SymphonyAI SRI), file 13 (Lucinity, Silent Eight, WorkFusion, Napier, Unit21, Flagright, Hummingbird) | What to copy and avoid; autonomy claims versus the regulator-compiled 7% |
| Failure modes in enforcement actions | file 20 (FM-24 backlog and late STRs, FM-25 poor disposition quality, FM-26 edited records; L-16 to L-19, L-31, L-46, L-47, L-52 to L-57) | Requirements are cross-referenced by ID, not copied |
| Triage, hibernation, explainability, GenAI use cases and evaluation | file 21 sections 9, 11, 12 | The ladder of triage techniques and the evaluation protocol |
| Governance of rules, models and GenAI | file 29 sections 9j, 9k; EU AMLR Article 76(5) | No solely automated adverse decision; human review; logging |
| Roles, tiers, tipping-off, access history, audit, retention, legal hold | file 30 sections 6-10, 12, 19 | Cedar roles and attributes, suspicion tier, evidence snapshot at case opening |
| Name and match display | file 22 section 13 | How a screening case shows names and evidence |
| Stack: DBS workflow engine, case aggregate, evidence vault | file 32 sections 11, 12, 18 | Two-layer recommendation; evidence vault on MinIO kept by the user's decision of 2026-10-07 (pinned, maintained Silo build), with WORM certification |
| goAML reports and XML | file 23 (in progress, parallel) | Linked, not designed here |
| Alert sources and scenario IDs | file 27 (in progress, parallel) | Scenario codes such as "CBI 2.2" are those of the CBI catalogue |

**What this file adds.** (1) A reading of the Iraqi chain from the texts of the AML/CFT Office (2025 and 2026 guidance, read in the Arabic originals on 2026-10-07), including a reporting tier that the earlier research did not capture: the **unusual transaction report (UTR)**. (2) A state machine with clocks, ageing and escalation. (3) Request-for-information templates and clocks. (4) A QA design with sample-size arithmetic. (5) A capacity and backlog model. (6) The case-file and records design tied to the vault and retention classes. (7) A comparison of workflow-engine options for the Rust stack, with facts read from the projects on 2026-10-07. (8) A concrete design.

**Method.** Existing files were read first (sections named above). Primary Iraqi texts were read from the copies saved by the file 23 agent in `scratchpad/src/rp/aml/` (normalised Arabic text). Project facts for engines were read from GitHub's public API and the projects' own documentation on 2026-10-07. Web search was not available; pages that could not be reached are listed in the final section.

## 2. The Iraqi chain as written

### 2.1 Who does what, in the texts

Sources: CBI AML/CFT/CPF supervisory controls of 8 January 2025 (file 01 section 2d); MENAFATF mutual evaluation of Iraq paragraphs 566-567 (file 18 section 1); AML/CFT Office, "Guidance Guide on Reporting Suspicious Transactions and Activities" (the 2025 manual that the CBI's 26 August 2026 circular cites) and "Guidance Guide on Reporting Unusual Transactions", first edition 2026 (both read in the Arabic originals, copies in `scratchpad/src/rp/aml/`, 2026-10-07); Law 39 of 2015. Grade A unless marked.

| Step | Actor | What the text says (paraphrase; key phrase quoted) | Source |
|---|---|---|---|
| Alert generation | The bank's automated system | Real-time follow-up of all account movements against declared KYC flows and the CBI-approved scenarios, "raising alerts as they happen"; AI and machine learning to find possibly suspicious transactions | CBI 2025, ch. 5 art. 5 |
| First review | **Branch liaison officer** (موظف الارتباط) | "Each branch ... has a designated liaising officer who has been trained to use the automated systems for monitoring transactions. They initially examine the alerts ... then the employees of the AML/CFT central departments ... review"; the central review closes for insufficient grounds or escalates | MER para 567 (via file 18) |
| Branch window | Liaison officer | A "digital window in the AML system specific to their branch" to examine all the branch's transactions and check documents are attached; reports suspicion "to the MLRO only; branch management has no part in it"; technical reporting line to the MLRO only; a dedicated phone line and e-mail to the MLRO | CBI 2025, ch. 11 |
| Monthly report | Liaison officer to MLRO | By the 10th of the following month: the cases reported and the latest action on each; available to inspectors; the MLRO must "review and audit" these reports | CBI 2025, ch. 11 and ch. 3 (MLRO duty j) |
| Front-line reports | Any staff member | Short reporting line; internal report to the head of the AML reporting department; the head documents it, gives the employee a written acknowledgment with the tipping-off reminder, examines it, decides, and "notifies the employee of the decision in writing" | CBI 2025, ch. 2 art. 3 |
| Central review | Central AML department | "Special attention to internal reports on large, complex and suspicious operations; an integrated system that defines how these reports flow, who owns them and how they are analysed, **reviewed daily by the head of the AML department**"; findings on unusual transactions recorded in writing and kept at least 5 years | CBI 2025, ch. 5 art. 4 |
| Hold | MLRO and the bank's departments | "Stops any transaction that matches one or more suspicion scenarios until the MLRO verifies it is sound" (a stop-until-cleared duty that a read-only system can only support by creating a task; phase 2 adds the pre-release API) | CBI 2025, ch. 5 art. 5(6) |
| Decision | **MLRO** (or deputy) | Examine system-surfaced unusual transactions and staff reports; file STRs on the Office's form; "decide to close ('keep') a case where no suspicion is found — the decision must state its reasons"; right of access to all information and to contact any employee; full confidentiality | CBI 2025, ch. 3 (MLRO duties a-c) |
| The three outcomes | MLRO | The Office's guide: after "initial scrutiny" the reporting officer decides "either to send the report immediately and without delay to the Office, or to gather more evidence and corroboration, or to shelve (الحفظ) with justification of the decision through the necessary documents" | Office STR guide, section "timings" |
| Order of attention | MLRO | Work "according to priorities": immediate attention to transactions or customers linked to terrorism or its financing or to a person on international or local ban lists, then ML suspicions or predicate offences | Office STR guide |
| Filing | MLRO or deputy only | The suspicion report is raised by the reporting officer or deputy through goAML; the Office must hold their contact details and be told of changes; attempted transactions are reportable; no minimum amount; "immediately" (فوراً; "without delay" in the English translation) | Law 39 art. 12 Fifth; Office STR guide |
| Records | Bank | Unusual-transaction reports and the results of reviewing them, STR records with copies, and "records of cases the MLRO decided to close" kept at least 5 years, stored securely with backup in another location, retrievable "easily and quickly" | CBI 2025, ch. 4; Law 39 art. 11 |
| Supervisor | CBI, the Office | Third line: supervisors "uncover suspicious transactions not reported **or that were shelved**, and the reasons behind that" | Office STR guide, section on the third line |
| Quality grading | The Office | Each STR graded good, moderate, acceptable or poor; poor ones go back to the reporter and the supervisor is notified (since 2017, automated with goAML) | MER (file 01 section 2) |

### 2.2 The reporting tier the earlier research missed: the UTR

The Office's first-edition 2026 guide introduces the **unusual transaction report (UTR, تقرير المعاملات غير الاعتيادية)**, an early-warning report filed "exclusively through goAML" without any requirement that suspicion exists. Its flow chart (section 4 of the guide) is:

```
unusual or unjustified activity detected
  -> file a UTR
  -> apply enhanced monitoring to the customer or the activity
        -> indicators do not develop, no suspicion arises : the UTR already sent is enough
        -> indicators develop and suspicion appears       : file an STR / SAR
```

Other points of the guide that matter for a case system:

- A UTR "does not require suspicion of a financial crime", "is not an accusation or a judicial act against the customer" and is used for analysis, monitoring and risk assessment. Activity is unusual when it "departs from the customer's usual pattern", "does not match his profile or declared activity" or "raises objective questions that require analysis".
- It must **not** be used for purely administrative compliance gaps: "expired identity cards, incomplete customer files, delay in updating KYC" are handled by internal compliance procedures and do not require a report.
- Principles when filing: good faith under Article 48; do not tell the customer; **do not delay reporting until the case is finally verified**; enhanced monitoring continues after the UTR; UTRs are retained under Article 11. A report carries all information available, "without requiring that suspicion be complete": a clear description, the reasons for classifying the case as a UTR and any supporting documents.
- Classification "depends on a professional risk-based assessment" and "the accumulation of indicators over time may change the nature of the classification": a UTR can later become an STR, and the STR guide asks that a prior report on the same subject be referenced by number and date.
- Scope: banks and non-bank institutions and the designated non-financial businesses and professions; the examples include unusual import payments, reactivated dormant accounts with moderate activity, outgoing transfers disproportionate to financial history even with invoices attached, structuring of foreign-currency purchases, sudden volume rises, card use inconsistent with the declared profile.

**Why it changes the design.** Until this guide a case had two ends, "report" or "close". Iraq now has three: **no report (shelved with reasons), UTR (report and watch), STR (report on suspicion)**, plus a legal path from UTR to STR. The case outcome field, the clocks, the watch list, the "prior report" link and the effectiveness metrics (UTR-to-STR conversion) must model that. The UTR page is dated 2026 and the Office's pages were uploaded in August and September 2026 (file 23 section 2.2); the guide is a "first edition" and an Iraqi bank's internal policy may not yet use it: the outcome is therefore a per-bank switch that is on by default.

### 2.3 What an STR must say, as it bears on the case file

The Office's guide asks that the report answer **who, what, when, where, how** and carry: the reporter's data; the customer's data (natural or legal person); a description of the suspected transaction or activity "and the reasons and grounds of suspicion precisely and clearly"; the predicate offence if known; **the action taken by the reporting entity** ("for example strengthening control of the account, stopping the account, not accepting cash deposits, not executing the transaction"); and the reference and date of **any earlier report** on the same subject. The "how" element asks the bank to say "whether the suspicion arose from an alert of the electronic automated systems" or otherwise, and which predicate offences the proceeds relate to. For each linked transaction the report gives date, amount, currency, accounts and counterparties, "chronologically".

Consequences for the case: the system must keep (a) the **origin of the suspicion** (which scenario, screening hit, staff report, authority request, RFI answer or adverse-media item started it); (b) the **actions taken** as structured data, not only prose; (c) the **link to earlier reports**; (d) a chronological transaction list ready for the report. The case file in section 12 is shaped by this list.

### 2.4 What the texts do not say

| Gap | Evidence | AKR-TMS answer |
|---|---|---|
| No number of days for "immediately" or "without delay" for the internal chain or for filing | Law 39 art. 12 Fifth and the Office guide use only "immediately" | Each bank sets clocks in policy; AKR-TMS ships conservative defaults and reports actual times (section 5). US 30/60-day clocks are reference policies only (file 05 section 1b) |
| No service level for the liaison officer's first review or the central department's second review | MER para 567 describes the flow, not the time | Configurable clocks per alert class; breach escalation (section 5) |
| No sampling rate or method for QA of closed alerts | The CBI controls of January 2025 are silent; the 2025 Detailed Assessment Guide (D2.1) does require "periodic calibration and quality assurance controls" in the monitoring system and "timely" investigations "proportionate to the potential risk" (section 2.5), but sets no rate | Built-in QA sampling with sample-size arithmetic (section 10); risk-proportionate clocks by priority class (section 5.6) |
| No format for the MLRO's reasons beyond "must state its reasons" | CBI 2025, ch. 3 | Structured reason codes plus free text (section 6) |
| Feedback from the Office is an acknowledgement, not an outcome | MER: banks say feedback is "inappropriate in all cases" (file 01 section 2) | An FIU-feedback register that accepts whatever comes back, including quality grades, so the tuning loop can use it (section 10) |
| Which transactions the MLRO may hold, and for how long, when the system cannot stop a payment | CBI 2025 ch. 5 art. 5(6); the Office can suspend a transaction "up to seven days" (file 01) | Hold clocks in phase 2; in phase 1 a task to operations with verification (section 7) |
| Volumes | Banks filed 451 STRs in 2024 across about 76 banks, roughly 6 per bank per year; 90 percent graded "good" in 2022 (file 01 section 5) | The case system is sized for alerts and internal reports in the thousands and STRs in the tens; the action plan asks for more and better STRs, so STR volume is expected to rise |

### 2.5 What the Banking Reform Initiative's D2 text adds (CBI Detailed Assessment Guide 2025)

The guide that the CBI-approved external auditor follows for standard D2 (AML/CFT and sanctions) contains the only written expectations on investigation workflow beyond the January 2025 controls. Read in the Arabic original (`cbi-guide.txt`, doc pp. 44-45, 2026-10-07; file 18 section 12.2 summarises it), grade A. Numbering used here follows the document: standard **D2.1** has three elements, (a) governance and organisation, (b) procedures and (c) enablers, each with numbered items, followed by an "assessment process" text that tells the external firm what to test:

| D2 clause | What it says (paraphrase; key phrase quoted) | Case-system consequence |
|---|---|---|
| D2.1(b)(4) "AML investigation" | Banks must define internal protocols for investigating alerts from transaction monitoring or customer behaviour; all financial-crime investigations must be documented "with clear audit trails", conducted or supervised by "qualified and appropriately experienced" staff, "started and completed in a timely manner proportionate to the potential risk", and "concluded by determining whether the matter is reportable" | Risk-proportionate clocks (P0 to P3); qualification and training records tied to queue access; every case ends in an explicit reportability decision (the three outcomes) |
| D2.1(b)(5) "Transaction monitoring" | The system "must support dynamic rule creation, alert generation, and facilitating investigation workflow, with periodic calibration and quality assurance controls" | The workflow, calibration (below-the-line testing) and QA are part of what is assessed, not extras |
| D2.1(b)(6) "Sanctions screening" | Real-time screening of customers, counterparties and transactions; documented procedures; "compliance of resolution timelines" with CBI obligations; escalation per the bank's protocol | Screening alert clocks and escalation records (section 5.5) |
| D2.1(a)(2) "Programme monitoring and reporting" | Structured reporting to senior management and the board on AML risks, compliance gaps, "escalation records" and mitigation, including key risk indicators and trends in SAR/STR reports and the results of inspections or audits | The board pack of section 11.5, generated from system facts |
| D2.1(a)(3) "Programme organisation" | The compliance controller and the MLRO act independently with sufficient authority and report **directly to a board committee**; resources adequate in staff, expertise and technology | Capacity ratio and staffing evidence (section 11); the MLRO's reporting line is visible in the role model |
| D2.1 assessment process: procedures | The firm examines a representative sample of customer files across all risk levels, tests TM systems for "rule logic, alert generation, **communication and reporting paths between the parties concerned**, and the review and improvement mechanism", examines "how suspicious transactions are handled", and reviews sanctions matching for "time limits for sanctioned customers and completeness of escalation documents" | The branch window, the escalation path and the RFI trail are tested as paths; the end-to-end trace of section 12.6 |
| D2.1 assessment process: enablers | The auditor evaluates system capabilities and configuration files, verifies alert thresholds, user access permissions, audit-trail integrity, integration across KYC, monitoring and sanctions modules, data-management controls, backup, compliance with legal retention periods, and whether the data used to detect and report can be traced | Assessor workspace (section 12.6) |
| D2.1(c)(1) data | "High-quality data for all AML-relevant operations for **7 years**", accurate, complete, up to date, accessible for all internal functions and regulatory reports, with record-keeping that ensures traceability "including STRs, CDD files and audit logs" | **A 7-year floor** for case, alert and STR classes in Iraq mode (file 18 VD-19); this is longer than the 5 years of Law 39 and the CBI controls and shorter than the file 30 default of 10 |

**Consequence for file 30.** The retention classes R5 to R7 of file 30 section 12.3 carry a 5-year floor from the controls. Under the reform assessment the effective Iraqi floor for AML data is 7 years; the case system sets the floor of the Iraq jurisdiction profile to 7 years for the classes it owns (cases, alerts, decisions, RFIs, reports), keeps 10 as the default, and shows both sources in the retention report.

### AKR-TMS hooks (section 2)

- **Outcome set of three**: `no_report`, `utr`, `str` (with `tfr` where the reporting category is terrorist financing), each with a mandatory structured reason and free text; the choice is recorded by a named user.
- **Origin of suspicion** stored on every case and passed to the report builder for the "how" element (file 23).
- **Prior-report link** between a UTR and a later STR (and between STRs on the same subject), carrying the goAML reference and date.
- **Priority law of attention**: cases whose subject is a list match (UN, national list) or whose typology is terrorist financing rank above all others by rule, not by score (section 4).
- **Shelved-decision register**: every `no_report` decision is a queryable record with its reasons, because the Office's guide says supervisors look at shelved decisions and why.
- **Per-bank policy switch** for the UTR tier, its default text, and its link to the watch list.

## 3. Alert intake and the case model

### 3.1 Objects and words

A case system fails first at vocabulary: when "alert", "case", "task" and "report" mean different things on different screens, ageing and quality figures cannot be trusted. AKR-TMS uses the following objects (the screens that show them are in file 31).

| Object | Meaning | Created by | Immutable part |
|---|---|---|---|
| **Signal** | Any raw detection output: a rule hit, a score, a list hit, a staff observation | The engines (files 14, 21, 22, 27) or a person | The input rows and the configuration version |
| **Alert** | A presented signal: scenario or screening hit, party, trigger evidence, score and reason codes, versions of rule, list and matcher | The detection engines | An **evidence snapshot** taken at creation (the triggering transactions, party profile as of that time, list-entry detail, scenario version), so dropping old monitoring partitions never damages a case (file 30 section 12.2) |
| **Internal report** | A staff or branch report of suspicion or of an observed indicator (CBI ch. 2 art. 3; the CBI 26 August 2026 behavioural indicators) | A staff member or the liaison officer | The report text, the acknowledgment and its time |
| **Authority request** | A request or order from the Office, a court, the CBI or law enforcement | The MLRO's office | The request and its register entry (restricted, file 30 section 8.2 row 12) |
| **RFI** | A request for information, in or out (branch, relationship manager, customer through the bank's own process, correspondent) | The analyst or the correspondent | The question set and each answer |
| **Case** | The investigation container for one party or one related-party group; holds alerts, notes, evidence, entities, tasks, RFIs, decisions and the outcome | The grouping service or a person | Append-only after closure (file 30 section 9.5) |
| **Task** | The smallest assignable unit of work (review, answer, approve, sign, carry out) with an owner, a clock and a result | The workflow | Its event history |
| **Decision** | A recorded choice with a reason code, free text, the person and the evidence considered; at alert level and at case level | A named human | Never edited; corrected by a new decision that references it |
| **Report** | A UTR, STR, TFR, CTR or other filing prepared in the report builder (file 23) | The reporting module | The filed version and the goAML acknowledgment |
| **Control incident** | A problem with the system itself (reconciliation breach, blind period, silent rule) | Data and platform monitors (file 25, file 30) | Not a customer matter; goes to the data steward and the security officer, never to analysts' alert queues |

### 3.2 Where work comes from

| Source | Examples | First route | Notes |
|---|---|---|---|
| Transaction monitoring | The CBI scenario catalogue, bank scenarios, ML scores (files 21, 27) | Branch window or central L1 by alert class (section 4) | Daily batch plus near-real-time; every scenario has a data-readiness grade in file 27 |
| Customer or counterparty screening | Delta screening after a list change, onboarding, payment parties (files 14, 22) | Central screening queue; never the branch for freeze lists | A list match to the UN or national list has legal priority (section 4.1) |
| Customer risk events | Rating change, overdue periodic review, EDD refresh, undisclosed PEP (file 26) | KYC review queue | Case family `kyc_review`, not mixed with monitoring |
| Branch observation log and internal reports | The CBI behavioural indicators and the seven SAR escalation criteria (file 01 section 2c) | Straight to the MLRO's office; the branch cannot see the disposition | Written acknowledgment with the tipping-off reminder |
| Authority requests and court orders | Office requests, orders, CBI queries | MLRO's office; suspicion tier | Linked to the customer and shown to the closing analyst (L-46) |
| Incoming correspondent RFIs | A USD correspondent asks about a customer's transactions (file 04 section 2p) | RFI desk | An incoming RFI "opens or updates" an internal case; if no alert had fired, it is logged as "missed by our monitoring" |
| Adverse media | Arabic and English news items (files 15, 21) | Central L1 | Source link and quotation for every claim |
| Staff and restricted-tier accounts | Know-your-employee scenarios (file 27), relatives of staff, board members, PEPs (file 30 section 8.3) | Compliance-only queue | Level-1 analysts never see them |
| Platform user-behaviour rules | Mass lookups, snooping, export spikes (file 30 section 9.3) | Second line and security, not the person's manager | Not a customer case |

### 3.3 Grouping alerts into cases

**Evidence for customer-level grouping.** ING's alert system "took place at account level and not at client level", so small amounts spread over several accounts were not seen (file 20 section 4.3, grade A). The HKMA's 2024 circular lists, as a benefit of scoring, alerts consolidated "into a holistic review on a customer-relationship basis, which can be conducted by a single investigator" (file 29 section 5a). SAS aggregates alerts "at the entity level" (file 10); SymphonyAI reports 20 percent fewer alerts at Metro Bank from a customer-level roll-up (file 11, vendor-reported); NICE ActOne is "agnostic to where alerts originate" (file 08). The open-source Marble case manager documents the same pattern: an action "add to case if possible" attaches a decision to an existing open case "for the same decision group value in the same inbox, else a new case will be created" (Marble documentation, workflows, read 2026-10-07, grade C). The plan's promise, "analysts investigate customers, not alerts", is therefore the industry direction and not a risk.

**Default grouping policy (configurable per bank, every setting versioned and maker-checker approved).**

1. **Key**: the resolved party (entity resolution, file 21 section 7), then the case family (`monitoring`, `screening`, `kyc_review`, `internal_report`, `authority_request`, `rfi`). One open case per party per family; alerts of the same family attach to it. Families are separate on purpose: a sanctions match must not wait inside a monitoring case, and a periodic review must not close a suspicion case.
2. **Window**: an alert attaches to an open case of the same party and family at any age; if the last case of that family closed less than N days ago (default 30), the new alert **reopens** it with a visible "reopened by new alert" event instead of creating a parallel case; after N days a new case opens and links to the earlier one.
3. **Non-customer parties** (walk-in cash, third-party senders, agents) are first-class parties with a pseudonymous key, so they can own cases (file 20 section 2.2, "non-customer parties").
4. **Related-party groups**: when entity resolution or the graph links parties by evidenced relationships (shared account, address, device, ownership, signatory, documented family record; never a shared surname or tribe name, file 30 section 19.4), the case shows the group and offers "open a network case" (section 3.5). Grouping is never automatic across parties.
5. **Merge, never suppress**: duplicate and repeat alerts are grouped into one case with every underlying alert visible; the engine has no suppression of reviewable alerts and no cap (file 20 L-16, L-17; US Bancorp's 90-day re-alert rule hid about 6,888 productive alerts in a limited review).
6. **Merge and split of cases**: merging two cases of the same party and family keeps both histories and marks one `merged_into`; splitting is allowed when a case covers two typologies that need different decisions, producing a child case with a reference and its own decision. Neither action deletes anything.

**Alerts inside a case.** Each alert keeps a stable ID, its snapshot and its own **contribution outcome** (`explained`, `not_explained`, `not_assessed`, `duplicate`, `data_error`) so the per-scenario effectiveness figures (productive rate, alert-to-STR) can still be computed per alert and per scenario. The **decision** is taken at case level; a case decision writes a default outcome onto every `not_assessed` alert, visibly marked as inherited. This avoids two failure modes: analysts closing 30 alerts one by one for one customer (the Coinbase and Block pattern, file 20 sections 8.1, 8.2) and scenario statistics that vanish because alerts were folded into cases.

### 3.4 Screening alerts: a related but different object

A screening alert is a **pair** (party, list entry) with a score, an evidence breakdown and a pair-keyed memory (file 22 sections 13.7-13.8). **Two Iraqi vocabularies and two clocks exist and must both be supported** (found by reading the Arabic originals on 2026-10-07; file 23 section 3.4 reached the same conflict):

| | CBI sanctions guidelines, 1 August 2023 (p. 21) | AML/CFT Office, Guidance on the Prompt Implementation of TFS, uploaded 10 September 2026, art. 12; Terrorist Funds Freezing Regulation No. 6 of 2023 |
|---|---|---|
| States (Arabic) | تطابق محتمل (potential), تطابق مؤكد (confirmed), نتيجة إيجابية خاطئة (false positive) | التطابق الحقيقي (true), التطابق المحتمل (potential), التطابق غير الصحيح (incorrect) |
| The doubt rule | A match is confirmed "when there is any suspicion or doubt" | A match is true "when sufficient indicators confirm" identity; similarity in name or data is a **potential** match that needs further review, comparison of identifiers and a documented result, and nothing is disclosed to the customer |
| Duties on the strongest state | Freeze "immediately"; inform the Committee within **three days** (quoting Regulation No. 5 of 2016, since repealed) | Freeze "without delay" (the 2026 guide: within 24 hours of the listing decision; the March 2023 manual: within 8 hours of publication of the lists); report to the Committee with a copy to the Office within **48 hours** and a second report within **two months** (Regulation 6/2023 art. 4, via file 23) |
| Incorrect match | Document | Document the result, keep supporting documents, **record the reason for excluding the match**; the relationship continues under risk management |

The product therefore keeps **one internal state machine with a per-bank "doubt policy"** and shows the Office's terms as the primary labels with the CBI's terms as aliases in help and exports. The default clocks are the **strictest** found (freeze task 8 hours, first Committee report 48 hours, second report at two months, file 23 section 8) and any relaxation needs a recorded legal basis. A screening alert has its own short state machine (section 5.5), its own queue and its own clocks, and becomes a **case** (family `screening`) only when it is escalated or becomes a true match. The analyst's decision options and the display of names are in file 22 and file 31; the decision record fields are in section 6.

### 3.5 Network cases

When the graph or the analyst identifies a cluster (the mule ring and hawala-style flows of mockup 06, file 21 section 6e), the product opens a **network case**: a parent case that holds the cluster summary, the evidence subgraph snapshot ("a snapshot of the graph is kept with the case", mockup 06), the list of member parties, and **one child case per subject** that needs its own decision. The parent has a coordinator; the children can be worked by different people and branches (each child still obeys branch scope and tiers); the parent closes only when every child has an outcome. A single goAML report may cover several subjects (file 23), so the report builder can start from the parent and select subjects.

### 3.6 Intake integrity

- **Stable identity**: an alert's key is a hash of (scenario or rule ID and version, party key, window, triggering transaction IDs). Re-running a window after a CBS restore or a late-arriving record (file 25 C-64) updates the same alert instead of creating a duplicate; a changed trigger set creates a new version of the alert with a visible link.
- **No silent loss**: alerts that cannot be created (missing party, rejected record) go to the quarantine queue of file 25 (L-05) with their reason, never to the void.
- **Surplus is queued, never dropped**: if volume exceeds capacity the surplus waits with an age clock; the engine has no "maximum alerts per day" parameter, and a flat-line detector flags a scenario that produces the same count for N days (L-16, L-29).
- **Two clocks from the first moment** (L-31): *data latency* (transaction booked, alert created) and *handling time* (alert created, decision). A latency above the bank's limit is shown beside the queue, as in the Mashreq finding of a three-month gap in alert generation (file 20 section 3.5).

### AKR-TMS hooks (section 3)

- `alert`, `case`, `case_alert`, `case_party`, `task`, `decision`, `rfi`, `internal_report`, `authority_request` tables as in section 16, with `case_family` and `parent_case_id`.
- **Grouping service** with the policy above, a dry-run that shows how many cases a month of history would have produced under a policy change, and a ledger of what it merged.
- **Contribution outcome** per alert and **inherited outcome** marking.
- **Network case** creation from the graph view with subgraph snapshot and per-subject child cases.
- **Intake guard**: alert fingerprinting, quarantine link, flat-line detector, latency display.

## 4. Triage and prioritisation

### 4.1 The legal order of attention

The Office's guide tells the reporting officer to "work according to priorities": first "transactions or customers linked to terrorism or its financing or to a person on international or local ban lists", then money-laundering suspicions and predicate offences (section 2.1). A score cannot be allowed to reorder that. AKR-TMS therefore ranks in two layers:

| Layer | Rule | Example |
|---|---|---|
| **Priority class (by rule, not score)** | P0: a true or potential match to the UN list, the national list or an internal prohibition, a terrorist-financing or proliferation indicator, an order from the Office or a court, a stop-until-cleared request. P1: restricted-tier parties (staff, relatives of staff, board members, PEPs, VIP), network cases, a case already at "STR candidate", repeat alerts on a party with an earlier UTR or STR. P2: all other monitoring alerts. P3: low-risk classes under a documented policy | A national-list hit on a customer ranks above any monitoring case regardless of amount |
| **Score within a class** | The documented priority score of section 4.2 | Two P2 cases ordered by score, then age |

### 4.2 The priority score

The score must be explainable to an analyst and to an auditor, built from parts that are displayed, and tested. The ING case shows what goes wrong otherwise: accounts were ranked by percentage deviation from history, so a EUR 100 account with a EUR 10,000 transaction (100 times) outranked a EUR 1,000,000 account with a EUR 99,000,000 transaction (99 times) (file 20 section 4.3). File 21 section 9b and file 20 L-27 set the rule that the ranking formula is documented and tested with that case.

| Component | What it measures | Notes |
|---|---|---|
| Typology weight | The scenario or model family: structuring, hawala-like flows, mule patterns, PEP red flags, dormant reactivation | Set by the MLRO per scenario, versioned (a governed parameter, file 29) |
| Deviation and size together | Relative deviation from the declared profile and from the peer segment, and the absolute value in IQD and USD | Both, never the ratio alone (L-27) |
| Customer risk | The CBI rating level (high, medium, low or five levels) and its factors (file 26) | Raises priority; never removes coverage |
| Counterparty and corridor risk | High-risk country, exchange company or money-transfer company counterparty, sanctioned-corridor nexus | From files 24, 25, 27 |
| Network exposure | Links to parties with open or filed reports | From the graph (file 21 section 6) |
| History | Earlier alerts and outcomes, UTRs and STRs on the party, open authority requests | Regret metric (file 21 section 9b) uses the same data |
| Data confidence | KYC completeness and data-quality flags | A degraded customer is stricter, not looser (L-44) |
| Age | Elapsed time since the alert was created | A rising term so that nothing waits indefinitely |
| ML score (when its gate is passed) | Supervised or anomaly score as one input | The HKMA's regulator-compiled example (Bank C) reports that 80 percent of alerts scored high-risk led to an STR; that is a precision-at-the-top measure, not proof for an Iraqi bank (file 12 section 8.4) |

**Evidence from a neighbouring field.** In security operations, which has the same alert-fatigue structure, a 2026 systematic evaluation of risk-based alerting (Uetz and colleagues, arXiv 2609.02465, eight alert datasets, six created or extended for the study) found that combinations of five "risk hypotheses" prioritised alerts at AUROC 0.92 on average against 0.72 for a straightforward ranking by alert severity (grade A as a paper, analogy only, not AML data). The lesson for AKR-TMS is the design, not the number: rank by accumulated, explainable risk about the **entity** across alerts, not by the severity of each alert on its own.

**Display.** The score is shown as its parts (a short bar list), never as a bare number; branch roles see reason codes without thresholds (file 21 section 11b). **Test.** A fixed set of synthetic cases, including the EUR 100 versus EUR 99 million pair, runs on every release and every parameter change; a failing ordering blocks activation.

### 4.3 Queues and routing

| Queue | Who works it | What enters | Scope rule |
|---|---|---|---|
| Branch window | Liaison officer (and named alternate) of the party's branch | Alert classes marked branch-first (below) | Branch scope; limited fields; no suspicion-tier cases; no thresholds |
| Central L1 | Level-1 analysts | Central-only classes; branch submissions; branch SLA breaches | Pool by team, language and skill |
| Central L2 (investigation) | Senior analysts and investigators | Escalations from L1; network cases; complex typologies | Case assignment |
| Specialist queues | Screening and sanctions, trade finance, Islamic products, PEP and adverse media | By alert class | Skill-based |
| MLRO desk | MLRO and deputy | Recommendations awaiting decision; P0 escalations; closure approvals above delegated authority | Suspicion tier; access list |
| Compliance-only | Named compliance staff | Restricted-tier parties (staff, relatives, board, PEPs, VIP) | Never visible to L1 or the branch |
| RFI desk | Analyst or desk officer | Incoming correspondent RFIs; outgoing RFIs past their clock | See section 9 |
| QA queue | Second-line QA reviewers | Samples of closed alerts and cases (section 10) | Reviewer is not the closer |
| KYC review queue | KYC reviewers | Periodic and trigger reviews (file 26) | Separate family |

**Which alerts go to the branch first.** Today the liaison officer performs first review of the branch's alerts (MER para 567). That helps where the branch knows something the data does not (a known trader's cash deposits, a documented seasonal pattern) and hurts where the branch is conflicted or where asking the customer could tip him off. An alert class is **branch-first** only if all of these hold, set per class by the MLRO and shown in the assessor pack:

1. The branch can supply information that changes the decision (an explanation, a document, a visit result).
2. The party is normal-tier: not restricted, not suspicion tier, no open authority request.
3. The class is not a list match, terrorist financing, proliferation or staff typology.
4. The MLRO's tipping-off flag for the party allows branch contact (default: allowed for ordinary monitoring classes, refused once a case reaches "STR candidate"; the Iraqi MER already records that when more information could tip the customer off, institutions stop asking and report with what they have, file 01 section 2).
5. The liaison officer has a recorded training completion and an active appointment (CBI ch. 11 names an alternate).

The branch's work is **information gathering and a recommendation**, not a closure. The central department "reviews those decisions (close for insufficient grounds, or escalate)", as the MER describes it. This also respects the CBI's rule that the liaison officer reports technically to the MLRO only and that "branch management has no part in it".

**Assignment.** Hybrid: a pool with pull (claim) for L1 and specialist queues, push for L2, MLRO and compliance-only. Open-source Marble's auto-assignment is a usable reference: cases are distributed "sequentially and fairly across active users" until a queue limit (default 10) is reached, on case creation and when a case closes; users declare time off so they are skipped (Marble documentation, read 2026-10-07, grade C). The DBS workflow engine already has `claim` on assignees, delegation with windows and a leave ledger (file 32 section 11.1). AKR-TMS adds: skills and language, conflict checks at assignment (a user may not be given a case on a relative or on an account they manage, file 30 section 8.5), and a block on sign-in during recorded leave with a mandatory handover of open cases (file 30 section 6.5).

**Starvation and low-score alerts.** Ageing raises priority; a "must be touched by" date prevents a low-ranked alert from waiting beyond a limit; a **forced-review sample** of low scores is drawn daily so the bottom of the queue is measured, not assumed (file 21 section 9b, ladder step 2).

**No individual speed league tables.** Time per disposition is a bank-level and queue-level measure only; ranking analysts by alerts closed per hour invites the pattern the US Bancorp record shows (file 20 section 2.2) and feeds automation bias (file 29 section 9j).

### AKR-TMS hooks (section 4)

- **Priority classes by rule** (P0 to P3), visible on every row, with the legal-order explanation in the help text.
- **Score decomposition** with the EUR 100 versus EUR 99 million test in the release gate.
- **Routing table** (alert class, tier, branch-first flag, tipping-off flag default), versioned with maker-checker.
- **Assignment service** with pool pull, push for L2 and MLRO, queue limits, time-off and conflict checks.
- **Forced-sample generator** for low-scored alerts and an age-based "must touch by" rule.

## 5. Statuses, transitions, clocks and ageing

### 5.1 Design choices

1. **Stage and flag, not a state per combination.** The case has one **status** that says who owns it (a stage in the Iraqi chain) and a few derived **flags** (`awaiting_information`, `overdue`, `on_hold_request`, `reopened`, `restricted`). A status per combination of stage and waiting would multiply states and hide ownership.
2. **Every transition is a recorded event** with actor, time, reason code, evidence references and the version of the workflow definition; the state is a projection of events (the DBS `instance_events` pattern, file 32 section 11.1).
3. **Guards are enforced in the backend** and the screen only renders what the backend says is possible ("never render a control that cannot work", DBS frontend rules).
4. **Four-eyes by construction**: the person who proposes cannot approve; the person who worked a case at level 1 cannot be its level-2 reviewer; the person who prepared a report cannot sign it (file 20 L-54; file 30 section 7.4).

### 5.2 Statuses of a monitoring or KYC-event case

| Status | Owner | Meaning | Exits |
|---|---|---|---|
| `new` | Triage service | Created, in the triage queue, priority class and route set, not yet opened by anyone | `branch_review`, `central_review` |
| `branch_review` | Liaison officer | Branch-first class: the branch adds explanation, documents, visit result; recommends | `central_review` |
| `central_review` | Level-1 analyst | First central review of the alert or the branch input | `closure_review`, `closed` (within delegated authority), `investigation`, `mlro_decision` (P0) |
| `investigation` | Level-2 analyst or investigator | In-depth work: history, network, documents, RFIs, narrative | `mlro_decision`, `closure_review`, `central_review` (returned) |
| `closure_review` | Approver per delegated-authority matrix | A proposed closure awaits a second person | `closed`, back to the proposer's stage with comments |
| `mlro_decision` | MLRO or deputy | The decision: report (UTR, STR, TFR), shelve with reasons, or gather more evidence | `report_preparation`, `closed`, `investigation` |
| `report_preparation` | Preparer (L2 or MLRO office) | Report built in the builder (file 23), checks run, attachments linked | `report_approval` |
| `report_approval` | MLRO or deputy | Sign-off with a single-use step-up | `filed`, back to `report_preparation` |
| `filed` | MLRO office | The goAML reference and time are recorded; the receipt and any Office feedback are attached when they arrive | `actions_open`, `watching`, `closed` |
| `actions_open` | Operations, with the analyst tracking | Post-decision tasks outstanding (hold, freeze, exit steps); each verified from the replica or confirmed with a reason | `watching`, `closed` |
| `watching` | Assigned analyst | Enhanced monitoring with an expiry date and triggers (after a UTR or STR, or a "continue watch" decision) | `closed`, `central_review` (trigger fired) |
| `closed` | None | Outcome recorded; append-only | `reopened` |

`reopened` is an event, not a resting state: it moves the case back to `central_review`, `investigation` or `mlro_decision` according to the trigger (section 8). The screening alert has its own, shorter machine (section 5.5).

**Outcome values at closure:** `no_report_explained`, `no_report_no_suspicion`, `no_report_data_error`, `duplicate_or_merged`, `utr_filed`, `str_filed`, `tfr_filed`, `kyc_action_completed`, `transferred_to_authority_request`. A closure always carries a reason code and text; a report outcome carries the goAML reference.

```
                 +--------------------------- reopened (new alert, authority request, RFI,
                 |                             adverse media, QA failure, MLRO) ----------+
                 v                                                                        |
new -> branch_review -> central_review -> investigation -> mlro_decision -> report_preparation
 |          (branch-first)     |   \            |   ^           |  \              |
 |                             |    \           |   |           |   \             v
 +------> central_review       |     +--------> closure_review  |    +-> closed  report_approval
                               |                  |             |    (shelved,      |
                               +--> closed <------+<------------+     reasons)      v
                               (delegated authority)                              filed
                                                                                    |
                                          closed <-- watching <-- actions_open <----+
```

### 5.3 Transitions, guards and effects (monitoring family)

| # | From to | Who | Guard (must be true) | Effects |
|---|---|---|---|---|
| T1 | `new` to `branch_review` | Triage service | Class is branch-first (section 4.3); party normal-tier; branch has an active liaison officer; tipping-off flag allows contact | Task to the liaison officer; branch clock starts; neutral notification |
| T2 | `new` to `central_review` | Triage service | Central-only class, or no branch route | Task in the central pool |
| T3 | `branch_review` to `central_review` | Liaison officer, or timer | Structured branch input (explanation as a **claim** with source and corroboration status, documents attached yes or no) or the branch clock expired | Branch input stored; a clock expiry adds the flag `no_branch_response` and notifies the MLRO desk |
| T4 | `central_review` to `closed` | L1 | Closure within delegated authority for the class (section 5.4); structured disposition complete; evidence opened (see below); not restricted or suspicion tier | Outcome set; sampled into QA with the class's rate |
| T5 | `central_review` to `closure_review` | L1 or L2 | Same structured disposition; class needs approval | Task to the approver |
| T6 | `central_review` to `investigation` | L1 | Indicators present or complexity; reason code | L2 task; the L1 user is excluded from the L2 role on this case |
| T7 | `investigation` to `mlro_decision` | L2 | Recommendation (report, no report, watch) with the **CBI escalation checklist** (the seven criteria of the 26 August 2026 circular) and a draft rationale | MLRO task; case moves to the suspicion tier when the recommendation is "STR candidate" (file 30 section 8.3) |
| T8 | `closure_review` to `closed` or back | Approver (not the proposer) | Approver opened the evidence; reasons recorded; step-up if the class is high-risk | Outcome set, or the case returns with comments |
| T9a | `mlro_decision` to `report_preparation` | MLRO | Decision `utr`, `str` or `tfr` with structured reasons; the MLRO's own decision is not delegated to the AI (file 29 section 9k) | Report draft created from the case (file 23); filing clock starts |
| T9b | `mlro_decision` to `closed` | MLRO | Decision `no_report` with reasons (the CBI's "must state its reasons") | Entry in the shelved-decision register; sampled into QA at a higher rate |
| T9c | `mlro_decision` to `investigation` | MLRO | "More evidence" with a due date and a list of questions | Tasks created |
| T10 | `report_preparation` to `report_approval` | Preparer | goAML pre-submission checks pass (file 23, VB-49); mandatory fields complete; narrative edited and accepted by a human; attachments linked | Task to the MLRO |
| T11 | `report_approval` to `filed` | MLRO (single-use step-up) | Preparer is not the approver; the filing is recorded with the goAML reference and time | Report locked; acknowledgement tracked; links to earlier reports set |
| T12 | `filed` to `actions_open`, `watching` or `closed` | MLRO or system | Post-decision actions chosen (section 7) | Operations tasks created with neutral wording; watch registered |
| T13 | `actions_open` to `watching` or `closed` | Analyst | Each task verified from the replica, or closed manually with a reason and a second person | Verification result stored |
| T14 | `watching` to `closed` or `central_review` | System or analyst | Expiry with a review decision, or a trigger fired (new alert, authority request, adverse media) | A trigger raises priority and reopens review; expiry requires an explicit decision (extend, close, upgrade UTR to STR) |
| T15 | `closed` to a review status | System, MLRO, QA | A reopening trigger (section 8) | `reopened` event; same case ID; history retained |
| T16 | any to `mlro_decision` | Any analyst or the system | Emergency: list match, terrorist-financing indicator, authority order | P0 class; MLRO and deputy notified |

**Evidence must be opened before a decision button works.** For high-priority classes, the closure and recommendation actions are enabled only after the analyst has opened the evidence panels the class requires (file 29 section 9j, "analysts must open the evidence before an approve/close button activates"). The cost is a few seconds a case; the benefit is a record that the evidence was in front of the decider and a defence against automation bias (Buçinca and colleagues' experiments on "cognitive forcing functions", file 31 section 3, reduce over-reliance on a suggestion).

### 5.4 Who may close what: the delegated-authority matrix

The CBI says the MLRO decides to close a case where no suspicion is found, with reasons. In practice an MLRO cannot sign every alert, and the Iraqi chain already lets the central department "review those decisions". The tool therefore makes the delegation explicit and testable: a matrix, signed by the MLRO under maker-checker, versioned, printed in the assessor pack, and enforced by the guards above.

| Class (default proposal) | Proposer | Approver | QA sampling by the second line |
|---|---|---|---|
| P3, low-risk monitoring class | L1 | None (closure within authority) | 10 percent, with a minimum of 10 per month per analyst |
| P2, standard monitoring | L1 or L2 | L2 (second person) | 5 percent of approved closures |
| P1, high-risk, PEP, network, any case with a prior UTR or STR | L2 | MLRO or deputy | 100 percent seen by the MLRO; 10 percent re-reviewed by QA |
| P0, list match, terrorist financing, authority order | L2 | MLRO or deputy, with a second signature for a freeze-list false positive | 100 percent |
| MLRO `no_report` decisions on cases that reached `mlro_decision` | MLRO | None (the MLRO decides) | Compliance head or internal audit samples at a higher rate (section 10) |

These percentages are design starting points. They are not rules in Iraq; the bank sets them against its risk and staffing, and the tool reports the actual sampled share and the failure rate. The mockup text "a second reviewer must approve any closure (four-eyes)" is stricter than this table for P3 and P2; the product should allow a bank to choose either policy, and should default to the table above with a visible, signed delegation because a blanket second review doubles reviewing effort without an evidence base (open point for the pilot banks).

### 5.5 Screening alert states

Labels follow the Office's terms (true, potential, incorrect match) with the CBI's terms as aliases (section 3.4).

| Status | Meaning | Exits |
|---|---|---|
| `new` | Raised by list update, onboarding, delta screening or payment screening | `in_review` |
| `in_review` | An analyst compares the parties (file 22 section 13.7 for the display) | `incorrect_proposed`, `potential`, `true_match` |
| `incorrect_proposed` | Reason code and evidence recorded, including the reason for excluding the match; freeze lists need a second person within one day (file 30 SP-98) | `closed_incorrect` (pair memory written with fingerprints), back to `in_review` |
| `potential` | Similarity remains; additional review, identifiers compared (four-part name, mother's name, birth data, nationality, residence, ID number), result and reasons documented; nothing frozen on a potential match alone **unless the bank's doubt policy treats unresolved doubt as confirmed** (the CBI 2023 reading); a verification deadline equal to the freeze clock | `true_match`, `incorrect_proposed` |
| `true_match` | Sufficient indicators that the person is the listed one (or doubt remains and the doubt policy says so); MLRO informed | `freeze_task_open` |
| `freeze_task_open` | A neutral task to bank operations; the freeze clock (default 8 hours) and the 48-hour Committee report clock run | `freeze_verified` |
| `freeze_verified` | The replica shows the account block and no debit after the order (section 7) | `reported` |
| `reported` | The first report to the Committee (copy to the Office) is lodged within 48 hours, the second at two months, and any STR or TFR is filed (file 23) | `closed_confirmed` |

### 5.6 Clocks, ageing and escalation

AKR-TMS keeps several clocks apart because one number hides the problem (Mashreq's three-month gap in alert generation and Block's 129-day alert-to-SAR average, file 20). Defaults below are design proposals unless a source is given; the business calendar of the assignee's branch applies. The report-side clocks (A to J: alert to MLRO triage, triage to decision, decision to filing, suspicion-formed date, freeze, Office request, hold, CTR, liaison report, regulatory calendar) are defined in file 23 section 8.10; this table covers the case-side clocks and mirrors the ones that overlap.

| Clock | Starts | Stops | Default target | On breach |
|---|---|---|---|---|
| Data latency | Transaction booked | Alert created | Bank-set (for example 4 hours batch, 15 minutes near real time) | Shown beside the queue; control incident if above limit (L-31) |
| Elapsed age | Alert created | Case closed | Ageing buckets 0-7, 8-30, 31-60, over 60 days (the buckets of mockup 01) | Never paused; feeds the backlog dashboard and board pack |
| Handling time | Stage entered | Stage left | P3 20 working days, P2 10, P1 5, P0 same day, to a decision | Warn at 50 percent, notify supervisor at 100 percent, MLRO at 150 percent |
| Branch stage | Task created | Branch input submitted | 2 working days | Auto-escalate to central with `no_branch_response` |
| MLRO decision | Case enters `mlro_decision` | Decision recorded | P0 same day; others 3 working days | MLRO and deputy notified; appears on the command centre |
| Decision to filing | MLRO decision to report | goAML filing recorded | Same-day target, escalation at 24 hours (aligned with file 23 section 8.10; the law says "immediately") | Overdue alarm to the MLRO; counted in the timeliness report |
| First alert to filing | First linked alert | Filing | Reported, not a target: median, 90th percentile and maximum (Block 129 days average; ING over three years) | Board pack |
| RFI | RFI sent | Answer received | Internal 2 working days; correspondent as stated, default 10 working days (file 04 section 2p) | Reminders; escalation to the MLRO; see section 9 |
| Hold request | Stop-until-cleared task created | Operations confirms or MLRO clears | Same day | Escalation; see section 7 |
| Office instruction to refrain | Instruction received | A "release or extend" decision recorded | Up to 7 working days (Law 39 art. 9/First/c, via file 23 section 2.6 and 8.9) | MLRO and deputy; the decision is forced at expiry |
| Freeze | True match confirmed (or listing published) | Account block verified | 8 hours default (strictest of the Office documents; 24 hours in the 2026 guide) | Escalation to the MLRO and deputy; incident if unverified (file 23 section 3.4) |
| Committee reports | Freeze applied | First report lodged; second report lodged | First report **48 hours**, second **two months** (Regulation 6/2023 art. 4, via file 23); the CBI 2023 guideline's three days is superseded | Overdue alarm to the MLRO |
| Watch expiry | Watch registered | Review decision | Default 90 days, configurable | Task to the owner; case reopens for review |
| Liaison monthly report | Month end | Report signed by the liaison and reviewed by the MLRO | Due by the 10th (CBI ch. 11) | Overdue branches listed on the command centre |

**Pause rules.** "Waiting for information" stops the **handling-time** clock but never the **elapsed-age** clock, and a case may stay in that flag only for a bank-set maximum (default 10 working days for branch and customer questions) before it escalates. This prevents a "clock stopped" label from hiding a case, which is how backlog statistics mislead.

**Business calendar.** Iraq's working week is Sunday to Thursday with a Friday and Saturday weekend (Wikipedia, "Workweek and weekend", grade B; the Unicode CLDR week data for Iraq read through Node 22's ICU on 2026-10-07 agree: weekend Friday and Saturday, first day of the week Saturday, grade A; to be confirmed per bank), with fixed Gregorian holidays (for example 1 January, 21 March Nowruz, 3 October) and Islamic holidays whose Gregorian dates move each year and depend on moon sighting; the Kurdistan Region has additional days (Wikipedia, "Public holidays in Iraq", grade B). The calendar is therefore bank-owned data per region and branch, editable under maker-checker, with the Islamic holidays entered when announced; the DBS workflow engine already consults "the calendar of the assignee's org unit" for business-hour timers (DBS DSL section 4).

### AKR-TMS hooks (section 5)

- **Workflow definitions** (JSON documents in the DBS DSL style) for the case, the screening alert, the report, the RFI and the hold, each versioned and pinned per instance (section 15).
- **Clock service** that stores each clock as start, stop, pause and target, so any ageing figure can be recomputed; configurable per class and calendar.
- **Delegated-authority matrix** as a governed object with maker-checker, rendered in the assessor pack.
- **Guards as policy**: the same Cedar policy that governs access decides which transitions are offered (file 30 section 7).

## 6. The decision: STR, UTR or no report, with recorded reasons

### 6.1 What a decision record holds

A decision is the one record every outside reader will ask for: the CBI-approved assessor (standard D2 tests "the investigation workflow"), the supervisor reading shelved decisions, the Office grading an STR, a correspondent's auditor, in the worst case a court. It is therefore a structured object, not a text box. Fields:

| Field | Content | Why |
|---|---|---|
| Decider | A named user and role; for the MLRO's decisions, the deputy only when the MLRO is absent (recorded) | CBI: the MLRO decides; "named human decides" (file 29) |
| Decision type | `no_report`, `utr`, `str`, `tfr`, `continue_watch`, `more_evidence`, `kyc_action`, plus the legal basis class | Three-way choice of the Office's guide plus the UTR tier (section 2) |
| Reason codes | One or more from a governed list, versioned (6.2) | Searchable, countable, reviewable |
| Free-text reasons | Required; a generic-text check blocks phrases such as "no suspicion" on their own (file 20 L-54) | CBI: "the decision must state its reasons" |
| Evidence considered | List of evidence items (alert snapshots, documents, RFI answers, network snapshot) with hash and version, and the log of which panels the decider opened | Proves what was in front of the decider |
| CBI escalation checklist | The seven criteria of the 26 August 2026 circular, each yes, no or not applicable with a note (6.3) | The circular's escalation test, embedded |
| Origin of suspicion | The scenario, list match, staff report, authority request, RFI answer or adverse-media item that started the case | Needed for the report's "how" element (section 2.3) |
| Typology and predicate offence | From the bank's typology list and the Office's indicator list (file 27, file 23) | Report content and statistics |
| Subjects and transactions | Parties and transaction IDs selected for the report | Hand-off to the builder |
| Actions taken or ordered | Structured: enhanced monitoring, rating change, restrictions, hold or freeze task, exit proposal | The STR guide asks for the action taken |
| Earlier reports | goAML references and dates of earlier UTRs or STRs on the same subject | The STR guide asks for them |
| AI involvement | Whether a summary or narrative draft was used, its ID, and the edit distance between draft and final | File 29 section 9k logging |
| Second opinion | Dissent or comments from L2, QA, a second signer | Shows challenge |
| Assurance | The single-use step-up assertion ID | File 30 SP-15 |
| Versions | Workflow definition, routing table, scenario version, matcher and list version | Replay and defence |
| Timeline facts | First alert, case open, decision, filing; computed, not typed | Timeliness evidence (section 11) |

Decisions are never edited. A correction is a new decision that references the old one and states why; the earlier one stays visible (the Metro Bank lesson that escalation records and minutes must not be removable, file 20 section 6.3, and Law 78 of 2012 on electronic records, section 12.4).

### 6.2 Reason codes

Two governed lists, owned by the MLRO, versioned, with Arabic and English text written by compliance staff (not machine-translated).

**For `no_report` (shelving).** Each code requires a free-text line and at least one evidence item.

| Code | Meaning | Extra fields |
|---|---|---|
| `EXPLAINED_PROFILE` | Activity is consistent with the documented KYC profile (declared turnover, income, purpose) | Declared versus actual figures |
| `EXPLAINED_CORROBORATED` | A customer, branch or relationship-manager explanation exists and is corroborated by documents | Explanation as a claim with source, date, corroboration status and expiry (L-47); document references |
| `KNOWN_PATTERN` | A documented, recurring legitimate pattern (payroll, seasonal trade, school fees) | Pattern note; optional hibernation (section 8) |
| `DATA_ERROR` | The CBS data were wrong or mapped wrongly; corrected at source | Data-quality ticket reference (file 25) |
| `DUPLICATE_OR_MERGED` | Same activity already handled in another case | Link |
| `ALREADY_REPORTED` | Covered by an existing UTR or STR | goAML reference; decision whether to supplement |
| `NOT_SAME_PERSON` | Screening: identifiers contradict (mother's chain, birth data, documents) | Pair memory written (file 22 section 13.8) |
| `NO_INDICATOR_AFTER_REVIEW` | After the checklist, no indicator was found | List of what was reviewed; mandatory for P1 and P0 |

Reasons that are **not allowed** as closures: "too many alerts", "below the team's capacity", "same as last month" without a recorded decision, and any reason that refers to workload. A capacity-driven closure is the US Bancorp and ING failure (file 20 L-16 to L-18). The tool rejects closures that carry only such text and reports reason-code distributions by analyst, class and week to the second line.

**For reports (`utr`, `str`, `tfr`).** Typology and indicator codes aligned with the Office's indicator lists and goAML's own enumerations (file 23 defines them); plus **profile mismatch quantified** (declared turnover versus actual), because the Office's grading criteria include whether the report "contains real indicators of suspicion ... not in line with the customer's known activity and income" and whether documents and the bank's actions support it (file 01 section 2).

### 6.3 The CBI escalation checklist inside the case

The circular of 26 August 2026 lists seven criteria for escalating to a SAR when one or more applies (file 01 section 2c): (1) more than one behavioural, operational or financial indicator in one transaction or a chain of linked transactions; (2) incoherent or contradictory explanations of source of funds, purpose or relationship; (3) a link to a high-risk customer, beneficiary, third party, activity or country; (4) a clear attempt to avoid due diligence or controls; (5) signs the customer takes instructions from someone else or does not understand the transaction; (6) the same indicators recurring; (7) other information from the bank, supervisors or law enforcement that strengthens suspicion. The case shows these seven as a checklist beside the decision, pre-filled where the system can compute them (criteria 1, 3, 6 from alerts and the graph; 7 from authority-request links) and left to the analyst where it cannot (2, 4, 5). A "yes" on one or more does not force an STR; it forces a recorded reasoned decision (UTR, STR or why not).

### 6.4 UTR or STR: a decision aid, not a rule

The Office's definitions (section 2.2) are shown side by side in the decision panel: UTR, "behaviour outside the expected pattern without sufficient indicators of a crime"; STR/SAR, "unusual behaviour supported by indicators or suspicions that make a possible financial crime probable". Prompts: has more than one independent indicator accumulated? Has an explanation failed corroboration? Has a UTR already been filed on this subject? Is a list or terrorist-financing link present? The product records the answers and the MLRO's choice. It does not choose. The guide's own sentence is the safeguard against both under- and over-reporting: classification "depends on a professional risk-based assessment", and "accumulation of indicators over time may change the nature of the classification".

### 6.5 The decision not to report

Three reasons make this the most important record in the case:

1. The CBI requires the MLRO to state reasons for closing, and requires "records of cases the MLRO decided to close" to be kept (file 01 section 2d).
2. The Office's guide tells supervisors to look at suspicious transactions "that were shelved" and "the reasons behind that".
3. The MER reads some Iraqi banks as archiving many alerts daily against very few STRs, and also records that some banks avoid "defensive" filing (file 01 section 2). Both a too-low and a too-high filing rate are findings; only reasoned records separate them.

Contrast: the US FAQs of October 2025 say there is no expectation to document a decision not to file (file 05 section 1b). Iraq's rule is stricter, so AKR-TMS keeps the record **short and structured**: a reason code, one to three sentences, evidence items. A long free-text defence is not required. Every `no_report` decision made by the MLRO or at P0 and P1 is in the shelved-decision register, sampled by someone other than the decider (section 10).

### 6.6 Hand-off to the goAML report builder (file 23)

AKR-TMS does not design the XML here. The contract between case and builder is:

- **To the builder** (`ReportRequest`): case ID, report type (`utr`, `str`, `tfr`, and the case's link to a CTR or other report if relevant), subjects (parties with roles), accounts, transactions chosen from the evidence snapshot, indicators and typology codes, origin of suspicion, actions taken, earlier-report references, the narrative draft ID (human text, with AI provenance if any), attachment IDs from the vault.
- **From the builder** (`ReportResult`): report ID, version, validation results, goAML reference and filing time (entered or captured at upload), acknowledgement time and content, any Office feedback or grade, any request for more information. A filed report is locked; a change is an amendment with a new version and a link.
- **Rules**: the preparer cannot sign; the MLRO signs with a single-use step-up; filing "without delay" after sign-off is a clock (section 5.6); the case moves to `filed` only when a reference exists; a "poor" quality grade creates a rework task and a feedback-register entry (file 01 section 2: poor STRs go back to the reporter and the supervisor is notified).
- **Pre-submission quality check** that mirrors the Office's grading criteria (indicator present, profile mismatch quantified, documents attached, actions recorded) is in the builder (file 23, VB-49) and its result is stored on the decision record.

### AKR-TMS hooks (section 6)

- **Decision record** schema above, immutable, with correction-by-reference.
- **Reason-code libraries** (`no_report`, report typologies) versioned with maker-checker; closures that carry capacity-driven or generic text are rejected.
- **Escalation checklist** widget pre-filled from system facts.
- **UTR/STR decision aid** with the Office's definitions and the accumulation prompts.
- **ReportRequest/ReportResult** interface to file 23 and a rework path from FIU grades.
- **Shelved-decision register** and its QA sampling.

## 7. Post-decision actions: what AKR-TMS can and cannot do

### 7.1 The constraint

AKR-TMS never writes to the core banking system and never calls its application interfaces. It can **decide, record, instruct and verify**; it cannot freeze, hold, block, close or flag. Everything the decision implies for the account is therefore a **task for a human in bank operations**, with a neutral instruction, a clock and a verification step that reads the result from the replica. The same design applies in phase 2, where the pre-release API lets the gateway hold a payment but the freeze still happens through the bank's own process (file 25 section 15.7).

### 7.2 Action catalogue

| Action | Who does it | AKR-TMS role | Verification from the replica |
|---|---|---|---|
| **Enhanced monitoring (watch)** | AKR-TMS | Registers a watch on the party with an envelope, triggers and expiry; lowers thresholds and raises priority for the party; default expiry 90 days | The watch's own counters; nothing in the CBS |
| **Customer risk re-rating and EDD** | KYC process (file 26) | Event-driven re-rating, EDD checklist task with due date | Rating history; EDD evidence in the vault |
| **Restriction (for example no cash deposits, not executing an outgoing transfer)** | Operations or the branch under the bank's procedure | Task with neutral wording and the authorising decision ID | Account status or product flag if the CBS exposes one; absence of the restricted transaction type after the effective time |
| **Hold or stop-until-cleared (CBI ch. 5 art. 5(6))** | Operations / payment gateway | Phase 1: task and clock; phase 2: the gateway holds through the pre-release API and learns of release by pull | Hold state in the CBS or gateway; no posting of the held item |
| **Freeze after a true match** | Operations | Task with the freeze clock (default 8 hours), the 48-hour Committee report and the two-month update, report drafts (file 23) | Account block in place; **no debit after the order time** ("order-to-block time" and "post-order debit" checks, file 25 C-60) |
| **Relationship exit or closure** | Business with senior management approval | Exit proposal with reasons; approval record; the decision register keeps reversals (file 20 L-57: HSBC USA reversed an exit under commercial pressure) | Account status closed; no new accounts for the party (entity resolution including family number, L-48) |
| **Information to other units** (credit, trade finance, cards, treasury) | MLRO or analyst | Need-to-know notice with neutral wording; recorded in the case | Not applicable |
| **Authority liaison** (answering the Office, a court) | MLRO office | Register entry, response draft, disclosure-register entry (file 30 SP-52) | Response sent time |

### 7.3 The operations task contract

| Field | Rule |
|---|---|
| Instruction | A code from a **neutral-wording library** plus an order reference, for example "apply compliance hold, order ref ...": never the reason, never the words STR, UTR, suspicion or the case type (file 30 SP-31); the library is tested for forbidden words |
| Target | Account and party identifiers; the effective time |
| Authority | The decision ID and the approver(s); a freeze on doubt needs the second approval of section 5.5 |
| Visibility | The operations officer sees the instruction and reference only, not the case |
| Clock | Per action (hold same day; freeze immediately, "without delay" meaning hours for UN-designated persons, file 25 section 15.7) |
| Completion | The officer enters the CBS reference and confirms; AKR-TMS then runs the verification |
| Verification levels | V0 not observable (second-person confirmation required, flagged in reports); V1 status observed; V2 no debit after the effective time observed; V3 order-to-block time computed. The level actually achieved is stored and reported per bank, because it depends on what the CBS exposes (file 25) |
| Failure | A task that is overdue, or fails verification, or shows a post-order debit, raises an incident to the MLRO and the deputy; it cannot be closed without a recorded reason and a second person |
| Notification text | In-app and the bank's SMTP only, generic ("you have a task"); no external push (file 30 section 8.2 row 4) |

### 7.4 Watching after a report

The Office's UTR guide says enhanced monitoring continues after a UTR, and the STR guide asks the bank to say what action it took. The older US convention of a 90-day manual re-review after a SAR is now optional there (file 05 section 1b), so AKR-TMS offers a **watch** with a policy: envelope (accepted behaviour as numbers), triggers (new alert, adverse media, authority request, rating change), expiry (default 90 days, configurable) and a review decision at expiry (extend, close, upgrade UTR to STR). The watch is the mechanism for the UTR flow chart ("apply enhanced monitoring ... if indicators do not develop, the UTR already sent is enough; if they develop, file an STR").

### AKR-TMS hooks (section 7)

- **Action catalogue** with task templates, neutral wording library and forbidden-word tests.
- **Verification engine** with V0 to V3 levels, per-bank achievable level report, and incident rules.
- **Watch registry** with envelope, triggers, expiry and review decision, feeding the UTR-to-STR path.
- **Exit proposal workflow** with senior-management approval and a reversal flag to the board committee.
- **Authority-response workflow** with disclosure-register entries.

## 8. Closure, hibernation, re-opening and links

### 8.1 What closing a case does

1. The outcome, reason code, text and decider are fixed; the delegated-authority check has passed (section 5.4).
2. **Post-decision actions** are verified or explicitly waived with a reason and a second person.
3. **The case file is sealed**: a hash of the case record and of every attachment is written to the audit chain; the case becomes append-only; corrections are new entries (file 30 section 9.5, SP-38).
4. The retention class and clock are set: R6 for cases and STRs, with the later of closure and report date as the start, floor seven years in the Iraq profile (five under Law 39; M-05), counted from the report date or ruling, default ten (file 30 section 12.3); legal holds override.
5. The case is sampled for QA by its class (section 10).
6. If the outcome is `utr_filed` or `str_filed`, the **watch** (section 7.4) starts.

### 8.2 Hibernation (what to do after a "not suspicious" decision)

The ladder of file 21 section 9b applies: hibernation silences the *display* of the same scenario for the same party for a period or until behaviour leaves a stored envelope or a trigger fires, while the scenario **still runs and logs**. Design rules:

- Created only **inside a case, by a human**, so a human review is on record. Open-source Marble works the same way: a rule snooze "must be created in the context of a decision that is within a case, to guarantee that there has been a human review", for at most 180 days, with a comment and a case event each time (Marble documentation, rule snoozes, read 2026-10-07, grade C).
- Default expiry 90 days (file 21), hard maximum 180 days; renewal needs a new decision.
- Envelope stored as numbers; exceeding it by a margin ends the hibernation and raises priority.
- A **hibernation ledger** counts what was held back and why, shown daily to the head of AML; **regret rate** (hibernated parties later tied to an STR, an authority request or a confirmed case) is a KPI; high regret means envelopes are too generous.
- For the CBI minimum scenarios the evaluation always runs; hibernation changes only what analysts see, so coverage reporting stays complete.
- Classes that need MLRO approval for closure need MLRO approval for hibernation.

### 8.3 Re-opening

A closed case reopens (same case ID, `reopened` event, original closure kept in history, priority raised) when:

| Trigger | Notes |
|---|---|
| A new alert on the same party and family within the grouping window | Section 3.3 |
| An authority request, court order or Office request naming the party | Link and notify the MLRO |
| An incoming correspondent RFI on the party | "Missed by our monitoring" if no alert had fired |
| Adverse media, a list change or a rating change | From the screening and KYC services |
| A QA failure or an MLRO decision to reopen | Bulk reopen by reviewer and date range after a surge or a contractor failure (Coinbase, file 20 L-53) |
| A later report or FIU feedback that contradicts the closure | A poor grade or a request for supplementary information |
| A look-back after a scenario or data correction | From file 25 reconciliation and file 27 tuning |
| A hibernation or watch expiry that is not renewed | Review decision required |

**Reopen rate** is tracked per analyst, class and scenario. A high rate suggests premature closures; a rate near zero with a high closure speed suggests nobody is looking back. Both are read as questions, never as individual targets.

### 8.4 Links

Cases link to: earlier and later cases of the same party (merged, child, related); the **UTR to STR chain** with goAML references; KYC reviews and rating changes; authority requests; RFIs; the network case parent; and any change request that arose from the case (for example "scenario tuned after case"). The party page shows all open and recent cases of every family, so an analyst never works one family blind to another.

### 8.5 Late detection

Each filed report stores the age of the underlying activity at filing. The share of reports whose activity is older than N months is a control-timeliness figure (the FCA's 2026 review found 35 percent of breaches reported in 2025 related to earlier years, file 05 section 3b); the first-alert-to-filing distribution is a board metric (section 11).

### AKR-TMS hooks (sections 7 and 8)

- **Case sealing job** (hashes into the audit chain) and **retention-class assignment** at closure.
- **Hibernation ledger**, regret metric and expiry rules.
- **Reopen service** with triggers, bulk reopen, and a reopen-rate report.
- **Party page** that lists all cases and watches across families.

## 9. Requests for information: branches, relationship managers, correspondents, the Office

### 9.1 What exists to build on

| Source | What it gives | Grade |
|---|---|---|
| CBI ch. 11 and the MER | The branch liaison officer is the MLRO's assistant in the branch, with a dedicated line to the MLRO; branch management has no part in suspicion reporting | A |
| Iraqi MER (via file 01) | When asking the customer for more information could tip him off, institutions stop asking and report with what they have | A |
| Wolfsberg RFI Best Practice Guidance, August 2022 (file 04 section 2p) | 30 standard questions with expected answers; an indicative response window of **10 to 30 business days**; the respondent should start "without delay" and ask for more time early; the guidance says an incoming RFI should be treated as "an input to its own transaction monitoring or as a manual trigger for a local investigation"; poor or late answers can lead the correspondent to file an STR or restrict payments | A |
| Wolfsberg effective-monitoring Part II (file 04 section 2b) | "Downstream integration": the analyst can clarify activity with the customer through the relationship manager or contact centre | A |
| Salv Bridge (file 13 section 7) | A request-and-response channel between named investigators using **use-case templates** that fix which fields may be shared and why, logged and credentialled; Estonia pilot: one in four urgent requests answered in under five minutes, 75 percent within an hour, against 24 to 48 hours typical (vendor-reported, supervised by three authorities) | C |
| Oracle Investigation Hub (file 09) | RFIs "with participant support" and substitution variables for RFI templates | C |

### 9.2 One RFI object, six kinds

| Kind | From | To | Channel and rule |
|---|---|---|---|
| `branch_info` | Analyst | The branch's liaison officer only (never branch management) | In-app task; neutral wording; the officer answers from the file and observation log; customer contact only if the party's tipping-off flag allows it |
| `rm_info` | Analyst | Relationship manager | Only for normal-tier parties and only when the MLRO's flag allows; every question is neutral; answers are claims, not evidence |
| `unit_info` | Analyst | Credit, trade finance, cards, treasury, operations | Need-to-know fields only; neutral wording |
| `customer_outreach` | Analyst, with MLRO approval | The customer, through the bank's own staff and channels | AKR-TMS has no customer-facing surface (file 30 SP-96): it supplies a neutral script and records that contact happened and what was answered |
| `correspondent_in` / `correspondent_out` | A correspondent / the MLRO office | The bank / a respondent (an exchange company, a smaller bank) | RFI desk; Wolfsberg's 30 questions pre-filled from the replica; four-eyes release; deadline from the sender, default 10 business days |
| `authority_in` | The Office, the CBI, a court, law enforcement | The MLRO office | Register entry; suspicion-tier case access list; response drafted from system facts; MLRO approves; disclosure-register entry (file 30 SP-52); the deadline is the one the Office states in the request: Law 39 art. 9/First/b requires replies "within the period it specifies" (file 23 section 2.6); no numeric default exists, so the bank sets an internal default (3 working days) that the request overrides |

### 9.3 Template anatomy

Every template is a versioned record with Arabic and English text written by compliance staff:

| Element | Rule |
|---|---|
| Purpose code | An internal neutral code (for example `KYC-CLARIFY`, `DOC-CHECK`); the words STR, UTR, suspicion, investigation and alert never appear in text visible outside the compliance zone |
| Legal-basis tag | The statute or contract clause that permits the exchange (Law 39 for authorities; customer terms that allow disclosure to correspondents; banking secrecy exceptions per Banking Law art. 49 and Law 39 art. 52) |
| Minimal questions | Only what the purpose needs; each question has a "why this is asked" line for the respondent, as the Wolfsberg guidance recommends |
| Response format | Structured answers (yes, no, unknown, a value, a document) with limited free text, so the answer can be read without follow-up |
| Expiry and clock | A deadline in business days from the sender's calendar, reminders, an extension request |
| Allowed attachments | Types and size; antivirus; stored in the vault with the case's access list |
| Release rule | Internal RFIs: the case owner; external RFIs and authority responses: a second person (the MLRO or deputy) with a single-use step-up |
| Logging | Every send, reminder, answer, extension and release is an event |

### 9.4 Templates the product ships

1. **Branch information request** (liaison officer): Does the branch know the customer's business and its normal cash and transfer volumes? Which documents on the file support the recent activity (contracts, invoices, licences)? Has an explanation been received, from whom, when (only if the contact flag allows asking)? Any branch observations or visits? Any link to staff or officials? Any other relationships known to the branch?
2. **Relationship-manager clarification**: three to five of the above, reduced to facts the manager legitimately holds.
3. **Documents request**: source-of-funds and source-of-wealth documents requested through the bank's own process (file 26).
4. **Incoming correspondent RFI**: the 30 Wolfsberg questions grouped as account, background KYC, transactions, counterparty, the bank's view, adverse media and sanctions, documents; AKR-TMS **pre-fills** the answers it holds (opening date, account types and purpose, expected versus actual activity, status and closure reason, KYC profile, source of wealth record, owners and signatories, counterparty history and whether the counterparty also banks with us, rating history with reasons, last periodic review, screening and adverse-media history, linked documents); the officer adds the bank's own view (Q22) and releases with a second signature; fields that would reveal an investigation, an STR or an exit are flagged and need MLRO approval (file 04 section 2p).
5. **Outgoing correspondent RFI** (when the bank is the correspondent for exchange companies or smaller banks): a pre-RFI checklist (internal records, public sources, earlier RFIs), questions taken from the 30 with a brief rationale each, a stated deadline, quality grading of the answer, escalation of non-response, an option to restrict payments for the subject until answered.
6. **Authority request response**: a register form (sender, reference, received date, deadline, subjects, questions) and a draft assembled from system facts; no free-form disclosure beyond the questions.

### 9.5 Clocks

| Clock | Default | Pause | Escalation |
|---|---|---|---|
| Internal branch or unit RFI | Answer in 2 working days; one reminder at 1 day | Handling clock only (never the elapsed age) | At the deadline to the MLRO desk, **not** to the branch manager; a case may remain "awaiting information" for at most 10 working days |
| Customer outreach | Bank-set, default 5 working days | Same | The MLRO decides whether to continue or to decide on what is known |
| Incoming correspondent RFI | The sender's deadline, default 10 business days (Wolfsberg indicative window 10 to 30); start the same day; ask for an extension early | Never | Reminders at 50 and 80 percent; MLRO notified at 100 percent; late answers counted in the correspondent evidence pack |
| Outgoing correspondent RFI | The deadline stated in the request | Never | Non-response is escalated to the governance view and may prompt a payment restriction or an STR decision (file 04 section 2p) |
| Authority request | As stated by the authority; 3 working days if none is stated (design default) | Never | MLRO and deputy; overdue shown on the command centre |

### 9.6 Answers are claims

An RFI answer is stored as a **claim** with source (who answered), date, corroboration status (none, partly, documented) and expiry (L-47). An alert cannot be closed on an uncorroborated explanation alone; the Rabobank N.A., Capital One and Monzo cases show how explanations accepted in place of evidence let activity continue (file 20 FM-21). The RFI desk records for each answer whether it needed follow-up rounds and whether the subject had alerted before; the share of RFIs about customers who never alerted is the "missed by our monitoring" rate and feeds scenario tuning (file 04 W-23).

### 9.7 Tipping-off controls in RFIs

- Text is neutral and templated; free text is scanned for forbidden words and for any mention of the case type before sending.
- Branch RFIs go to the liaison officer only; the branch manager never sees them (CBI ch. 11).
- No external push or chat tools; in-app and the bank's own SMTP only (file 30 section 8.2 row 4).
- A relationship manager is not contacted when the party is restricted-tier, when the RM manages the account in question and a conflict is detected, or after the case reaches "STR candidate".
- The customer-outreach script is generic and approved by the MLRO; staff are trained on what not to say, which the product supports with training content but cannot enforce (file 30 section 8.7).

### AKR-TMS hooks (section 9)

- `rfi` object with six kinds, template library, legal-basis and purpose codes, minimal-field enforcement.
- **RFI desk** with pre-fill from the replica for Wolfsberg questions, four-eyes release and a forbidden-word scan.
- **Clock service** wiring (section 5.6) and RFI KPIs (median and 90th percentile time to answer, share by deadline, follow-up rounds, top questions, "missed by monitoring" rate).
- **Claims model** for answers with corroboration status and expiry.

## 10. Quality assurance

### 10.1 What QA is for

QA tests the **decision**, not the speed. The CBI's D2 text requires the monitoring system to support "periodic calibration and quality assurance controls" and investigations that are "timely ... proportionate to the potential risk" (section 2.5), so QA is assessed by the external auditor, not optional. Its questions: Did the reviewer see the evidence the class requires? Does the reason code match what the evidence shows? Was an escalation criterion missed? Was the explanation corroborated? Was the case in the right queue and handled in time? Was the narrative accurate and free of tipping-off phrasing? For MLRO `no_report` decisions the question is whether a reasonable second reader would have decided the same.

The enforcement record is blunt about what happens without it:

| Case | Finding | Source |
|---|---|---|
| Coinbase (NYDFS 2023) | Over 100,000 unreviewed alerts; 1,000+ contractors hired to "burn through" the backlog; "initially, Coinbase did not have a system in place to audit the quality control"; "more than half failed the quality check" among about 73,000 alerts cleared by three contractors; one had a 96 percent failure rate in a sample of 186 alerts, so about 11,000 were re-reviewed; another had 73 percent in a sample, so about 41,000 were re-reviewed | File 20 section 8.1, A |
| Mashreq New York (NYDFS 2018) | "The same analyst conducted first- and second-level reviews of the same transaction"; closing rationales lacked "essential information, such as the relationship between the transacting parties"; quality reviews "suffered from a five-month backlog"; temporary staff hired to clear the backlog were inadequately trained | File 20 section 3.5, A |
| ING (Dutch prosecutors 2018) | Forty-nine alerts on one Curacao company "all ... closed, virtually without further investigation, as 'not suspicious'"; "for years we only monitored the tip of the iceberg without taking samples in the remaining alerts" | File 20 section 4.3, A |
| US Bancorp (FinCEN 2018) | Sampling below the alert cap found "an average of 50% of the transactions that were reviewed ... resulted in the filing of a SAR"; the testing was stopped in April 2012; of 57,000 customers who alerted in a wire review, "the Bank reviewed less than 100" | File 20 section 2.2, A |
| HSBC USA (OCC 2010-2012) | Backlog of over 17,000 alerts; "closure of alerts based on ineffective review"; 890 late SARs on USD 6.34 billion | File 20 section 2.4, A |

### 10.2 Sampling design

**Population and strata.** Closed alerts and cases of the period, stratified by priority class, outcome, analyst, tenure and scenario; plus the MLRO's `no_report` decisions as a separate stratum; plus auto-hibernated and low-score alerts (the below-the-line stream of file 21).

**Random plus targeted.** A random base sample gives unbiased rates. A **targeted** sample adds the cases most likely to be wrong: closures in under a minimum time, closures outside working hours, generic reason text, closures by analysts in their first 90 days, closures of parties with earlier UTRs or STRs, closures just before a deadline. Targeted samples are reported separately and never mixed into the rate.

**Sample size.** If a population contains error rate p, a random sample of n items with no errors found supports the statement "the error rate is below p with confidence C" when n = ln(1 - C) / ln(1 - p). Computed on 2026-10-07:

| Upper bound p on the critical-error rate | n for 90% confidence | n for 95% | n for 99% |
|---|---|---|---|
| 10% | 22 | 29 | 44 |
| 5% | 45 | 59 | 90 |
| 3% | 76 | 99 | 152 |
| 2% | 114 | 149 | 228 |
| 1% | 230 | 299 | 459 |

For small populations the hypergeometric version needs slightly fewer (at 95 percent confidence and p = 5 percent: 51 of 200 items, 56 of 500, 57 of 1,000, 59 of 10,000; at p = 2 percent: 105 of 200, 129 of 500, 148 of 10,000). File 21 section 12e quotes the same arithmetic as the "rule of three" (the 95 percent upper bound is about 3 divided by n). When errors are found, the interval widens: with 2 errors in 59 reviewed, the Wilson 95 percent interval is 0.9 to 11.5 percent; with 5 errors in 149 it is 1.4 to 7.6 percent. The product reports the rate **with its interval** and never a bare percentage.

**Rates by reviewer.** Reviewer-level failure rates use the same intervals; a reviewer above a bank-set limit moves to higher sampling, and above a hard limit the product offers **bulk reopen** of that reviewer's cleared items by date range to a re-review queue (Coinbase, file 20 L-53).

### 10.3 The QA rubric

Each reviewed item is scored on a short rubric. Scores are Pass, Pass with comment, Minor, Major, Critical.

| Criterion | Question | Critical when |
|---|---|---|
| Evidence | Did the decider open the required evidence? Is it sufficient for the reason code? | A suspicious pattern was closed with no evidence reviewed |
| Reason | Does the code match the facts? Is the free text specific rather than generic? | The reason is contradicted by the data |
| Escalation | Were the CBI escalation criteria applied; was an indicator missed? | An STR-level indicator was missed |
| Corroboration | Did closure rest on an explanation alone? | Closed on an uncorroborated explanation for a P1 or P0 class |
| Timeliness | Were stage clocks met; was an RFI chased? | A reportable case sat beyond its clock |
| Confidentiality | Any tipping-off wording or access outside the list? | A disclosure risk |
| Narrative (for reports) | Facts only, who-what-when-where-how, no unsupported claim | A misleading or unsupported narrative |
| Authority | Did the person have delegated authority; was four-eyes respected? | A closure outside authority |

A **critical** finding creates a re-review of the reviewer's related items and a feedback task; a **major** finding creates a coaching item; both are counted by class, scenario, analyst, week.

### 10.4 Four-eyes and independence

- Maker and checker are different people; the L1 and L2 on the same case are different people; the preparer and approver of a report are different (section 5).
- QA reviewers sit in the second line outside the analysts' reporting line, or are internal audit when the bank is too small for a separate team; the validator is not the owner (file 20 L-30).
- QA reviewers receive **time-boxed access** to exactly the sampled items (a per-case grant that expires), with the conflict check applied and every read logged (file 30 sections 8.4, 8.5, 9.2); they cannot export.
- Internal audit samples QA's own work yearly; the CBI-approved assessor's test of "user permissions" and "audit-trail integrity" covers this role design.
- The MLRO's own decisions are sampled by the compliance head or internal audit, never by a subordinate.

### 10.5 Feedback loops

| Loop | From | To | Mechanism |
|---|---|---|---|
| Coaching | QA findings | Analyst and team lead | Items with the case link, tracked to closure |
| Procedure | Repeated confusion | Policy owner | A "procedure gap" tag on findings; a monthly list for the MLRO |
| Tuning | Scenarios with high explained-closure rates, or with missed items | Compliance configuration manager | Findings feed the above- and below-the-line review (file 27, file 20 L-19) |
| AI evaluation | QA-confirmed decisions and disagreements | The golden set and shadow evaluation of assistants | Only QA-confirmed cases enter the golden set, to avoid learning analysts' errors (file 21 section 12e, VB-39) |
| Training | Gaps by topic | Training register | Competency tests and records (the CBI asks for training records) |
| Board | Rates with intervals, critical findings | The board pack | Generated from system facts, any manual edit shown (L-56) |

### 10.6 Calibration

- **Monthly calibration**: reviewers and a sample of analysts independently score the same ten or so cases (some re-used from the golden set); the product shows agreement per criterion and flags divergent reviewers. A chance-corrected agreement statistic (Cohen's kappa) is reported by criterion; the bank sets its own acceptable level.
- **Analyst calibration**: new analysts review a set of seeded training cases (synthetic, from the test pack, never real) before queue access, with the completion recorded; a failure keeps queue access closed (file 20 L-53: "new analysts are not given queue access until their training and supervisor sign-off are recorded").
- **Optional seeded-case programme**: synthetic, flagged-internally cases injected into live queues to measure the share of suspicious items an analyst closes. Safeguards: the cases carry synthetic parties that exist only in AKR-TMS, are excluded from every statistic and report, cannot produce a goAML report, and any access to them outside the programme is an alert (the canary design of file 30 section 8.6). The programme is the MLRO's decision and is disclosed to staff in policy.

### 10.7 QA metrics

QA failure rate and critical rate with intervals, by class, scenario, team and reviewer; time from closure to QA; QA backlog age (its own queue and age, file 20 section 3.5); coaching items open and closed; calibration agreement; share of `no_report` decisions sampled; reopen rate after QA; disagreement between QA and the original decider on re-review.

### AKR-TMS hooks (section 10)

- **Sampling service** with strata, random and targeted modes, size calculator (formula above), and a sample register that cannot be re-drawn.
- **QA workspace** with the rubric, scoring, findings, bulk reopen and coaching tasks; time-boxed per-case grants.
- **Calibration kit** (golden cases, agreement report) and the training-gated queue access.
- **QA reporting** with intervals, by reviewer, class and scenario.

## 11. Workload and backlog management

### 11.1 What went wrong elsewhere, and what regulators did

| Case | Capacity facts | What the authority did | Source |
|---|---|---|---|
| HSBC Bank USA (2010-2012) | "A backlog of over 17,000 alerts, in four business units ... that had not been investigated"; 98 percent in one group; 14 percent six months old or more; causes: "inadequate staffing and procedures in the alert investigations unit" | OCC supervisory letter of March 2010 with a deadline of 30 June 2010; look-back; 890 late SARs on USD 6.34 billion; consent order and later penalty | File 20 section 2.4, A |
| US Bancorp (2018) | Alert volume "based solely on staffing levels"; 25 to 30 investigators for a bank with over USD 340 billion in assets | FinCEN: staff-based alert caps "reckless"; USD 185 million; look-back found 2,121 late SARs | File 20 section 2.2, A |
| ING (2018) | Daily alert caps of three for several categories; "the maximum number of alerts ... determined to a large extent by the staff capacity" | Dutch prosecutors: EUR 775 million | File 20 section 4.3, A |
| Mashreq NY (2018) | Three-month backlog in alert generation; five-month QC backlog; same person at L1 and L2 | NYDFS consent order, USD 40 million (and USD 100 million in 2021 for sanctions) | File 20 section 3.5, A |
| Coinbase (2023) | Alerts grew to over 100,000, many months old; EDD backlog over 14,000; business grew 15 and 25 times in under two years; a 2019-2021 case system "inadequate" | NYDFS: independent monitor by MOU (February 2022); USD 50 million penalty plus USD 50 million compliance spend; the firm "did not inform the Department" of issues until July 2022 | File 20 section 8.1, A |
| Block (2025) | Backlog 18,000 (2018) to 169,000 (2020); policy 60 days, actual average 129 days from alert to SAR, 70 days to case start | NYDFS consent order, USD 40 million | File 20 section 8.2, A |
| N26 (2021-2024) | Suspicious activity reports "systematically too late" | BaFin: EUR 4.25 million (2021); a cap of 50,000 new customers a month and a special representative (2021); EUR 9.2 million (2024) | File 20 section 7.1, B |
| FCA multi-firm review (28 May 2026) | Backlogs and late detection: 35 percent of breaches reported in 2025 related to earlier years | The FCA has "little patience for failures caused by backlogs, particularly if those are due to known resourcing issues" | File 05 section 3b, A |
| OFSI (2 September 2026) | "Delays in sanctions alert handling" and weak escalation during rapid list change | Citibank London branch fined GBP 4.73 million | File 05 section 3c, A |
| Iraq (MER IO4) | "some FIs have insufficient human resources to effectively monitor and investigate alerts for STRs within a reasonable timeframe, especially considering the high number of alerts" | A named weakness of the 2024 evaluation; grey list June 2026 with an action plan to raise the quality and number of STRs | File 01 section 2 |

Three regularities. **First**, the regulator treats a backlog as a control failure with a date, and the remedy is a deadline, a look-back and an outside monitor, not an explanation. **Second**, capacity-driven shortcuts (caps, suppression, contractor surges without QC) are punished more heavily than the backlog itself. **Third**, supervisors ask when the board knew: Coinbase informed the regulator late, and US Bancorp's compliance officers avoided telling the OCC about caps. The product therefore makes the backlog visible to the MLRO and the board automatically, and generates the notification task.

### 11.2 A capacity model the bank can run before the backlog exists

The inputs are measured by the product from the first week; the model is an arithmetic sketch, not a forecast. Handling times in this table are **assumptions for illustration**: no independent measurement of minutes per alert was found (vendor pages quote a 38-minute manual baseline for transaction-monitoring alerts, Flagright, and "3 hours to 30 minutes" for case preparation, Lucinity, both vendor-reported and undefined, file 13). The pilot banks' own data replace them.

**Workload** (hours per day) = sum over classes of (new cases per day x mean handling minutes) / 60.
**Staff needed** = workload / (productive hours per analyst-day x occupancy). Productive hours = paid hours less shrinkage (leave, training, meetings, breaks, QA duty); occupancy is the share of productive time spent on cases.

Worked example (illustrative): 100 new cases a day after customer-level grouping, working week Sunday to Thursday.

| Class | Share | Mean handling (min) | Weighted (min) |
|---|---|---|---|
| P3 low-risk | 40% | 10 | 4.0 |
| P2 standard | 45% | 35 | 15.8 |
| P1 high-risk, network | 12% | 120 | 14.4 |
| P0 list match, TF | 3% | 90 | 2.7 |
| **Mean per case** | | | **36.9** |

Workload = 100 x 36.9 / 60 = **61.4 hours a day**. With an 8-hour day and 30 percent shrinkage (5.6 productive hours): at 85 percent occupancy one analyst gives 4.76 hours, so **12.9 analysts**; at 80 percent 13.7; at 90 percent 12.2. Sensitivity: handling time +20 percent needs 15.5; arrivals +25 percent needs 16.1; both need 19.4. QA (5 to 10 percent of effort), the MLRO desk, RFI desk and supervisors are additional.

**Backlog dynamics (fluid model).** If the same bank is staffed at 11 analysts, capacity is 52.4 hours a day against a workload of 61.4: the deficit of 9.1 hours a day (14.7 cases a day) adds about **885 cases in 60 days**. Backlog is not a slow leak that stabilises; below capacity it grows linearly until someone acts, which is why the alarms of section 11.4 look at **arrivals against throughput**, not at the backlog alone.

**Little's law.** Average backlog L = arrival rate x average time in the system W. With 100 cases a day and a 10-day average time to decision, about 1,000 cases are open at any moment; if W doubles with the same arrivals, so does the backlog. The ageing buckets of the command centre are this relation made visible.

**Why occupancy is capped.** In a multi-server queue (M/M/c, computed with the Erlang C formula, 12 analysts), the mean wait before work starts, as a multiple of the mean handling time, is 0.15 at 80 percent occupancy, 0.27 at 85, 0.53 at 90, 1.35 at 95 and 3.84 at 98 percent: the waiting time rises sharply above about 90 percent. Real AML queues have priorities, bursts and waits for information, so they are worse than this model, which is why the default planning occupancy is 80 to 85 percent and why the product reports the actual figure (productive time spent over productive time available).

**Volume elasticity.** Alerts per 1,000 transactions are computed per scenario; a forecast uses transaction and customer growth. Coinbase grew 15 times and 25 times in under two years; the Block backlog came partly from "the increase in alerts generated by the implementation of new transaction monitoring tools". Any go-live of a new rule set, a new CBS source or a new product shows its **expected alert volume change in analyst hours** before activation (file 20 L-52, file 27 tuning) and runs in shadow mode first.

### 11.3 Queue design for surges

- **Priority classes** keep P0 and P1 flowing when volume spikes (section 4); P3 can wait but not forever (must-touch-by).
- **Surge mode** (a list event, a new CBI circular, an alert-volume spike): pre-clear known false positives from pair memory, route to a temporary pool, raise QA sampling for the pool, and report progress to the MLRO hourly (file 05 section 3d).
- **Temporary and outsourced reviewers**: time-boxed roles, training completion tracked before queue access, a higher automatic QC rate, reviewer-level failure rates and bulk reopen (file 20 L-53).
- **Work-in-progress limits**: a per-analyst limit on open cases (default 10, as in Marble's auto-assignment) so cases do not sit half-worked.
- **Detection never throttled**: surplus is queued, never capped; any configuration that reduces alert volume needs a reason code (risk-driven or capacity-driven), a second approver outside operations and a below-the-line test (file 20 L-18; file 30 section 7.5).

### 11.4 Backlog alarms

| Alarm | Rule (defaults, bank-set) | To |
|---|---|---|
| Ageing | Share of open cases older than the class SLA above x percent; any case older than 60 days | Team lead, then MLRO |
| Growth | Arrivals exceed throughput for 5 consecutive working days; backlog up more than y percent in a week | MLRO and head of AML |
| Latency | Data latency (transaction to alert) above the limit | MLRO, data steward (control incident) |
| Silence | A scenario produces zero or the same count for N days (flat line) | Compliance configuration manager (file 20 L-29) |
| Notification | A threshold on overdue alerts crossed, or an event under an MOU or direction | A **task for the MLRO** to notify the supervisor, with a record of whether and when it was done (file 20 section 8.1) |
| Capacity forecast | Forecast hours exceed available hours in the next 4 weeks | MLRO, board pack |
| QA | Critical-error rate above limit | MLRO, QA lead |
| Liaison reports | A branch has not submitted its monthly report by the 10th | MLRO desk |

### 11.5 Metrics

All are **bank- or queue-level**; none is an individual speed ranking.

| Metric | Definition | Notes |
|---|---|---|
| Ageing | Count and share of open cases by elapsed age bucket (0-7, 8-30, 31-60, over 60 days), by class and queue | The command-centre bars (mockup 01) |
| Throughput | Cases decided per working day, by class | Against arrivals |
| Arrival-to-throughput ratio | Arrivals divided by decisions over a window | Above 1 for 5 days is an alarm |
| Time to decision | Median, 90th percentile, maximum, from alert creation to decision, by class | Elapsed, never paused |
| Time to file | Decision to goAML filing; and first alert to filing | Block 129 days; ING over three years |
| Alert-to-case rate | Alerts that join a case divided by alerts created | Reflects grouping, not quality |
| Case-to-report rates | Cases ending in UTR, STR or TFR divided by cases closed, per scenario and segment | Effectiveness proxy, not a target (Wolfsberg: alert-to-SAR ratios "measure quantity, not usefulness", file 04 section 2a) |
| UTR-to-STR conversion | UTRs later followed by an STR on the same subject within N months | The Office's flow made measurable |
| Re-open rate | Closed cases reopened within 90 days | Questions, not targets |
| QA failure rate | With interval, by class and reviewer | Section 10 |
| Regret rate | Hibernated or whitelisted parties later tied to an STR or authority request | Section 8.2 |
| Backlog of related work | Overdue periodic reviews, EDD cases, RFIs, unreviewed hold tasks | Coinbase's 14,000 EDD cases |
| Productive share | Productive (decided and QA-passed) cases per analyst-hour, bank-level only | Never a league table |
| Capacity ratio | Available productive hours over forecast workload | The staffing signal |
| Missed-by-monitoring rate | Cases that started from staff, RFI, authority or media and had no earlier alert | Wolfsberg false-negative assessment |

### AKR-TMS hooks (section 11)

- **Capacity workbench**: workload by class from measured handling times, staffing and occupancy inputs, shrinkage settings, a fluid backlog projection and a sensitivity table.
- **Alarm service** with the rules above and the **supervisor-notification task**.
- **Surge mode** and the **go-live volume preview** in analyst hours.
- **Metric catalogue** as code (definition, formula, caveat) so the dashboard, board pack and assessor pack share one meaning; no metric ranks individuals by speed (file 29 section 9j).

## 12. Evidence and records

### 12.1 What the case file contains

| Part | Content | Mutable? | Where it lives |
|---|---|---|---|
| Header | Case ID, family, party or group, tier, access list, priority class, policy and workflow versions | Status fields change by event only | PostgreSQL, event-sourced |
| Alerts | Each alert with its **evidence snapshot** (triggering transactions, party profile as of the time, list-entry detail, scenario and matcher versions, score and reason codes) | No | PostgreSQL (structured) plus vault object for the snapshot |
| Context pulled during the work | Transaction history windows the analyst viewed, peer comparison, counterparties, screening and adverse-media results, **graph snapshot** | Captured when the analyst "adds to evidence"; no later edit | Vault objects with hashes |
| Documents | KYC documents, customer or branch documents, invoices, contracts, RFI answers, authority letters | No; replaced by a new version, old kept | Vault, object lock |
| Notes and comments | Analyst notes, branch input, second-reviewer comments | Posted notes are immutable; an **amendment** is a new note referencing the old | PostgreSQL events |
| Decisions | Section 6.1 | No | PostgreSQL events |
| Tasks, RFIs, verifications | Clocks, answers (as claims), operations tasks and their verification results | Event history only | PostgreSQL |
| Reports | Drafts, the filed version, validation results, goAML reference, acknowledgement, FIU feedback | Draft mutable until submitted for approval; filed version locked | PostgreSQL plus vault (XML and rendered copy) |
| AI provenance | For each assistant output used: user, model and version, prompt template ID and version, retrieved record IDs, output, human edits and final text | No | Audit store (suspicion-tier class) |
| Access history | Who opened the case, evidence and reports, when, from where, with what purpose | No | `audit.access_event` (file 30 section 9.2) |
| Seal | Hash of the case and attachments at closure; anchor reference | No | Audit chain and anchors bucket |

### 12.2 Immutability and versioned edits

- **Events, not rows.** The case is a stream of events (the DBS `instance_events` pattern); the current state is a projection. Nothing is updated in place that matters to a decision.
- **Drafts are not evidence.** A "save draft" holds unsent text; only **submit** creates a record. The mockup's "Submit decision / Save draft" pair therefore maps to two stores.
- **Corrections are new records.** A note, decision or report field that was wrong is corrected by an amendment that references the original and states why; the original stays visible with an "amended by" mark. This is the opposite of the editing found in enforcement files: the CCO deleting references to alert caps from a CEO presentation (US Bancorp), minutes edited to remove a decision (Metro Bank), "nearly 400 documents" falsified (Canaccord) (file 20 sections 2.2, 6.3, 2.7).
- **Time and author are system facts.** UTC from synchronised clocks; the author is the authenticated user, never a typed field; uploaded files keep their received time and hash.
- **Closed means append-only**; a reopening is an event; the closing seal is re-computed after a reopen and both seals are kept (file 30 section 9.5).

### 12.3 Attachments in the vault

The evidence vault uses the DBS port with an object-locked store certified by a WORM conformance suite in CI and at install (file 32 section 12; the upstream MinIO repository is archived, and the user decided on 2026-10-07 to keep MinIO as a pinned, maintained and renamed build (Silo, Chainguard's fork as second source), with SeaweedFS and RustFS as tested exits only). Rules for the case:

1. **Lock at the floor, extend at closure**: objects are written with COMPLIANCE retention equal to the floor (seven years in the Iraq profile, M-05; five under Law 39) because COMPLIANCE retention cannot be shortened; at closure it is extended to the class period (default ten years); legal hold uses the store's hold flag (file 30 section 12.4 item 5).
2. **Hash on write and on serve**: SHA-256 in the manifest (size, type, classification, store version ID); the viewer verifies on every fetch.
3. **Antivirus and card guard** on upload: quarantine bucket first; a full card number or sensitive authentication data in a document is refused or masked (file 30 section 11.5).
4. **Access follows the case**: an evidence item inherits the case's access list; presigned URLs are short-lived and bound to the user; every fetch is logged (file 30 section 8.2 row 7).
5. **Language tags and summaries** for documents not in the reviewing unit's working language (Arabic, English, Kurdish), with a required short summary (file 20 L-68).
6. **Structured over screenshot**: the product prefers capturing the structured data behind a view (a transaction window, a graph definition with layout) to a screenshot, because it can be searched, hashed and re-rendered; screenshots are allowed as attachments.

### 12.4 Records law that the case file must satisfy

| Source | Requirement | Case-file design |
|---|---|---|
| Law 39 of 2015 art. 11 and CBI ch. 4 (file 30 section 2.1, file 01 section 2d) | Keep for at least five years from the end of the relationship, account closure or occasional transaction, whichever is longer; copies of STRs and related documents at least five years from the report date or a ruling; records of unusual-transaction reviews and of cases the MLRO closed; securely, with backup in another location; retrievable "easily and quickly" | Retention classes R5 to R7 with floors; backup per file 30; search and export (12.6) |
| **Law 78 of 2012** (Electronic Signature and Electronic Transactions Law, Iraqi Official Gazette no. 4256, 5 November 2012; Arabic text read from the gazette scan, archived copy of the Iraqi Legal Library for Local Governance, 2026-10-07; grade A for the text) | Art. 13(1): electronic documents and electronic writing have "the legal weight of their paper equivalents" if (a) the information can be saved and stored so that it can be retrieved at any time, (b) it can be kept in the form in which it was created, sent or received, or in a form that makes it easy to prove the accuracy of the information, so that it cannot be modified by addition or deletion, and (c) it indicates who created it and who received it and the date and time of sending and receipt; art. 13(3): the parties may prove validity of an electronic document by all means of proof provided by law; art. 14: a copy of an electronic document has the status of the original if the data are identical, the electronic document and signature exist on the medium, it can be stored and retrieved, and it shows sender, recipient and date and time; art. 17(2): an electronic document is considered authenticated from its creation date if it was not subject to modification, unless the contrary is proved | The three conditions map directly: retrievable (vault and search), unmodifiable with provable accuracy (object lock, hashes, append-only events, sealing), origin and time (system author and UTC). Exports carry a verification manifest and the MLRO's certification. Art. 3(2) excludes court procedures and documents that law requires to be notarised from the law's scope. **NOT VERIFIED:** how Iraqi courts treat bank-generated case records in practice and whether the law has been amended; a legal review per bank is required |
| Disclosure register (file 30 SP-52) | Every transfer of customer data outside the bank is registered with requester, legal basis, fields and approver | Every export, authority response and goAML submission writes an entry |

### 12.5 Retention and legal hold

Case objects use class **R6** (cases and STRs: clock starts at the later of closure and report date; floor seven years in the Iraq profile (five under Law 39; M-05) from the report date or ruling; default ten) and alerts **R5**; screening decisions **R7**; configuration history **R9** outlives every alert it explains; audit **R10** unlimited (file 30 section 12.3). The retention engine proposes disposal with a dry run, maker-checker by the data owner and the CISO, a notice period (default 7 days) and detach-then-drop; evidence objects are deleted by lifecycle only after their lock expires. A **legal hold** (source: court, the Office, the CBI or an internal investigation) is itself suspicion-tier information, blocks purge and disposal of everything in scope, and is listed only to the MLRO, legal counsel and the CISO with a periodic review date (file 30 section 12.5). **Opening a case freezes the evidence snapshot** so that dropping old monitoring partitions never damages a case (file 30 section 12.2, SP-54).

### 12.6 Export for the assessor, the supervisor and the courts

**For the CBI-approved external assessor (standard D2).** The assessor tests "rule logic, alert generation, communication and reporting paths between the parties concerned, and the review and improvement mechanism", examines "how suspicious transactions are handled", and evaluates alert thresholds, user access permissions, audit-trail integrity and the traceability of the data used to detect and report (CBI Detailed Assessment Guide 2025, D2.1 assessment process; section 2.5). The case system supplies:

| Assessor test | Evidence the product generates |
|---|---|
| Rule logic and alert generation | Scenario register and versions, alert counts and latency per scenario, reconciliation and coverage reports (files 25, 27); the alert register from which the assessor may draw a random sample |
| The investigation workflow | The state machine and clock definitions (section 5), the delegated-authority matrix, routing table and reason-code libraries as versioned exports |
| Case walk-throughs | For each sampled alert, an **end-to-end trace**: alert and snapshot, grouping, every task and decision with times and actors, RFIs and answers, report and acknowledgement, QA record, audit events; delivered through a **time-boxed, named, read-only account** whose every action is logged (file 30 section 7.2 row "CBI or external inspector") |
| User permissions | The generated access matrix (file 30 SP-20), role-grant history, access reviews, break-glass log |
| Audit-trail integrity | The chain verifier report: chain, Merkle roots, signatures per key version, existence and retention of the locked copy (file 30 SP-40); a re-run by the assessor on its own schedule |
| Data traceability | Lineage from CBS column to scenario input and from alert to source rows (file 25, file 20 L-08) |
| Effectiveness | The metrics of section 11.5 and QA results with intervals; shelved-decision register |

**For courts, prosecutors and the Office.** An **evidence pack** per request: a PDF/A rendering of the case file with Arabic shaped correctly (HTML templates through the Gotenberg image that carries Arabic fonts; the DBS records that pure-Rust PDF libraries cannot shape Arabic, file 32 section 18.2), the structured data as JSON and CSV, the attachments, a **manifest** with SHA-256 for every item, the **chain-of-custody log** (who generated the pack, when, on whose approval), a verification page and QR code that points to the bank's internal verifier, and a **certification statement** signed by the MLRO with a single-use step-up. The pack carries the requester's watermark, writes a disclosure-register entry and needs an approval for suspicion-tier content (file 30 section 8.2 row 6). A **redaction** tool removes third-party data (other customers' names) from the rendered copy with a redaction log; the unredacted original stays in the vault. Chain-of-custody practice follows the general forensic principles (collection, examination, analysis, reporting, with documented handling), for example NIST SP 800-86 (August 2006, grade A as a general guide; not Iraqi law).

**Verification without the application.** An offline **verifier tool** takes a pack and checks every hash against the manifest and, where the receiving party has access, against the anchored chain; this is how a court expert or the assessor checks integrity without trusting the application's own screen.

### AKR-TMS hooks (section 12)

- **Evidence snapshot service** at case open and at "add to evidence".
- **Vault port** with WORM conformance suite, hash-on-serve and class-based retention extension.
- **Case sealing** and **amendment model** for notes, decisions and report fields.
- **Assessor workspace**: sample draw, end-to-end trace, time-boxed read-only account, generated packs.
- **Evidence pack builder** with manifest, chain of custody, MLRO certification, redaction and an offline verifier.
- **Legal-hold registry and hold store** reused from file 30.

## 13. Access and confidentiality in the case system

### 13.1 Who sees which part (default matrix)

"Y" sees; "L" limited; "-" none. The controls behind each cell are in file 30 sections 7 and 8; this table shows what the **case** exposes. Suspicion-tier cases (a case at "STR candidate" or later, any filed report, any authority request) additionally need membership of the case's access list.

| Role | Alert list | Normal-tier case | Restricted-tier case | Suspicion-tier case | Decision text | Existence of a report | Evidence | Access history |
|---|---|---|---|---|---|---|---|---|
| Liaison officer | L (own branch, reason codes without thresholds) | L (own submissions and neutral status) | - | - | - | - | L (branch documents) | - |
| Level-1 analyst | Y (assigned or pool) | Y | - | - | Own | - | Y (assigned) | - |
| Level-2 analyst | Y | Y | If assigned | If on list | Y | If on list | Y | - |
| MLRO and deputy | Y | Y | Y | Y | Y | Y | Y | Y |
| Compliance head | Y | Y | Y | If on list | Y | Y | If on list | Y |
| QA reviewer | Sampled items | Sampled items | If sampled and cleared | Sampled, time-boxed | Y | Y | Sampled | - |
| Compliance configuration manager | Aggregates | - | - | - | - | - | - | - |
| Internal auditor (time-boxed, scope-based) | Y | Y | If scope grants | If scope grants | Y | Y | If scope grants | Y |
| CBI or external assessor (named, time-boxed, read-only) | Y | Y | If scope grants | If scope grants | Y | Y | If scope grants | Y |
| Platform administrator, DBA, CISO, vendor support | - | - | - | - | - | - | - | CISO: Y (metadata) |
| Branch manager, relationship manager | - | - | - | - | - | - | - | - |

### 13.2 What the branch sees

CBI ch. 11 makes the liaison officer the MLRO's assistant and says "branch management has no part in it". To prevent inference, the branch window uses a **neutral status vocabulary** that does not change with the outcome:

| Internal state | Shown to the liaison officer |
|---|---|
| `new`, `central_review`, `investigation`, `mlro_decision`, `report_*`, `filed`, `actions_open`, `watching` | "Received by the central department" |
| An open branch RFI | "Information requested from the branch" |
| `closed` with any outcome (no report, UTR, STR, TFR) | "Reviewed; no further action by the branch" |
| `reopened` | The same as the state it returns to |

The liaison's **monthly report** (CBI ch. 11: cases reported and latest action) is drafted automatically from the officer's own submissions and actions and lists the same neutral statuses; the MLRO reviews and signs it. The MLRO's decision notice to a **staff member who filed an internal report** (CBI ch. 2 art. 3: written acknowledgment with the tipping-off reminder, and written notice of the decision) uses default neutral wording: "your report was examined and the decision recorded; the confidentiality duty continues". Whether the notice may state the outcome is the MLRO's configuration and a legal-review point for each bank.

### 13.3 Conflicts of interest

- **At assignment**: a user is never assigned a party with whom the conflict check finds a link (same family number or identifier, shared contact, listed related party, own or relative's account, same employer, manages the account); the case goes to another person and an alert goes to the MLRO and the CISO, with the reason visible to them only (file 30 section 8.5).
- **At open**: the same check runs on every open of a restricted or suspicion-tier record; a conflict denies access as "not found".
- **Declared conflicts**: staff may declare a conflict from the task; it reassigns without a reason being shown to colleagues.
- **Restricted tier** (staff, relatives, board, PEPs, VIP): compliance-only queue; level-1 analysts never see them; the staff-account monitoring path is separate from the staff member's manager.
- **The reporter's identity** (a staff member who raised an internal report) is protected: the case shows "internal report" and the reporter's name only to the MLRO desk (file 30 section 8.2 row 10).

### 13.4 Tipping-off controls specific to cases

All 14 leak paths of file 30 section 8.2 apply. In the case system the practical rules are: suspicion-tier cases and flags do not appear in global search, type-ahead, counts or dashboards for non-members; the case has its own comment thread with the same access list so people do not use e-mail or chat; notifications are generic with no name, amount or case type; operations tasks carry only neutral instruction and reference; exports need step-up, purpose and approval and carry a per-user watermark; denial looks identical to "not found" (the response is byte-identical); and the **inference test suite** tries to discover a suspicion-tier case by search, counts, timing, errors, exports, notifications and the audit viewer and fails the build if it can (file 30 section 8.9, SP-33).

### AKR-TMS hooks (section 13)

- **Visibility policy** (Cedar) generated from the matrix; a test that no cell can be widened without a change request.
- **Neutral status mapper** for the branch window and staff notices.
- **Conflict service** in assignment and open paths; **declared-conflict** action.
- **Case-level comment thread** and notification templates tested for forbidden words.

## 14. GenAI in investigations

Files 21 (section 12: nine use cases, evidence, regulators, evaluation protocol) and 29 (section 9k: controls; AMLR Article 76(5)) own the substance. This section places the assistant inside the case lifecycle and sets the interface between assistant and case.

### 14.1 Where the assistant may act

| Stage | Assistant task | Mode (file 29 tier) | Not allowed |
|---|---|---|---|
| Intake and triage | Group explanation ("why these alerts belong together"); duplicate detection suggestions | Suggest (3) | Re-ranking P0 or P1 classes; hiding alerts |
| Evidence gathering | Parameterised, read-only queries against a whitelisted analytic schema (transaction windows, counterparties, earlier alerts and outcomes); result tables with source rows | Retrieve (2-3) | Free SQL; reading raw tables into a prompt; any write |
| Summary | A case brief: profile, declared versus actual, pattern, counterparties, prior outcomes; every sentence linked to source rows, numbers verified by code | Draft (3) | Statements without a source link |
| Next-best action | Suggestions such as "request documents through the branch", "check the counterparty's other relationships" | Suggest (3) | Executing any action |
| Disposition recommendation (level-1) | A recommended outcome and the checklist items it relies on | **Recommend** (2), staged release | Closing, escalating or hibernating by itself |
| RFI | Drafting free-text parts of answers where the 30 Wolfsberg questions are pre-filled deterministically | Draft (3) | Sending |
| Narrative | A UTR or STR narrative in the Office's who-what-when-where-how structure, Arabic and English, from the case record only | Draft (3) | Filing; unsupported claims; suspicion language aimed at the customer |
| Translation | Arabic, English and Kurdish summaries; names and numbers **copied** programmatically, never regenerated | Draft (3) | Rewriting names or amounts |
| QA aid | Flagging closures with generic text, missing evidence or fast closure for sampling | Triage (3) | Acting as the QA reviewer |

Autonomous closure is excluded from phase 1 and from the default configuration after (file 21 section 9b, file 29 section 9k). The position is consistent with the CBI texts (an MLRO decision with reasons), the EU AMLR Article 76(5)(b) ("meaningful human intervention" for any decision to enter, refuse or maintain a relationship or to change the extent of due diligence), and the one regulator-compiled production figure of 7 percent auto-closed (file 12 section 8.4).

### 14.2 The case-side contract for assistant output

1. **Provenance record** on every output used: user, case ID, time, model name and exact version, prompt template ID and version, retrieved record IDs, output, the human's edits and final text; stored as suspicion-tier audit data with the case's retention (file 29 section 9k; file 30 SP-94).
2. **Draft status is a state**: output is stamped "AI-generated draft" and cannot be saved to the case, exported or sent until a human edits or accepts it; the saved text records who is responsible. The mockups show this as the badge "DRAFT · REQUIRES REVIEW" and "DRAFT BY ASSISTANT · EDITED BY ANALYST".
3. **Grounding check**: a deterministic verifier re-reads every number, date, name and account reference in the output against the supplied records and removes or marks anything that does not appear (file 21 section 12a use case 1).
4. **Tipping-off guard**: outputs inherit the highest tier of their inputs; no path leads from an output to a customer-facing channel; narrative text is scanned for phrasing that would reveal an investigation if copied into an RFI or a customer script.
5. **Untrusted text**: free-text fields (remittance purpose, beneficiary names, KYC notes, document text, RFI answers) are passed as delimited data, never as instructions; the assistant has no tools that write; output is treated as untrusted until displayed through the verifier (file 29 section 9k, OWASP LLM01/LLM05).
6. **Kill switch** per task and global, two-person, falling back to rules-only and manual drafting (file 30 SP-95).
7. **Model on the bank's premises**; no suspicion content to an external API; Arabic-capable open-weight models exist (Jais, ALLaM) but their quality on Iraqi dialect and compliance writing is untested, so a bank-specific evaluation decides (file 21 section 12f).

### 14.3 Fighting automation bias in the interface

The decision panel is designed so that the assistant cannot quietly become the decider (file 29 section 9j; evidence in file 31 section 3):

- **Evidence before suggestion** for P0 and P1 classes: the recommendation appears after the analyst has opened the required evidence panels, and the analyst enters an initial assessment first when the bank enables "assess first" mode (a cognitive forcing function).
- **Confidence is never a bare number**; the recommendation lists the checklist items and source rows it used.
- **Disagreement is cheap and logged**: overriding the suggestion costs one click and a reason code; override and acceptance rates are monitored per task, not per person.
- **No speed league tables** and no "accept all" (file 29 section 9j).
- **A random QA sample of assisted dispositions never goes to zero** (10 to 20 percent of LLM-assisted dispositions re-reviewed, file 21 section 12e).

### 14.4 Evaluation and monitoring (applied to cases)

The protocol of file 21 section 12e is the gate: a golden set of at least 200 anonymised, expert-annotated cases per use case in Arabic and English, built from **QA-confirmed** decisions; measures (factual error, omission, unsupported inference, critical-error rate, agreement with senior reviewers); shadow mode for a defined period; staged release (summary, recommend, pre-fill); regression on every model, prompt or retrieval change; time saved measured **including review time**. With zero critical errors in n reviewed alerts the 95 percent upper bound on the critical-error rate is about 3 over n (300 alerts bound it near 1 percent), the same arithmetic as the QA table in section 10.2. In production the dashboard shows, per task: use rate, acceptance rate, edit distance (how much humans change drafts), override rate and reasons, factual-error findings from QA, critical findings, and the date and sample of the last audit (file 13 VB-51).

Vendor time-saving claims should not be used as planning figures: Lucinity "3 hours to 30 minutes" (vendor-reported, one customer), Flagright "4 minutes versus 38" (vendor), Oracle "over 80 percent reduction in investigation time" (vendor), against the HKMA's regulator-compiled "1 to 3 hours less work per investigator per day" at a bank that auto-closed 7 percent (file 13 section 10.3). The product reports its own measured numbers.

### AKR-TMS hooks (section 14)

- **Assistant gateway** in the case context: task registry (summary, query, narrative, recommend, translate), tiers, autonomy ladder, kill switch, per-task evaluation status.
- **Provenance store** and **verifier** (grounding check) shared by all tasks.
- **Decision-panel modes**: evidence-first, assess-first, recommendation display with checklist items.
- **AI monitoring** panel (use, acceptance, edit distance, override, QA findings) in the governance module (file 29).

## 15. Workflow engine options for the Rust stack

### 15.1 What the case workflow needs

| Need | Detail |
|---|---|
| Long-lived work | Cases live days to years; watches live months; holds live as long as a court needs |
| Human tasks | Pools, claims, push assignment, delegation with windows, leave handover |
| Timers on calendars | Business-day clocks per branch and region, reminders, escalation, deadlines (section 5.6) |
| Guards | Four-eyes, separation of duties, delegated-authority matrix, step-up authentication on signing-grade actions |
| Ad hoc work | An analyst adds an RFI, a document request or a network look-up at any time; the process is not a straight line |
| Versioned definitions | Instances pinned to the definition version they started on (so a decision can be explained under the rules of the day); a record of which version governed each transition |
| Per-bank configuration | Routing, clocks, reason codes, templates, delegated authority, UTR switch, grouping policy, edited by compliance staff on screens, under maker-checker, with a dry run on history |
| Bilingual by construction | Arabic and English labels required to publish (the DBS rule) |
| Audit | Every transition an immutable event with actor, time and versions |
| Footprint | On-premises, one installation per bank on K3s, offline updates, no call-home, a small team that writes Rust (file 32) |
| Security | Tier and branch ACL on every task; neutral notification text; no external push |

### 15.2 Options, with facts read on 2026-10-07

| Option | Facts | Fit | Verdict |
|---|---|---|---|
| **A. Hand-written state machines in Rust** (inside the `case` context; crates such as `statig` 0.4.1, MIT, 4.6 million downloads, last release July 2025; `rust-fsm` 0.8.0, MIT, 1.4 million; `smlang` 0.8.0, MIT or Apache-2.0, 1.0 million, last release August 2024) | Compile-time or table-driven transitions, full control, no new runtime, easy property tests | Excellent for invariants (no path to `filed` without MLRO approval); poor for compliance staff editing flows and for timers and human-task plumbing, which must be built | Use for the **case aggregate's invariants** |
| **B. The DBS workflow engine and DSL** (event-sourced, JSON definitions with steps `form`, `approve`, `sign`, `reply`, `attach`, `acknowledge`, `review`, `parallel`, `terminal` and the `execute` step "carrying out what was decided", versions pinned per instance and immutable once published, timers with business hours and escalation, delegation, break-glass, bilingual labels required, test-run and validate endpoints; DBS docs/DSL.md) | Built, tested, owned by the same team, same stack; the `execute` step already separates "somebody executed" from "somebody approved" in the audit | Excellent for approval flows (closure approval, STR sign-off, change requests, freeze tasks); less natural for ad hoc investigation | Reuse (by copy, as file 32 prescribes) for **approvals and operations tasks** |
| **C. Temporal** (MIT; Go server; persistence and visibility on PostgreSQL 12+ with advanced visibility from server 1.20, no Elasticsearch needed; **Rust SDK 1.0.0 released 2026-09-04, 1.1.0 on 2026-10-06**, MIT; temporalio/temporal 23.5 thousand stars, active) | Durable execution of code-defined workflows; timers, retries, signals; strong for system orchestration | A multi-service server to run per bank; no human-task inbox, no calendars, no UI for compliance staff; the Rust SDK is a month past 1.0; workflow versioning is by code patching | Not for case work; possible later for heavy back-end pipelines |
| **D. Camunda 8 / Zeebe** (repository under the Camunda License 1.0 except listed Apache-2.0 parts; the licence allows "Non-Production" use only and otherwise "you must purchase a commercial license") | BPMN, Java, needs Elasticsearch or OpenSearch | Commercial licence and Java stack; no CMMN | Excluded |
| **E. Operaton** (Apache-2.0 fork of Camunda 7 Community Edition; BPMN 2.0, DMN, Cockpit, Tasklist and Admin web apps; JVM 17 to 25; PostgreSQL; 485 stars; active) | Mature lineage, embeddable, but a JVM service and a second UI stack; ownership of a community fork | Heavy for a Rust-first team; the UIs are in English and not RTL-first (NOT VERIFIED) | Not recommended; keep as a reference for BPMN semantics |
| **F. Flowable** (Apache-2.0; BPMN process engine, **CMMN case engine** and DMN rule engine in Java, JDK 17+; 9.6 thousand stars; Flowable Design modelling is a free cloud account) | The only open engine here with a case-management notation (CMMN: discretionary, ad hoc tasks) | JVM service; Arabic UI work would be ours; adds a second runtime | Not recommended; its CMMN idea is worth copying |
| **G. Conductor OSS, Argo Workflows, Restate, DBOS** | Conductor (Apache-2.0, Java, 32 thousand stars) and Argo (Apache-2.0, Kubernetes batch) are orchestration engines; Restate is Rust but under the Business Source License 1.1 with a use grant that permits internal production deployment; DBOS libraries (MIT) are Python, TypeScript, Go and Java | Orchestration of services, not human case work | Not for this purpose |
| **H. Rust BPMN crates** (`snurr` 0.15.0 reads and runs BPMN 2.0, MIT, 13 thousand downloads; `acts` 0.25.0 workflow engine, Apache-2.0, 27 thousand; `wfrs-engine` last release February 2024; `zeebe` client last release 2022) | Small, young and thinly used | Would make the bank depend on a hobby-scale crate for a regulated process | Excluded |

Camunda 7 Community Edition's end-of-life date is **NOT VERIFIED**: Camunda's own announcement was not reached; Operaton's page describes itself as the successor to Camunda 7 without stating a date.

### 15.3 BPMN, CMMN or a bank-owned DSL

BPMN draws sequence flows and suits approval chains; CMMN (the OMG case notation) models discretionary tasks around a case file and suits investigations; neither is something an Iraqi compliance officer edits unaided. The DBS route is a **JSON DSL with bilingual labels, a validator, a test-run, a visual builder (`@xyflow/react`) and an LLM-authoring walkthrough**, whose definitions are plain documents that can be exported, diffed and read by an assessor. For AKR-TMS the split is:

- **Case aggregate** (Rust code, event-sourced): alerts, notes, evidence, tasks, RFIs, decisions, links; ad hoc tasks at any time; invariants as property tests.
- **Workflow DSL** for the *bounded flows*: closure approval, report approval, operations tasks with verification, RFI release, change requests, access requests, QA review.

This is the two-layer recommendation of file 32 section 11.3, now backed by the option table.

### 15.4 Configurability per bank without losing control

| Configurable (policy pack, JSON, versioned) | Invariants (not configurable) |
|---|---|
| Routing table, branch-first flags, priority classes' membership, queue names, skills | Four-eyes on closure above delegated authority; no self-approval at any level |
| Clocks, calendars, SLAs, ageing buckets, escalation targets | No auto-close in phase 1; autonomy only through the signed-policy ladder with sampling |
| Delegated-authority matrix, QA rates and rubric weights | Closed cases are append-only; decisions are never edited |
| Reason-code libraries, checklists, RFI templates, narrative templates, neutral wording | Tipping-off controls: neutral notifications, existence masking, no external push |
| The UTR tier on or off, watch defaults, hibernation maximum, grouping window | Evidence snapshot at case open; object-lock retention at or above the floor |
| Alarm thresholds, capacity inputs | The audit trail and its anchors; the MLRO's decision cannot be delegated to the assistant |

Every change goes through the generic `change_request` (maker cannot approve, fresh second factor, atomic activation and audit entry, a second approver and a backtest if the change reduces work or alerts; file 30 section 7.5). A **dry run** replays a month of closed cases through the proposed policy and reports changed routing, volumes and clocks.

### 15.5 Versioning

Instances **pin** the definition and policy-pack versions at start and **drain** on them (the DBS rule); publishing never migrates in-flight cases. Long-lived cases (a `watching` case lasts 90 days) may adopt a new pack at their **next transition** if the bank opts in per change, with the switch recorded as an event. The decision record stores the versions in force, so "explain this closure under the rules of that day" is answerable years later (file 29: changes are versioned, reasoned and approved).

### 15.6 Testing the workflow

Property tests on the state machine (no path from `central_review` to `filed` without a recorded MLRO decision and approval; the L1 and L2 users on a case are never equal; the preparer and signer are never equal; a closed case cannot be edited); a **simulation harness** that replays historical alert streams through candidate policies to estimate queue size, clock breaches and analyst hours; golden-file tests for every definition; and the release gate of section 4.2 (ranking tests).

### AKR-TMS hooks (section 15)

- `case` context with a Rust aggregate and property tests; **workflow DSL runtime copied from the DBS** for approvals and tasks.
- **Policy-pack** object with versioning, dry run and pinning; `change_request` for every edit.
- **Simulation harness** for policy changes.

## 16. A concrete design for AKR-TMS case management

### 16.1 Shape

```
                 +----------------------- policy pack (versioned, maker-checker) -----------------------+
                 |  routing  clocks  calendars  delegated authority  reason codes  templates  switches  |
                 +----------------------------------------+---------------------------------------------+
                                                          |
 detection engines                                        v
 (scenarios, screening, scores)       +-------------+   +-----------------+   +-------------------+
 internal reports, observations ----> | alert store |-->| grouping        |-->| case aggregate     |
 authority requests, RFIs in -------> | + snapshot  |   | + triage + route|   | (events, projection)|
                                      +-------------+   +--------+--------+   +----+---------+-----+
                                                                  |                |         |
                              clocks and alarms <-----------------+                |         |
                              assignment and queues <------------------------------+         |
                                                                                    v         v
        branch window (liaison)   analyst workspace   MLRO desk   RFI desk   QA workspace   workflow DSL
                  \                      |                |           |           |        (approvals, tasks,
                   +---------------------+----------------+-----------+-----------+         verification)
                                         |                                                   |
                          assistant gateway (drafts only) --> provenance store               v
                                         |                                      report builder (file 23) -> goAML
                                         v                                                   |
                       evidence vault (object lock) + audit chain + disclosure register <----+
```

### 16.2 Data model (PostgreSQL, `alert` and `case` contexts; names are working titles)

| Table | Key columns | Notes |
|---|---|---|
| `alert` | `id`, `fingerprint` (unique), `source_kind`, `scenario_id`, `scenario_version`, `party_id`, `branch_id`, `created_at`, `data_as_of`, `latency_s`, `score`, `reason_codes`, `snapshot_object_id`, `contribution_outcome` | Immutable except `contribution_outcome`; partitioned by month; fingerprint makes re-runs idempotent |
| `case` | `id`, `family`, `party_id`, `group_id`, `parent_case_id`, `priority_class`, `status`, `tier`, `acl_id`, `opened_at`, `closed_at`, `outcome`, `policy_pack_version`, `workflow_version` | `status` and `outcome` are projections of events |
| `case_event` | `case_id`, `seq`, `at`, `actor_id`, `kind`, `payload`, `prev_hash`, `hash` | Append-only; UPDATE and DELETE revoked; per-case hash chain feeding the global audit chain |
| `case_alert` | `case_id`, `alert_id`, `attached_at`, `attached_by` | Many alerts to one case; merge sets `merged_into` on the case |
| `case_party` | `case_id`, `party_id`, `role` (subject, related, counterparty, reporter) | Network cases list members and per-subject child cases |
| `task` | `id`, `case_id`, `kind`, `assignee`, `pool`, `due_at`, `clock_id`, `state`, `result` | A projection of workflow events (DBS `tasks` pattern) |
| `clock` | `id`, `case_id`, `kind`, `started_at`, `paused_at[]`, `target_at`, `stopped_at`, `calendar_id` | Recomputable from events; one row per clock |
| `decision` | `id`, `case_id`, `alert_id` (nullable), `type`, `reason_codes`, `text`, `decider_id`, `evidence_refs`, `checklist`, `actions`, `ai_involvement`, `versions`, `stepup_id`, `supersedes` | Immutable; corrections reference `supersedes` |
| `note` | `id`, `case_id`, `author_id`, `body`, `at`, `amends` | Immutable; amendments reference the original |
| `rfi` and `rfi_question` and `rfi_answer` | `kind`, `template_version`, `purpose_code`, `legal_basis`, `deadline_at`, `status`; answers as claims (`source`, `corroboration`, `expires_at`) | Section 9 |
| `internal_report` and `authority_request` | Register fields, acknowledgment time, restricted tier | CBI ch. 2 art. 3; file 30 section 8.2 row 12 |
| `watch` | `party_id`, `case_id`, `envelope`, `triggers`, `expires_at`, `review_decision` | Section 7.4 |
| `hibernation` | `party_id`, `scenario_id`, `case_id`, `envelope`, `expires_at`, `created_by` | Section 8.2 |
| `ops_task` | `case_id`, `action`, `neutral_text_code`, `target_accounts`, `effective_at`, `verification_level`, `verified_at`, `result` | Section 7.3 |
| `qa_sample`, `qa_review`, `qa_finding` | Stratum, draw seed, reviewer, rubric scores, findings | Section 10 |
| `report_link` | `case_id`, `report_id`, `kind`, `goaml_reference`, `filed_at`, `ack_at`, `fiu_grade` | Links to the report context (file 23) |
| `evidence_item` | `id`, `case_id`, `object_id`, `sha256`, `classification`, `lang`, `added_by`, `added_at` | Vault manifest rows |
| `policy_pack` | `id`, `version`, `document`, `approved_by`, `effective_from` | Pinned per case |

### 16.3 Event catalogue (names are working titles)

`case.opened`, `alert.attached`, `case.merged`, `case.split`, `case.routed`, `branch.submitted`, `evidence.opened`, `evidence.added`, `note.posted`, `note.amended`, `rfi.sent`, `rfi.answered`, `rfi.extended`, `decision.proposed`, `decision.approved`, `decision.returned`, `case.escalated`, `mlro.decided`, `report.drafted`, `report.approved`, `report.filed`, `report.acknowledged`, `ops_task.created`, `ops_task.verified`, `ops_task.failed`, `hold.requested`, `hold.released`, `watch.registered`, `watch.expired`, `hibernation.created`, `case.closed`, `case.sealed`, `case.reopened`, `qa.sampled`, `qa.scored`, `access.requested`, `access.granted`, `assist.generated`, `assist.accepted`. Every event is routed through the audit chain; a test fails the build if a mutating route lacks its event (file 30 SP-41).

### 16.4 API surface (REST, Axum, OpenAPI, Orval-generated hooks as in the DBS)

| Group | Endpoints (illustrative) |
|---|---|
| Queues | `GET /queues`, `GET /queues/{id}/items` (keyset pagination, saved views), `POST /items/{id}/claim` |
| Cases | `GET /cases/{id}` (includes `affordances`: which actions this user may take now), `POST /cases/{id}/notes`, `POST /cases/{id}/evidence`, `POST /cases/{id}/decisions`, `POST /cases/{id}/escalate`, `POST /cases/{id}/merge`, `POST /cases/{id}/reopen` |
| RFIs | `POST /cases/{id}/rfis`, `POST /rfis/{id}/answers`, `POST /rfis/{id}/release` (second signature) |
| Reports | `POST /cases/{id}/reports` (creates a draft through file 23), `POST /reports/{id}/approve` |
| Operations | `POST /cases/{id}/ops-tasks`, `POST /ops-tasks/{id}/complete` (operations side, neutral view) |
| Branch window | `GET /branch/{id}/items`, `POST /branch/items/{id}/submit`, `GET /branch/{id}/monthly-report` |
| QA | `POST /qa/samples`, `POST /qa/reviews`, `POST /qa/bulk-reopen` |
| Assessor | `GET /assessor/trace/{alert_id}`, `POST /assessor/packs` |

Rules carried over from the DBS: every mutating call needs an `Idempotency-Key` and `If-Match` on the version; the API returns **affordances** instead of letting the browser re-derive permissions; a refused request never looks like an empty list; machine IDs are never printed to a reader (file 32 section 18.1).

### 16.5 Jobs (in `akr-worker`)

Grouping and triage; clock sweeper and alarm service; assignment; sampling; sealing; hibernation and watch expiry; retention and deletion certificates (file 30); liaison monthly report drafting; capacity forecasting; verification polling for operations tasks; assistant tasks with their evaluation gates.

### 16.6 Screens (file 31 specifies them)

Work queue; case workspace with decision panel; screening review; network case view; RFI desk; MLRO desk; branch window; QA workspace; capacity workbench; policy studio; assessor workspace; evidence pack builder; command centre (backlog, clocks, deadlines).

### 16.7 Build order for the first release (phase 1)

| Slice | Content | Done when |
|---|---|---|
| 1 | Alert store with snapshots and fingerprints; grouping; the case aggregate; queues and assignment; reason-code libraries; the decision record; clocks and ageing; the state machine with guards; audit events | An alert becomes a case, is worked by L1 and L2 users, approved by the MLRO and closed with a sealed record; property tests pass |
| 2 | Branch window with neutral statuses, branch RFIs, the liaison monthly report, the internal-report channel with acknowledgment | A liaison officer completes a branch-first alert and the monthly report without seeing any outcome |
| 3 | Operations tasks with verification levels, hold and freeze templates, the watch registry, hibernation | A true screening match produces a freeze task, the freeze and 48-hour report clocks and a verification result from the replica |
| 4 | Report hand-off to the builder (file 23), UTR and STR outcomes, FIU feedback register | A case produces a validated draft and records the goAML reference and acknowledgment |
| 5 | QA workspace, sampling service, calibration kit, bulk reopen | A sample is drawn, scored with intervals and a critical finding reopens a reviewer's items |
| 6 | Capacity workbench, alarms, supervisor-notification task, command-centre feeds | A simulated surge raises the alarms and the notification task |
| 7 | RFI desk for correspondents with pre-fill; assessor workspace and packs; evidence pack builder with verifier | An assessor draws a random alert and receives an end-to-end trace |
| 8 | Assistant gateway (summary first, then narrative, then recommendation) behind the evaluation gate | Shadow-mode results on the bank's golden set are accepted by the MLRO |

**Pilot measurements from week one**: handling minutes per class and per stage; arrivals and throughput; share of cases where branch input changed the outcome; reopen rate; QA failure rate with interval; clock breaches; no-report reason distribution; RFI response times. These replace the assumptions of section 11.2.

### 16.8 Questions for the first banks and the CBI

1. Will the bank adopt the UTR tier immediately, and does its policy treat "enhanced monitoring" as the watch of section 7.4?
2. Which alert classes should be branch-first, and what first-review times do liaison officers actually achieve today?
3. Is a blanket second review of every closure the bank's policy, or a delegated-authority matrix with sampling?
4. Which channel does the Office use for its information requests (goAML request types IRD and IRI, or letters), and is the bank's internal default of three working days acceptable when the request states no period (file 23 section 2.6)?
5. May the decision notice to a reporting staff member state the outcome?
6. What can the bank's CBS expose for verifying holds and freezes (status fields, hold amounts, post-order postings)? This sets the verification level (file 25).
7. Will the CBI-approved assessor accept sample-based QA with intervals as evidence of "quality assurance controls", and does the CBI set a minimum sampling rate?
8. Is the 7-year floor of the reform guide to override the 5-year floor of the controls for case records?

## 17. What it means for AKR-TMS

1. **The Iraqi chain is already a workflow specification, and it now has three outcomes.** The CBI chain (liaison officer, central department, MLRO) is written down; the Office's 2025-2026 guides give the MLRO three choices (report at once, gather more evidence, shelve with documented reasons) and add a UTR tier with enhanced monitoring and a path to an STR. A case system that models exactly this, with reasons recorded at every step, is closer to the rules than any imported "alert to SAR" workflow (NICE, Oracle and others are built around the US SAR cycle, file 08 section 6, file 09 section 6).
2. **The branch is an information source, not a decider.** The branch window gathers explanations and documents, sees neutral statuses that do not change with the outcome, and reports to the MLRO only; branch management sees nothing. This honours CBI chapter 11 and removes the inference path that would tip off a customer.
3. **Cases are customers, alerts stay countable.** Group by resolved party and case family, merge never suppress, keep a contribution outcome on every alert so scenario effectiveness survives grouping. ING's account-level monitoring and US Bancorp's suppression rule are the warnings.
4. **Law first, score second, capacity never in the detector.** P0 (list match, terrorist financing, authority order) outranks every score by rule, as the Office's guide says; within a class a documented, tested score decomposes into visible parts; surplus alerts queue with an age clock; no cap, no staffing-driven suppression.
5. **Authority is a matrix, not a habit.** A signed delegated-authority matrix says who may close what; the tool enforces four-eyes by construction (proposer, approver, L1, L2, preparer and signer are different people) and samples what it did not review. The mockup's "second reviewer must approve any closure" is a policy choice the product should offer, not hard-code.
6. **AKR-TMS decides, records, instructs and verifies; it never executes.** Holds, freezes and exits are tasks with neutral wording, clocks and verification levels (V0 to V3) that depend on what each CBS exposes. The failure mode to design out is a freeze that nobody verified.
7. **QA is assessed, so it must be measurable.** The CBI's D2 text requires "periodic calibration and quality assurance controls" and timely, risk-proportionate investigations. A sample-size calculator, a rubric, reviewer-level failure rates with intervals, bulk reopen, calibration and training-gated queue access turn that sentence into evidence; Coinbase's 73,000 contractor-cleared alerts show the cost of having none.
8. **A backlog is a dated control failure.** Regulators set deadlines, ordered look-backs and appointed monitors; they punish shortcuts harder than backlogs. The product shows arrivals against throughput, forecasts hours before go-lives, alarms the MLRO, and creates the task that records whether the supervisor was told.
9. **Records must meet Iraqi law, not only good practice.** Law 78 of 2012 gives electronic records paper-equivalent weight under three conditions (retrievable; unmodifiable and provably accurate; showing author and time); the vault, hash chain, sealing and time-boxed assessor workspace satisfy them by design. The CBI's D2 guide asks for 7 years, longer than the 5 years of the controls.
10. **AI drafts and ranks; a named human decides.** The assistant sits at the edges (summaries, narrative drafts, translation, RFI wording, QA triage), logs everything, never closes, escalates or files, and the decision panel is built against automation bias. The one regulator-compiled production figure (7 percent auto-closed) is the planning number, not the vendor claims of 75 to 98 percent.
11. **Do not buy a workflow engine for this.** The DBS engine already provides versioned, event-sourced approval flows with timers, delegation and an `execute` step; Rust crates for BPMN are thin; Camunda 8 needs a commercial licence for production; Temporal, Flowable and Operaton add a second runtime and UI stack. Use a Rust case aggregate for invariants and the copied DBS DSL for bounded flows.

**Limits and open items.** (a) No independent figure for minutes per alert was found; the capacity model uses stated assumptions and the pilot measures the real numbers. (b) The Office's information requests are due "within the period it specifies" (Law 39 art. 9/First/b, via file 23); no numeric default was found, so the three-working-day default is a design choice. (c) Whether Iraqi courts accept bank-generated case records under Law 78 in practice is NOT VERIFIED and needs counsel. (d) The UTR guide is a first edition dated 2026; banks may not yet have adopted it. (e) The delegated-authority percentages and clock defaults are design proposals, not rules. (f) Camunda 7 Community Edition's end-of-life date was not reached. (g) The behaviour of liaison officers in practice (time per alert, how often branch input changes the outcome) is unmeasured.

## 18. Derived requirements (M-xx)

Priority: M = must have at go-live of the case module (phase 1); S = should have within the first year; C = could have later. "File" refers to cross-references.

| ID | Requirement | Source |
|---|---|---|
| M-01 | Case outcome set `no_report`, `utr`, `str` (with `tfr` for terrorist-financing reports), each requiring a named decider and structured reasons; the UTR tier is a per-bank switch, on by default, with its watch and its link to a later STR | Law 39 art. 12; Office STR guide (three MLRO outcomes); Office UTR guide 2026; CBI ch. 3 (section 2) |
| M-02 | Store the origin of suspicion on every case (scenario, list match, staff report, authority request, RFI answer, adverse media) and pass it to the report builder; store earlier-report references (goAML reference and date) and link UTR to STR and STR to STR | Office STR guide ("how", earlier report); section 2.3 |
| M-03 | Rank by priority class before score: P0 (list match, terrorist financing or proliferation indicator, authority order, stop-until-cleared request) above P1 to P3, by rule; the class is visible on every row | Office STR guide (priorities); CBI sanctions guidelines (section 4.1) |
| M-04 | Keep a queryable shelved-decision register (every `no_report` with reason codes and evidence) for supervisors and QA; sample it by someone other than the decider | CBI ch. 3 MLRO duty (c); Office STR guide (third line); D2.1(b)(4) (sections 2.1, 6.5) |
| M-05 | Iraq jurisdiction profile sets a 7-year floor for case, alert, decision, RFI and report classes (default 10), showing both sources (Law 39 and CBI controls: 5; D2 guide: 7) in the retention report | CBI Detailed Assessment Guide D2.1(c)(1); file 18 VD-19; file 30 section 12 (section 2.5) |
| M-06 | Use one vocabulary: signal, alert, internal report, authority request, RFI, case, task, decision, report, control incident; control incidents never enter analysts' alert queues | Section 3.1 |
| M-07 | Every alert carries an evidence snapshot taken at creation and a stable fingerprint so re-runs are idempotent; records that cannot form an alert go to the quarantine queue, never lost | File 30 section 12.2; file 25 L-05; file 20 L-13 (section 3.6) |
| M-08 | Grouping service: one open case per party per case family, attach at any age, reopen within N days instead of opening in parallel, related-party groups only from evidenced links, merge never suppress, a dry run for policy changes and a ledger of merges | ING (file 20 section 4.3); HKMA circular 2024; SAS; Marble workflows; file 20 L-17, L-26 (section 3.3) |
| M-09 | Keep a contribution outcome per alert and mark outcomes inherited from the case decision, so per-scenario productivity survives grouping | Section 3.3; file 21 section 9c |
| M-10 | Network cases: a parent with cluster summary and subgraph snapshot, one child case per subject with its own decision, parent closes when all children have outcomes | Mockup 06; file 21 section 6; section 3.5 |
| M-11 | Record data latency and handling time as separate clocks from creation; the engine has no alert cap; surplus alerts queue with an age clock; a flat-line detector flags scenarios with an unchanged count | Mashreq (file 20 section 3.5); file 20 L-16, L-29, L-31 (section 3.6) |
| M-12 | Model screening alerts as (party, list entry) pairs with their own queue, clocks and state machine; they become cases only on escalation or confirmation | File 22 section 13.8; CBI sanctions guidelines (sections 3.4, 5.5) |
| M-13 | Score within a class from displayed components (typology, deviation and size, customer risk, counterparty and corridor, network exposure, history, data confidence, age, ML score when gated); include the EUR 100 versus EUR 99 million ordering test in the release gate | ING (file 20 section 4.3); file 20 L-27; file 21 section 9b; Uetz et al. 2026 (section 4.2) |
| M-14 | Versioned routing table (queues, branch-first flag with the five conditions, tipping-off flag default, skills); the liaison officer sees reason codes without thresholds | MER para 567; CBI ch. 11; file 21 section 11b (section 4.3) |
| M-15 | Assignment service: pool pull for L1 and specialist queues, push for L2, MLRO and compliance-only; queue limit (default 10); time off; skills and language; conflict check at assignment; sign-in blocked during leave with mandatory handover | Marble auto-assignment; DBS claim and delegation; file 30 sections 6.5, 8.5 (section 4.3) |
| M-16 | Age raises priority; a must-touch-by date prevents starvation; a daily forced sample of low-scored alerts goes to review | File 21 section 9b (section 4.3) |
| M-17 | No screen, report or alarm ranks individuals by speed or closures per hour; productivity is bank- and queue-level only | File 29 section 9j; US Bancorp (file 20 section 2.2) |
| M-18 | Implement the statuses and transitions of sections 5.2-5.3 as events with actor, time, reason, evidence references and workflow version; the status is a projection; guards are enforced in the backend and the API returns affordances | DBS `instance_events` and affordance rules (file 32 section 11.1); section 5.1 |
| M-19 | Four-eyes by construction: proposer and approver differ; L1 and L2 on one case differ; preparer and signer differ; Cedar `forbid` plus a database constraint; a failing property test blocks release | Mashreq (file 20 section 3.5); file 20 L-54; file 30 section 7.4 |
| M-20 | A delegated-authority matrix (who may close or approve what, by class), signed by the MLRO under maker-checker, versioned, enforced by guards and printed in the assessor pack; the bank may choose blanket second review instead | CBI ch. 3; MER para 567; section 5.4; file 30 SP-22 |
| M-21 | For configured classes (default P0 and P1) decision actions are enabled only after the required evidence panels were opened; the log of opened panels is stored on the decision | File 29 section 9j; section 5.3 |
| M-22 | Clock service: start, stop, pause and target stored for every clock in section 5.6; elapsed age is never paused; handling clock pauses only while awaiting information; a maximum stay in that flag; breach actions at 50, 100 and 150 percent | D2.1(b)(4) (timely, proportionate to risk); Block (file 20 section 8.2); section 5.6 |
| M-23 | Business calendars per bank, region and branch (default Sunday to Thursday), holidays editable under maker-checker, Islamic holidays entered when announced; timers use the assignee's calendar | DBS DSL section 4; Wikipedia (Iraq calendars); section 5.6 |
| M-24 | Screening alert states in the Office's terms (true, potential, incorrect match) with the CBI's terms as aliases and a per-bank doubt policy; second review within a day for freeze-list exclusions with the reason recorded; freeze task with an 8-hour default clock, first Committee report within 48 hours, second at two months; verification; reported | Office TFS guide art. 12 (Sept 2026); Regulation 6/2023; CBI sanctions guidelines 2023; file 23 section 3.4; file 25 C-60; file 30 SP-98 (sections 3.4, 5.5) |
| M-25 | Liaison monthly report: auto-drafted by the 10th from the officer's own submissions with neutral statuses, signed by the liaison officer, reviewed and signed by the MLRO; overdue branches on the command centre | CBI ch. 11 and ch. 3 (MLRO duty j); section 13.2 |
| M-26 | Decision record with the fields of section 6.1 (decider, type, reason codes, text, evidence considered and opened, escalation checklist, origin, typology, subjects, actions, earlier reports, AI involvement, second opinion, step-up ID, versions); immutable; corrections reference the original | CBI ch. 3; Metro Bank (file 20 section 6.3); file 29 section 9k (section 6.1) |
| M-27 | Governed reason-code libraries for `no_report` and report typologies, in Arabic and English written by compliance staff; closures with capacity-driven or generic text are rejected; reason distributions are reported to the second line | CBI ch. 3; file 20 L-16 to L-18, L-54; Office indicator lists (section 6.2) |
| M-28 | Embed the seven CBI escalation criteria as a checklist in the decision panel, pre-filled where computable; one "yes" forces a recorded reasoned decision | CBI circular 26 Aug 2026 (section 6.3) |
| M-29 | UTR versus STR decision aid showing the Office's definitions and accumulation prompts; the answers are recorded; the choice is the MLRO's | Office UTR guide (section 6.4) |
| M-30 | `ReportRequest` and `ReportResult` contract with the report builder; filed report locked, amendments versioned; FIU grade creates a rework task and a feedback-register entry; pre-submission check results stored on the decision | File 23; file 13 VB-43, VB-49; MER STR grading (section 6.6) |
| M-31 | A decision-to-filing clock (same-day target, escalation at 24 hours, aligned with file 23 section 8.10) and a first-alert-to-filing distribution (median, 90th percentile, maximum) per scenario | Law 39 art. 12; Block; N26 (file 20 sections 8.2, 7.1) |
| M-32 | Action catalogue with task templates for hold, release, freeze, restriction, exit, information to units and authority response; AKR-TMS never writes to the CBS; a neutral-wording library tested for forbidden words | Brief; file 30 SP-31 (section 7) |
| M-33 | Operations task contract: authority, clock, completion with the CBS reference, verification levels V0 to V3 with the achieved level stored and reported per bank, failure incident to the MLRO | File 25 C-60 (section 7.3) |
| M-34 | On a true list match, create the freeze task with the freeze clock (default 8 hours) and the Committee report clocks (48 hours and two months), and verify the block in place and no debit after the order time; the earlier three-day clock of file 25 C-63 is superseded | Regulation 6/2023 art. 4; Office TFS guide; file 23 section 3.4; file 25 C-60 (sections 5.5, 7.2) |
| M-35 | Watch registry with envelope, triggers, expiry (default 90 days) and a review decision at expiry (extend, close, upgrade UTR to STR) | Office UTR guide; file 21 section 9b; file 05 section 1b (section 7.4) |
| M-36 | Exit proposal workflow with senior-management approval, individual assessment (no group exit), and a reversal flag to the board committee | CBI ch. 2 (senior approval); file 20 L-57; file 29 design rule 6 (section 7.2) |
| M-37 | Authority-response workflow with register entry, draft from system facts, MLRO approval and a disclosure-register entry | File 30 SP-52; file 04 E-01, E-07 (section 9.2) |
| M-38 | Closing sequence: outcome and reasons fixed, actions verified or waived with a second person, case sealed with hashes in the audit chain, retention class and clock set, QA sampling applied, watch started where relevant | File 30 sections 9.5, 12; SP-38 (section 8.1) |
| M-39 | Hibernation only inside a case by a human, default 90 and maximum 180 days, stored envelope, ledger and regret metric, scenario keeps running and logging, MLRO approval for classes that need it | File 21 section 9b; Marble snoozes (section 8.2) |
| M-40 | Reopen service with the triggers of section 8.3, bulk reopen by reviewer and date range, and reopen-rate reports | Coinbase (file 20 L-53); section 8.3 |
| M-41 | Record the age of the underlying activity at filing and report the share older than N months | FCA 2026 (file 05 section 3b); section 8.5 |
| M-42 | RFI object with six kinds and versioned templates (purpose code, legal basis, minimal questions, response format, clock, release rule); free text scanned for forbidden words | Wolfsberg RFI guidance (file 04 section 2p); Salv (file 13 section 7); section 9 |
| M-43 | Branch RFIs go to the liaison officer only and escalate to the MLRO desk, never to branch management; relationship managers are contacted only when the party's flag allows | CBI ch. 11; Iraqi MER (tipping-off) (section 9.7) |
| M-44 | Correspondent RFI desk: log, pre-fill the Wolfsberg questions from the replica, deadline clock (default 10 business days), four-eyes release, tipping-off flag on revealing fields, "missed by our monitoring" signal | File 04 W-23; Wolfsberg RFI (section 9.4) |
| M-45 | Store RFI answers as claims with source, date, corroboration status and expiry; a P1 or P0 closure cannot rest on an uncorroborated explanation | File 20 L-47, FM-21 (section 9.6) |
| M-46 | RFI KPIs: median and 90th-percentile time to answer, share by deadline, follow-up rounds, top question types, missed-by-monitoring rate | File 04 section 2p; CBDDQ Q88 and Q89 (section 9.6) |
| M-47 | Sampling service with strata, random and targeted modes, a sample-size calculator (n = ln(1 - C) / ln(1 - p), hypergeometric for small populations) and an immutable sample register | File 21 section 12e; section 10.2 |
| M-48 | QA workspace with the rubric of section 10.3, findings, coaching tasks, reviewer-level rates with Wilson intervals, and bulk reopen on a critical finding | Coinbase, Mashreq (file 20); D2.1(b)(5) (section 10) |
| M-49 | QA independence: reviewer differs from the closer, time-boxed per-case grants for sampled items with conflict check and logging, MLRO decisions sampled by the compliance head or internal audit | File 30 sections 8.4, 8.5, 9.2; section 10.4 |
| M-50 | Calibration kit (golden cases, inter-rater agreement), queue access gated by recorded training and supervisor sign-off, and an optional seeded-case programme with the safeguards of section 10.6 | File 20 L-53; D2.1(b)(4) (qualified staff); section 10.6 |
| M-51 | Feedback loops from QA to coaching, procedure gaps, tuning, the AI golden set (QA-confirmed cases only), the training register and the board pack | File 21 section 12e; file 13 VB-39 (section 10.5) |
| M-52 | Capacity workbench: workload by class from measured handling times, staffing, shrinkage and occupancy inputs, fluid backlog projection, sensitivity table | File 20 L-52; section 11.2 |
| M-53 | Alarm service for ageing, growth, latency, silence, capacity forecast, QA, missing liaison reports, and a supervisor-notification task with a record of whether and when the supervisor was informed | Coinbase (file 20 section 8.1); section 11.4 |
| M-54 | Surge mode: pre-clear known false positives from pair memory, temporary pool, higher QC sampling, hourly progress to the MLRO | File 05 section 3d; file 20 L-53 (section 11.3) |
| M-55 | Show the expected alert-volume change in analyst hours before activating a rule set, source or product; shadow run first; any volume-reducing change carries a reason code, a second approver outside operations and a below-the-line test | Block, Coinbase (file 20); file 20 L-18; file 30 section 7.5 (section 11.2) |
| M-56 | Metric catalogue as code (definition, formula, caveat) shared by dashboard, board pack and assessor pack; the board pack is generated from system facts with manual edits shown | File 20 L-56; D2.1(a)(2) (section 11.5) |
| M-57 | Per-analyst work-in-progress limit (default 10) | Marble; section 11.3 |
| M-58 | Evidence snapshot at case open and at "add to evidence"; prefer structured data to screenshots | File 30 SP-54 (section 12.1, 12.3) |
| M-59 | Posted notes and decisions are immutable; amendments reference the original; drafts are not evidence; author and time are system facts | US Bancorp, Metro Bank, Canaccord (file 20); Law 78 art. 13 (section 12.2, 12.4) |
| M-60 | Vault rules: lock at the floor and extend at closure, hash on write and on serve, antivirus and card guard, inherited access list, language tags and summaries | File 32 sections 12.4-12.5; file 30 sections 11.5, 12.4; file 20 L-68 (section 12.3) |
| M-61 | Case sealing job (hashes into the audit chain) with re-seal on reopen, keeping both seals | File 30 section 9.5, SP-38 (section 12.2) |
| M-62 | Assessor workspace: random sample draw from the alert register, end-to-end trace, named time-boxed read-only account, generated pack mapped to the D2 tests | CBI Detailed Assessment Guide D2.1 assessment process; file 18 section 12.2 (section 12.6) |
| M-63 | Evidence pack builder: PDF/A (Arabic shaped by the Gotenberg image), JSON and CSV, attachments, SHA-256 manifest, chain-of-custody log, MLRO certification with step-up, redaction with log, requester watermark, disclosure-register entry, and an offline verifier | Law 78 arts. 13, 14; NIST SP 800-86; file 32 section 18.2; file 30 SP-52 (section 12.6) |
| M-64 | A statement, produced per bank, that maps the three conditions of Law 78 art. 13(1) to the product's controls, with an explicit legal-review item for the bank's counsel | Law 78 of 2012 (section 12.4) |
| M-65 | Generate the visibility policy from the role matrix of section 13.1; a test fails if a cell can be widened without a change request; suspicion-tier cases are absent from search, counts and dashboards for non-members | File 30 sections 7, 8; SP-19, SP-27 |
| M-66 | Neutral status mapper for the branch window and a configurable, neutral default text for notices to staff who filed internal reports | CBI ch. 11 and ch. 2 art. 3 (section 13.2) |
| M-67 | Conflict service at assignment and at open (restricted and suspicion tiers), with MLRO and CISO alerts and a declared-conflict action that reassigns without a visible reason | File 30 section 8.5, SP-29 (section 13.3) |
| M-68 | Case-level comment thread with the case access list; generic notifications; reporter identity visible only to the MLRO desk | File 30 sections 8.2 (rows 4, 8, 10) (sections 13.3, 13.4) |
| M-69 | Inference test suite for the case system (search, counts, timing, errors, exports, notifications, audit viewer) that fails the build | File 30 SP-33 (section 13.4) |
| M-70 | Assistant gateway: task registry with tiers, autonomy ladder and kill switch; drafts, summaries, retrieval, translation and QA triage only; no closing, escalating, hibernating, filing or contacting | File 29 section 9k; EU AMLR 76(5); file 21 section 12 (section 14.1) |
| M-71 | Provenance store for every assistant output used, a grounding verifier that checks numbers, names and dates against supplied records, a draft stamp, save and export blocked until a human edits or accepts, and stored edit distance | File 29 section 9k; file 30 SP-94 (section 14.2) |
| M-72 | Decision-panel modes: evidence first, optional assess-first, recommendations that list the checklist items and source rows, cheap logged override, no accept-all, QA sample of assisted dispositions never zero | File 29 section 9j; file 21 section 12e; file 31 section 3 (section 14.3) |
| M-73 | Evaluation gate per assistant task (golden set of at least 200 QA-confirmed cases in Arabic and English, shadow period, staged release, regression on every change) and an AI-monitoring panel (use, acceptance, edit distance, override, QA findings) | File 21 section 12e; file 13 VB-39, VB-51 (section 14.4) |
| M-74 | Case aggregate written in Rust with property tests of the invariants; DBS workflow DSL runtime copied for approvals, operations tasks, RFI release, change requests, access requests and QA review | File 32 section 11.3; DBS DSL; section 15 |
| M-75 | Policy pack (routing, clocks, calendars, delegated authority, reason codes, templates, switches) versioned per bank, pinned per case, drained not migrated, adopted at next transition only on opt-in; invariants not configurable; every edit through `change_request` with a dry run on history | DBS DSL section 7; file 30 section 7.5; section 15.4-15.5 |
| M-76 | Simulation harness that replays historical alert streams through a candidate policy and reports queue size, clock breaches and analyst hours | Section 15.6; file 20 L-55 |

**Count:** 76 requirements (M-01 to M-76). Cross-referenced and not repeated: file 20 L-16 to L-19, L-26 to L-31, L-46, L-47, L-52 to L-57, L-68; file 30 SP-15, SP-19 to SP-38, SP-41, SP-52, SP-54, SP-94, SP-95, SP-98; file 13 VB-30 to VB-35, VB-39 to VB-44, VB-49, VB-51; file 04 W-23.

## 19. Sources

All read on 2026-10-07 unless marked. "Local" means a copy in the scratchpad `src` folder of this session.

**Iraqi primary sources**
- AML/CFT Office, Guidance Guide on Reporting Unusual Transactions (UTR), first edition 2026, uploaded 2026-08-11 (Arabic; local `rp/aml/guide-unusual-transactions`): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/40c281c6-d5b7-480d-919d-2a88c454b2e3_20260811093728.pdf
- AML/CFT Office, Guidance Guide on Reporting Suspicious Transactions and Activities, uploaded 2026-08-11 (Arabic and English cover; local `rp/aml/guide-reporting-suspicious`): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/ecd23f30-19dd-4268-84e2-cb2d14cce9be_20260811085852.pdf
- Office document list (JSON API used by file 23): https://api.aml.iq/api/regulatorydocument?pageNo=1&documentType=2
- CBI AML/CFT/CPF supervisory controls, 8 January 2025 (Arabic; file 01 section 2d): https://cbi.iq/static/uploads/up/file-173666439114818.pdf
- CBI circular on behavioural indicators, 26 August 2026 (file 01 section 2c): https://cbi.iq/static/uploads/up/file-178963291760044.pdf
- CBI Detailed Assessment Guide 2025, standard D2 (Arabic, local `cbi-guide.txt` pp. 44-45; file 18 section 12.2): https://cbi.iq/static/uploads/up/file-177615833416828.pdf
- CBI sanctions guidelines, 1 August 2023 (file 25 section 15.7): https://cbi.iq/static/uploads/up/file-169554063119385.pdf
- AML/CFT Office, Guidance on the Prompt Implementation of Targeted Financial Sanctions, uploaded 10 September 2026, art. 12 and 13 (Arabic; local `rp/aml/guide-tfs-prompt`): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/e6fde006-431a-4fc7-8c9b-fdd61b99cd12_20260910121330.pdf ; Terrorists' Funds Freezing Regulation No. 6 of 2023, art. 4 (48-hour and two-month reports; read by the file 23 agent, see file 23 section 3.4): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/24d01159-7ec5-4274-bb22-cbc75a129214_20260810234152.pdf
- Law 39 of 2015, English translation on the CBI site: https://cbi.iq/static/uploads/up/file-150199913642139.pdf
- Law 78 of 2012, Electronic Signature and Electronic Transactions Law, Iraqi Official Gazette no. 4256, 5 November 2012 (13-page scan; articles 3, 4, 13, 14, 15, 17 read from page images). Page: http://www.iraq-lg-law.org/ar/content/قانون-التوقيع-الالكتروني-والمعاملات-الالكترونية-رقم-78-لسنة-2012 ; PDF through the Wayback Machine: https://web.archive.org/web/20180814210438if_/http://www.iraq-lg-law.org/ar/webfm_send/1353 (local `web/law78.pdf`)
- MENAFATF Mutual Evaluation of Iraq, 2024, paragraphs 563-567 (via files 01 and 18; MER text local `iraq-mer-2024-en.txt`)
- University of Baghdad study of STR statistics (file 01 section 5): https://jols.uobaghdad.edu.iq/index.php/jols/article/view/1190/version/1151

**Engines, libraries and open-source case managers (facts from the repositories and documentation)**
- Temporal server (MIT; GitHub API): https://github.com/temporalio/temporal ; Rust SDK: https://github.com/temporalio/sdk-rust ; crate `temporalio-sdk` 1.0.0 (2026-09-04), 1.1.0 (2026-10-06): https://crates.io/crates/temporalio-sdk ; visibility and persistence databases: https://docs.temporal.io/self-hosted-guide/visibility
- Camunda 8 repository and licence: https://github.com/camunda/camunda ; https://raw.githubusercontent.com/camunda/camunda/main/licenses/CAMUNDA-LICENSE-1.0.txt
- Operaton (Apache-2.0 fork of Camunda 7 CE): https://github.com/operaton/operaton ; https://operaton.org/
- Flowable (Apache-2.0; BPMN, CMMN, DMN): https://github.com/flowable/flowable-engine
- Restate (Business Source License 1.1): https://github.com/restatedev/restate ; Conductor OSS: https://github.com/conductor-oss/conductor ; Argo Workflows: https://github.com/argoproj/argo-workflows ; DBOS: https://github.com/dbos-inc/dbos-transact-py
- Rust crates: https://crates.io/crates/statig , https://crates.io/crates/rust-fsm , https://crates.io/crates/smlang , https://crates.io/crates/snurr , https://crates.io/crates/acts , https://crates.io/crates/wfrs-engine , https://crates.io/crates/zeebe , https://crates.io/crates/cedar-policy
- Marble (open-source decision engine and case manager): https://github.com/checkmarble/marble ; documentation index https://docs.checkmarble.com/llms.txt ; https://docs.checkmarble.com/docs/workflows ; https://docs.checkmarble.com/docs/rule-snoozes ; https://docs.checkmarble.com/docs/auto-case-assignment
- Digital Banking System (read only, local): `/home/haithem/Projects/digital-banking-system/docs/DSL.md` (workflow DSL: steps, timers, versioning), `docs/DESIGN-SYSTEM.md`, `.claude/rules/frontend.md`

**Papers, standards and reference pages**
- Uetz, Bönninghausen, Hackländer-Jansen, Henze, "Can Risk-Based Alerting Mitigate Cybersecurity Alert Fatigue?" (2026): https://arxiv.org/abs/2609.02465
- NIST SP 800-86, Guide to Integrating Forensic Techniques into Incident Response (August 2006): https://csrc.nist.gov/pubs/sp/800/86/final
- Wikipedia, Workweek and weekend (Iraq: Sunday to Thursday): https://en.wikipedia.org/wiki/Workweek_and_weekend ; Public holidays in Iraq: https://en.wikipedia.org/wiki/Public_holidays_in_Iraq
- Sample-size arithmetic: attribute (zero-defect) sampling n = ln(1 - C) / ln(1 - p) and the hypergeometric form, computed locally on 2026-10-07; Wilson score interval, computed locally

**Research files in this folder (cross-references)**
- 01 Iraqi framework (sections 2c, 2d, 5); 04 Wolfsberg (2a, 2b, 2p); 05 other regimes (1b, 1d, 3b, 3c, 3d); 06 fundamentals (7); 08-11 vendor suites (case management sections); 12 AI-native vendors (8.4); 13 investigation automation (sections 1-10, VB-xx); 18 Iraq market (1, 12.2); 20 enforcement lessons (2.2, 2.4, 3.5, 4.3, 6.3, 7.1, 8.1, 8.2, 10, L-xx); 21 technology (9, 11, 12); 22 Arabic names (13); 23 goAML (in progress); 25 CBS integration (15.7, C-60, C-63); 27 scenarios (in progress); 29 governance (3b, 9j, 9k); 30 security (6-10, 12, 19, SP-xx); 32 stack (3, 11, 12, 18)

**Not reached, or reached only in part**
- The Office's response time for information requests: file 23 section 2.6 reports that replies are due "within the period it specifies" (Law 39 art. 9/First/b); no numeric default was found.
- Camunda's announcement on the end of Camunda 7 Community Edition (404 on the guessed URLs).
- ISO/IEC 27037 (iso.org returned 403); the NIST guide is cited as a general forensic reference instead.
- Independent measurements of minutes per alert and of liaison-officer review times (none found; vendor figures are cited as vendor-reported only).
- Whether Iraqi courts apply Law 78 of 2012 to bank-generated case records, and whether the law has been amended since 2012 (the archived copy is from the Iraqi Legal Library for Local Governance, snapshot 2018-08-14).
- G2, Gartner Peer Insights and Business Wire pages (blocked) for vendor user reviews of case tools; see file 31 for the user-experience evidence that was reachable.
