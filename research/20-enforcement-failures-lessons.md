# 20. Enforcement failures: what they teach about monitoring and screening system design

Status: done (2026-10-06)

Purpose: AKR-TMS will be sold to Iraqi banks that must satisfy the CBI (file 01) and the dollar correspondents,
supervisors and prosecutors of the US, UK and EU (files 04 and 05). Regulators do not write their expectations in a
vacuum: the clearest statement of what a monitoring or screening system must do is the list of ways such systems have
failed in enforcement cases. This file reads those cases for the system, the data and the operation, not for the
headline fine. For each case: facts and numbers, what failed, root cause, design lesson. Then a failure-mode
catalogue with the control AKR-TMS must have and the evidence it must be able to produce.

Contents:

- 1. Scope, method and evidence grades
- 2. United States: capacity, coverage and concealment (TD Bank 2024, US Bancorp 2018, Rabobank N.A. 2018, HSBC USA 2012, Deutsche Bank 2017-2023, UBS Financial Services 2026, Canaccord 2026, Capital One 2021)
- 3. Foreign banks clearing dollars: sanctions evasion, message stripping, lists and the Middle East (Standard Chartered 2012 and 2019, BNP Paribas 2014, Commerzbank 2015, Habib Bank NY 2017, Mashreq NY 2018 and 2021, National Bank of Pakistan NY 2022)
- 4. Europe: groups, branches and data (Danske Bank Estonia, Swedbank, ING 2018, ABN AMRO 2021)
- 5. Australia: Westpac 2020 and Commonwealth Bank 2018
- 6. United Kingdom: HSBC UK 2021, NatWest 2021, Metro Bank 2024, Starling 2024, Monzo 2025, Barclays 2025, Nationwide 2025, Dinosaur Merchant Bank 2026
- 7. EU digital banks: N26 and Revolut
- 8. Crypto and payments firms: Coinbase 2023, Block/Cash App 2025, Binance 2023, Paxful 2025, OKX 2025
- 9. Other 2025-2026 cases: FinCEN special measures (Banque Misr UAE, MBaer and others), Swedbank NYDFS 2026, Paxos 2025, OFAC actions; what the record does not show
- 10. Failure-mode catalogue (36 failure modes: failure, cases, control, evidence, requirements)
- 11. What it means for AKR-TMS (findings, architecture, phases, Iraq, ten board questions, cautions)
- 12. Derived requirements L-01 to L-68
- 13. Sources and what could not be reached

## 1. Scope, method and evidence grades

**Scope.** Cases where the monitoring system, the screening system, the data feeding them, or the way they were run
failed, and a supervisor, prosecutor or regulator wrote that failure down. Cases are grouped by the kind of institution
(US bank, foreign bank clearing dollars, European group, Australian bank, UK bank, EU digital bank, crypto and payments
firm), not by date, because the lessons repeat across groups.

**Evidence grades used in each case header.**

| Grade | Meaning |
|---|---|
| A | Primary document read in full or in the relevant parts (plea agreement and statement of facts, consent order, final notice, statement of claim, court judgment) |
| B | Official press release or official summary of the primary document (DOJ, Treasury, FCA, AUSTRAC, NYDFS, OCC, Federal Reserve, Dutch prosecution service) |
| C | Reputable secondary source (law-firm client alert, press agency, specialist press). Numbers from grade C are marked "(C)" |

**How each case is written.** Facts and numbers; what failed in the system, the data or the operation; root cause;
design lesson (marked "Design lesson"). A lesson is a statement of what AKR-TMS must be able to do or show, not a
general moral. Section 10 collects the lessons into a failure-mode catalogue; section 12 turns them into requirements
L-01 onward.

**Conventions.** Names of individuals are not used except where a person is central and publicly convicted or charged
(none are needed here); bank and company names are used. Amounts are in the currency of the document. Dates are the
dates of the announcement of the resolution unless stated.

**Relation to other files.** File 04 (Basel, Wolfsberg) states what standards expect of a monitoring system; file 05
section 1e holds OFAC's 2019 framework root causes and section 1f the NYDFS Part 504 attributes. This file shows what
happened when those expectations were not met. Where a case proves a requirement in files 03 to 05 this file refers to
its ID (F-xx, B-xx, W-xx, R-xx) instead of repeating it.

## 2. United States banks: capacity, coverage and concealment

Eight cases from US banks and bank-owned firms (and one Dutch-owned US bank) that ran commercial transaction-monitoring (TM) systems and still failed. The
common thread is not a defect in the software. It is that management decided, or never checked, what the system would see, how many alerts it would produce and
how much it would cost, and the people who were supposed to challenge those decisions were not told or were overruled. Sections 2.1 to 2.3 are about capacity and
concealment, 2.4 and 2.5 about country-based coverage and unfinished remediation, 2.6 to 2.8 about data feeds and coding changes (two of them from 2026).

### 2.1 TD Bank (DOJ, FinCEN, OCC, Federal Reserve, 10 October 2024) - evidence grade A

**Facts and numbers**

| Item | Detail |
|---|---|
| Resolution | 10 October 2024, coordinated. TD Bank N.A. (TDBNA) pleaded guilty to conspiracy to fail to maintain an AML programme, fail to file accurate currency transaction reports (CTRs) and launder money; TD Bank US Holding Company pleaded to causing those failures |
| Amounts | DOJ USD 1.89 billion (the plea agreement sets a criminal penalty of USD 1,434,013,478.40 and forfeiture of USD 452,432,302); FinCEN USD 1.3 billion (largest civil penalty against a US bank); OCC USD 450 million; Federal Reserve USD 123.5 million; total about USD 3.09 billion (Federal Reserve statement). The DOJ penalty used the BSA provision of up to USD 500,000 per day without a functioning AML programme, applied to January 2014 to October 2023 (Latham client alert, grade C) |
| Period | January 2014 to October 2023 (programme failure); January 2018 to 12 April 2024 (unmonitored volume) |
| Restrictions | OCC asset cap at the 30 September 2024 level (reduction of up to 7 percent a year for non-compliance); no new branches, markets or medium/high-risk products without OCC approval; DOJ monitor 3 years; FinCEN monitor 4 years; Fed requires the AML functions that serve US law to be relocated to the US |
| Laundering enabled | Three money laundering networks moved over USD 670 million through TD accounts (plea); the largest network laundered more than USD 470 million through the bank (of USD 653 million it conspired to move); at least one scheme had the help of five store insiders, and its organiser bribed employees with more than USD 57,000 in gift cards |

**What failed in the system (statement of facts, paragraphs 6, 29-43)**

1. **Most of the volume was never fed to the system.** "TDBNA did not monitor any domestic ACH activity, most check
   activity, internal transfers between accounts at TDBNA, or numerous other transaction types" (para 33). Between
   1 January 2018 and 12 April 2024 the automated monitoring "failed to monitor 92% of transaction volume and 74% of
   transaction value, which corresponded to over 14.6 billion unmonitored transactions and over USD 18.3 trillion". The
   exclusion dated from around 2008 to 2011 (first decisions) and was never reviewed: "Since 2008, TDBNA did not
   conduct any systematic analysis to review decisions not to monitor certain transaction types" (para 34). In 2012 the
   bank itself raised the AML risk of domestic ACH to "medium", "largely due to the lack of monitoring", and a proposal
   to add scenarios was rejected by an executive.
2. **The scenario set was frozen.** "From at least 2014 to late 2022, TDBNA failed to implement any new transaction
   monitoring scenarios or make any substantive changes to the parameters of its existing transaction monitoring
   scenarios" (para 30). There was no scenario for changes in a customer's behaviour, none for money services
   businesses or precious metals dealers, and "a Fortune 500 company was subject to the same scenarios and dollar
   thresholds as a sole proprietorship". A 2021 consultant found "technology barriers to developing new scenarios or
   adding new parameters".
3. **Every change reduced alerts.** "The limited changes TDBNA made to its scenarios during the relevant period almost
   exclusively - and intentionally - reduced the universe of alerts being generated and thereby lowered the associated
   cost of their review. Indeed, while TDBNA did not add any new transaction monitoring scenarios during the relevant
   period, it removed at least nine" (para 39). Scenarios for large business cash activity were switched off in about
   2011 "to test and recalibrate" and stayed off until late 2022 (para 38). For one network, that scenario "would have
   generated approximately 161 transaction monitoring alerts" on a single management account that received USD 122
   million in cash, and 271 more on related business accounts (para 59 and footnote 3).
4. **A new product was bolted onto scenarios that could not fire.** Zelle started in April 2017 (over USD 75 billion
   moved through it) and was not screened at all until March 2020. In August 2020 it was added to two wire scenarios that
   flagged personal activity above USD 10,000 in deposits or USD 9,000 in transfers over 5 days, but "personal Zelle
   activity was capped at USD 10,000 during a rolling 30-day period. In other words, the scenarios captured activity that
   effectively could not occur through Zelle" (para 35). The Zelle project was put on hold in late 2021 because it was
   not "an exposed risk or regulatory need" (para 36).
5. **Velocity rules ignored intrabank transfers.** The "high-velocity" scenarios did not see intrabank transfers, so cash
   deposited into one account could move to others without any alert (para 60).
6. **Threshold testing stopped.** From 2018 through 2022 only one of about 40 US scenarios received "quantitative
   tuning" (para 40). A senior AML executive told the bank's AML oversight committee, after another US bank's February 2018 resolution (not
   named in the statement of facts; see sections 2.2 and 2.3), that "for each one of our scenarios we will do a lot of
   analysis and work below each threshold" while the other bank had "either ignored or discontinued that below the line
   threshold testing". Yet "by the beginning of 2018" TD had itself "effectively stopped conducting threshold testing".
7. **A stale country list.** Only "high high risk countries" (a subset not updated after 2013) were monitored; the
   Dominican Republic and Jamaica were not on it until at least December 2023, and executives "only approved threshold
   changes that would have no impact or lower the volume of false positives" (para 41).
8. **Unusual-transaction reports (UTRs) from branches were throttled.** Because the intake team was understaffed, an
   August 2020 procedure told stores that no further UTRs were needed for specific customers unless their unusual
   activity changed or continued beyond 60 days; stores concluded that the activity was within risk tolerance (para 55).

**What failed in operation.** The "flat cost paradigm" (or "zero expense growth paradigm") meant every budget, including
AML, stayed flat while profit rose about 26 percent: "GAML's base and project expenditures on US-AML were less in fiscal
year 2021 than they were in fiscal year 2018 and were not sufficient to address AML deficiencies including substantial
backlogs of alerts across multiple workstreams" (para 42). Projects were postponed in 2019 because "new scenario
development means new data and a lot of work effort" and "manual monitoring" had "no capacity" (para 43). Staff raised
the gaps repeatedly; leadership told the OCC that fixes were "in development" in 2017, 2018 and 2019 while the work was
on hold (para 37).

**Root cause.** A cost target set above the compliance function, no owner for the question "what is not being
monitored?", and legacy technology in which adding a scenario or a data feed was slow and expensive, so that scenario
change was treated as a cost rather than as risk control.

**Design lesson**

- *Coverage must be measured, not assumed.* AKR-TMS must reconcile, every day, the CBS transaction types, channels and
  products against the monitoring scope, and show the monitored share by count and by value. A transaction type that
  appears in the CBS but is not mapped must appear as a red item on the main compliance screen (extends B-01).
- *Every "not monitored" decision is a risk acceptance.* It needs an owner, a date, a reason, a review date and approval
  by the board committee or MLRO; an exclusion without a current approval is an open finding.
- *A new CBS product or transaction code is a trigger.* The mapping layer must raise "unmapped new code" and block the
  bank from calling the product covered until a monitoring decision exists.
- *Scenario reachability test.* For every scenario and threshold, AKR-TMS must compute from history how many events
  could ever reach it (the Zelle limit versus the 30-day threshold) and flag dead rules (no alert in N months with
  eligible volume present).
- *Threshold changes that cut alert volume need below-the-line evidence* (what the rule would have caught at the old
  value, with sample review) and a named approver outside operations. Switching a scenario off needs an end date.
- *Segment thresholds* by customer type, size and risk; one threshold for "business accounts" is a finding.
- *Staff and branch reports are data.* Branch unusual-activity reports and employee-linked accounts must be captured and
  trended, and the rule "no more reports needed on this customer" must not exist.
- *Alert backlog age is a first-class control metric*, shown to the board next to staffing (see section 8 for Coinbase).

### 2.2 U.S. Bancorp / U.S. Bank N.A. (DOJ, FinCEN, OCC, Federal Reserve, 15 February 2018) - evidence grade A (FinCEN assessment read in full)

**Facts and numbers.** Total USD 613 million: DOJ USD 528 million (USD 453 million forfeiture plus USD 75 million credited
to the OCC penalty), FinCEN USD 185 million (USD 115 million credited against DOJ), OCC USD 75 million, Federal Reserve
USD 15 million (Paul Weiss client memo, grade C, for the split; FinCEN assessment for the facts). FinCEN found that the
bank "willfully violated the BSA's program and reporting requirements from 2011 to 2015". Related customer: a payday
lending network whose accounts the bank kept open; the monitoring failures are the part that matters here.

**What failed in the system (FinCEN Assessment of Civil Money Penalty, February 2018)**

1. **Alert caps tied to headcount.** The bank used a commercial monitoring product (Searchspace, installed April 2004).
   "From at least 2009 until July 2014, U.S. Bank configured this automated transaction monitoring system to generate a
   certain number of alerts each month. In addition, out of the 22 scenario-based queries U.S. Bank set-up for alert
   generation, the Bank set caps on the six largest alert generators and retained those caps even when below-threshold
   testing demonstrated significant missed SAR reporting." An internal note read: "The number of query alerts that we work
   are increasingly [sic] based solely on staffing levels. This is a risk item." FinCEN added that it "had previously
   communicated to the industry that the practice of artificially limiting compliance staff, including by setting
   staff-based numerical caps for alerts, was considered reckless."
2. **Below-the-line testing found the problem, then was stopped.** From 2007 to April 2012 the bank sampled alerts just
   under the cap and had investigators review them. In November 2011 AML staff found that "an average of 50% of the
   transactions that were reviewed during the below-threshold testing resulted in the filing of a SAR". The bank stopped
   the testing in April 2012 because "that testing showed that the Bank was failing to address an ongoing problem". The AML
   officer had written in December 2009 that a "regulator could very easily argue that [below-threshold] testing should lead
   to an increase in the number of queries worked".
3. **A suppression rule hid repeat activity.** A 90-day re-alerting rule stopped any new alert on an account that had
   alerted in the last 90 days, "regardless of how suspicious the activity appeared to be or whether the prior alert
   resulted in a SAR". A limited review of three queries in 2013 found about 6,888 "productive" alerts suppressed.
4. **Generated alerts were not reviewed.** A June 2013 review of wire activity found that of more than 57,000 customers who
   alerted, "the Bank reviewed less than 100".
5. **Scope gap: non-customers.** From May 2009 to July 2014 customers and non-customers sent currency transfers through a
   large money transmitter at branches. The bank's own memo called these "one of the riskiest products", knew that
   non-customer transfers "would not be processed through the Bank's automated transaction monitoring system", and
   continued until July 2014. A six-month look-back found over USD 12 million of suspicious transactions undetected.
6. **Customer risk rating missed inputs.** The rating did not look across business lines and "failed to include important
   information about its clients in its risk-rating analysis, such as a customer's country of citizenship and occupation".
   A look-back of May 2014 to April 2015 produced more than 136 late SARs.
7. **No independent validation.** The OCC asked for validation in 2008 and 2010. The bank had the product's own manager
   write a "validation" and a colleague sign it while being told he was "not making any representation that [he was]
   validating anything"; the manager wrote: "Until we are forced to go there... you are sufficient", because outside
   validation "would cost tens of thousands".
8. **Reporting data error.** From 7 July 2014 to 27 May 2015 about 5,000 CTRs listed the domestic respondent bank, not the
   money services business, as the party on whose behalf cash was deposited (over USD 600 million), although the correct tax
   ID was given. "A search of the CTRs using the names of the relevant MSBs would have yielded no responses."

**What failed in operation.** For much of the period the bank had 25 to 30 AML investigators; "as late as 2012, when U.S.
Bank had over USD 340 billion in assets, the Bank had only 30 investigators", with below-market pay. The compliance
officers "took steps to avoid disclosing the alert caps to the OCC", told new AML officers that regulators had "at least
tacitly approved" the caps, and the CCO deleted references to alert caps from a 2013 presentation to the CEO. An AML
officer described the programme in a 2013 memo as "smoke and mirrors" to "pull the wool over the eyes" of the OCC.

**Outcome of the look-backs.** The bank re-ran six-month samples only: 1.7 billion deposit-account transactions, 650,000
money-transmitter transactions and 250,000 cash-vault transactions. They produced "more than 24,000 alerts on transactions
that the Bank had previously failed to review" and 2,121 late SARs on more than USD 700 million. Alert suppression alone
had prevented reporting of "over USD 318 million in suspicious activity" (1,528 late SARs); a consultant sampling 68
accounts that fell below the alert limits found 26 (38 percent) "productive or potentially productive". FinCEN noted that
the remainder of each period "would have identified many more instances".

**Root cause.** Detection design was subordinated to staffing. Alert volume, the cost of reviewing alerts and the
regulator's view were managed together by the same executive, and the only test that exposed the problem was abolished
when it produced an unwelcome answer.

**Design lesson**

- *Detection and workload must be separate.* The detection engine has no "maximum alerts per period" parameter and no
  suppression keyed to staffing. Capacity is managed in the queue (prioritisation, tiering, additional staff), never by
  silently not generating alerts.
- *Suppression rules are logged and measured.* Any re-alert window, whitelist or duplicate rule must show how many alerts
  it suppressed and, from sampling, how many of those were productive (below-the-line outcome).
- *Below-the-line testing is a built-in recurring job, not a project.* For each scenario AKR-TMS samples events just
  under the threshold, routes them to analysts as calibration cases, and reports the productive rate; a rate above the
  bank's tolerance sets the scenario to "threshold review required" and cannot be dismissed without a recorded decision.
- *Unreviewed alerts are shown as unreviewed risk.* The board view shows alerts generated, reviewed, aged and closed in bulk,
  next to analyst capacity.
- *The validator cannot be the owner.* Validation records require a reviewer role that differs from the configuration
  owner, with an immutable record of what was tested.
- *Non-customer parties are first-class.* Walk-in cash, third-party and agent-channel transfers need a party model so they
  are monitored (relevant for Iraqi branch remittances and money transfer agents).
- *Customer risk rating must show which inputs it used* and the share of customers where an input (nationality, occupation,
  country, product holdings across all business lines) is missing.
- *Reports to supervisors come from system facts.* The board and supervisor pack is generated from configuration and
  alert data, and any manual edit is shown as an edit with author and time (the 2013 CEO presentation was edited to remove
  the alert caps).
- *Mapping fields in regulatory reports are tested*: a CTR (in Iraq, a cash transaction report in goAML) must be checked
  for the party on whose behalf the cash moved.

### 2.3 Rabobank N.A. (DOJ and OCC, 7 February 2018) - evidence grade B (DOJ press release text via ICE and Mondo Visione copies; Paul Weiss and Debevoise summaries of the plea agreement and the OCC notice of charges; the statement of facts itself was not reachable because justice.gov blocks automated downloads)

**Facts and numbers.** Rabobank N.A. (RNA), a California bank with branches in Imperial County on the Mexican border,
pleaded guilty on 7 February 2018 to one count of conspiracy to defraud the United States by obstructing the OCC's
examination of its BSA/AML programme (plea agreement dated 5 February 2018, S.D. Cal.). It forfeited USD 368,701,259; an
OCC civil penalty of USD 50 million was credited against that sum. The forfeiture was tied to suspicious transactions
"unreported or reported untimely". DOJ said the programme allowed "hundreds of millions of dollars in untraceable cash,
sourced from Mexico and elsewhere" to be deposited in 2009 to 2012; the Calexico branch, about two blocks from the border,
was the "highest performing" branch in the region because of those deposits. Note on the brief: the lead's working title
for this case was "thresholds altered and concealed". The documents available show different mechanisms (below); none of
the summaries read describes threshold values being changed, so that wording is not used here.

**What failed in the system and in operation**

1. **A "Verified List" that switched monitoring off per customer.** RNA "created a so-called 'Verified List' of customers
   deemed per se non-suspicious, even when a customer's transaction activity deviated from previous levels" (Paul Weiss
   summary of the statement of facts). Staff were told that if a customer was on the list "no further review of that
   customer's transactions was necessary". It "effectively allowed Rabobank to execute an end-run [around] the BSA/AML and
   SAR requirements". Growth: "In 2009, Rabobank had less than ten 'verified' customers; by 2012 ... it had more than
   1,000" (DOJ). This is a good-guy list in the sense of section 3.4 (Habib Bank).
2. **Repeat alerts closed without comparison.** Alerts on high-risk accounts that had been investigated before were closed
   "without further review, whether or not the new activity was different from the activity that was previously
   investigated" (Debevoise, summarising plea agreement p. 7).
3. **Customer explanation accepted as a reason not to report.** A "Security CMIR Mitigation Policy" told staff not to
   investigate or file SARs on cash withdrawals at border branches if the customer said they withdrew the cash in the US to
   avoid carrying it over the border (which would have required a CMIR); the plea agreement states the withdrawals were
   "excessive and structured" and should have produced SARs. A related policy treated structured transactions under USD
   10,000 as non-suspicious on the same explanation (Paul Weiss).
4. **Capacity.** "At certain times" three employees were assigned to investigate more than 2,300 alerts per month (Paul
   Weiss, from the statement of facts).
5. **Concealment from the supervisor.** The bank engaged a consulting firm in December 2012 for a BSA/AML assessment. When
   the OCC asked for the reports, management "withheld those reports from the OCC for nearly a month" and said no report
   existed. RNA answered the OCC's February 2013 initial examination report "with false and misleading information about
   the state of Rabobank's BSA/AML program" and "demoted or terminated two RNA employees who were raising questions about the
   adequacy of Rabobank's BSA/AML program" (DOJ). The OCC later brought a civil case against the former chief compliance
   officer (notice of charges 16 April 2018); its outcome is not relied on here.

**Root cause.** A branch network that made large profits from border cash, a compliance function that was overruled or
sidelined, and policy exemptions (a list, a repeat-alert rule, a customer-explanation rule) that removed risky customers
from review in a way that could not be seen from the alert statistics.

**Design lesson**

- *Exemption lists never remove monitoring.* An exemption ("verified", "trusted", "good guy", "VIP") may lower priority or
  raise a threshold within stated bounds; the customer's activity is still compared with expected behaviour and a
  deviation always alerts. Every entry needs a named approver, rationale, evidence, expiry date and re-approval.
- *The size and growth of every exemption list is a board metric*, with a trigger (for example list growth above a set
  percentage in a quarter) and a periodic sample review of entries by someone outside the business line.
- *Repeat alerts show a difference view.* When a case on a customer closes and later activity alerts again, the screen
  shows what is new against what was investigated, and "closed per earlier case" is available only after a recorded
  similarity check.
- *A customer's explanation is not evidence.* Explanations are recorded as claims with corroboration status and an expiry;
  no policy may route an explanation to automatic "no SAR".
- *Independent assessments reach the regulator by design.* Reports from consultants, internal audit and model validation
  are stored in the evidence vault with immutable time stamps and are visible to internal audit and the board committee,
  not only to the compliance head; the supervisor access role can list them.
- *Capacity is measured per analyst*: alerts per analyst per month against measured handling time, shown to management
  (2,300 alerts a month for three people).

### 2.4 HSBC Bank USA and HSBC Holdings (DOJ deferred prosecution agreement, OCC, Federal Reserve, FinCEN, OFAC, 11 December 2012; Senate Subcommittee report, 17 July 2012) - evidence grade A (OCC penalty order and the Senate report read; DOJ release read from the EDNY archive copy; the DPA statement of facts was not reachable)

Not on the lead's list, added because it is the cleanest case of **coverage decided by a risk rating**, of a **country
score that ignored a regulator's advisory**, and of **dollar notes sold to Middle East banks**.

**Facts and numbers.** HSBC Holdings and HSBC Bank USA (HBUS) forfeited USD 1.256 billion under a DPA and paid USD 665
million in civil penalties (OCC USD 500 million, which also satisfied FinCEN's USD 500 million; Federal Reserve USD 165
million); OFAC's USD 375 million settlement was satisfied by the forfeiture. DOJ: HBUS "violated the BSA by failing to
maintain an effective anti-money laundering program and to conduct appropriate due diligence on its foreign
correspondent account holders"; the group violated IEEPA and TWEA for Cuba, Iran, Libya, Sudan and Burma. DOJ states that
"from at least 2006 to 2009, HSBC Bank USA rated Mexico as 'standard' risk, its lowest AML risk category. As a result, HSBC
Bank USA failed to monitor over USD 670 billion in wire transfers and over USD 9.4 billion in purchases of physical U.S.
dollars from HSBC Mexico", and "at least USD 881 million in drug trafficking proceeds" passed through HBUS.

**What failed in the system, the data and operation**

1. **Wires were excluded from automated monitoring by country rating.** OCC penalty order: "The Bank excluded from
   automated BSA/AML monitoring wire transfers initiated by customers domiciled in countries risk rated as 'standard' or
   'medium,' representing two-thirds of total dollar volume for PCM [the payments and cash management unit]." The Senate
   report quotes the OCC as saying the practice amounted to "more than USD 60 trillion of wire transfers each year" and
   calls the group's rule of "assigning an overall risk rating to its non-SCC [special category client] customers based
   solely on the risk rating that the bank has given the country where the customer is located" "unacceptable".
2. **The country score missed a regulator advisory.** HBUS's February 2009 chart scored Mexico 2 on a 0 to 28 scale
   (rating bands standard, medium, cautionary, high). The compliance officer responsible told the Senate staff that since
   2006 the assessments "had inadvertently failed to take into account a 2006 FinCEN advisory related to Mexico that would
   have added 10 points to its score each year".
3. **Affiliates were exempt customers.** From mid-2006 to mid-2009 "the Bank did not perform BSA/AML monitoring for
   banknote (or 'bulk cash') transactions with Group Entities", and "did not collect or maintain customer due diligence
   ('CDD') or enhanced due diligence ('EDD') information for Group Entities", relying on affiliates to follow group policy.
4. **Alert backlog.** OCC supervisory letter of March 2010 (quoted in the Senate report): "a backlog of over 17,000 alerts,
   in four business units, identifying potentially suspicious activity that had not been investigated"; 98 percent were in
   the "High Risk Monitoring Group" and 14 percent were six months old or older; deadline 30 June 2010. The OCC penalty order
   lists the causes: "inadequate staffing and procedures in the alert investigations unit that resulted in a significant
   backlog of alerts; the closure of alerts based on ineffective review". The backlog review and the look-back led to the
   late filing of **890 SARs on USD 6.34 billion** of suspicious activity.
5. **PEPs and high-risk customers not flagged.** "The Bank did not appropriately designate customers as 'high-risk' for
   purposes of BSA/AML monitoring, even where a customer's association with politically-exposed persons could have harmed
   the Bank's reputation."
6. **The remediation system went live untested.** Under the October 2010 consent order the bank had to install, test and
   activate a new wire monitoring system within 180 days; the OCC found it "installed and activated a new wire monitoring
   system for its PCM unit without adequately testing the system" and did not complete validation (gap) testing, which
   "exposed the Bank to a material risk of failing to report suspicious activity".
7. **Sanctions filters were defeated by message content.** DOJ: group affiliates followed instructions from sanctioned
   parties "to omit their names from U.S. dollar payment messages", removed country identifiers, "deliberately used less-
   transparent payment messages, known as cover payments", and "worked with at least one sanctioned entity to format payment
   messages, which prevented the bank's filters from blocking prohibited payments"; notes in messages read "do not mention
   our name in NY". About USD 660 million of OFAC-prohibited transactions passed through US institutions from the
   mid-1990s to September 2006. The group knew by 2000; affiliates kept doing it for three more years through "dispensations"
   to group policy. OFAC's settlement agreement lists 203 Iran-related transactions of USD 164,308,904 between March 2004 and
   June 2010.
8. **Middle East dollar notes and a reversed exit decision.** After 9/11 evidence emerged that Al Rajhi Bank and some owners
   had links to financing organisations associated with terrorism. HSBC announced in 2005 that affiliates should cut ties,
   reversed four months later, HBUS closed its correspondent and banknote accounts, then "in December 2006, after Al Rajhi
   Bank threatened to pull all of its business from HSBC unless it regained access to HBUS' U.S. banknotes program, HBUS
   agreed to resume": "nearly USD 1 billion in U.S. dollars" until 2010 "despite ongoing troubling information". HBUS also
   supplied dollars to Islami Bank Bangladesh and another bank. The Senate exhibit list also shows a private-bank unit
   maintaining 1,679 bearer share corporation accounts in December 2007.

**Root cause.** A group that treated its own affiliates and "low-risk" countries as exempt from monitoring, a country
score that was a spreadsheet with stale inputs, an AML unit too small for the alert volume it generated, and commercial
pressure that reversed a compliance exit decision.

**Design lesson**

- *A risk rating may change how hard a customer is watched, never whether.* Coverage is 100 percent of transactions;
  rating moves thresholds, review frequency and queue priority only (B-01; section 2.1 coverage ledger).
- *Country-risk inputs are tied to published advisories.* AKR-TMS keeps an advisory ledger (CBI circulars, FATF
  statements, FinCEN and OFAC actions, UN changes) and shows, per advisory, the date it was reflected in the country table
  and who approved it; a published advisory not yet reflected after N days is an open item (extends W-19).
- *Related entities are customers.* Sister banks, exchange houses, group companies and staff accounts get CDD and
  monitoring; "group policy applies" is not a substitute.
- *Backlog age buckets and a closure-quality sample.* Each month a sample of closed alerts is re-reviewed by someone else;
  the failure rate is reported with the backlog.
- *Go-live acceptance by seeded test.* A new monitoring or screening configuration is accepted only after a documented gap
  test with seeded known-suspicious cases for each scenario and list; AKR-TMS ships the test pack and keeps the report.
- *High-risk relationship decisions are logged.* Decline, exit and reinstate decisions record the new information, the
  approver and the commercial request; a reversal of an earlier exit is flagged to the board committee.
- *Dollar-note and correspondent flows get their own monitoring* (relevant to Iraq's dollar cash and exchange-house
  channels, file 01 section 4).
- *Message-content screening is not enough* when the sender controls the content: compare gateway message data with CBS
  postings and flag stripped, generic or cover-payment patterns (see section 3.1).

### 2.5 Deutsche Bank (FCA and NYDFS 30 January 2017; Federal Reserve 26 May 2017; NYDFS 7 July 2020; Federal Reserve 19 July 2023) - evidence grade A (FCA final notice, NYDFS 2020 consent order and Fed 2023 consent order read; NYDFS 2017 from the DFS press release)

Four actions over six years against one bank, which makes this the best case study of **remediation that did not finish**.

| Date | Authority | Amount | Subject |
|---|---|---|---|
| 30 Jan 2017 | FCA | GBP 163,076,224 (after a 30 percent settlement discount; GBP 229 million without it) | Mirror trades through London, 1 Jan 2012 to 31 Dec 2015; breaches of Principle 3 and SYSC 6.1.1R and 6.3.1R |
| 30 Jan 2017 | NYDFS | USD 425 million plus independent monitor | Mirror trades moved about USD 10 billion out of Russia |
| 26 May 2017 | Federal Reserve | USD 41 million | "Significant deficiencies in the U.S. Operations' transaction monitoring capabilities" (2017 AML order) |
| 7 Jul 2020 | NYDFS | USD 150 million | A private-banking client who was a registered sex offender, the Danske Bank Estonia correspondent relationship, the FBME Bank correspondent relationship |
| 19 Jul 2023 | Federal Reserve | USD 186 million plus consent order and written agreement | "Insufficient remedial progress under the 2015 and 2017 consent orders"; the 2015 order (4 Nov 2015) had carried a USD 58 million penalty for sanctions failures |

**What failed: mirror trades (FCA final notice, 30 January 2017)**

- More than 2,400 mirror trades between April 2012 and October 2014: a Russian customer bought Russian securities for roubles
  from Deutsche Bank Moscow while a connected non-Russian customer sold the same securities for dollars in London, moving
  "more than USD 6 billion from Russia" to accounts in Cyprus, Estonia and Latvia; a further USD 3.8 billion of "one-sided"
  trades, so "approximately USD 10 billion" in total (paras 2.4-2.10).
- **Visible but not used.** "Both the Moscow and London Sides of the mirror trades were visible in the trading systems used
  by Deutsche Bank. These systems, however, were not used for the monitoring of money laundering risks" (para 4.80).
- **No single customer record.** "Deutsche Bank lacked a single authoritative repository of KYC information because it used
  multiple IT systems"; "Customers could be assigned different identifiers in each of the systems where their data was
  recorded. As a result, Deutsche Bank was unable to connect trading activity with underlying KYC information" (paras
  4.73-4.74). Internal Audit had called the lack of reconciliation between trading and onboarding systems "a critical issue
  in 2014".
- **Orphan accounts.** About 5,000 active customers had not been onboarded to KYC standard and "orphan accounts could still
  be used for trading" (para 4.59).
- **Remote booking.** Trades were booked in London by Moscow staff; London believed Moscow monitored them, Moscow had no
  data on them and "was unable to introduce an effective automated trade surveillance system because of a lack of
  resources" (paras 4.81-4.82).
- **Stale payment monitoring.** "The alerts generated by the payments monitoring system were reviewed in accordance with
  strictly defined criteria that did not allow for a risk-based review of individual alerts... Deutsche Bank failed to
  recalibrate the system between 2011 and 2014, which meant it was significantly out-of-date" (para 4.79).
- **Unknown underlying clients.** Customers placed orders "on behalf of others whose identities and source of wealth were not
  known to Deutsche Bank" (para 2.8). NYDFS (press release): Russia was rated high-risk only in late 2014; the compliance
  function was thin ("too many jobs"; staff had to "beg, borrow, and steal" resources).

**What failed: the 2020 NYDFS order (read in full text)**

- *High-risk rating, untailored monitoring.* The bank classified the private-banking client high-risk and informally as an
  "Honorary PEP", yet "this scrutiny was not tailored to the specific risks that he posed". Conditions set by a risk
  committee were not communicated to the whole relationship team, and an AML officer read "unusual" as unusual compared with
  the client's own prior history; a monitoring analyst closed an alert on payments to a Russian model and publicity agent
  with "[s]ince this type of activity is normal for this client it is not deemed suspicious" (paras 26, 40-42). An alert on a
  trust beneficiary alleged to be a co-conspirator "was cleared citing the Approval Email" from a senior executive (para 30).
  Order: "If a financial institution decides to do business with a high-risk client, that institution is required to ...
  tailor its transaction monitoring to detect suspicious or unlawful activity based on what the risk is" (para 55).
- *Danske Bank Estonia correspondent.* Rated high-risk (score 8) from the start in 2007; "an increase in AML-related alerts"
  on non-resident customers with a Russian or Latvian connection was seen by June 2008; the plan to put the Estonian branch on
  the group IT platform "fell through in 2008", so it lacked the home office's AML checks. Between 2007 and 2015 the bank
  cleared "more than USD 267 billion in 1,638,844 transactions for Danske Estonia", of which "at least USD 150 billion" came
  from Russia and other former Soviet states, and identified 340 suspicious transactions referencing Danske Estonia's dollar
  accounts, yet kept the relationship until October 2015 (paras 84-104).
- *FBME Bank correspondent.* Rated high-risk from 2005; the bank facilitated 478,379 dollar transactions of more than USD 618
  billion; FBME "sometimes refused to disclose in writing the ultimate beneficial owners of its own corporate clients,
  explaining that such information could not be shared without violating local law" (paras 69-77); Deutsche was the last
  major Western bank still servicing it when FinCEN acted.

**What failed: the 2023 Federal Reserve order.** Examinations since 2018 found "insufficient progress" on "compliance
oversight, customer due diligence, transaction data, transaction monitoring and filtering, suspicious activity reporting, and
facilitating independent third-party reviews". The remedy is a list of system tasks: "ensuring that systems and data at the
attribute level have been comprehensively identified and assessed for gaps end-to-end ('Data Trace Analyses')", a dated plan
to fix every data gap, "effective interim controls" until then, and "ensuring that existing scenarios function as intended,
with alerting thresholds tailored to the businesses' risks".

**Root cause.** Business lines (front office, remote desks, correspondent banking) that owned the customer relationship but
not the monitoring; customer data spread over systems with no common identifier; risk ratings that did not change what was
monitored; remediation owned by programmes that did not close the data gaps.

**Design lesson**

- *One customer key across everything AKR-TMS reads.* The mapping layer must resolve a single customer, account and
  counterparty identity across the CBS, trade finance, cards and payment systems, and report the share of transactions that
  cannot be linked to a KYC record ("orphan" rate) as a control metric; transactions on orphan accounts alert by default.
- *Monitoring ownership per booking location and channel.* Every booking channel (branch, agent, digital, correspondent,
  trade desk) has a named monitoring owner in the coverage ledger; "monitored elsewhere" must be proven by data, not asserted.
- *EDD conditions are structured data.* Conditions set for a high-risk customer (specific typologies, counterparties,
  payment types, cash limits) are stored as machine-readable monitoring rules attached to the customer and displayed on every
  alert; closing an alert as "normal for this client" requires a peer and risk-reason check, and a recorded executive approval
  cannot by itself clear an alert.
- *Attribute-level data lineage.* For each scenario and screening list, AKR-TMS shows the source field, its CBS column,
  null rate and last validation date (the Fed's "Data Trace Analysis"); a scenario with an unmapped or failing input is marked
  "not operating as intended".
- *Recalibration clock.* Every scenario carries a last-tuned date and an owner; a scenario not recalibrated within the bank's
  stated interval turns amber, and red after twice the interval.
- *Correspondent and nested-relationship tracker.* Respondent banks and exchange houses carry a risk score, alert counts,
  suspicious-transaction counts and cumulative value; a rising alert trend with no change of action is a standing escalation to
  the board, and refusal to disclose beneficial owners of the respondent's clients is a stated exit trigger.
- *Remediation closes with evidence.* Regulator or audit findings are tracked as items with a test that proves closure
  (a data-trace report, a seeded test result), not as projects.

### 2.6 UBS Financial Services Inc. (FinCEN consent order No. 2026-02, listed 3 August 2026) - evidence grade A (consent order read in full text)

A 2026 case, and the clearest written account of **how a new, automated monitoring system silently dropped transactions**. The
respondent is a US broker-dealer in a Swiss banking group; the lessons apply to any bank that replaces a legacy tool or adds a
data feed. It is also a **repeat case**: FinCEN's earlier consent order of 11 December 2018 (USD 14.5 million, conduct 2004 to
April 2017) had already cited the firm for monitoring that "failed to capture certain information related to the foreign
currency denomination and the identity of certain parties to the transaction, and the jurisdictions involved".

**Facts and numbers.** Civil money penalty of **USD 125 million** (credit of USD 48 million for payments to the CFTC, SEC and
FINRA; USD 62 million to Treasury; USD 15 million due by 31 May 2028 and waivable up to the cost of an AML programme review).
Relevant period 1 January 2019 to 30 June 2023. FinCEN's root-cause sentence: UBSFS "failed to apply relevant data-governance
policies, procedures, and internal controls to the data upon which its anti-money laundering transaction monitoring process
relied" and "did not apply a coherent data management framework to this process until April 2023".

**What failed in the data and the system**

1. **A manual spreadsheet as the control (2012 to March 2021).** Foreign-currency wires on commodities accounts were monitored by a
   report built by "multiple UBSFS personnel [taking] a dozen steps to manually query four different systems and then manually
   copy and paste the corresponding data points into an Excel document"; the data were "incomplete and very messy"; "in one run
   ... nearly half of the records lacked a valid account number"; household identifiers needed for alerts were often missing; it
   was run "at best, quarterly" (2019 was covered by two runs, not four).
2. **A coding glitch and a late, private discovery.** Code in the spreadsheet "systematically undercount[ed] the value of these
   foreign currency wires for roughly two years", so a parameter of USD 300,000 a month per household failed to fire on "hundreds
   of transactions". Staff found it in late 2020, did not escalate it for about two months, and a regulator's inquiry triggered
   the look-back.
3. **Alerts closed as out of scope.** When alerts showed foreign-currency wires on non-commodities accounts, staff closed them
   "without any investigation under the rationale that they were 'generated in error' or 'not in scope of the manual control'".
4. **The new automated system (March 2021) used the wrong feed.** "UBSFS selected an incorrect, partial data feed of foreign
   currency wires, instead of a complete, end-of-day data feed." A one-month sample in early 2022 found "more than five percent of
   foreign currency wires were completely excluded from the automated monitoring system and another roughly 12% were missing
   certain counterparty information". One example of how a transaction vanished: Payment Operations "changed the label of certain
   foreign currency wires from 'FX RECEIVED' to 'FX RCVD,' which prevented them from being sent to the monitoring system".
5. **Counterparty data held apart and matched by fragile logic.** Counterparty details sat separately from the wire; the firm
   "failed to implement a unique identifier" and used "complex matching logic" in which "mismatches were likely to (and did)
   occur, such as rounding up of the exchange rate".
6. **No exception queue.** "UBSFS's data repository had no exception queue or process to catch, resolve, and report when a foreign
   currency wire could not be processed by the automated monitoring system", so unmonitored items were never counted.
7. **Controls that should have caught it did not.** Data-quality tests at validation "did not include data lineage mapping and
   testing"; monthly model performance reports "identified [no] problems"; no group "had responsibility ... for conducting full
   data lineage mapping and testing back to the sources"; internal audit reports of 2019 and 2020 found compensating controls
   adequate. The new system was promised to the regulator for mid-2019 and went live in March 2021, with internal causes given as
   "data feed problems" and "a high volume of alerts in the model calibration phase", and the delay was not reported to FinCEN.
8. **Unknown treated as moderate risk; missing addresses.** Wires whose beneficiary country the system could not recognise were
   coded "moderate" before April 2019, although a recognised high-risk country would have been high; about 20 percent of sampled
   outgoing wires lacked a beneficiary address.
9. **Case quality.** Examples of closed alerts with no meaningful analysis included a customer whose owner was already the subject
   of negative news and of an affiliate's decision to exit, pass-through patterns, and an alert escalated mainly because the
   originator was unknown where a request to the branch "was sent ... without any response". Accounts restricted to third-party-wire
   limits then sent about three-quarters (by amount) of more than USD 60 million as third-party wires.

**Root cause.** No owner for data lineage; a system accepted into production after tests that checked model behaviour, not that
every source record arrived; reliance on spreadsheets at the point where the risk was greatest; and delays not reported to the
supervisor.

**Design lesson**

- *Completeness reconciliation per source, per day.* For each CBS table or feed feeding monitoring, AKR-TMS compares record
  counts and value sums with the source and stores the result; a drop below the tolerance stops the "monitored" status for that
  source and raises a ticket (extends B-01). The reconciliation is by transaction type, not only by table.
- *Reference-value drift detection.* The mapping layer keeps the set of distinct values (transaction codes, status labels,
  channel codes, currency codes) seen for each classifying field and alerts on a new, renamed or vanished value (the "FX
  RECEIVED" to "FX RCVD" case), with the volume affected; unmapped values go to an exception queue, never to a default
  bucket.
- *An exception queue that is a first-class screen.* Every record that fails mapping, lacks a key (account, counterparty, country,
  household), or cannot be matched to a customer is listed with reason, age and value; the board sees its count and trend.
- *Data lineage as a deliverable.* For each scenario, the lineage from the CBS column to the scenario input is documented,
  tested and re-tested on every schema or mapping change (the Fed's "Data Trace Analysis", section 2.5).
- *Unknown is not low.* An unknown country, missing beneficiary address or unmatched counterparty raises the default risk of
  the transaction to at least the highest tier of the field it would have belonged to, and is counted.
- *Counterparty keys.* Where the CBS keeps counterparty data apart from the transaction, the mapping layer builds an explicit key
  and reports the match rate; rounding or reformatting of amounts is not allowed as a join condition.
- *Go-live gate for any new system or feed*: reconciliation sample over a full period, lineage report, seeded-case test, and a
  signed statement of known exclusions; delays against a date promised to a supervisor are shown in the board pack.

### 2.7 Canaccord Genuity LLC (FinCEN consent order No. 2026-01, listed 6 March 2026) - evidence grade A (consent order read in part)

A second 2026 case, from the securities side, on **review evidence that was falsified** and **analyst-level filters that cut the
alert volume**. Penalty USD 80 million (USD 5 million suspended; credit of USD 20 million each for payments to the SEC and FINRA;
USD 35 million paid to Treasury); period 2 March 2018 to 30 June 2024.

- Until late 2021, "just four Canaccord employees - each of whom maintained responsibilities other than trade surveillance -
  were tasked with reviewing more than 100 unique reports, many of which were daily reports and some of which produced thousands
  or millions of combined line items to review each year". Some reports went entirely unreviewed "for stretches of time ranging
  from months to four years".
- Reviewers "set unreasonable parameters, including to reduce the volume of activity captured in the reports". One reviewer applied a
  manual filter (executions over 1 million shares and over 50 percent of daily volume) to "manage" a report of up to 50,000 lines
  a day; another used "trial and error" to reach "arbitrary numbers" to "get the total number down"; a third report "ran more than
  50 pages each day" and was "too long" to review. A report showed daily market volume as "null" because of a data issue that the
  reviewer knew about.
- "When FINRA requested evidence of the review of these reports, two compliance employees at Canaccord falsified records": one
  "falsified nearly 400 documents", the other marked reports as reviewed that had not been and "backdated versions of certain
  Firm policies and procedures".
- A third-party assessment as late as November 2023 still found no written procedures for trade surveillance, no standards for
  investigation documentation and no quality assurance.

**Design lesson**

- *System-generated evidence of review.* The record that an alert, report or list was reviewed is created by the tool (user,
  time, items opened, time spent, decision) and cannot be edited or uploaded; policy documents carry a system-held version history
  so a date cannot be backdated. An "unreviewed items" report lists every report or queue with no recorded review in the period.
- *Report design is a threshold decision.* A manual filter or sort applied by an analyst to shorten a report is a configuration
  change: AKR-TMS offers no private filters on the scenario output; narrowing a view for workload must be done by priority
  scoring, and every saved filter is logged with its effect on volume.
- *Workload ratio.* Items per reviewer per day, with the number of distinct reports or queues each person owns, is shown to
  management; more than the bank's set limit is an open risk.
- *Data-null checks on scenario inputs.* A scenario that reads a field with a null rate above a set value is marked degraded
  on every run.


### 2.8 Capital One, N.A. (FinCEN assessment, 15 January 2021: USD 390 million) - evidence grade A (assessment read in the relevant parts)

A case in which **a coding change in the data feed removed the highest-risk customers from the monitoring report**, **a missing code left USD 16 billion of cash unreported**, and
the compliance function accepted the business line's explanations. The unit was a check-cashing book (90 to 150 New York and New Jersey check cashers with more than 1,000
accounts) inherited through acquisitions, 2008 to 2014.

- **Data-stream coding changes silently dropped customers.** In 2010 the bank built a "Large Item Report" (checks above USD 9,000) to help analysts spot suspicious activity at the check
  cashers. "In December 2010, CONA made a change to the way its transactional data streams were coded for all customers, which caused checks cashed at several CCG customers ... [including]
  all of [one individual's] check cashing businesses (despite his status as the highest risk customer in the CCG and among the highest-risk customers at CONA overall) to not appear on the LIR
  until 2013. Then in August 2012, CONA made a generally-applicable coding change, which caused all of the remaining CCG customer data streams to disappear from the LIR." Analysts flagged it in
  September 2012; the report "was not repaired until July 2013", so "from August 2012 until July 2013, the LIR was wholly inoperative".
- **A missing code unreported USD 16 billion of cash.** The CTR system "assigned a 'cash' code for customer withdrawals to effectuate CTR filings", but "failed to assign this 'cash' code to armored car
  cash shipments for a number of customers using a specific accounting method", including this book. "The failure to properly code these cash shipments resulted in approximately 50,000 reportable cash
  transactions totaling over USD 16 billion in cash to the CCG over the course of over three years to go unreported to FinCEN", although business managers received near-daily reports of the shipments.
- **Business explanations accepted at face value.** AML management "routinely instructed analysts to contact CONA's relationship manager for the CCG business line"; the business line "often suggested vague and
  implausible explanations", such as "making payroll", "hurricane Sandy work", "known to customer" or "tax refunds being cashed at the stores", and analysts "accepted such justifications ... at face value".
  The assessment says the bank "improperly used consistency as the primary benchmark for reasonableness, overlooking the nature or apparent lawful purpose of their customer's underlying activity".
- **Wrong subject on reports.** One check-casher family generated 75 alerts in 2011 to 2013; 59 became SARs, but "none named [the owner] as a subject" and 53 listed him as a victim; the bank filed no SARs on the
  book at all until October 2009.

**Design lesson**

- *The monitored population is compared with the customer base after every data-stream change.* Any change to a transaction code, source table or mapping triggers an automatic comparison of customers, accounts and
  value appearing in each scenario and report before and after, and an alert on any high-risk customer who disappears (the 2010 and 2012 changes; extends sections 2.6 and 6.1).
- *Coverage of reportable events by transaction type.* The independent recomputation of reportable cash events (section 5) includes cash-in-transit and armoured-car shipments, bank-to-bank cash movements and branch
  vault transfers, with a reconciliation to filed reports by code.
- *Business-line explanations do not close alerts.* An explanation from the relationship manager is stored as a claim with source, date and corroboration; the closing analyst must cite independent evidence, and the
  relationship manager's own explanation cannot be the only basis (extends section 2.3).
- *Consistency is not reasonableness.* The alert screen shows the customer's activity against peers and against the declared purpose, not only against its own history (compare Deutsche Bank, section 2.5).
- *STR role checks.* Pre-submission checks flag SAR/STR drafts in which the account holder of a high-risk alerted account is listed as victim, or where no subject is named (file 04 E-03).


## 3. Foreign banks clearing dollars: sanctions evasion, message stripping, lists and the Middle East

An Iraqi bank is in the position of the banks in this section: it is not American, its dollars clear through New York, and
the US authorities judge it by what it does with US-dollar payments. File 01 section 4 records the 2023 to 2025 dollar
bans and the Al-Huda 311 action; file 05 section 1e records OFAC's 2019 framework root causes. The cases below show how
those root causes looked inside the payment, screening and monitoring systems of banks with the same exposure: Standard
Chartered (Dubai and London), BNP Paribas (satellite banks), Commerzbank (cover payments), Habib Bank and the National Bank
of Pakistan (New York branches of a Pakistani bank), and Mashreq Bank (New York branch of a UAE bank).

### 3.1 Standard Chartered Bank (OFAC, DOJ, NYDFS, Federal Reserve, 10 December 2012; the same authorities on 9 April 2019; FCA decision notice 5 February 2019) - evidence grade A (OFAC settlement agreements of 2012 and 2019, OFAC enforcement notices, Federal Reserve order of 2019 and FCA decision notice of 5 February 2019 read; the 2012 NYDFS order and DOJ statements of facts not reached)

**Facts and numbers**

| Item | 2012 | 2019 |
|---|---|---|
| OFAC settlement | USD 132,000,000 (egregious; deemed satisfied by larger payments to other US authorities) | USD 639,023,750 plus USD 18,016,283 for Zimbabwe (Zimbabwe self-disclosed, non-egregious) |
| Conduct | Iran, Sudan, Libya, Burma, Kingpin; London and Dubai payment practices, 2001 to 2007 | Iran-related Dubai accounts and others, June 2009 to June 2014: 9,335 transactions of USD 437,553,380; Zimbabwe: 1,795 transactions of USD 76,795,414 |
| Other authorities | DOJ, New York County DA, Federal Reserve, NYDFS | DOJ, New York County DA, Federal Reserve (USD 163,687,500), NYDFS, FCA (GBP 102,163,200) |
| Overall | - | about USD 1.1 billion across agencies (Federal Reserve statement); USD 2.8 billion spent on financial crime compliance since 2012 (OFAC notice) |

**What failed in 2012: message stripping as a written procedure (OFAC settlement agreement)**

- From February 2001 Bank Markazi (Iran) wanted to receive oil proceeds through SCB. "SCB and Markazi began to develop
  operating procedures to mask the involvement of Iranian entities in payment instructions sent to SCB's New York Branch."
  For a payment to an Iranian bank, London sent a message with full details to the beneficiary bank's non-US correspondent
  and "a separate MT 202 to SCB NY with no mention of the Iranian beneficiary bank".
- London "set up routing rules within its payment system to route all incoming SWIFT messages from Markazi to a repair queue",
  where wire operators changed field 52 (ordering institution) to SCB London's own BIC, and a written "Quality Operating
  Procedure - Iranian Bank Processing" (issued 20 February 2004) told staff to overtype field 52 "to a '.'" and to use cover
  payments. Outgoing messages were screened against a list of OFAC-sanctioned entities kept by SCB London, which could not
  see what had been removed.
- Dubai: "SCB Dubai used cover payments to process wires through the United States" for customers dealing with Iran, Sudan
  and Libya "regardless of whether the payments involved interests of parties subject to U.S. sanctions or not"; "There was
  typically no indication of the originating customers' Iranian nexus in payment messages".
- Scale: "SCB's omission of information affected approximately 60,000 payments related to Iran totaling USD 250 billion"
  (most of them lawful under the U-Turn general licence; only 488 Iran transfers of about USD 24 million were counted as
  violations).
- Warnings ignored: the head of legal and compliance for the wholesale bank wrote in October 2005 that the procedure "is
  clearly a process designed to hide, deliberately, the Iranian connection of payments". The New York branch had a 2004
  written agreement with regulators that expressly covered cover payments and "payments with special characters", and "chose
  not to make its regulators aware of the procedures in use by SCB London".
- Screening alert error: six payments for a designated company "stopped in the bank's interdiction filter, but were released
  due to human error".

**What failed in 2019: customer location data never used (OFAC settlement agreement, Federal Reserve order, FCA decision notice)**

- **Iran-based customers using UAE accounts.** SCB Dubai held accounts for general trading companies and a petrochemical
  company whose owners and staff were in Iran. "SCB Dubai processed USD transactions ... on behalf of customers that sent
  payment instructions to SCB Dubai while physically located and/or ordinarily resident in Iran." A later search of faxes
  with country code +98, 98 or 0098 found "11,809 faxed payment instructions from 176 distinct SCB Dubai customers"; the bank
  "did not block such fax access in the UAE until May 2014". The company's own file contained Iranian mobile numbers, an
  Iranian fax number and a "Tehran Representative Office".
- **Online banking from sanctioned countries.** The bank "did not take appropriate measures to ensure that online banking
  transactions initiated from countries subject to comprehensive OFAC-administered sanctions were compliant ... until 2014",
  although by "no later than 2011" there were warnings. By September 2012 it had counted "over 3,500 discrete log-ins ... in
  comprehensively sanctioned countries throughout 2012". IT proposed "reporting only ... on the assumption there is someone
  available to take action"; business staff argued against blocking because of technical limits and "customer experience";
  "internal emails show that SCB did not leverage data from accounts accessing SCB platforms in Iran to conduct relationship
  reviews". The system could block by country and had done so in 2010 for another reason.
- **A control that staff found "overly onerous" was never run.** The UAE "Iran Addendum" (late 2009) required reviewers to
  sample payment instructions for +98 faxes; "the business raised concerns ... overly onerous and impractical for staff to
  complete. As a result this important element ... was never put into operation. No substitute check replaced it" (FCA decision
  notice, para 4.76). The UAE branches received about 35,000 faxes a month; Iran faxed payment instructions peaked at 635 in
  August 2010.
- **Periodic reviews overdue.** In June and July 2012 the UAE consumer bank had "over 1,700 overdue periodic reviews for EDD
  customers", about 43 percent of them stuck on the Iran Addendum by October 2012. In the UK correspondent banking unit 72
  percent of the highest-risk files "had not been reviewed on an annual basis" (53 percent in one sample group) (FCA 4.64-4.80).
  Staff in the UAE "accepted unconvincing information too readily from their customers" and were reported in 2011 as "more
  concerned with maintaining client relationships than with complying with financial crime policies" (FCA 4.66). One account
  was opened for a consulate with about GBP 500,000 brought in cash "in a suitcase" without establishing the source of funds.
- **Zimbabwe: a product flow nobody mapped.** Customers on the SDN list or 50-percent owned by listed persons held accounts
  with the Zimbabwe affiliate; SCBZ "ring-fenced" them, but "multiple SCBZ personnel appear to have been unaware that the
  Bank's customers ... could use their SCBZ-issued credit cards outside of Zimbabwe (including in the United States)"; the net
  settlement flowed through SCB New York.
- **Federal Reserve remedy (2019 order).** "Appropriate OFAC compliance controls, policies, and procedures governing Standard
  Chartered's customer-facing payment channels, including online and mobile banking and fax-initiated payment instructions";
  "clearly defined roles, responsibilities, and accountability for the assessment of U.S. sanctions risks arising from new
  products or services"; annual affiliate reviews with "risk-based sampling of both U.S. dollar payments and U.S. dollar trade
  finance transactions". OFAC's finding: the compliance programme "suffered from multiple systemic deficiencies, including
  failure to respond to warning signs in a timely and efficient manner".

**Root cause.** Sanctions screening looked only at the content of a message that the bank's own staff and customers could
shape; the facts that revealed the customer's real location (fax number, IP address, mobile number, office address) sat in
other systems and were never joined to screening or to the customer risk rating; controls that depended on staff
sampling were abandoned when staff said they were burdensome; business incentives overruled compliance warnings.

**Design lesson**

- *Screen the data, not only the message.* AKR-TMS builds a "customer location evidence" profile per customer from every
  read-only source it can reach: instruction channel (fax, e-mail, branch, online), phone country codes, e-banking login
  country and IP, device, addresses in KYC, beneficial owner nationality and the address in payment messages; a match to a
  sanctioned or high-risk jurisdiction (Iran for an Iraqi bank first) is an alert and a risk-rating input.
- *Payment-message integrity analytics (phase 1 on stored messages, phase 2 inline).* Per sending operator, customer and
  correspondent: rate of blank, "." or generic ordering-institution fields, cover-payment share, repair-queue diversions,
  manual overtyping, and routing or hold rules that single out a customer or bank. Any configuration that diverts a named
  party to manual handling is itself reported.
- *Manual sampling controls are not controls.* Whatever the "Iran Addendum" asked staff to check by hand (country code of the
  fax number) must be an automated rule over the data; if no data exist, the gap is shown in the coverage ledger.
- *Filter releases need four eyes and a reason code*, with a monthly sample re-review of releases.
- *Overdue review ageing as a board metric.* Periodic and trigger reviews that cannot be completed for a stated reason (a
  customer will not give information) move to an "exit candidate" queue with a deadline.
- *Product-flow map for sanctions exposure.* Each product (cards, trade, remittance, e-wallet) gets a record of where it can
  settle and in which currency; a new product cannot be marked live in AKR-TMS until its sanctions and monitoring owner is
  recorded (the Fed's "new products or services" requirement).
- *Whistleblowing and warning register.* Concerns raised by compliance, audit or IT (for example the 2005 note and the 2012
  IP-address emails) are logged with a status and age; open items older than a set number of days go to the board committee.

### 3.2 BNP Paribas (DOJ, NYDFS, Federal Reserve, OFAC, New York County DA, 30 June 2014) - evidence grade A for the OFAC settlement agreement and enforcement notice (read in full); B for the NYDFS press release; the DOJ statement of facts was not reachable

**Facts and numbers.** Total about USD 8.9 billion (NYDFS press release), of which NYDFS USD 2.24
billion and OFAC USD 963,619,900 (deemed satisfied by DOJ and DA payments). OFAC counted 3,897 apparent violations:
Sudan 2,663 wires of about USD 8.37 billion (September 2005 to July 2009), Iran 318 wires of about USD 1.18 billion (July 2005
to November 2012), Cuba 909 wires of about USD 689 million, Burma 7 wires. NYDFS: the bank concealed "USD 190 billion in
transactions between 2002 and 2012"; sanctions included a one-year suspension of dollar clearing in six business lines and
locations and a two-year bar on correspondent banking for unaffiliated third-party banks; 13 people were dismissed and 45
disciplined. OFAC base and statutory maximum penalty: USD 19,272,380,006.

**What failed (OFAC settlement agreement, paras 3-17)**

1. **Message content was shaped by instruction.** "BNPP appears to have engaged in a systematic practice, spanning many
   years and involving multiple BNPP branches and business lines, that concealed, removed, omitted, or obscured references
   to ... sanctioned parties in USD SWIFT payment messages." Methods: "omitting references to sanctioned parties; replacing
   the names of sanctioned parties with BNPP's name or a code word; and structuring payments in a manner that did not
   identify the involvement of sanctioned parties". Sanctioned customers sent orders to the Geneva front office, which
   added its own instructions for the back office. Locations used the transparent MT 103 for non-sanctioned parties and the
   cover payment MT 202 "for transactions involving sanctioned parties", and filled mandatory fields "with BNPP's name if the
   originating financial institution was a sanctioned entity".
2. **Regional banks as a screen (a nested-correspondent pattern).** By January 2005 the Geneva subsidiary had opened dollar
   accounts for nine banks in third countries whose KYC papers said the accounts were opened "in the context of the US
   embargo against Sudan" to facilitate "transfers of funds in USD for Sudanese banks". A Sudanese customer's instruction
   produced a book transfer to a regional bank's account, then a dollar clearing payment in the regional bank's name "without
   referencing sanctioned parties or Sudan"; NYDFS adds that transfers were delayed "one to two days" to blur the link. For
   foreign exchange, the two legs "had the same value dates and exchange rates".
3. **Compliance warned, management waved it on.** A Geneva compliance officer wrote on 5 August 2005: "this practice
   effectively means that we are circumventing [or avoiding] the US embargo on transactions in USD by Sudan." At a September
   2005 meeting the group's chief operating officer reportedly asked whether "Switzerland has declared an embargo on Sudan"
   and "asked that no minutes be taken". Group policy of June 2007 banned Sudan dollar business; processing continued "through
   at least December 2008". A Cuba credit facility stayed in dollars until 2012. In September 2004 Geneva moved dollar
   clearing to another bank's New York branch "to shield BNPP New York from liability".
4. **Structured data ignored at the decisive moment (an Iraq-related flow).** A company incorporated in the UAE (Dubai
   address) was part of an Iranian energy group of eight companies, four incorporated in Iran, owned by an Iranian resident;
   it received payments for "its sale of Turkmen liquefied petroleum gas to Iraq". Between November 2008 and November 2012
   BNPP processed 114 payments of about USD 415 million for it through the United States; messages named the company but
   not Iran. When a US correspondent held a payment "due to OFAC concern" in January 2012, a BNPP compliance officer approved
   the company's denial of any Iran link, sent from an e-mail address of the Iranian group, "without verifying the
   information or consulting the customer profile form", which carried a handwritten note "Iranian ownership". Nothing was
   investigated: 64 further payments of over USD 292 million followed until group compliance learned of them in November 2012.
5. **Consequence for screening.** By using cover payments for sanctioned parties BNPP "effectively removed, omitted,
   obscured, or otherwise failed to include references to sanctioned parties in certain USD payments sent to U.S. clearing
   banks", so those banks' filters had no sanctioned name to match.

**Root cause.** A commercial strategy for sanctioned countries run through a small number of units with their own back
offices; group compliance policy without automated enforcement; compliance warnings recorded but not enforced; KYC facts held
in non-structured forms that the person answering a correspondent could not or did not see.

**Design lesson**

- *Nested pass-through detection on the ledger.* AKR-TMS tests every account held by another bank or exchange house for
  matched in-and-out pairs (same amount, same or next-day value date, different counterparty names) across a one to three day
  window, and for accounts whose KYC purpose or free-text mentions embargo, sanctions, OFAC or "avoid" language; a flagged
  account goes to the correspondent register (section 4 and file 04 W-22).
- *Evasion-language search.* KYC notes, payment narratives and message free-text fields (Arabic and English) are searched for
  terms such as sanction, embargo, OFAC, "do not mention", "no name", "code", with the hit shown on the case.
- *Party card at the point of decision.* The screen used to answer a correspondent's RFI or to release a held payment shows
  structured ownership, nationality of the owners, sanctions flags, and past rejects for the party, and the answer must
  acknowledge them. Ownership facts that exist only as a note on a form are a data-quality finding.
- *A returned or held payment is a trigger event.* Any payment rejected, returned or queried by a correspondent for sanctions
  or compliance reasons opens a case on the customer and runs a look-back for similar payments (64 payments after the
  January 2012 hold); the case cannot close on the customer's own explanation.
- *Policy to rule mapping.* Each prohibition in the bank's policy (for example "no dollar business with sanctioned
  country X") must map to a named detection rule with a test result; policy statements with no mapped rule show as gaps. The
  Sudan policy of June 2007 was breached until December 2008.
- *Decision register.* Decisions to continue, restrict or exit a high-risk relationship are recorded in the system with
  attendees, rationale and approver, and cannot be recorded as "no minutes".
- *Iraq typology.* Energy and commodity payments to or from Iraq through UAE-incorporated companies with Iranian owners belong
  in the Iran-nexus pack (files 04 U-08 and 05 section 1e), with ownership evidence required before release.

### 3.3 Commerzbank AG (NYDFS, DOJ, Federal Reserve, OFAC, Manhattan DA, 12 March 2015) - evidence grade A (NYDFS consent order read in full text); B for the DFS press release figures

**Facts and numbers.** Total USD 1.45 billion: DFS USD 610 million, SDNY USD 300 million, Federal Reserve USD 200 million,
Manhattan DA USD 172 million, DOJ USD 172 million (DFS press release). Iran: "60,000 transactions valued over USD 253
billion" for Iranian entities between 2002 and 2008, with a designated team manually stripping identifying information;
Sudan: accounts for 17 Sudanese banks and about 1,800 transactions of more than USD 224 million (2002 to 2006). Separately,
more than USD 1.6 billion supporting a Japanese company's accounting fraud passed through the New York branch between 1999
and 2010. Five employees were dismissed, the head of AML compliance for the branch resigned, and an independent monitor was
installed. The sanctions conduct is the same pattern as sections 3.1 and 3.2; this section concentrates on the monitoring
failures in the consent order.

**What failed in the monitoring system and operation (consent order paras 3-13, 23-29)**

1. **Incomplete payment data from overseas branches.** "Foreign branches often transmitted payment requests to Commerzbank's
   New York Branch using non-transparent SWIFT payments messages that did not disclose the identity of the remitter or
   beneficiary. As a result ... fewer alerts or red flags were raised than would have been if full information had been
   shared" (para 4).
2. **Alerts could not be investigated.** New York staff did not have the customer information; they "had to request relevant
   information directly from the foreign branch or from the Home Office in Frankfurt", and overseas staff "often did not
   respond ... for many months or sent inadequate or insufficient responses", which "led to alert backlogs" (paras 5-6).
   Overseas staff felt New York staff were "crying wolf". "On some occasions ... New York staff 'cleared' or closed alerts
   based on its own perfunctory internet searches ... without ever receiving responses" (para 8).
3. **Business overruled compliance on the monitoring list.** On at least two occasions New York staff tried to add foreign
   branch clients judged high-risk to the transaction monitoring filters; "employees in Germany initially overruled those
   steps and forbade the New York Branch from adding client names to its filters on its own initiative" (para 9).
4. **Thresholds were set and changed to reduce alert volume.** Under the heading "Compliance Management Altered the
   Transaction Monitoring System to Reduce Alert Output": "the threshold floors were driven by the volume of the output of
   alerts - that is, the threshold floors were set based on a desire not to generate 'too many alerts'" (para 11). The head
   of regional compliance "required a weekly update as to the number of alerts generated" (para 12) and "in 2011, both the Head
   of Regional Compliance and the Head of AML Compliance asked him to change the thresholds in the automated system to reduce
   the number of alerts generated. The compliance staff member reported that he refused to do so" (para 13).
5. **Missing fundamental rules.** A 2010 joint examination found that the system "lacked 'other fundamental rules typically
   used to monitor wire activity'", which "heightened the risk that [the Bank] did not detect suspicious activity for
   approximately nine months" (para 28).
6. **A USD 1.6 billion fraud that did not alert.** The transactions linked to the fraud (1999 to 2010) "did not trigger alerts
   in the New York Branch's transaction monitoring system"; when two large payments did alert in March 2010, Singapore
   compliance, which had already investigated and closed the same alerts, passed on none of its concerns, and New York
   cleared the alerts on a brief e-mail (paras 23-24). Concerns of Singapore staff over the years "were not shared with
   relevant staff in New York" (para 17).
7. **Understaffing and slow remediation.** A compliance officer who joined in late 2009 said the New York department "did not
   have an adequate number of employees or sufficiently experienced personnel" (para 10); examiners were still finding
   deficiencies in July 2014.

**Root cause.** A branch that was monitored on the basis of messages the group's other offices controlled; head-office
business lines with the power to overrule monitoring decisions; and alert volume used as the performance measure of the
compliance function, which in turn drove threshold changes.

**Design lesson**

- *Reason codes on every threshold change.* A change is recorded as risk-driven (new typology, below-the-line finding, new
  data) or capacity-driven (reduce workload). Capacity-driven changes require below-the-line evidence, a second approver
  outside compliance operations, and appear in the board report; a verbal request to lower thresholds has a place to be
  logged (the 2011 request was refused by a staff member and recorded only in an interview).
- *No alert-count KPI for the owner of the thresholds.* Weekly alert counts are shown with productive-alert yield and ageing,
  never alone; AKR-TMS dashboards do not rank analysts or compliance heads by alerts reduced.
- *Only compliance edits the monitoring list.* Adding a customer to enhanced monitoring needs no business approval; the
  role model gives the business read access only, and any business objection is recorded as a challenge item.
- *Internal request-for-information tracker.* RFIs from analysts to branches, subsidiaries and relationship managers have a
  deadline, escalation after N days and a visible "waiting for information" age; an alert closed without the requested
  answer is marked "closed without information" and counted in the backlog metrics (CBI branch liaison officers, file 01
  section 2d, become users of this tracker).
- *Coverage inventory against a reference catalogue.* AKR-TMS compares the bank's active scenarios with the CBI scenario
  catalogue (file 27) and the typology list, and reports "fundamental rules missing" (for example a wire velocity rule).
- *Staff concerns reach the AML function.* Concerns recorded by staff in any unit (branch, subsidiary, exchange-house
  affiliate) are logged as internal reports with acknowledgement and decision, as the CBI controls require, and are
  linked to alerts on the same customer so a closing analyst sees them.
- *Message-quality monitor* (file 04 W-12) as the control for "non-transparent" payment messages: share of messages with
  missing or generic originator and beneficiary data by source branch.

### 3.4 Habib Bank Limited, New York branch (NYDFS, 7 September 2017) - evidence grade B (DFS press release; the 2017 consent order PDF is a large scan that could not be fetched; the December 2015 consent order was read and is generic)

**Facts and numbers.** USD 225 million penalty; the branch had to surrender its New York licence and wind down in an orderly way,
and run an expanded look-back covering October 2013 to July 2017. DFS found that the bank "allowed for at least 13,000
transactions to flow through the New York branch that potentially omitted information adequate to properly screen for
prohibited transactions". A "Good Guy" list of customers was used to bypass screening, which "permit[ted] at least USD 250
million in transactions without any screening, including transactions by an identified terrorist, an international arms
dealer, an Iranian oil tanker". The bank also "facilitated billions of dollars in transactions with a Saudi private bank, the Al
Rajhi Bank, with reported links to al Qaeda, without adequate anti-money laundering controls". A 2016 examination gave the
lowest rating ("5") because of weak BSA/AML compliance, "inadequate customer risk ratings", weak OFAC screening, insufficient
training and poor senior management oversight. This was not the first action: the branch had a written agreement of 19
December 2006 that it "had not achieved full compliance" with by the consent order of 15 December 2015.

**What failed in the system**

1. **A suppression list outside the screening control framework.** The "Good Guy" list was used to bypass screening: payments
   by "an identified terrorist, an international arms dealer, an Iranian oil tanker" passed "without any screening". The press
   release does not describe how names were added or reviewed and the order itself could not be read, so the mechanics (who
   could add a name, whether entries were ever re-tested against updated lists) are not stated here.
2. **Incomplete screening data accepted.** At least 13,000 payments "potentially omitted information adequate to properly
   screen" and went through; the controls did not stop messages that lacked what screening needed.
3. **Customer risk rating inadequate** for a branch whose business was foreign-bank and trade clearing for high-risk markets.

**Root cause.** Operations pressure to reduce false positives (the purpose of a good-guy list) without governance of the list
itself, a screening configuration that failed open on incomplete data, and a head office and branch that did not act on a decade
of supervisory warnings.

**Design lesson**

- *Good-guy lists are governed objects with a re-test.* Every entry is pair-keyed (the specific customer and the specific
  list entry it matched on named fields), approved by compliance (maker-checker), time-limited, and re-evaluated automatically
  whenever either side changes: any list update re-screens every whitelisted pair (a new designation or a changed
  identifier reopens the decision), and any change to the customer's attributes (address, vessel, owner) reopens it. A
  whitelisted pair suppresses only that pair; it never exempts a customer from screening.
- *Report on the whitelist.* AKR-TMS shows number of entries, age, approver, date of last review, volume and value that passed
  because of the list in the last 12 months, and the number of entries that now match a current designation (must be zero).
- *Screening fails closed.* A payment, onboarding or periodic screening record missing a screening-critical field (name of
  originator or beneficiary, country, vessel identifier in trade items) is held or flagged, never silently passed; the
  share of incomplete records per source system is a control metric (file 04 W-12).
- *Vessel, bank and country identifiers are screened*, not only person names (the "Iranian oil tanker").
- *Findings that recur are escalated.* The system compares new examination or audit findings with older ones and shows the
  recurrence count to the board (see NBP, section 3.6).

### 3.5 Mashreqbank PSC, New York branch (NYDFS, 10 October 2018: USD 40 million; 26 October 2021: USD 100 million) - evidence grade A (both consent orders read in full text)

The Dubai bank is the closest peer to an Iraqi bank in this file: Middle East, USD clearing for high-risk regions, regional
correspondents, and a record of cover payments for a sanctioned neighbour.

**2018 order: the monitoring failures.** The branch cleared "more than 1.2 million USD transactions with an aggregate value of
over USD 367 billion" in 2016 and over USD 350 billion in 2017, for about 139 foreign financial institution customers, "nearly
two-thirds" of them high-risk, in "Southeast Asia, the Middle East and Northern Africa" (paras 2-3). Findings:

- A system "with some manual components, which was plainly inadequate to address risks associated with an annual USD
  clearing volume of USD 300 billion", which did not "make appropriate use of relevant information in Know Your Customer
  files, including documentation detailing the customer's line of business and anticipated activity" (para 8).
- "The same analyst conducted first- and second-level reviews of the same transaction, defeating the purpose of a second-level
  review"; alert dispositions used "broad, generic language" (para 9).
- "The New York Branch had accumulated a three-month backlog in its generation of any transaction monitoring alerts", with
  1,500 to 1,600 alerts a month (para 11); by 2017 nearly 2,000 a month, each alert "reviewed only once by a single reviewer",
  and "quality reviews of alerts suffered from a five-month backlog" (paras 15-16). Rationales for closing alerts lacked
  "essential information, such as the relationship between the transacting parties" (para 15).
- Rules "did not adequately address the Branch's customer base and transaction volume, and were not properly calibrated to
  monitor risks associated with certain common scenarios in foreign correspondent activities"; a third-party vendor's
  validation was "deficient, doing little more than summarizing the interim rules and indicating their purported adequacy" (para
  17). The branch had also "failed to properly set the sensitivity settings for screening and filtering U.S. dollar
  transactions" (para 6).
- OFAC alert files could not "substantiate or provide documented rationales for waiving specific alerts and cases" (para 19).
- The external auditor engaged for the 2017 audit "merely signed off on documentation provided by management", with an OFAC
  alert sample "too small"; temporary staff hired to clear the backlog lacked adequate training (paras 20-21 and note 3).

**2021 order: Sudan, cover payments and a legal opinion.** Between January 2005 and February 2009 the bank "sent 1,747 outgoing
Sudan-related payments, worth over USD 4 billion" through US institutions; from 2009 to 2014 the New York branch processed 91
further prohibited payments of about USD 2.5 million (para 8). Mashreq London used cover payments (MT 202) for a Sudanese bank
"by purposefully failing to populate optional field 52 (originating institution)"; "the intermediary banks in the payment chain
did not interdict these payments" and the payments passed one US bank's OFAC filters "without triggering alerts or freezes"
(paras 10-13). Before 2001 staff were told to omit "any information (about Sudan) which could trigger [a] freeze" and to enter
"Mashreqbank" or "self" as ordering party (paras 19-20); from May 2001 Sudan payments were screened manually "against a hard-copy
of the OFAC SDN list" and field 52 was left blank on messages routed through a Swiss bank (paras 21-22). A Dubai law firm's
opinion that US sanctions did not directly apply to the non-US bank was relied on, although the opinion itself warned that the US
branch must be vigilant (paras 23-26). A New York compliance employee wrote in April 2001 that incomplete originator and
beneficiary details were "a direct violation" of the Bank Secrecy Act; in June 2002 a risk employee recorded that the heads of
compliance, risk and audit had agreed the procedure "is not compliant as it suppresses some fields", and "no changes to the
operating procedures were made" (para 27).

**Root cause.** A high-volume correspondent business run on a system with manual parts, thin staff and untested rules;
oversight (external audit, vendor validation) that signed off on management's documents; an older practice of omitting
fields for sanctioned countries that survived warnings from within.

**Design lesson**

- *Two clocks per alert.* AKR-TMS measures data latency (transaction time to alert creation) and handling time (alert creation
  to disposition) separately; a gap between a transaction and its alert of more than the bank's set limit is a control
  failure displayed beside the alert queue (the "three-month backlog in generation").
- *The tool enforces first and second line.* The same user cannot perform level 1 and level 2 on one alert; the quality-
  control sample has its own queue and age; QC backlog age is reported.
- *Structured disposition.* Closing an alert requires fields (relationship between the parties, expected versus actual
  activity, source of funds evidence, counterparty bank and country, decision, rationale), a generic-text check, and for
  sanctions alerts a coded rationale list; "waived" requires a named approver and evidence.
- *Validation by replay.* The validator workspace re-runs each rule on the bank's own history, shows alert counts and
  below-the-line samples, and stores the result; a validation consisting of a document review is flagged "not tested".
- *New analysts are not given queue access until their training and supervisor sign-off are recorded.*
- *Instruction channel and message field analytics* for correspondent flows: share of blank field 52, "self" or own-bank
  names as ordering party, by customer, branch and operator (section 3.1).
- *Legal opinions are evidence, not controls.* An external opinion that excuses a practice is stored with the facts given to
  the adviser and expires on a stated date; it cannot switch off a detection rule.

### 3.6 National Bank of Pakistan, New York branch (NYDFS, 16 February 2022) - evidence grade A (consent order read in full text)

**Facts and numbers.** USD 35,000,000 penalty. NBP is a state-majority-owned bank (over USD 20 billion in assets; 1,500 branches
in Pakistan; a New York branch with about USD 188 million in assets). DFS found "serious deficiencies" that "persist over
numerous examination cycles": the BSA/AML programme was rated inadequate in six consecutive examinations (2014 to 2019) despite
a written agreement of 3 March 2016, and the branch had by then reduced its business to treasury services, select trade and
servicing the State Bank of Pakistan.

**What failed**

- A **new remittance service launched in 2014 "without appropriate safeguards such as identification and verification
  processes or formalized risk-based due diligence controls"**; no comprehensive risk assessment (paras 5-6).
- Findings repeated year after year in "management oversight, independent testing, transaction monitoring, and customer due
  diligence" (para 13) and "governance, transaction monitoring, due diligence, customer reviews, and monitoring and screening"
  (para 14); "the internal audit function failed to track and validate remedial actions from prior examinations" (para 13).
- Management kept the branch manager (in place since about 2008) and the senior compliance officer (since 2016) "despite their
  demonstrated track record in failing to fix comprehensively the issues"; "a local management team that failed to communicate
  fully the appropriate risk to Head Office" (para 14). Compliance staff rose from seven in May 2020 to 24 in November 2021
  (para 15). DFS concluded that the bank "failed to take heed of repeated and increasingly urgent regulatory warnings" and
  "failed to adequately supervise the New York Branch" (para 17).

**Root cause.** A head office that treated a small foreign branch as low priority, no independent tracking of remediation, and
product launches that preceded controls.

**Design lesson**

- *Regulatory findings register with independent validation.* Findings from CBI examinations, correspondent audits, external
  audit and internal audit are logged once, tagged by theme (monitoring, screening, CDD, governance), owned, dated and closed
  only after a test that an independent function accepts; recurrence across cycles is counted and shown to the board and,
  on request, the supervisor.
- *Product launch gate.* A product, channel or service (remittance, wallet, card, mobile payments) cannot be marked live in the
  AKR-TMS coverage ledger without an approved risk assessment, a monitoring decision and an identity-verification standard;
  the gate record is evidence (relevant to the CBI e-payment controls, file 01).
- *Head office visibility of branch risk.* Branch- and subsidiary-level dashboards (alerts, backlog, overdue reviews, rejected
  payments, findings) roll up to a head-office view; a risk reported only through the branch manager is a design smell.
- *State-owned banks.* State-majority-owned banks (the Iraqi state banks are the analogue) need the same findings tracker and a
  board-level owner with a reporting line independent of the ministry.

## 4. Europe: groups, branches and data

Three cases where the failure was structural: a branch that was never on the group's systems (Danske Estonia), a payment channel whose
customer location data never reached screening (Swedbank Latvia), and a monitoring tool whose parameters were set by staff capacity
(ING). A fourth Dutch case (ABN AMRO, 2021) is listed in section 4.4 as not verified.

### 4.1 Danske Bank, Estonian branch (Bruun & Hjejle report 19 September 2018; resolution with the DOJ, SEC and Danish authorities 13 December 2022) - evidence grade A for the 2018 report (read in part: executive summary and chapter 9), B for the 2022 resolution (Danske Bank company announcement)

**Facts and numbers.** A branch inherited in 2007 with a "Non-Resident Portfolio" of about 10,000 customers (about 15,000 under
investigation) from Russia, other CIS countries and elsewhere. "From 2007 through 2015 ... the flow converted into EUR ... was
approximately EUR 200 billion", through about 7.5 million payments (9.5 million for the wider 15,000), mostly in USD and EUR. The
bank paid a total of USD 2.06 billion in December 2022: DOJ USD 1,209 million, SEC USD 178.6 million, Danish authorities DKK 4,749
million; it pleaded guilty to conspiracy to commit bank fraud (it had misled US correspondents about the branch's controls). Its
dollar correspondent, Deutsche Bank, cleared more than USD 267 billion for the branch (section 2.5).

**What failed in the systems and operation (report, executive summary and chapter 9)**

1. **A separate IT platform and no group view.** "The Estonian branch had its own IT platform. This meant that the branch was not
   covered by the same customer systems and transaction and risk monitoring as Danske Bank Group ... and it also meant that Group
   did not have the same insight into the branch." In August 2008 the plan to migrate the Baltic business to the group platform was
   "abandoned ... Migration was then found to be too expensive and take up too much capacity. It was made clear at the time that the
   cancelled IT migration called for additional initiative in the area of compliance" (section 9.3.1; summary para 16).
2. **Screening missing.** The report's table of failings lists "No screening of customers against lists of politically exposed
   persons", "No screening of incoming payments against sanctions or terror lists" and "In general, no automatic screening of
   incoming payments". Group compliance reports of 2013 stated the lack of screening of incoming payments, and the branch's risk
   analysis was still green.
3. **Manual monitoring that could not work.** Group Internal Audit wrote on 13 January 2014 that "ongoing monitoring" was done
   manually by account managers who were responsible for so many customers that it was "in fact impossible to perform the monitoring
   in an effective and efficient way".
4. **Customers that were not customers.** The portfolio "included so-called intermediaries, which were unregulated and represented
   unknown end-customers"; beneficial owners and "controlling interests" were not identified; source and origin of funds were not
   identified; the branch did not respond to suspicious customers and transactions.
5. **Reports that said green.** Branch AML reports were "predominantly green", Group Internal Audit rated the branch's AML
   procedures "satisfactory" in April 2008, and most business presentations on the branch carried "little or no information about the
   Non-Resident Portfolio" although it dominated the branch's profit. Documents were in Estonian or Russian. "All control functions
   (or lines of defence) had failed" (summary para 9). Group received information from the Russian central bank, passed on by the
   Danish FSA, pointing to "billions of rubles monthly" of suspicious payments, and "Danske Bank missed this first real opportunity".

**Root cause.** A profitable portfolio, a branch outside the group's systems because of cost, and reporting that depended on the branch
describing its own controls.

**Design lesson**

- *Entity and system coverage ledger.* Every branch, subsidiary, agency and affiliate of the bank appears with its CBS or other
  source system and the date its data entered AKR-TMS; any unit not connected is shown as "dark" with its transaction volume where
  known. A decision not to integrate a unit is a risk acceptance with an owner, expiry date and named compensating controls (extends
  section 2.1).
- *Screening covers every payment direction.* Incoming, outgoing, domestic and cross-border payments, and every customer, owner and
  related party; the coverage ledger reports each direction separately.
- *Control status computed from evidence.* A control is green only if its latest test result (reconciliation, seeded-case test,
  sample review) is in date and passed; a control that does not exist cannot be reported green. Self-assessments cannot override.
- *Segment concentration.* AKR-TMS reports each customer segment's share of volume, profit-proxy (fee income where available) and
  alerts; a segment that is a large share of profit and of high-risk flows (non-resident, intermediary, exchange house) is flagged to
  the board.
- *Intermediaries and unknown end-customers.* Accounts used as pass-through for third parties are identified by behaviour (many
  unrelated counterparties, matched in and out) and by customer type; they require end-customer data or are restricted.
- *External signals register.* Signals from regulators, correspondents, foreign FIUs and the press (here a Russian central bank
  warning relayed by the Danish FSA) are logged with an action and a closing decision, and counted in the board report.
- *Language is part of oversight.* Documents in the evidence vault carry a language tag and, for those not in the head office's
  working language, a required English or Arabic summary (Arabic, English and Kurdish in Iraq), so that a unit's local language
  cannot hide its files from group or supervisory review.

### 4.2 Swedbank (Finansinspektionen, March 2020; OFAC, 20 June 2023) - evidence grade A for the OFAC enforcement release (read in full); C for the Swedish group-level facts (secondary summary; the FI decision was not reachable)

**Group level (C).** Swedish and Estonian regulators fined the group SEK 4 billion (about USD 380 million) in 2020 over failings in its
Baltic operations; at least SEK 40 billion moved between Swedbank and Danske Bank accounts in the Baltics from 2007 to 2015; the
bank's former CEO was convicted in September 2024 of gross fraud in connection with her handling of anti-money-laundering matters in
Estonia (secondary source; not verified in the primary record).

**Latvia, 2023: customer location data never reached screening (OFAC enforcement release, 20 June 2023).** Swedbank Latvia AS settled
for USD 3,430,900 for 386 apparent violations of the Crimea sanctions, total USD 3,312,120, between 5 February 2015 and 14 October
2016. The owner of three shipping companies in Crimea used the e-banking platform "from an IP address in Crimea" to send payments
to Crimea through US correspondents. In March 2016 a US correspondent "rejected the payments citing a potential connection to
Crimea and alerted Swedbank Latvia"; the customer "falsely assured" the bank that none involved Crimea, and "a relationship manager at
Swedbank Latvia re-routed the rejected payments to a different U.S. correspondent bank, which ultimately processed the transactions".
OFAC: "When Swedbank Latvia onboarded the Client and the SPCs, Swedbank Latvia obtained Know Your Customer ('KYC') data, including
addresses, telephone numbers, and a customer questionnaire, clearly indicating that the Client and the SPCs had a physical presence in
Crimea ... Although Swedbank Latvia collected and stored customer IP data, it did not integrate this IP data into its sanctions'
screening processes." Remediation: "geofencing that prevents customers from sending payments through online banking platforms from
IP addresses in comprehensively sanctioned jurisdictions"; "an automated system control within their transaction screening solution to
identify potential resubmissions of payments after rejection"; enhanced due diligence for high-risk customers paying in dollars; and
"enhanced diligence and transparency protocols for responses to correspondent banks". OFAC's stated lesson: "ensuring that KYC
information (such as passports, phone numbers, nationalities, and addresses) and IP data are appropriately integrated into sanctions
screening protocols."

**Design lesson**

- *Rejected-and-resubmitted payment detector.* AKR-TMS reads payment, return and rejection records and flags any payment that is the
  same or near-same (beneficiary, amount, reference) as one rejected, returned or held by a correspondent, especially when routed through
  a different correspondent or by a different user; release needs compliance approval (phase 2: inline block).
- *Location evidence in screening* (as section 3.1): KYC addresses, phone prefixes and e-banking IP data feed the sanctions and
  geographic-risk rules; the share of customers and payments with each evidence type available is a coverage metric.
- *Answers to correspondents are controlled.* A response to a correspondent's query is drafted from system facts, shows the customer's
  KYC location data and previous rejects, and records the approver (file 04 W-23).
- *Relationship managers cannot re-route.* In the audit trail, any change of correspondent, route or instruction on a payment after a
  rejection is a reportable event with the user and reason.

### 4.3 ING Bank N.V. (Dutch Public Prosecution Service settlement, 4 September 2018) - evidence grade A (statement of facts "Onderzoek Houston", in Dutch, read in the relevant chapters); B for the amounts (OM press release)

**Facts and numbers.** EUR 775 million: a fine of EUR 675 million and EUR 100 million for confiscation of unlawfully obtained benefits.
Period 2010 to 2016 at ING Netherlands (the Wwft, the Dutch AML Act). The OM found "ernstige tekortkomingen" in seven areas: missing or
incomplete CDD files; wrong risk classifications; failing periodic and event-driven reviews; late exit of customers; an insufficiently
functioning transaction monitoring system; clients placed in wrong segments; and too little qualified staff.

**What failed in the monitoring system (statement of facts, section 3.7; translations by the author)**

1. **Daily caps on alerts ("aftoppen").** "ING NL used a method of limiting alerts that was called 'aftoppen' or 'toppings'. The system
   was set so that for certain categories of money-laundering signals, each day after a predetermined maximum number of alerts (potential
   money-laundering signals) the system stopped monitoring on those categories. The maximum number of alerts was limited to just three
   per day for several relevant categories." "The maximum number of alerts the system produced was determined to a large extent by the
   staff capacity ING had available." An internal advice read: "Turn the parameters to cap off the (over)flow of alerts and so limit the
   workload", annotated: "is already being done".
2. **Ranking by relative change, not size.** Accounts were selected and sorted by percentage deviation from historical account activity,
   not by absolute amount: the statement's example is a EUR 100 account with a EUR 10,000 transaction (100 times) ranking above a EUR
   1,000,000 account with a EUR 99,000,000 transaction (99 times).
3. **Account level, not client level.** "Monitoring took place at account level and not at client level", so small amounts spread across
   several accounts of one client ("smurfing") were not seen.
4. **Incomplete input data** "for correct (and risk-based) monitoring", and no review of the system's effectiveness in 2010 to 2016: "ING
   NL hardly adjusted its criteria to developments in money laundering".
5. **Only the tip of the iceberg.** An internal memorandum: "For years we only monitored the tip of the iceberg without taking samples
   in the remaining alerts, which could have given us a picture of the quality and effectiveness of our monitoring programme and the
   risk views."
6. **No stored results.** Footnote 13: "The monitoring system does not retain results. To re-investigate, all hundreds of millions of
   historical transactions over the period must be loaded into the monitoring system again."
7. **Wrong segment, weaker rules.** The monitoring criteria differed per client segment; clients were placed in segments on their own
   declaration and not re-checked, so high-risk clients (for example Dutch shell companies of foreign owners managed by trust offices)
   sat in segments with weaker diligence and monitoring.
8. **Cases.** A Curacao company received about EUR 150 million in card credits (May 2010 to 2014); the monitoring generated 49 alerts
   from 2010 to 2013, "all ... closed, virtually without further investigation, as 'not suspicious'", and the first report to the
   FIU came more than three years after the first alert (July 2010 to 1 August 2013). A sole trader that was in reality a money exchange
   office connected 15 mobile card terminals and moved more than EUR 9 million; an importer (company F) made 90 cash deposits totalling
   EUR 343,035, including EUR 500 notes, with no alert at all, and its sister company (A) made 41 deposits of EUR 164,530 and produced
   one alert, on 1 December 2014, after a police request; a telecom group's subsidiary paid USD 55 million of bribes through an ING
   account in 2007 and 2011 and no alert was generated.

**CDD data.** A remediation of more than one million clients onboarded before 2007 ran from 2011 to 2014; an internal audit found at the
end of 2016 that "in October 2016 new clients, both private and SME, had been accepted without CDD" because of "errors in the onboarding
process" that control systems had not detected; signals such as FCRM alerts and law-enforcement information requests often did not
trigger a CDD review.

**Root causes named by the OM:** insufficient attention and priority ("tone at the top"), "business above compliance", no sustainable
solutions, dysfunctional internal controls and silos ("verzuiling"), and no culture of escalation.

**Design lesson**

- *No daily or periodic cap on alerts, ever* (see section 2.2): AKR-TMS has no parameter limiting the number of alerts per scenario
  per day; if volume exceeds capacity the surplus is queued with an age clock. The configuration screen rejects a cap, and the
  detection engine logs any rule that has produced the same count for N consecutive days (a flat-line check that exposes a hidden cap).
- *Rank by risk score, not by relative change alone.* Prioritisation combines relative deviation, absolute value, customer risk,
  counterparty risk and typology; the ranking formula is documented and tested for the EUR 100 versus EUR 99 million case.
- *Client-level aggregation.* Every scenario that can be aggregated runs at customer (and related-party group) level across all
  accounts and products, not only per account (the CBI requires linkage across accounts, file 01).
- *Below-the-line sampling of the remaining alerts* is a monthly scheduled job with a report to the board (sections 2.2 and 3.3).
- *Keep scenario results.* Every run stores the triggering transactions, scores and parameters so a look-back can be done from the
  stored results without reloading history.
- *Segment membership is verified from data.* Segment and risk class are recomputed from observed turnover, products and counterparties;
  disagreement with the declared segment is an alert on the customer record.
- *Alert quality sample.* A statistical sample of closed "not suspicious" alerts is re-reviewed by a second analyst each month; the
  failure rate is reported (the Curacao case had 49 closed alerts).
- *Time-to-report metric.* Days from first alert on a customer to the first STR (in the Curacao case more than three years) is tracked
  per case and escalated at set ages.

### 4.4 ABN AMRO (Dutch Public Prosecution Service settlement, 19 April 2021) - evidence grade C, not verified in primary documents

The OM published a statement of facts and a transaction agreement, but the OM pages (guessed addresses return 404; the OM search is a script-driven page) could not be reached in this session. What a secondary source (Dutch
Wikipedia) records: ABN AMRO accepted a transaction offered by the OM of **EUR 480 million**, made up of a fine of EUR 300 million and EUR 180 million of disgorgement of unlawfully obtained benefits (reported as
representing savings on personnel costs), for the period **2014 to 2020**; the OM later declined, in December 2024, to prosecute three former executives for lack of sufficient evidence. No other detail is relied on here.

**Design lesson.** If the disgorgement was computed from compliance staff cost savings, the case states the economic logic of the "flat cost" cases in section 2 (the bank profited from under-resourcing, and the
settlement took that profit). AKR-TMS cannot decide staffing, but it can make the gap visible: the board report shows alerts per analyst, backlog age and overdue reviews against a staffing benchmark (L-52; sections 2.2 and 8.1). The ING case
(section 4.3), from the same prosecutor, covers the same themes with a primary document and carries the detailed lessons.

## 5. Australia: Westpac 2020 and Commonwealth Bank 2018

**Source note for this section.** AUSTRAC's statements of claim and agreed facts, and the Federal Court judgments (CBA [2018] FCA 930;
Westpac [2020] FCA 1538), could not be reached from this environment (austrac.gov.au does not answer; AustLII and the Federal Court
site return 403 or a challenge page). Both cases are therefore grade C, built from news reports of AUSTRAC's published allegations
and the lead's brief; numbers marked "(C)" should be re-checked against the AUSTRAC documents before the plan PDF quotes them.

### 5.1 Westpac Banking Corporation (AUSTRAC civil penalty proceedings, filed 20 November 2019; AUD 1.3 billion penalty agreed 24 September 2020) - evidence grade C

**Facts and numbers (C).** AUSTRAC alleged more than 23 million contraventions of the AML/CTF Act of 2006, including that Westpac failed
to report "more than 19.5 million international funds transfer instructions (IFTIs)" over about five years; a large share
arose from a low-value recurring payments product called LitePay (later closed), used for "recurring, low-value payments made by
foreign-government pension funds". AUSTRAC identified 12 customers with patterns indicative of child exploitation (frequent low-value
payments to the Philippines and South East Asia), and alleged Westpac did not "carry out appropriate due diligence on customers sending
money to the Philippines and South East Asia" and did not implement "appropriate automated detection scenarios to monitor for the
known child exploitation risks through other channels". AUSTRAC's stated cause: failure "to properly resource the AML-CTF function, to
invest in appropriate IT systems and automated solutions". The unreported instructions are described in a secondary source as involving about AUD 11 billion. The penalty agreed in September 2020, AUD 1.3 billion, was the largest in
Australian corporate history; the chief executive resigned.

**What failed (as reported).** A regulatory reporting pipeline that silently did not submit millions of reports (the data/process
failure); no automated detection for a known typology; customer due diligence and monitoring not aligned to the corridor risk
(Philippines and South East Asia); correspondent banking information not passed on and recorded; an under-resourced and under-invested
AML function.

**Design lesson**

- *Independent recomputation of reportable events.* AKR-TMS recomputes from the read-only CBS data which transactions are reportable to
  the FIU (cash above the CBI threshold, cross-border transfers, STR-linked events) and reconciles that population, per day, with the
  reports that the bank's goAML pipeline actually submitted (counts, values, report identifiers and FIU acknowledgements where the
  bank gives access). A shortfall opens an incident the same day. The check does not depend on the bank's own reporting code.
- *Typology scenarios for low-value, high-frequency corridor payments.* The pack includes recurring small payments to named
  corridors (with the bank's own high-risk corridor list), repeated beneficiaries and customers with relevant history; Iraq-specific
  corridors are set in file 27.
- *Resourcing evidence.* The board report shows analysts, alerts per analyst, backlog age and automated-coverage share, so that
  "invest in appropriate IT systems and automated solutions" has a measurable counterpart.

### 5.2 Commonwealth Bank of Australia (AUSTRAC civil penalty proceedings, filed 3 August 2017; AUD 700 million penalty approved 20 June 2018) - evidence grade C

**Facts and numbers (C).** AUSTRAC alleged about 53,700 contraventions (53,506 of them are widely cited as late threshold transaction reports; not verified in the sources reached) tied to
"Intelligent Deposit Machines" (IDMs) between November 2012 and September 2015. The machines accepted up to 200 notes and up to AUD
20,000 per transaction with no daily limit and credited the account instantly, domestic or international. CBA failed to report cash
transactions of AUD 10,000 or more made through them "within the required 10 business day period, or at all" (reported as AUD 624.7 million of unreported transactions), and 149 suspicious matter reports were filed late or not at all
(Wikipedia on AUSTRAC, grade C); it "did not take any
steps to assess the risk of money laundering or terrorism financing via IDMs for three years after they were rolled out", by which time
about AUD 8.9 billion had been deposited through them; and it did not monitor customers or report suspicious matters on accounts where
"cash deposits, many through IDMs, [were] followed immediately by international and domestic transfers". The lead's brief attributes
the missed threshold reports to a coding error; that detail was not confirmed in the sources reached.

**What failed (as reported).** A new cash channel launched without a risk assessment or limits; a reporting feed that did not carry
the machine transactions; monitoring that did not cover the machine channel or the cash-in then wire-out pattern.

**Design lesson**

- *Channel launch gate and per-channel limits* (as section 3.6): a deposit machine, agent, e-wallet or kiosk channel is marked live in
  the coverage ledger only with a risk assessment, a monitoring decision and limits; the structuring and cash-in/transfer-out
  scenarios run on the channel from day one.
- *Cash-in then immediate transfer-out* is a standing scenario at customer level (CBI cash and transfer scenarios, file 27).
- *Same reconciliation as Westpac*: the population of reportable cash events is recomputed independently from the CBS, by channel, and
  compared with submitted reports; the check is run by channel because a failing channel can hide in a large total.

## 6. United Kingdom (FCA and courts): the most detailed written accounts of monitoring-system failure

The FCA publishes long notices that describe the monitoring tool, its data feeds, thresholds and governance in terms an engineer can use.
Three of the notices below (HSBC 2021, Metro Bank 2024, Starling 2024) read like post-mortems of a monitoring system and are the closest
analogue to what AKR-TMS will do (a monitoring system fed from a copy of the core banking data). All notices were downloaded from
fca.org.uk and read; the penalty figures are after the 30 percent settlement discount unless stated.

### 6.1 HSBC Bank plc (FCA decision notice 14 December 2021, announced 17 December 2021: GBP 63,946,800; GBP 91,352,600 before discount) - evidence grade A

**Facts and numbers.** Relevant period 31 March 2010 to 31 March 2018. Two key automated systems (the long-standing CAMP and an
external system, ETMS, for correspondent banking from July 2017) monitored "around 284.8m transactions per month" for 13.6 million
active customers. The FCA found three components "deficient": scenario coverage, parameters and data (para 2.5).

**What failed: scenarios.** No review of whether scenarios covered the bank's risk indicators until 2014 (the bank's own maintenance
guide required an annual review from September 2010). The 15 "GMSAS" scenarios rolled out in 2016 were described by the bank as "94%
coverage" but "did not reflect the seriousness of the risk indicators not covered" (para 4.44). For correspondent banking, risk
indicators such as high-risk jurisdictions (other than by wire), concealment of location, round amounts, high-risk originators and
large amounts "were not covered by any scenario"; "over a 4-year period up until early 2017, the Monitor estimated that 43m
correspondent banking transactions were not adequately monitored" (paras 4.37-4.38).

**What failed: parameters.**

- *Extreme thresholds.* "For 1 of the change in behaviour GMSAS scenarios, 52% of RBWM customers (5.4 million customers) and over
  570,000 CMB customers needed a 500,000% (or 5,000 times) increase in their yearly peak transactional activity to trigger an alert."
  Another scenario required a customer who typically spent GBP 19,322 a month to spend "over GBP 153m per month" to alert; the
  thresholds were "effectively non-coverage", yet the bank "determined that extreme thresholds were a low-risk issue" (paras 4.61-4.62).
- *No below-the-line testing.* The 2016 tuning programme tested alerts generated (above the line) but "did not test transactions that
  did not generate alerts (BTL testing)"; thresholds had no documented rationale (paras 4.63-4.65).
- *Suppression rules.* "Suppression rules were used from the implementation of CAMP in 2002 until ... 2016 ... a type of parameter used
  in CAMP to manage alert volumes ... Suppressed alerts were not subsequently reviewed. By 2015, 67% of all alerts generated by CAMP in
  the UK were being suppressed by these rules" (paras 4.68-4.69). A look-back on a sample of 100,000 of the 4.4 million alerts suppressed
  between 2008 and 2016 produced 68 SARs, including a customer rated high risk "with potential links to a known terrorist organisation"
  and low-risk customers suspected of laundering (paras 4.79-4.80). The decision to lift suppression for high-risk customers was taken
  in February 2014 and applied in October 2015.
- *Scenario design errors that created backlog.* The "Large Reportable Transaction" scenario did not distinguish cash, wire and
  cheque and had a minimum alerting value of GBP 25; "nearly 35% of alerts generated by this scenario between August 2016 and March
  2018 took more than 90 days to review and so [were] overdue" against the bank's 90-day service level; the bank rolled it out
  "knowing that ... doing so would lead to a very high alert volume" (para 4.45). The 2016 structuring scenario "produced alerts on
  single transactions when, by definition, structuring involves multiple transactions"; just over half of its alerts were single
  transactions, the bank knew nine months after go-live and did not fix it during the period (para 4.46). Threshold tuning had been
  descoped from the programme to deliver other items faster, then put on hold; the Monitor had advised the alert volume was
  "unsustainable" (paras 4.57-4.58).

**What failed: data (paras 4.82-4.118).** "A failure throughout the relevant period to check the completeness and accuracy of data
fed into its transaction monitoring systems." Examples:

| Data failure | Scale |
|---|---|
| A rule meant to prevent duplicate monitoring excluded SEPA payments | About 12 percent of SEPA payments from April 2014; later about 520,000 transactions and over GBP 5.8bn a month |
| No list of correspondent banking relationships to feed the system | In six months (October 2012 to March 2013), 12,122 correspondent accounts with 21.6m transactions worth GBP 107bn were not identified |
| Two source systems did not reconcile after the 2016 ETL rollout, so transactions were not linked to customers and monitored only at account level | Over 62,000 customers, about 92,500 accounts, about 45m payments (para 4.105); by January 2018 about 400,000 orphan accounts; 0.25 percent of value, about GBP 1tn, involved orphan accounts |
| Cash amount not populated in the field the scenarios read | About 24m cash transactions worth about GBP 24.5bn not monitored by two of 15 scenarios (August 2016 to March 2018) |
| Wrong transaction-type mapping | 52 of 540 transaction codes from three source systems were incorrect; 29 codes covering 17.6m transactions worth over GBP 64bn were classed as wires when they were cheques and monetary instruments; in ETMS 24 codes worth GBP 239.6bn were wrongly mapped |
| Poor country codes, counterparty identifiers and originator or beneficiary identifiers "for a prolonged period" | "A risk with a critical impact" |

The bank had no "exceptions management reporting process to identify and escalate any new, deleted or changes to transaction codes"
until after 2016, and no formal data quality assessment for ETMS.

**Root cause.** A tool built in 2002 and extended by projects; no owner for reconciliation of source data to the monitoring tool;
alert volume managed by suppression rather than by prioritisation; remediation that descoped threshold tuning from the programme.

**Design lesson**

- *Every parameter has a below-the-line result and a reach figure.* AKR-TMS shows, for each scenario and threshold, the share of
  customers in each segment who could ever trigger it (the "5,000 times" test), the below-the-line sample result and the last tuning
  date; a threshold that needs more than a set multiple of a customer's peak activity is flagged "effectively not covering".
- *No suppression rules that hide alerts.* Where the bank wants to reduce duplicates, AKR-TMS merges alerts into one case and keeps
  every underlying alert visible; suppression of reviewable alerts is not offered (see sections 2.2 and 4.3).
- *Transaction-code mapping register.* The mapping layer holds a governed table from each CBS transaction code to the canonical
  type; it reports unmapped, changed and deleted codes daily, shows the volume and value behind each code, and requires a mapping
  review when a CBS code appears or disappears (extends section 2.6).
- *Population lists per specialised book.* Correspondent, exchange-house, trade and card books are generated from the CBS and
  compared with the list the bank declares; accounts in a book but not in monitoring are listed.
- *Orphan and unlinked account rate* as a daily metric with a limit; transactions on unlinked accounts are monitored at account
  level and flagged "degraded", never silently reduced.
- *Duplicate-prevention rules are reconciled* like any filter: records excluded by a de-duplication rule are counted and sampled.
- *Scenario coverage review on a clock*: a dated annual review of coverage against the typology inventory, with the uncovered
  indicators listed and ranked, not a single coverage percentage.
- *Scenario logic tests shipped with the scenario.* Each scenario has positive and negative unit cases in the product (a structuring
  scenario must not fire on a single transaction; a large-cash scenario must not treat a cheque as cash) and runs them on every release
  and every parameter change; a failing test blocks activation.

### 6.2 National Westminster Bank plc (FCA criminal prosecution; guilty plea 7 October 2021; sentence 13 December 2021: GBP 264,772,619.95) - evidence grade B (FCA press release; the agreed statement of facts was not read)

**Facts and numbers.** The first FCA prosecution under the Money Laundering Regulations 2007: three offences of failing to comply
with the regulations while monitoring a jewellery business (Fowler Oldfield), 8 November 2012 to 23 June 2016. "When taking on the
customer, NatWest initially understood it would not handle cash." Over the relationship about GBP 365 million was deposited, "of which
around GBP 264m was in cash". Fine before the one-third plea reduction: GBP 397,156,944.14. The sentencing judge: "Without the Bank -
and without the Bank's failures - the money could not be effectively laundered."

**What failed.** Branch staff "reported their suspicions to bank staff responsible for investigating suspected money laundering,
however no appropriate action was ever taken"; the red flags included "significant amounts of Scottish bank notes deposited
throughout England, deposits of notes carrying a prominent musty smell, and individuals acting suspiciously when depositing cash".
"In addition, the bank's automated transaction monitoring system incorrectly recognised some cash deposits as cheque deposits. As
cheques carry a lower money laundering risk than cash, this was a significant gap in the bank's monitoring of a large number of
customers depositing cash." The expected-activity profile at onboarding (no cash) was not compared with actual cash volumes.

**Design lesson**

- *Expected-versus-actual profile alert.* Each business customer's declared activity (cash use, turnover, counterparties, products) is
  stored as structured data; actual activity outside the declared profile (cash for a customer declared as non-cash; turnover above
  declared by a set multiple) alerts, and the alert shows the declared profile beside the actual figures.
- *Branch observations are data.* Teller and branch-liaison reports of suspicious behaviour are captured in AKR-TMS as internal
  reports linked to the customer, accounts and cash transactions concerned; an analyst closing an alert sees them, and an internal
  report with no recorded decision after a set number of days escalates (CBI internal-report and acknowledgement rules, file 01).
- *Cash classification test.* The mapping layer checks every cash deposit channel against the transaction type used by the scenarios and
  reports cash deposits classified as non-cash (the NatWest and HSBC errors) by comparing teller and cash-in codes with the type
  assigned.

### 6.3 Metro Bank plc (FCA final notice 12 November 2024: GBP 16,675,200; GBP 23,821,700 before discount) - evidence grade A (read in full)

This notice describes, almost exactly, the architecture AKR-TMS will use: monitoring fed from "a separate database ... that contained a
near real-time view of the data within Metro's core banking records system".

**Facts and numbers.** Relevant period 6 June 2016 (go-live of the automated monitoring system, ATMS) to 17 December 2020. "Metro failed to
monitor over 60 million transactions (circa 6.0% of the total transaction volume) with a value of over GBP 51 billion (circa 7.6% of the
total transaction value)" (para 2.5). A look-back completed in 2022 led to 153 further SARs and 43 account-closure notices, in addition to
1,403 SARs filed earlier.

**What failed in the data feed (paras 2.7-2.15)**

1. **The Time Stamp Code Logic Error.** "Where a customer had opened an account and transacted on the same day, the associated Account Record
   was not included in the data feed, and therefore those Transaction Records and ensuing Transaction Records were rejected by the ATMS
   until there was an update to the Account Record." Over about three years it affected about 166,000 accounts, "meaning that over 46.5
   million transactions ... with an associated value of over GBP 31.5 billion had not been monitored". It was found on 17 April 2019
   "during testing of an upgrade to Metro's core banking records system"; a tactical fix followed on 21 July 2019.
2. **No completeness check.** "Before the Lookback Review in July 2019, Metro did not check the completeness of data fed into the ATMS", and
   "prior to 17 December 2020, Metro failed [to] put in place an effective reconciliation process between the DS [data store] and the
   destination ATMS". On 17 December 2020 the data store began sending daily "count files" so that records received plus records rejected
   could be compared with records sent; a mismatch generated an e-mail.
3. **"Bad Data" folders nobody read.** Rejected customer, account and transaction records were placed in folders; "Metro failed to put into
   place a regular review process for the Account and Transaction Records in the Bad Data folders, which were only intermittently
   reviewed". There was an exceptions process only for rejected customer records. The true volume of rejected customer transactions was
   obscured for over four years by internal transfers in the feed that did not need monitoring; it became visible only on 14 December 2020.
4. **Governance that deleted the issue.** Staff escalated "Bad Data" in 2017 and 2018; the Financial Crime Steering Group agreed in January
   2018 to review it, but "this decision was subsequently removed from the final minutes ... with the intention of revisiting the issue once
   it was better understood", and it was not discussed again until April 2019. Escalation to a senior person left "no formal records".

**Root cause.** A data feed built for operational resilience, with the monitoring tool silently rejecting records that failed its own
referential rules, no end-to-end count reconciliation, and an exceptions process that nobody owned.

**Design lesson** (the three controls the FCA describes are all mandatory for AKR-TMS)

- *Daily count and sum reconciliation per source table and per transaction type, with automatic breach tickets.* The pipeline compares
  records read from the read-only replica, records accepted by the canonical model and records rejected, per day; any mismatch beyond
  zero (or a documented tolerance) is a ticket with an owner and an age (extends B-01 and section 2.6).
- *No silent rejection.* A transaction whose account, customer or reference record is missing is not discarded: it goes to a quarantine queue
  with the reason and is re-tried when the missing record arrives (the Metro same-day account case is a late-arriving-dimension
  problem); the quarantine count, age and value are shown on the compliance dashboard.
- *Separate the transactions that need no monitoring* (internal transfers, bank-own entries) by an explicit, reviewed rule so they never
  mask real rejections; the exclusion rule is itself a governed item with a count.
- *Test every CBS upgrade against the mapping.* An upgrade, patch or schema change of the CBS triggers an automated comparison of the
  mapped population before and after (the Metro error appeared only in upgrade testing).
- *Escalation records are immutable.* Entries in the risk and issue log cannot be removed or edited without a visible history; a
  committee decision to review an issue creates an action with a due date that cannot be dropped by editing minutes.

### 6.4 Starling Bank Limited (FCA final notice 27 September 2024: GBP 28,959,426; GBP 40,959,426 before discount) - evidence grade A (read in full)

**Facts and numbers.** Relevant period 1 December 2019 to 30 November 2023. A bank that grew to about 3.6 million customers (2023) and revenue
of GBP 452.8 million. In September 2021 it accepted a voluntary requirement (VREQ) "not to open any new accounts for high or higher risk
customers"; it then "opened 54,359 accounts for 49,183 high or higher-risk customers in breach of the terms of the VREQ" because it "failed
to implement all of the underlying requirements and sub-requirements of the VREQ properly and did not adequately monitor its compliance".

**What failed in the screening system (paras 4.42-4.50)**

1. **The system screened a fraction of the list.** "Since the implementation of its financial sanctions screening framework in 2017, its
   automated screening system had only been screening the names of new and existing customers against a fraction of the names on the
   Consolidated List." The cause was a misconfiguration of the matching between customers and listed individuals "which had existed since
   20 July 2017 and resulted in customers or prospective customers only being screened against individuals on the Consolidated List with
   UK citizenship or UK residency ... (i.e. 39 of the 3088 Designated Persons)". At least one Designated Person had opened an account.
2. **The warning sign was zero alerts.** The 2LOD review "identified on 30 January 2023 that Starling's automated customer screening system
   had not produced any financial sanctions screening alerts for individual customers between 1 July 2022 and 30 January 2023". After the
   fix, a back-book screening of 3.5 million customers "generated approximately 48,000 alerts".
3. **Payments screened with the wrong tool and incompletely.** "Starling was not screening all of its cross border/international payments
   against the Consolidated List"; where it did screen, it used "a tool designed for customer screening and as such not designed to screen
   against payments". A later review of 3,988,143 payments from 24 May 2017 to 9 November 2023 "generated 795,712 alerts".
4. **Customer screening once every 14 days, after onboarding.** "The 14-day period was a leftover metric from when Starling was a smaller
   institution"; screening happened only after the customer had been onboarded.
5. **No testing, no MI, rated low.** "No formal methodology or mechanism for the testing and calibration of its financial sanctions screening
   systems at or after implementation ... There was also no record of testing and calibration having been carried out"; "no operational MI
   relating to financial sanctions, this included alert volumes and trends"; sanctions risk was "rated ... as low" without considering
   crypto-related payment sources and multi-currency accounts; a "capability gap" at governance level; no second-line assurance reviews of
   sanctions screening until the first quarter of 2023, and the third-line audit delayed to the third quarter of 2023.
6. **A regulator's requirement not tested.** The VREQ breach arose because the control was built without an independent test of whether it
   worked and without monitoring of compliance with it.

**Remediation named by the FCA:** daily screening; a new payments screening solution; an alert management system updated "to ensure that
previously raised matches with similar names or other details are not missed in the case of updates to sanctions lists"; a testing
methodology and regular configuration testing; third-party testing of both systems; MI on alert volumes and trends; "governance and version
control around lists used in the sanctions screening process".

**Root cause.** A screening configuration that was never tested end to end, a growth-first operating model with no assurance, and no
indicator that would show a screening engine going quiet.

**Design lesson**

- *Screening test harness on a schedule.* AKR-TMS ships a known-answer list (seeded names and aliases of listed persons, in Arabic and Latin
  script, including persons from every nationality on the lists) and runs it against the bank's live configuration daily; each list and
  matching mode has a pass or fail and a dated report. A failing seed raises an incident. This directly prevents "39 of 3,088"
  (extends W-10).
- *Silence alarm.* For every screening list and scenario, a baseline of alert counts by day is kept; zero alerts, or a fall of more than a set
  percentage, for a set number of days raises an incident automatically (Starling had no alert for seven months; HSBC and ING show the
  same pattern from caps and suppression).
- *Customers and payments are different screening problems.* Payment screening covers message fields, free text and identifiers (BIC,
  IBAN, vessel, address) and every cross-border payment; customer screening covers the full name set, aliases, owners and signatories.
  The coverage ledger shows both, per list.
- *Screen at onboarding and on every list change.* No batch interval is configured above the daily maximum; delta re-screen on list change
  (file 04 U-05).
- *Regulatory requirement tracker.* Any restriction accepted from a supervisor (a VREQ, a CBI direction, a correspondent's condition) is
  entered as a control with a test and an owner, and compliance with it is measured from CBS data (accounts opened in breach).

### 6.5 Monzo Bank Limited (FCA final notice 7 July 2025: GBP 21,091,300; GBP 30,130,475 before discount) - evidence grade A (summary read)

**Facts and numbers.** A digital bank that grew from about 250,000 customers in early 2017 to over 12 million by April 2025. In the "Pre-VREQ
Period" it ran a limited CDD approach at onboarding: it "failed to obtain information about the purpose and nature of the proposed customer
relationship", "failed to review adverse media hits for a customer unless another risk factor was present", did not verify all beneficial
owners and persons of significant control for business customers, and "stopped verifying customer addresses for certain personal banking
customers", so that it found "PO Boxes, foreign addresses with UK postcodes or obviously implausible UK addresses, such as well-known London
landmarks" although its risk appetite was UK-based customers only. Customers opened multiple accounts, "including 2 customers whose
accounts had previously been closed as a result of financial crime concerns". "Until August 2020, Monzo's EDD processes for most categories
of personal banking customers did not provide for the circumstances in which EDD would be necessary", so "no EDD was carried out" for some
higher-risk customers. It "had no clear internal definition of what constituted a PEP", let potential PEPs open accounts and transact
while identification was ongoing, with "consistent backlogs in the assessment of PEP flags". Transaction monitoring suffered from "a lack
of adequate procedural guidance ... on how to investigate transaction monitoring alerts" and "key transaction monitoring tasks being
performed by insufficiently experienced or trained staff". The weaknesses came from "incorrect assumptions ... about the effectiveness of
Monzo's transaction monitoring systems to mitigate the lack of customer data gathered at onboarding". After the supervisor's requirement of
5 August 2020 not to onboard high-risk customers, Monzo "opened 33,039 accounts in breach of the VREQ; 26,325 of these accounts were for
high-risk customers", and applied controls "to incorrect information" in connection with 167,444 accounts (an estimated 34,262 high-risk
customers onboarded).

**Root cause.** Monitoring was expected to compensate for thin onboarding data, but monitoring needs the customer data to judge
"expected" activity; growth and low-friction onboarding outpaced the framework.

**Design lesson**

- *Monitoring quality depends on KYC completeness, and AKR-TMS measures it.* Each customer has a completeness score (purpose and nature
  of relationship, expected activity, source of funds, owners, verified address, occupation); monitoring scenarios that depend on a field
  mark every customer with that field missing as "degraded" and apply the stricter default; completeness by segment and by branch is a
  board metric.
- *Address and identity plausibility checks.* The mapping layer flags PO boxes, landmarks, addresses outside the country of operation,
  duplicate addresses and shared contact data across unrelated customers; a bank's declared domestic-only appetite is tested against data.
- *Duplicate and previously exited customers.* Entity resolution flags a new account or customer that matches a previously exited or
  refused customer (identifiers, phones, addresses, relatives' family numbers in Iraq).
- *PEP identification is time-boxed.* A customer flagged as a possible PEP cannot transact above a set limit while the flag is open;
  open-flag age is reported with the backlog.
- *Controls built to satisfy a restriction use the right data.* Each rule implementing a supervisory restriction states its input fields;
  a test compares the population it blocks with the population defined by the supervisor (33,039 accounts in breach).

### 6.6 Barclays Bank plc and Barclays Bank UK plc (FCA final notices 14 July 2025: GBP 39,314,700 and GBP 3,093,600) - evidence grade A (summaries read)

**Facts and numbers.** Barclays Bank plc, GBP 39,314,700 (GBP 56,163,900 before discount), for 9 January 2015 to 23 April 2021: it "failed
adequately to identify, assess, monitor and manage the money laundering risks" of a corporate customer, Stunt & Co Ltd, which received
GBP 46.8 million in electronic transfers from the jeweller later involved in the NatWest case (section 6.2) between July 2015 and August
2016. Barclays Bank UK plc, GBP 3,093,600 (GBP 4,419,500 before discount), for client-account opening failures concerning an investment
firm, plus a voluntary payment of GBP 6,281,757 to that firm's clients. Together about GBP 42.4 million.

**What failed (Stunt & Co, para 2.9).** The customer was rated low risk on onboarding with insufficient information on "the nature of its
business, including on its source of wealth and source of funds"; the rating was not updated although (a) funds from the jeweller were
"inconsistent with Barclays' understanding of Stunt & Co's business and significantly in excess of the turnover Stunt & Co had informed
Barclays was likely at account opening", (b) a joint venture was notified on 8 October 2015, (c) a law-enforcement request arrived in
August 2016 and production orders in August and November 2016, (d) adverse media in September 2016 reported police raids. "Although
Barclays had received the request from law enforcement concerning Fowler Oldfield in August 2016, the [Intelligence, Monitoring and
Investigation] team were not made aware of the law enforcement request for its review." A KYC refresh in January 2017 was certified
complete "despite Barclays not having received responses to outstanding queries" and with a police investigation known. No annual
periodic review took place in 2018 or 2019 although accounts were frozen on 29 August 2018. The charge of money laundering against the
owner in May 2020 "did not prompt any consideration" of a review of account activity; a significant investigation began only after
Barclays learned in March 2021 of NatWest's prosecution.

**Root cause.** A static risk rating, information held by the unit that received the law-enforcement request and not shared with the
monitoring team, and a KYC refresh that closed without the open questions answered.

**Design lesson**

- *Event-driven re-rating.* Law-enforcement and FIU requests, court and production orders, freezes, adverse media, a joint venture or a
  change in business model, and large unexplained inflows from a counterparty under investigation are trigger events that automatically
  reopen the customer's risk rating and create a task; the rating cannot be "confirmed unchanged" without a recorded reason that
  addresses each trigger (file 04 W-24).
- *The authority request register feeds monitoring.* A request about a counterparty (not only the customer) searches the ledger for every
  customer that transacted with that counterparty and opens linked reviews (file 04 E-01); the analyst on an alert sees the requests.
- *Third-party exposure search.* When a counterparty is the subject of an investigation, conviction or designation, AKR-TMS lists all bank
  customers who received funds from or paid it, with amounts and dates (the Fowler Oldfield flows reached several banks).
- *KYC refresh cannot be certified with open items.* The refresh workflow lists unanswered queries and blocks completion; certification
  needs a named approver and appears in the audit trail.
- *Frozen or restricted accounts remain under review.* A freeze does not suspend periodic review or monitoring of related accounts.

### 6.7 Nationwide Building Society (FCA final notice 11 December 2025: GBP 44,078,500; GBP 62,969,297 before discount) - evidence grade A (summary and selected paragraphs read)

**Facts and numbers.** Relevant period 1 October 2016 to 1 July 2021; about 18 million UK customers, a 10 percent share of UK current accounts.
**What failed in the data and the monitoring (paras 2.7, 4.20-4.22, 4.36-4.41)**

1. **A customer risk rating that defaulted to standard.** "Unless the customer fell into certain limited categories ... they were automatically
   classed as standard risk." The risk system was not fully operational until early 2019; additional data items were fed in August 2020 and
   considered for scoring only from April 2021. At mid-2020 only about 2,000 customers were rated high risk. Internal Audit found "a lack
   of oversight to ensure the quality of data being transferred between those systems", which "culminated in 44,957 customer accounts having
   not been fed into Nationwide's customer risk assessment tool".
2. **Enhanced CDD that never reached the engines.** The uplift applied to "some 888,618 customers that year ... representing just 5% of
   Nationwide's customer base"; the enhanced CDD sat in a database for the SAR and EDD teams but "had not been fed into the firm's main customer
   data repository and so did not inform the RRP customer risk assessment or transaction monitoring systems".
3. **No periodic or event-driven reviews for most customers.** Beyond the roughly 2,000 high-risk customers, the firm "had no process for
   undertaking either periodic or event-driven reviews of a substantial proportion of its customer relationships".
4. **Rules that ignored the customer.** "Other than for high risk customers, Nationwide's existing transaction monitoring rules applied
   uniformly across its entire personal account customer base"; singular thresholds per product; maturity assessed at "stage two out of five"
   against peers at four; "alert thresholds for the pre-existing rules were set very high"; rules run monthly. A 2016 internal review had noted
   "there is no formal documentation which determines the purpose of the rules, the anticipated tolerances for alerting volumes, what the
   success of the rules should look like". In January 2020 the firm explained to the FCA that its system "was not fed by individual customer
   CDD such as expected income" and did not alert when a customer with declared annual income of GBP 37,500 received credits of GBP 40,000,
   GBP 40,000 and GBP 50,000; it decided against lowering thresholds because that "would introduce significantly increased false positive
   alert levels".
5. **Business use of personal accounts.** The firm offered no business accounts; a February 2018 risk assessment found controls "not
   effectively designed for business activity" and that business-use customers produced 10 percent of all internal SARs and monitoring alerts.

**Root cause.** A risk-rating architecture built as an interim solution and a monitoring system that did not use the customer data the bank
held, for a large retail base.

**Design lesson**

- *The rating engine reads from the same repository as monitoring, and reports its own input completeness.* AKR-TMS shows, per customer
  segment, how many customers have no computed rating, and how many accounts are not in the rating population (the 44,957 case); a
  customer without a computed rating is treated as unrated-high until rated, not as standard.
- *Expected-income and declared-profile ratios.* Credits above a multiple of declared income or expected turnover alert; the alert shows the
  declared figure beside the actual (the GBP 37,500 example).
- *Peer and behavioural scenarios from the start.* Segment-based thresholds (peer groups by customer type, occupation, region, product, size)
  and behavioural change rules, not a single threshold per product (file 04 W-02, W-03).
- *Rule documentation is mandatory.* Each scenario record has purpose, expected alert volume range, success measure, owner and relevance to the
  bank's typologies; AKR-TMS refuses to activate a scenario without them (the 2016 review finding).
- *Business activity on personal accounts is detected from data* (cash and counterparty patterns typical of trading, regular payments from many
  unrelated payers) and routed to the business-use review, particularly for Iraqi accounts used for informal trading or exchange activity.

### 6.8 Dinosaur Merchant Bank Limited (FCA final notice 24 March 2026: GBP 338,000; GBP 482,900 before discount) - evidence grade A (summary read)

A small 2026 case, in market abuse surveillance rather than AML, but the failure is the one this file keeps finding: a new channel that never
reached the monitoring system. From June 2024 the firm's contract-for-difference business grew after "its introduction of a new direct market
access (DMA) order execution platform. It did not perform any additional risk assessment or engage in any other preparations to ensure its
market abuse surveillance systems were performing correctly for the DMA order execution platform." "Until October 2024, it failed to ensure that
trading undertaken via the new execution platform was being captured and ingested into the surveillance system." The failings "impacted 2,194
trades (with a notional value of USD 3.05 billion), which, following a later review, generated 2,916 surveillance alerts, of which 2,723
related to insider dealing and 193 to types of market manipulation"; suspicious transaction and order reports (STORs) were submitted
retrospectively. The firm also lacked policies for reviewing and escalating suspicious activity and "for the review and management of alert
calibration".

**Design lesson.** A new execution, payment or product platform raises an "unconnected source" item in the coverage ledger the day it
appears in the CBS or in the bank's channel list; it stays open until reconciled volumes show the platform's records flowing into monitoring,
and until the first look-back is run (section 2.1, section 5.2).

## 7. EU digital banks: N26 and Revolut

**Source note.** For N26, two BaFin measure pages were reached (9 November 2021 and 21 May 2024, grade B); later steps rest on secondary summaries (German and English Wikipedia citing the Financial Times and
WirtschaftsWoche). The Bank of Lithuania's page for Revolut could not be retrieved (lb.lt is behind a challenge page), so Revolut is grade C. The pattern (growth faster than controls, late suspicious activity reports,
supervisor limits on growth) matters for product design; figures marked C must be re-checked before they appear in the plan PDF.

### 7.1 N26 Bank AG (BaFin 2021, 2024 and 2025) - evidence grade B for 2021 and 2024 (BaFin measure pages), C for later steps

| Date | Action | Grade |
|---|---|---|
| June 2021 | BaFin fine of EUR 4,250,000 for late suspicious activity reports (cited in the November 2021 BaFin page) | B |
| 5 Oct 2021, published 9 Nov 2021 (final since 6 Nov 2021) | BaFin order under sections 25a, 44, 45b and 45c of the Banking Act: remedy "deficiencies in risk management, particularly in IT and outsourcing management"; growth cap of **50,000 new customers a month** and EUR 500 million worldwide for real-estate-secured exposures; special representative to monitor implementation. BaFin: "The deficiencies in risk management lie in the bank's strong growth"; the bank was to use resources to strengthen customer identification and money laundering prevention (translated from German) | B |
| 2023 | Cap raised to 60,000 a month (secondary) | C |
| 21 May 2024 | BaFin fine of **EUR 9.2 million**: "N26 Bank AG acted systematically too late in 2022 in submitting money laundering suspicious activity reports"; the order is final; the number of late reports is not stated (BaFin page; legal basis sections 60b KWG, 84 WpIG, 57 GwG) | B |
| 1 June 2024 | Growth restrictions lifted after progress (German Wikipedia) | C |
| July 2025 | BaFin reported to have threatened new sanctions after further violations (English Wikipedia) | C |
| 15 December 2025 | BaFin reported to have assigned a supervisor and imposed business restrictions (English Wikipedia; measures not verified) | C |

**What failed (as stated by BaFin).** Reports of suspicious activity were filed late, systematically in 2022 and in earlier years; risk management, especially IT and outsourcing, did not scale with a fast-growing
customer base; the supervisor tied the number of customers the firm could add to its progress in remedying identification and money laundering controls.

**Design lesson**

- *Time-to-report as a monitored service level.* For each case, AKR-TMS measures elapsed days from first alert to the decision, and from decision to the STR in goAML, against the bank's service level and the CBI's rule (files 01 and
  23); cases approaching the limit are shown to the MLRO daily, and the late rate is a board metric (L-52).
- *Capacity-to-growth indicator.* A dashboard relates new customers and new accounts per month to alerts per month, analysts, open backlog and overdue reviews; it shows when growth is outrunning controls, which is the signal BaFin
  used here (L-52).
- *A supervisor's growth cap is a control to model.* If the CBI or a correspondent restricts a bank (no new high-risk accounts, a limit on new accounts per month), AKR-TMS holds the restriction as a rule with a daily test of
  breaches from CBS data (sections 6.4 and 6.5; L-58).

### 7.2 Revolut (Bank of Lithuania 2025; ECB 2026) - evidence grade C

**Facts (C).** The lead's brief lists a 2025 Bank of Lithuania action against Revolut Bank UAB (a fine of about EUR 3.5 million for AML deficiencies);
the original could not be retrieved. A secondary summary (Wikipedia citing AML Intelligence and Sifted, June 2026) records that the ECB
"restricted some activities of Revolut's European arm following concerns about its product-approval processes, requiring an independent risk
review and changes to staffing and oversight". No AML-specific numbers are given here.

**Design lesson.** Product-approval is where the failures start in most cases above (TD Zelle, NBP remittance, CBA machines, Dinosaur platform).
The product launch gate (section 3.6) is therefore a core module, not an optional report.

## 8. Crypto and payments firms: growth, backlogs, identity-less accounts and nested access

Payments and crypto firms are not banks, but their cases are the best-documented examples of **alert backlogs under growth**, **tiered KYC that
leaves accounts unscreenable**, **location controls that can be bypassed**, and **nested access through sub-accounts**. Iraqi banks face the same
patterns through e-wallets, card programmes, exchange houses and remittance agents (file 24 and the CBI e-payment controls, file 01).

### 8.1 Coinbase (NYDFS consent order 4 January 2023: USD 50 million penalty plus USD 50 million compliance investment) - evidence grade A (consent order read in full text)

**Facts and numbers.** The compliance system "failed to keep up with the dramatic and unexpected growth of Coinbase's business, and, by the end
of 2021, was overwhelmed". "By the end of 2021 ... unreviewed transaction monitoring alerts grew to more than 100,000 (many of which were
months old), and the backlog of customers requiring enhanced due diligence ('EDD') exceeded 14,000" (para 9); over 10,000 EDD cases were
still open on 11 July 2022. Growth: "customer sign ups in May 2021 were fifteen times January 2020 levels, and monthly transactions in
November 2021 were twenty-five times" (para 10). The firm "lacked sufficient personnel, resources, and tools" and relied from 2019 to
November 2021 "on an inadequate case management system for dispositioning alerts and filing" (para 11). The regulator required an independent
monitor by MOU on 10 February 2022. The order sets USD 50 million of penalty and USD 50 million to be spent on compliance.

**What failed in operation (paras 47-59)**

- *The surge fix produced bad work.* Coinbase "hired more than one thousand third-party contractors to 'burn through' the remainder of the
  backlog" and reported the backlog resolved in April 2022. But "the training Coinbase provided was not scalable for the size of the contractor
  force, and attendance at the training sessions was not adequately tracked"; "initially, Coinbase did not have a system in place to audit the
  quality control that was done". Three contractors "cleared" about 73,000 alerts; a third-party audit found "more than half failed the quality
  check"; one contractor had a 96 percent failure rate in a sample of 186 alerts of one type, so about 11,000 of its alerts were re-reviewed, and
  about 41,000 alerts of another contractor (73 percent failure rate in a sample) were re-reviewed in August 2022.
- *The regulator learned late.* "Coinbase did not inform the Department of these issues until July 2022 notwithstanding that it was already
  subject to the February 2022 MOU."
- *Suspicious reports late.* As alerts "languished for months in the backlog, Coinbase routinely failed to timely investigate and report suspicious
  activity" within the 30-day period (paras 58-59); the order cites suspected money laundering, child-abuse material and narcotics examples that
  were not stopped at first because of the backlog.
- *KYC underneath.* Before December 2020 Coinbase "often failed to assign an informed 'risk rating' to individual retail customers at the time of
  onboarding", with no quality assurance on ratings until September 2021; the due diligence file "historically consisted of little more than a
  copy of a photo ID"; before July 2021 accounts opened without annual expected activity or account purpose (para 39).

**Design lesson**

- *Volume elasticity forecast.* AKR-TMS computes alerts per 1,000 transactions per scenario and forecasts alert volume and analyst hours from
  transaction and customer growth (here 15 times and 25 times in under two years); a forecast above capacity raises a staffing risk to the board
  before the backlog exists.
- *Surge staffing under control.* Temporary and outsourced reviewers get time-boxed roles, mandatory training with tracked completion before queue
  access, a higher automatic QC sampling rate, and a reviewer-level failure rate; above a limit, the reviewer's cleared alerts are re-opened in bulk
  (by reviewer and date range) and routed to a re-review queue.
- *Backlog age is reported to the supervisor on a schedule.* A threshold on overdue alerts (for example older than the bank's own service level)
  creates a notification task for the MLRO with a record of whether and when the supervisor was informed; an event that is subject to an MOU or
  direction triggers the notification automatically.
- *Case management that scales.* Bulk triage, linked-alert grouping at customer level, templates, keyboard-first screens and one-click SAR drafts are
  product requirements (file 28 and 31), because the 2019 to 2021 tool was named as a cause.
- *Rating and EDD queues are tracked like alerts*: open EDD cases by age, accounts without a rating, accounts without expected activity or purpose.

### 8.2 Block, Inc. (Cash App) (NYDFS consent order 10 April 2025: USD 40 million) - evidence grade A (consent order read in part)

**Facts and numbers.** Block's money transmitter and BitLicense examinations covered 2021 and 2022; assets more than doubled from USD 15.02 billion
(2021) to USD 34.06 billion (2023). Findings: "Between 2018 and 2021, Block experienced a significant transaction monitoring alerts backlog. In 2018, Block
had accumulated a transaction monitoring backlog of approximately 18,000 alerts, which grew to over 169,000 by 2020", caused "in part by Block's
inability to predict the impact of Cash App's growing customer base on alert volumes and staffing needs, as well as the increase in alerts generated
by the implementation of new transaction monitoring tools" (para 13). Procedures required a recommendation within 60 days of an alert; yet "between
February 2021 and September 2022 SARs ... were at times filed over a year after the alerts were first generated. The average number of days between the
date of the transaction monitoring alert and the SAR filing was 129 days. The average number of days between the date of the transaction monitoring alert
and the start of a case investigation was 70 days" (para 15).

**What failed in the screening and monitoring design (paras 19-22).**

- *A materiality threshold on a prohibited exposure.* With one blockchain-analytics vendor, "Block's system did not generate alerts on Bitcoin transactions
  until the recipient's wallet had more than 1% exposure to terrorism-connected wallets, and Block did not automatically block transactions ... until the
  exposure exceeded 10%". DFS: "Any amount of funds transferred to terrorism-connected wallets is illegal and setting threshold alerts above 0%
  without a risk-based analysis supporting that decision, falls short of the regulatory requirement."
- *Screening scope.* "Existing service providers and employees were not subject to subsequent screening against updated OFAC sanctions listings"; "prior
  to 2023, Block did not conduct OFAC screening for restricted accounts"; fifteen rejected Bitcoin transactions were reported late to OFAC in 2021.
- *Deficient monitoring and risk rating of transactions using anonymising services ("mixers")*, a typology DFS had flagged in guidance of 28 April 2022.

**Design lesson**

- *Zero tolerance categories are not thresholds.* Matches to designated persons, terrorist lists and prohibited jurisdictions have no minimum score,
  materiality or exposure percentage in AKR-TMS; any hit is an alert, and for Iraqi banks the IQTFS and UN lists carry a freeze obligation (file 04
  U-06). A configuration with a non-zero tolerance on such a list is rejected by the governance rules.
- *Screening scope includes employees, suppliers, service providers and restricted or low-tier accounts*, re-screened on every list change.
- *Alert-to-case and alert-to-STR clocks.* Median, 90th percentile and maximum days from alert to case start and to STR, per scenario; policy time (60
  days here) against actual time (129 days) is a standard board chart.
- *A new monitoring tool must not swamp the old one.* A go-live plan states the expected alert volume change and capacity; the run in parallel shows the
  volume before the old tool is retired (the increase in alerts after new tools contributed to the backlog).
- *Typology updates from the regulator or FIU become scenarios on a clock.* A DFS guidance item on mixers (April 2022) had no matching rule; AKR-TMS keeps a
  register of CBI and FIU typology circulars with the date each was mapped to a scenario (file 27).

### 8.3 Binance (FinCEN consent order 2023-04, 21 November 2023: USD 3.4 billion; with DOJ and OFAC resolutions) - evidence grade A (FinCEN consent order read in part)

**Facts and numbers.** FinCEN found that Binance "willfully failed to develop, implement, and maintain an effective money laundering program": it
"launched its initial platform without any AML controls in place, failing to establish a written AML program until July 2018", and filed no SARs
despite billions of dollars of suspicious transactions. FinCEN imposed a civil penalty of USD 3.4 billion, credited by USD 2.47 billion of payments to the DOJ and the CFTC and with USD 150 million suspended pending
compliance, and an independent compliance monitor.

**What failed in the systems and data**

- **Accounts that could not be screened.** From July 2017 to at least August 2021 "Tier One (also referred to as 'no-KYC') customers were permitted to
  open accounts and conduct CVC-to-CVC transactions with only an email address", with daily withdrawals of under two bitcoin, "a value that at times
  exceeded the equivalent of USD 130,000 a day"; "Binance performed no due diligence on such accounts." A service provider told it in March 2019 that
  "effective screening could not be performed without minimal information, including first and last name (as opposed to an email address). In other
  words, no sanctions screening could occur for 'no KYC' accounts." By Binance's own calculation it had "four times the number of U.S. users for
  whom Binance had not collected KYC (based on IP address) than U.S. users for whom Binance had conducted KYC".
- **Structuring below the bank's own limit, known to compliance.** In August 2018 a service provider told the compliance head of "structuring ... just
  under your KYC limit of 2 [bitcoin]" in a USD 130 million theft; nothing changed for over four years.
- **A "paper-only" programme.** Written procedures prohibited gambling counterparties and said they would be flagged; but without KYC "users were free to
  transact with these high-risk counterparties with impunity", and Binance had sent and received over USD 1 billion with known gambling services.
- **Location controls that depended on the customer's word.** "Binance's geofencing controls were either nonexistent, or were superficial and
  ineffective: starting in the summer of 2019 ... U.S. users were identified based on their IP addresses, but users accessing Binance.com from a U.S. IP
  address were not fully blocked. Instead, those users needed only to 'self-certify' that they were not U.S. persons. Until January 2021, Binance processed
  transactions with U.S. persons who completed this self-certification, even where Binance possessed information contrary to their certification."
- **Nested exchanges through sub-accounts.** "A single Enterprise User could open up to 1,000 subaccounts", brokers an unlimited number; "until the end of 2021,
  Binance failed to require attestation from Exchange Brokers to confirm that these exchange brokers perform any AML checks on their own customers". Two
  OFAC-designated exchanges operated as nested exchanges inside Binance accounts for years.

**Design lesson**

- *Identity-less and low-tier accounts are first-class risk objects.* Every account tier (including the CBI's simplified due diligence tier below IQD 1
  million, file 01) has a stated list of limits, products and counterparties; monitoring compares behaviour to the tier; accounts for which screening
  fields (name, ID) are missing are listed with value and volume, and cannot send or receive cross-border funds.
- *The bank's own thresholds are typologies.* A scenario watches activity just below each KYC, reporting and tier limit (the 2-bitcoin case; in Iraq the
  SDD ceiling, the cash reporting threshold and the FX limits).
- *Policy-to-detection map* (section 3.2): each prohibited counterparty class in policy maps to a rule with a test; unmapped policy statements are gaps.
- *Omnibus and sub-account register.* Exchange houses, PSPs and agents that give their own customers access through the bank are listed with the number of
  sub-accounts and flows, the date of their last AML attestation and whether the bank receives their client list; flows through them without client data
  are labelled "opaque" (file 04 W-25) and capped.
- *Location evidence beyond a declaration* (sections 3.1 and 4.2): IP, device and phone data checked against the declared country; VPN indicators are a risk
  factor.

### 8.4 Paxful (FinCEN consent order, listed 9 December 2025: USD 3.5 million) - evidence grade A (consent order read in part)

A 2025 case that repeats the Binance pattern at smaller scale: a peer-to-peer marketplace with over four million users and over USD 10 billion of external
bitcoin transactions. "Written transaction monitoring policies, procedures, and internal controls were not in place prior to July 2019. As a result, all
transactions conducted on Paxful ... were not appropriately reviewed for potentially suspicious activity before then", and "even minimal transaction monitoring
was not in place until at least 2018". Blockchain analytics tools bought in March 2020 "omitted coverage for several CVCs", so Paxful "had no ability to identify and
report suspicious activity within the significant volume of transactions in over 15 CVCs". It had no KYC until February 2019, then only above USD 1,500 "without any
controls to identify users who sought to evade these controls by structuring". Its own review found "over 1,500 accounts opened between May 2015 and August 2018 with
Iranian, Syrian, Cuban, Crimean, or Sudanese IP addresses", and it did not detect users hiding location behind VPNs. No SAR was filed until November 2019.

**Design lesson**

- *Monitoring that covers only the products the tool supports is not monitoring.* The coverage ledger must list the products and currencies **not** covered (here 15 virtual currencies; for a bank, prepaid cards, wallets and
  every payment scheme; L-01).
- *Tiered KYC needs structuring detection at the tier limit* (USD 1,500 here; the SDD ceiling in Iraq; L-50, L-51).
- *Location data are a screening input* (1,500 accounts with sanctioned-country IP addresses; L-37) and monitoring policies must exist in writing before go-live (no written monitoring procedure before July 2019 at a firm that started in 2015).

### 8.5 OKX (DOJ, 24 February 2025: about USD 504 million) - evidence grade C, not verified

The lead's brief lists OKX (2025). The DOJ press release and plea documents could not be reached (justice.gov blocks automated access); the only secondary reference
available (Wikipedia citing Fortune) records "a USD 500 million settlement with DOJ" and a US relaunch in April 2025. From memory, unverified: the operating company pleaded
guilty to operating an unlicensed money transmitting business and failing to maintain an AML programme, with a combined forfeiture and fine of about USD 504 million.
No design lessons are drawn from this entry; the Binance and Paxful orders cover the same patterns from primary documents.

## 9. Other 2025-2026 cases: foreign banks, regulators' candour, partner due diligence and sanctions screening

Cases found by reading the enforcement pages of NYDFS, FinCEN, OFAC and the FCA directly (the web search tool was exhausted for this session, so the list is
not a full sweep; sources not reached are listed in section 13). Entries already written elsewhere in this file: Nationwide, Monzo and Barclays (UK, section 6), UBS
Financial Services and Canaccord (US, sections 2.6 and 2.7), Paxful (section 8.4), Dinosaur Merchant Bank (section 6.8), Swedbank Latvia (section 4.2).

### 9.1 FinCEN section 311 and related special measures against foreign financial institutions, 2025-2026 - evidence grade A for Banque Misr UAE (Federal Register notice read); B for the others (abstracts)

A bank's dollar access is decided by US Treasury findings about its customers and flows, not only by what its own supervisor thinks. The actions since mid-2025:

| Date | Target | Measure and stated basis |
|---|---|---|
| 25 Jun 2025 | CIBanco S.A., Intercam, Vector Casa de Bolsa (Mexico) | Orders prohibiting certain transmittals of funds, "determined to be of primary money laundering concern in connection with illicit opioid trafficking" (FinCEN list; Federal Register abstract) |
| 1 May 2025 (NPRM), 15 Oct 2025 (final) | Huione Group (Cambodia) | Prohibition on correspondent accounts, special due diligence by US banks |
| 26 Feb 2026 (NPRM; Federal Register 2 Mar 2026) | MBaer Merchant Bank AG (Switzerland), "a small, private Swiss financial institution with a single office" | Proposed prohibition; FinCEN assesses it "for years ... directly or indirectly facilitated money laundering ... including through processing transactions related to Venezuelan corruption and Russian and Iranian illicit activities" |
| 23 Jun 2026 (NPRM) | H-Pay Service PLC | Proposed measure linked to the Huione definition (not read) |
| 28 Aug 2026 (NPRM; Federal Register 1 Sep 2026, 91 FR 56085; comments to 1 Oct 2026) | **Banque Misr UAE** (five UAE branches of an Egyptian state-owned bank) | Proposed prohibition on correspondent accounts and on processing for other banks' correspondent accounts any transaction involving it |
| 3 Jul 2024 (final rule, 89 FR 55051, effective 2 Aug 2024) | Al-Huda Bank (Iraq) | Prohibition on US correspondent accounts for or on behalf of the bank (file 01 section 4) |

**Banque Misr UAE in detail (Federal Register notice, September 2026).** FinCEN "assesses that Banque Misr UAE serves as a critical access node to the U.S.
dollar (USD) for Iranian illicit finance". The branches belong to a state-owned bank and had operated in the UAE since 1972. FinCEN "analyzed Iranian shadow banking
between June 2025 and June 2026, from non-public information ... and observed Banque Misr UAE to have a concerningly high percentage of potential Iranian shadow
banking to assets in the data available and given the size of its assets". "Overall, FinCEN identified 103 potential Iranian shadow banking front companies transacting
approximately USD 1.8 billion, using accounts with Banque Misr UAE from January 2024 to June 2026. This includes approximately USD 520 million over the most recent
12-month period." Examples: a UAE free-zone front company that was designated in September 2025 had USD 32 million processed through the bank in 2024-2025; a trading
company designated in July 2026 had over USD 29 million processed between March and July 2025; an oil-trading company had a USD 1 million-plus transaction in January 2025.
FinCEN describes the typology: "Iran-based exchange houses and front companies that enable sanctioned Iranian entities to access USD ... through U.S. correspondent banking
accounts. Front companies are predominantly registered in third-country jurisdictions such as the UAE and ... Hong Kong to obscure beneficial ownership". It cites its
earlier products on the same pattern: advisory FIN-2025-A002 (6 June 2025, file 05 section 1g), a financial trend analysis of Iranian shadow banking in BSA data (October
2025) and alert FIN-2026-Alert-002 of 11 May 2026 on front companies, financial facilitators and digital asset infrastructure of the IRGC. It "does not have fulsome insight
into the scope of Banque Misr UAE's legitimate activities" and assumes some are legitimate; it concludes that "any legitimate activities do not outweigh the risks".

**Why it matters for AKR-TMS.** A US Treasury finding of this kind needs no accusation that the bank's management knew; it rests on **flows through the bank's accounts
that match a typology**, the ratio of those flows to the bank's size, and later designations of its customers. An Iraqi bank is exposed to the same reasoning (Iranian oil
disguised as Iraqi crude, militia-linked companies, exchange houses; file 05 section 1e).

**Design lesson**

- *Typology exposure index per bank, per branch and per correspondent.* AKR-TMS computes, monthly and from the CBS data, the value and count of flows that match each
  published typology (Iran shadow banking, IRGC front companies, Russia evasion, oil laundering) and the ratio to total assets and total flows, the same ratio FinCEN
  used; the board sees the trend, and the bank can show a respondent view of what its correspondents can compute (file 04 W-22).
- *Front-company profile scoring.* Customers newly registered in third-country free zones (UAE, Hong Kong, Turkey, Iraq's free zones), with shared addresses, directors or
  signatories, trading in oil, commodities or electronics, with in-and-out flows to many unrelated parties, no operating footprint and counterparties in exchange houses,
  score high; each score shows its reasons.
- *Designation look-back.* On every new designation or sanctions-evasion alert, AKR-TMS lists all historical flows with the designated party (before and after the
  designation date), all accounts connected through ownership, signatory, address, device, phone and counterparties, and the total value; flows after the date are
  violations to freeze or report, flows before are exposure to be reviewed (the 2025 designation and USD 32 million example).
- *Direct and indirect exposure view for the board* before a public action, the "what would a correspondent see" report: share of USD flows with high-risk jurisdictions,
  exchange houses, designated parties' counterparties, and sudden growth in new corporate customers.
- *Foreign branches and state-owned banks.* Branches abroad (and Kurdistan or free-zone branches) report into the same typology index; a state-owned bank is not exempt.

### 9.2 Swedbank AB and Swedbank AB New York Branch (NYDFS consent order 16 July 2026: USD 50 million) - evidence grade A (consent order read in full text)

A 2026 order that is not about monitoring failure but about **producing evidence**: the regulator found that the bank withheld information about its Baltic subsidiaries' links to
a law firm exposed in the Panama Papers (leak of 3 April 2016). The branch answered a 19 April 2016 request for a special report "as if the Department were seeking
information relating only to the New York Branch, not the Bank as a whole" with a one-page reply (para 12), although the home office knew of responsive information
(para 14). In 2018 DFS asked for "a list of all transactions processed through the Bank involving [the law firm] or any entity formed with the assistance of [the law firm]" and
"a list of any Bank customer that had a direct or indirect relationship" (para 18). The bank told DFS its search would cover its "global operations" but "chose to exclude the Baltic
Subsidiaries from its review" (paras 20-22); an internal e-mail from the chief compliance officer, on learning that a search of the Baltic subsidiaries produced many matches, read
that this was "why I wanted to remove them out of the scope" (para 26); the bank omitted the Estonian supervisor's inquiries and three recommendations of the Swedish supervisor (paras
27-29). DFS learned of the omission from media reports in 2019 (para 31). Order: the bank "failed to produce responsive information when replying to multiple Department requests for
special reports", in violation of Banking Law section 125(3); USD 50 million; leadership and most of the board have since been replaced.

**Design lesson**

- *Regulatory data request workbench.* A request from the CBI, the FIU, the US Treasury or a correspondent (all transactions involving entity X or entities formed with the help of
  firm Y; every customer with a direct or indirect relationship) is answered from AKR-TMS by one search over customers, owners, signatories, counterparties, payment text and
  documents across **all** units, branches and subsidiaries in the data set, with a result count per unit and an export (file 04 E-01).
- *Scope decisions are logged and cannot be silent.* The workbench records the scope chosen (entities, systems, dates, search terms), any exclusion and its reason, who approved it, and
  shows the excluded units' size; an exclusion of a unit is displayed on the response cover sheet.
- *Group-wide indicator of unconnected units* (section 4.1): the list of units and systems not in AKR-TMS is on the cover sheet of any group-wide answer.
- *Supervisor correspondence register.* Inquiries, recommendations and findings received from any supervisor, home or host, are logged with their disposition so that a response to a
  second authority can be checked against what the first authority said.

### 9.3 Paxos Trust Company (NYDFS consent order 7 August 2025: USD 26.5 million penalty plus USD 22 million compliance commitment) - evidence grade A (consent order read in part)

Not a bank, but the closest written example of a **correspondent-style relationship accepted on the partner's word**. Paxos partnered with Binance for a stablecoin; in 2019 it "accepted Binance at its word and did not undertake an independent review of Binance's
assertions" on geofencing and AML (para 10), and helped draft the letter Binance sent to the regulator ("Paxos did not test or otherwise verify Binance's claims", para 11). When a 2020
press report alleged US customers were reaching Binance by VPN, Paxos asked for audit reports and received a review of policies; its own monthly refreshes found 99 US-based Paxos clients
transferring to Binance, and "two of Paxos's employees with legacy Binance.com accounts tested Binance's supposed geographic restrictions. Both employees were able to access the exchange and make
trades from New York IP addresses" (para 17). A 2022 examination found it "failed to demonstrate that it had the appropriate controls in place to effectively monitor for significant illicit
activity occurring at or through Binance"; an enhanced due diligence review of Binance's history later "identified USD 1.6 billion in transactions flowing to or from the Binance platform involving
illicit actors", including more than USD 32 million to or from a sanctioned exchange, part of it after designation (paras 19-21).

**Design lesson**

- *Assurances are not evidence.* For every correspondent, exchange house, PSP and agent, AKR-TMS keeps (a) what the counterparty said (questionnaire answers, attestations), (b) what the bank's own
  data show (flows by counterparty country, share to high-risk typologies, rejected payments), and (c) the date of the last independent test; a gap between (a) and (b) is displayed on the
  relationship card.
- *Own-flow analytics on counterparties.* Flows between the bank's customers and a counterparty institution are measured the way Paxos' monthly refresh did (here US-based users reaching the
  partner), and any institution whose declared restriction is contradicted by the bank's data is escalated.
- *Test the control as a user.* AKR-TMS' test harness includes scripts for the bank to run like a user (login from a restricted country, submit a payment with a listed name) and records the
  result; this was the cheapest test in the case.

### 9.4 OFAC enforcement against financial firms, 2025-2026 (Interactive Brokers, ShapeShift) - evidence grade A (enforcement releases read)

- **Interactive Brokers LLC, 15 July 2025, USD 11,832,136.** From 15 July 2016 to 31 January 2024 the broker provided services to persons in Iran, Cuba, Syria and Crimea and committed 12,367 apparent violations (self-disclosed,
  non-egregious). Customers "provided Know Your Customer information indicating ordinary residence in a non-sanctioned jurisdiction" but were identified as located in sanctioned jurisdictions through IP address data. "Certain
  deficiencies in IB's IP geo-blocking controls (including a technical bug) allowed access by a limited number of customers located in Iran, Cuba, and Syria ... IB did not adequately audit or test these systems." Before May 2019 the
  block list omitted Crimea and from May 2019 to June 2021 omitted IP addresses of Sevastopol; the firm "relied on representations from third-party introducing brokers".
- **ShapeShift AG, 22 September 2025, USD 750,000.** Between December 2016 and October 2018 the exchange had "no sanctions compliance program in place to screen users or transactions", although "IP addresses were the 'only available
  indicator'" of location, and exchanged USD 12,570,956 in 17,183 instances with users in Cuba, Iran, Sudan and Syria; it adopted a programme only after an OFAC subpoena.
- Also listed by OFAC for 2026 (not read): TradeStation Securities, Inc., 17 March 2026, USD 1,110,661.

**Design lesson.** Location lists and geo-block tables are screening lists: they need an owner, a change log, an independent test (including from inside the sanctioned region), and a completeness check against the sanctions programmes in force
(Crimea and Sevastopol, here). The "known-answer" harness of section 6.4 covers IP and region lists as well as names.

### 9.5 What the 2025-2026 record does not show

- **UK.** On the FCA's published fines lists (fetched 6 October 2026), the AML-related firm fines of 2025 are Monzo, Barclays (two notices), Nationwide, and two trading-firm financial-crime cases (Mako Financial Markets Partnership LLP,
  GBP 1,662,700 on 17 February 2025, for inadequate systems and controls and insufficient skill and care in assessing purported high-value over-the-counter equity trading introduced by a third party,
  about GBP 68.6 billion of Danish and GBP 23.6 billion of Belgian equities in 2013 to 2015; Arian Financial LLP, GBP 288,962.53 on 9 January 2025). For 2026 up to 30 September the list shows no bank AML fine; the only systems-and-controls firm
  fine is Dinosaur Merchant Bank (market abuse, section 6.8).
- **US.** FinCEN's enforcement-actions page shows five actions since TD Bank's October 2024 order (Sahara Dunes Casino, Brink's, Paxful, Canaccord, UBS Financial Services). NYDFS's list shows Block, Sigue, Wise US, Paxos, Santander Consumer USA, Order Express and
  Swedbank in 2025-2026 (Sigue, Wise US and Order Express were not read).
- **Middle East.** Apart from Banque Misr UAE and the earlier Mashreq, Standard Chartered UAE and Habib cases, no 2025-2026 supervisory fine on a Gulf, Jordanian or Iraqi bank was reached (the Central Bank of the UAE site is closed to automated
  access; other regional supervisors do not publish in a form that could be fetched). The Dutch prosecutor's page lists a Credit Suisse/UBS settlement of September 2026 that was not read.

## 10. Failure-mode catalogue: what went wrong, the control AKR-TMS must have, and the evidence it must produce

How to read the tables. "Cases" refer to sections of this file (2.1 is TD Bank, 3.1 Standard Chartered, and so on). "Control" is a capability of AKR-TMS or a rule for
the bank's use of it. "Evidence" is the artefact the bank can hand to the CBI, a correspondent, an auditor or a court to prove the control works; the evidence must be
generated by AKR-TMS from system data, not typed in. The requirement IDs in the last column are in section 12.

### 10.A Coverage and data

| ID | Failure mode | Cases | Control AKR-TMS must have | Evidence AKR-TMS must produce | Req. |
|---|---|---|---|---|---|
| FM-01 | **Coverage gap**: transaction types, channels, products, currencies, payment directions or whole units never reach monitoring (and nobody owns the question) | TD 92 percent of volume (2.1); HSBC SEPA, correspondent book, Group Entities (2.4, 6.1); Danske branch on own IT platform (4.1); UBSFS partial feed (2.6); CBA machines (5.2); Dinosaur platform (6.8) | Coverage ledger by entity, source system, product, channel, currency and direction; monitored share by count and value; "dark" units shown; exclusion register with owner, reason, approver and expiry; new-code and new-channel detection; launch gate | Monthly coverage report (share monitored by count and value, per source and per transaction type), exclusion register with approvals and expiry, launch-gate records, dated list of units not connected | L-01, L-02, L-03 |
| FM-02 | **Data feed errors**: records silently rejected, mapped to the wrong type, dropped by a label or code change, or unlinked to a customer | Metro same-day account (6.3); UBSFS "FX RCVD", partial feed (2.6); HSBC 52 of 540 codes, orphan accounts, SEPA (6.1); NatWest cash as cheque (6.2); Capital One coding changes (2.8); Canaccord null fields (2.7) | Canonical-model data contract; daily count and sum reconciliation per source and per type; quarantine queue (no silent rejection); reference-value drift detection; governed mapping register; CBS-upgrade comparison; null-rate monitors; orphan-account metric; lineage from CBS column to scenario input | Daily reconciliation log with breaches and tickets; quarantine age and value; mapping change log; before and after population comparison per CBS change; lineage report with null rates | L-04 to L-09, L-14, L-15 |
| FM-03 | **Reporting pipeline errors**: FIU reports not filed or filed with wrong parties because of a code or file error | CBA threshold reports (5.2); Westpac cross-border reports (5.1); Capital One USD 16bn of cash (2.8); US Bancorp CTR names (2.2) | Independent recomputation of reportable events from the read-only CBS data by channel and code; reconciliation with goAML submissions and FIU acknowledgements; pre-submission role and identifier checks | Daily ledger: reportable events, submitted reports, acknowledged reports, differences by channel; list of STR drafts failing role checks | L-11, L-12 |
| FM-04 | **Unknown treated as low**: missing country, name, address or counterparty lowers the risk or exempts the record | UBSFS unknown country rated moderate (2.6); Binance "no-KYC" accounts (8.3); Nationwide default "standard" (6.7); Monzo addresses (6.5) | Unknown gets the highest tier of its field; completeness score per customer and per scenario input; identity-less and low-tier account register with limits | Counts of records with each missing field and the risk treatment applied; low-tier account report with limits and breaches | L-10, L-44, L-50 |

### 10.B Detection design, tuning and validation

| ID | Failure mode | Cases | Control AKR-TMS must have | Evidence AKR-TMS must produce | Req. |
|---|---|---|---|---|---|
| FM-05 | **Thresholds, caps and suppression set by capacity** (alert caps, "too many alerts", scenarios switched off, suppression rules, private filters) | US Bancorp caps (2.2); ING "aftoppen" three a day (4.3); Commerzbank (3.3); TD nine scenarios removed (2.1); HSBC 67 percent suppressed (6.1); Canaccord manual filters (2.7); Nationwide (6.7) | No cap parameter; flat-line detector; suppression is merge, not hide; reason codes (risk or capacity) with below-the-line evidence and second approver on every change; no private filters; no alert-count KPI for the owner of thresholds | Change log with reason codes and approvers; flat-line incidents; suppression ledger (count, productive rate from samples); list of saved filters and their effect on volume | L-16 to L-19 |
| FM-06 | **Unreachable, dead or extreme rules** | HSBC "5,000 times" thresholds (6.1); TD Zelle cap vs threshold (2.1); Nationwide very high thresholds (6.7) | Reachability test per scenario and segment; segmented, peer-based thresholds; scenario logic unit tests | Reach table (share of customers who could trigger each rule); dead-rule report; unit-test pack results per release | L-20 to L-22 |
| FM-07 | **Frozen scenario set, no recalibration or coverage review** | TD no new scenario 2014 to 2022 (2.1); HSBC annual review not done (6.1); Nationwide stage 2 of 5 (6.7); Deutsche (2.5) | Recalibration clock per scenario; annual coverage review against the typology inventory and the CBI catalogue; typology circular register; mandatory scenario record | Last-tuned dates; coverage inventory with uncovered indicators ranked; circulars mapped to scenarios with dates | L-23 to L-25 |
| FM-08 | **Scenario design errors**: wrong aggregation, wrong ranking, wrong logic | HSBC structuring on single transactions, GBP 25 minimum (6.1); ING account level and relative-change ranking (4.3) | Client and related-party aggregation; documented risk-ranked prioritisation; unit tests | Aggregation level per scenario; ranking validation; test results | L-22, L-26, L-27 |
| FM-09 | **No effective validation or testing** | US Bancorp self-validation (2.2); Mashreq vendor summary (3.5); Starling (6.4); UBSFS model reports (2.6) | Independent validation workspace with replay on the bank's own history and below-the-line samples; known-answer harness; validator is not the owner | Validation reports with replay counts; harness pass or fail history; role separation log | L-13, L-30, L-32 |
| FM-10 | **Expected-versus-actual profile not used** | NatWest cash (6.2); Nationwide income (6.7); Barclays turnover (6.6); Mashreq KYC data (3.5) | Structured declared profile (turnover, cash use, purpose, counterparties); ratio alerts with the declared figure beside the actual | Share of customers with a profile; ratio alerts by segment | L-28 |

### 10.C Screening

| ID | Failure mode | Cases | Control AKR-TMS must have | Evidence AKR-TMS must produce | Req. |
|---|---|---|---|---|---|
| FM-11 | **Incomplete list or matching coverage** (a fraction of a list, a missing region, a silent engine) | Starling 39 of 3,088 (6.4); Interactive Brokers block list lacked Crimea (9.4); Block thresholds above 0 percent (8.2) | Daily known-answer harness for every list and mode, in Arabic and Latin script; silence alarm on alert counts; zero-tolerance categories with no materiality threshold; list version control | Daily harness report; alert-count baselines and incidents; list version and load log | L-29, L-32, L-36 |
| FM-12 | **Good-guy and exemption list abuse** | Habib "Good Guy" list USD 250m unscreened (3.4); Rabobank N.A. Verified List 10 to 1,000+ (2.3); HSBC suppression (6.1) | Exemptions as governed, pair-keyed objects with maker-checker, expiry and automatic re-test on list or attribute change; growth metric; never customer-level exemption | Exemption register: entries, age, approver, volume passed, entries now matching a current designation (must be zero) | L-35 |
| FM-13 | **Screening scope gaps** (incoming payments, employees, vendors, restricted accounts, free text, incomplete fields) | Danske incoming payments unscreened (4.1); Block employees and restricted accounts (8.2); Habib 13,000 incomplete (3.4); Starling payments (6.4) | Scope matrix by party type, payment direction and field; fail-closed on missing screening fields; re-screen on every list change | Scope matrix report; incomplete-record rate per source; delta re-screen counts | L-33, L-34 |
| FM-14 | **Location evidence ignored** (fax number, IP address, phone prefix, instruction channel) | Standard Chartered Dubai fax and IP (3.1); Swedbank Latvia IP (4.2); Interactive Brokers (9.4); Paxful, ShapeShift, Binance (8, 9.4) | Customer location evidence profile feeding screening and risk; geofence list completeness and test; VPN indicator; instruction-channel field | Share of customers and payments with each evidence type; test log (login from a restricted country); alerts raised | L-37 |
| FM-15 | **Message stripping, cover payments and field manipulation** | Standard Chartered field 52 "." (3.1); BNP code word, MT 202 (3.2); Commerzbank (3.3); HSBC (2.4); Mashreq blank field 52 (3.5) | Message-integrity analytics (blank or generic fields, cover share, manual overtyping, routing diversions) by operator, customer and correspondent; payment-quality monitor; phase 2 inline checks; evasion-language search | Rates per operator, customer and correspondent; list of rules that divert named parties; change history of routing rules | L-38, L-40 |
| FM-16 | **Rejected or held payments re-routed or answered falsely** | Swedbank Latvia re-routing (4.2); BNP January 2012 hold (3.2); Standard Chartered filter release error (3.1) | Resubmission detector; four-eyes release with reason code; a returned or held payment opens a case and look-back; answers to correspondents drafted from system facts | Reject and hold register; resubmission alerts; look-back results; response audit trail | L-39 |
| FM-17 | **Designation exposure discovered late** | Banque Misr UAE customers designated after USD 32m processed (9.1); HSBC USA (2.4) | Designation look-back (flows before and after the date), derived-ownership checks, time-to-freeze measure | Look-back report per designation; time from list publication to re-screen and freeze | L-41 |

### 10.D Customer risk and KYC

| ID | Failure mode | Cases | Control AKR-TMS must have | Evidence AKR-TMS must produce | Req. |
|---|---|---|---|---|---|
| FM-18 | **Weak or missing customer risk rating** (country-only, default standard, missing inputs, static) | HSBC USA country-only rating, Mexico score (2.4); Nationwide (6.7); US Bancorp (2.2); Barclays static low (6.6); Coinbase (8.1); ING (4.3) | Rating coverage, input completeness and event-driven re-rating; no-rating means high; rating changes intensity, never coverage | Ratings by segment, customers unrated, migration matrix, trigger log | L-43, L-44 |
| FM-19 | **Periodic and EDD reviews overdue** | Standard Chartered UAE 1,700 overdue (3.1); Coinbase 14,000 EDD (8.1); Nationwide (6.7) | Review calendar with ageing; exit-candidate queue for reviews that cannot complete | Overdue reviews by age, risk and reason; exits | L-45 |
| FM-20 | **Information silos** (law-enforcement requests, adverse media, staff reports, branch observations) | Barclays (6.6); NatWest (6.2); TD UTR throttling (2.1); Capital One (2.8) | Authority and FIU request register and internal reports linked to alerts and re-rating; third-party exposure search | Linkage statistics; time from trigger to re-assessment; unanswered internal reports | L-46 |
| FM-21 | **Explanations accepted in place of evidence** | Rabobank N.A. (2.3); Capital One (2.8); Monzo (6.5) | Explanations stored as claims with corroboration status and expiry; closure needs independent evidence | List of closures that rest only on an explanation | L-47 |
| FM-22 | **Identity and address plausibility, duplicates, previously exited customers** | Monzo (6.5); Binance (8.3); ING segmentation (4.3) | Plausibility checks, entity resolution against exited and refused customers, segment verification from data | Flagged records and their disposition | L-48, L-49 |
| FM-23 | **Thresholds of the bank's own limits exploited** (just below KYC, tier or reporting limits) | Binance 2 bitcoin (8.3); US structuring (2.3) | Scenarios for activity just below each limit; low-tier behaviour monitoring | Near-threshold alert report | L-51 |

### 10.E Operations and governance

| ID | Failure mode | Cases | Control AKR-TMS must have | Evidence AKR-TMS must produce | Req. |
|---|---|---|---|---|---|
| FM-24 | **Alert and case backlog; late STRs** | HSBC 17,000 (2.4); Coinbase 100,000 (8.1); Block 169,000 and 129 days (8.2); Mashreq (3.5); N26 (7.1); Metro (6.3) | Ageing and capacity dashboard; volume elasticity forecast; alert-to-case and alert-to-STR clocks; supervisor notification tasks | Ageing buckets, forecasts, time-to-STR distribution, notification records | L-31, L-52 |
| FM-25 | **Poor disposition quality; same-person review; contractor surge** | Mashreq (3.5); Coinbase 73,000 alerts (8.1); ING 49 alerts closed (4.3); Deutsche (2.5) | Four-eyes enforced in the tool; structured disposition; QC sample queue; reviewer-level failure rate; bulk reopen | QC results; closure quality by reviewer; re-review records | L-53, L-54 |
| FM-26 | **Governance failure: cost targets, concealment, edited minutes, falsified records** | TD flat cost (2.1); US Bancorp (2.2); Rabobank N.A. (2.3); Metro minutes (6.3); Canaccord (2.7); Swedbank 2026 (9.2) | System-generated review evidence; immutable logs; board pack from system facts; decision register; edit history | Audit-ready log of parameter changes, reviews and decisions with edit history | L-55 to L-57 |
| FM-27 | **Remediation never finishes; findings recur** | NBP six exams (3.6); Deutsche 2017 to 2023 (2.5); UBSFS repeat after 2018 (2.6) | Findings register with independent validation and recurrence count; supervisory requirement tracker | Findings by theme, age, recurrence, closure tests | L-58 |
| FM-28 | **Warnings not escalated** | Standard Chartered 2005 and 2012 (3.1); Danske 2014 (4.1); Metro 2017 to 2018 (6.3) | Warnings register (staff, audit, IT, external signals) with action and age | Open warnings by age and owner | L-59 |
| FM-29 | **Controls reported green without evidence** | Danske branch reports (4.1) | Control status computed from latest test evidence | Control status table with last test date and result | L-61 |
| FM-30 | **New product or channel without assessment** | TD Zelle (2.1); NBP remittance (3.6); CBA machines (5.2); Dinosaur (6.8); Revolut ECB (7.2) | Launch gate: risk assessment, monitoring decision, limits, identity standard, owner | Gate records per product | L-03 |

### 10.F Relationships and groups

| ID | Failure mode | Cases | Control AKR-TMS must have | Evidence AKR-TMS must produce | Req. |
|---|---|---|---|---|---|
| FM-31 | **Nested, omnibus and pass-through relationships; unknown end-customers** | BNP regional banks (3.2); Danske intermediaries (4.1); Binance sub-accounts (8.3); Deutsche correspondents (2.5); HSBC correspondents (6.1) | Omnibus and sub-account register; pass-through detection; client-list requirement; "opaque" label and cap; exit triggers when beneficial owners are refused | Respondent cards: flows, share to typologies, attestation date, client-list status | L-63 |
| FM-32 | **Assurances accepted without verification** | Paxos (9.3); Binance (8.3); Danske (4.1) | Assertion-versus-data card; control tests as a user | Relationship cards with last independent test | L-64 |
| FM-33 | **Group blind spots; evidence cannot be produced across the group** | Danske (4.1); Commerzbank (3.3); Swedbank 2026 (9.2); HSBC Group Entities (2.4) | Entity and system ledger; regulatory request workbench across all units; scope log with exclusions on the cover sheet | Request response packs with unit-by-unit counts and recorded scope | L-62, L-68 |
| FM-34 | **Policy contradicted by practice; legal opinions used to excuse practice** | BNP Sudan policy (3.2); Mashreq opinion (3.5); Binance paper-only programme (8.3) | Policy-to-rule map; opinions stored with the facts given and an expiry; opinions cannot switch off rules | Policy statements with mapped rules and test dates; gaps | L-42 |
| FM-35 | **Typology exposure through customers' flows** | Banque Misr UAE (9.1); MBaer (9.1) | Typology exposure index and front-company profile scoring; respondent view | Monthly index and trend per bank, branch and correspondent | L-65, L-66 |
| FM-36 | **Insider facilitation** | TD employees bribed (2.1); NatWest staff suspicions (6.2); Danske collusion (4.1) | Staff and staff-linked account monitoring; teller and branch pattern analytics; conflict rules | Staff-linked alert report | L-67 |

## 11. What it means for AKR-TMS

### 11.1 What the record shows

About 40 enforcement actions and findings were read for this file; roughly 35 rest on primary documents or official summaries (grades A and B), and five
(Westpac, Commonwealth Bank, Revolut, OKX, ABN AMRO) are grade C; N26 is grade B for 2021 and 2024 and C for later steps. Across them, the cause was seldom a weak algorithm. The recurring causes were:

1. **The monitoring system was not given the data** (TD Bank 92 percent of volume, Metro Bank 60 million transactions, HSBC SEPA and correspondent books, UBS Financial
   Services' partial feed, Danske's separate platform, Capital One's coding changes, Dinosaur's new platform).
2. **The system produced alerts that someone had decided to limit** (US Bancorp, ING, Commerzbank, HSBC suppression, TD's removed scenarios, Canaccord's filters), usually
   because of staff capacity or cost.
3. **Nobody measured whether the system worked**: no below-the-line testing, no reconciliation, no known-answer tests (Starling's 39 of 3,088, HSBC's 5,000-times thresholds),
   no check of the reporting pipeline (CBA, Westpac, Capital One).
4. **The customer data that makes monitoring meaningful was missing or unused** (Nationwide's default rating, Monzo's thin onboarding, NatWest's declared "no cash", Standard
   Chartered's fax numbers and IP addresses, Swedbank Latvia's IP data, Binance's identity-less accounts).
5. **Governance bent or hid the problem** (cost targets in TD, edited minutes in Metro, altered presentations in US Bancorp, withheld reports in Rabobank, falsified review records in
   Canaccord, narrowed scope in Swedbank 2026) and **remediation did not finish** (Deutsche, National Bank of Pakistan, UBS).
6. **A relationship was accepted on assurances** (Paxos and Binance, Danske and its branch, BNP's regional banks, Mashreq's legal opinion).

### 11.2 The ten findings that matter most for the product

1. **Prove coverage, daily.** A monitoring system must produce, from the source data and independently of its own pipeline, the share of transactions by count and value that it
   monitors, and must reconcile that share every day. TD, Metro, HSBC and UBS are four versions of the same failure; Metro's FCA notice describes the three missing controls (count
   reconciliation, a bad-data queue that is read, an escalation that cannot be deleted).
2. **Never let capacity set the detection logic.** No alert cap, no suppression of reviewable alerts, no private filter. Capacity is handled in the queue. A capacity-driven change needs
   reason codes, below-the-line evidence and approval outside operations.
3. **Test the engines with known answers.** A daily harness of seeded names, accounts and transactions against every list and scenario catches "39 of 3,088", a vanished
   transaction type and an unreachable threshold. Silence (zero alerts) is itself an alarm.
4. **Unknown is not low.** Missing country, name, address, counterparty or rating raises risk, is counted and is visible.
5. **Use all the location and instruction evidence the bank already has** (IP address, phone prefix, fax, branch, device, KYC address) in screening and risk. Standard Chartered, Swedbank,
   Interactive Brokers, Paxful and Binance show the sanction exposure is created by data that was collected and not used.
6. **Treat exemption lists and explanations as risks.** Good-guy lists, "verified" lists, relationship-manager explanations and legal opinions can switch controls off; each is a governed,
   expiring, re-tested object.
7. **Make the evidence system-generated.** Review logs, parameter changes, board packs and decisions come from the system with edit history; typed attestations are exactly what failed in
   US Bancorp, Rabobank, Metro and Canaccord.
8. **Measure the clocks and the queues**: data latency, alert to case, alert to STR, overdue reviews, alerts per analyst, QC failure rate, surge-staff quality; forecast volume from
   growth (Coinbase 25 times in under two years).
9. **Show the bank what its correspondents can compute.** FinCEN's finding on Banque Misr UAE is a ratio of typology flows to assets and a count of front companies, both computable by the
   bank itself from its own ledger; the same view per correspondent and per branch defends dollar access (file 01 section 4, file 04 W-22).
10. **Be able to answer any request across the whole group**, with the scope recorded, because evasive scoping cost Swedbank USD 50 million in 2026 and the lack of group data cost Danske and
    Commerzbank.

### 11.3 Architecture implications for a read-only, CBS-agnostic design

- **AKR-TMS is the independent check.** Because it reads the CBS database directly and does not depend on the bank's reporting or monitoring code, it can recompute coverage, reportable events and
  screening populations from the source, which is what Metro, Capital One, CBA and UBS lacked. This is the product's main design advantage over a vendor tool fed by the bank's own extracts and
  should be stated as a principle: **AKR-TMS never trusts a feed it did not reconcile**.
- **The canonical model needs a "control plane" next to the data plane.** The data plane holds customers, accounts, transactions and lists. The control plane holds the coverage ledger, reconciliation
  results, mapping register, exclusion and exemption registers, parameter change log, test-harness results, queue clocks, findings and warnings registers. Every item in section 10's "Evidence" column
  is a view on the control plane; the control plane is append-only and exportable (file 32 for the stack, file 29 for governance).
- **Mapping layer rules** (file 25): quarantine rather than reject; defaults never silently assigned; drift detection on every classifying field; before and after population comparison after every
  CBS upgrade; a single customer key with an orphan-account metric.
- **Scenarios are products, not rules in a file**: purpose, owner, inputs with lineage, unit tests, reach table, below-the-line sampling, recalibration clock, change history (file 27, file 29).
- **Screening is configured, tested and watched**: governed lists and exemption objects, known-answer harness, silence alarm, location evidence, delta re-screen (file 04 U-01 to U-09, W-07 to W-10).
- **Case management is part of detection quality**: structured disposition, enforced four-eyes, QC queues, clocks, surge-staff controls, internal-request tracker for branches (file 28, file 31).

### 11.4 Phase 1 and phase 2

| Control | Phase 1 (read-only post-event) | Phase 2 (optional pre-release API) |
|---|---|---|
| Coverage ledger, reconciliation, mapping register, quarantine | Full | Same, plus gateway message counts |
| Scenarios, tuning, below-the-line, validation | Full | Scores for held payments |
| Screening: customers, owners, employees, lists | Full, near-real-time on CBS changes | - |
| Screening: payments | Post-event on stored payment and message data; lists | Pre-release check; resubmission block; four-eyes release |
| Message-integrity analytics | On message data stored in the CBS or gateway database (field quality, cover share) | Inline enforcement of completeness; detection of routing diversions as they occur |
| Location evidence | KYC address, phones, stored login or IP data if the bank grants read access | Gateway IP, device, geofence test hooks |
| Rejected or held payment look-back | Using reject and return records | Block of re-routing in real time |
| Evidence generation | Full | Same |

Message stripping and re-routing are the failures that **post-event** monitoring detects only after the money has left; the case for phase 2 rests on Standard Chartered, BNP, Commerzbank
and Swedbank Latvia. In phase 1 AKR-TMS must still detect their statistical footprint (section 3.1 lessons) and report it.

### 11.5 Iraq-specific implications

| Iraqi context (files 01, 02, 05) | Implication from the cases |
|---|---|
| Dollar access under US Treasury and Federal Reserve scrutiny; correspondents in Jordan, the UAE, Kuwait | Respondent view and typology exposure index (L-65); evidence pack for questionnaires; resubmission and rejected-payment look-back (L-39); Mashreq and Banque Misr UAE show the Gulf correspondent chain is itself under review |
| Proximity to Iran; oil and gas payments; Iranian oil "disguised as Iraqi crude" | Location evidence (L-37); front-company profile (L-66); designation look-back (L-41); BNP's Turkmen gas payments for Iraq via a UAE company with Iranian owners (3.2) |
| State-owned banks and ministry-linked customers | Findings register with an independent owner (L-58); no exemption lists for "important" customers (L-35); National Bank of Pakistan (3.6) and Banque Misr (9.1) are state-owned banks |
| Exchange houses, money transfer agents, e-wallets, cash economy, informal channels | Omnibus register and opaque-flow label (L-63); channel launch gate (L-03); identity-less account tiers with limits (L-50); near-threshold scenarios on the SDD ceiling (L-51) |
| Many branches, Kurdistan, Islamic windows, possibly several CBS instances | Entity and system ledger (L-01, L-62); "dark unit" rule (Danske); request workbench across units |
| Arabic and Kurdish data, family number and mother's name | Language tags for documents (L-68); Arabic-and-Latin known-answer harness (L-32); duplicate and previously exited customer detection using family number (L-48) |
| CBI controls require direct CBS link, real-time monitoring, branch liaison officers and internal report acknowledgements (file 01 2d) | Internal-request tracker and linked internal reports (L-46); two clocks (L-31); the CBI's own wording should be mapped to the L requirements in the policy map (L-42) |

### 11.6 Ten questions a board or MLRO can ask on the strength of these cases

1. What share of our transactions, by count and by value, does the monitoring system see today, and when was that last reconciled against the core banking system?
2. List every product, channel, branch, subsidiary and currency that is not monitored, who accepted that, and until when.
3. When did we last change a threshold or switch a scenario off, and was the reason a risk or a workload?
4. How many alerts are older than our service level, how many analysts do we have per 1,000 alerts, and what is the forecast for the next two quarters?
5. Does the screening system find a seeded test name from each list today, in Arabic and in Latin script?
6. How many customers have no computed risk rating, an unknown country or a missing expected-activity profile?
7. Which exemption, "trusted" or whitelist entries exist, who approved them, and does any match a current designation?
8. What do our correspondents see about our flows to Iran-linked, front-company and exchange-house typologies, and has that figure moved?
9. For the last request from an authority, which units and systems were searched, and which were left out?
10. What did the independent validator or internal audit find last year, which findings repeat, and who closed them?

### 11.7 Cautions and gaps

- Grade C cases (Westpac, CBA, Revolut, OKX, ABN AMRO, and N26 after 2024) must be re-checked before the plan PDF quotes numbers; the lead's brief wording for Rabobank N.A. ("thresholds altered") is not supported by the documents read.
- No primary document was reached for DOJ resolutions (justice.gov blocks automated access), AUSTRAC, the Dutch ABN AMRO statement of facts, BaFin's 2025 N26 measures, Bank of Lithuania, the Central Bank of the UAE
  and the Swedish FSA; the web search tool was exhausted for this session (200 of 200 calls), so 2025-2026 coverage comes from reading regulators' own listing pages and is not a complete sweep.
- Cases show what supervisors found, not the whole truth; many involve fines negotiated under settlement, with admissions limited to the statements of facts.
- The requirements below describe controls, not products: AKR-TMS cannot decide how much a bank spends on staff (the most common root cause), but it can make the consequence of that decision visible and exportable.

## 12. Derived requirements for AKR-TMS (from this file)

IDs use prefix L (lessons). They extend, and do not repeat, the requirements in files 03 (F), 04 (B, W, E, U) and 05 (R); where a requirement sharpens one of those it says so. "Phase" is 1 (read-only
post-event) or 2 (optional pre-release API). Priority: M = must have at go-live; S = should have within the first year; C = could have later.

| ID | Requirement | Source | Phase | Pri |
|---|---|---|---|---|
| L-01 | **Coverage ledger** listing every entity, source system, product, channel, currency and payment direction with the share of transactions monitored by count and by value; units or sources not connected are shown as "dark" with known volume (extends B-01) | TD 2.1; Danske 4.1; Dinosaur 6.8; UBSFS 2.6; HSBC 6.1 | 1 | M |
| L-02 | **Exclusion register**: every decision not to monitor a type, product, entity or population has an owner, reason, approver, review date and expiry; an expired or missing approval is an open finding on the main compliance screen | TD 2.1 | 1 | M |
| L-03 | **New code, product and channel gate**: a new CBS transaction code, product or channel raises an "unmapped" item; the item stays open until a risk assessment, monitoring decision, limits and identity standard are recorded; the gate record is evidence | TD Zelle 2.1; NBP 3.6; CBA 5.2; Dinosaur 6.8; Revolut 7.2 | 1 | M |
| L-04 | **Daily reconciliation** of records read, accepted and rejected, with counts and sums, per source table and per transaction type, between the CBS replica and the canonical model; any difference beyond a stated tolerance creates a ticket with owner and age (extends B-01) | Metro 6.3; HSBC 6.1; UBSFS 2.6 | 1 | M |
| L-05 | **No silent rejection**: records failing referential or format rules go to a quarantine queue with reason, age and value, are retried when the missing record arrives (late-arriving account or customer), and are shown on the dashboard | Metro 6.3; UBSFS 2.6 | 1 | M |
| L-06 | **Reference-value drift detection**: the set of distinct values of every classifying field (transaction code, status label, channel, currency) is stored; a new, renamed or vanished value alerts with the volume affected; unmapped values are never assigned a default type | UBSFS "FX RCVD" 2.6; HSBC 6.1; Capital One 2.8 | 1 | M |
| L-07 | **Mapping register** from each CBS code to the canonical type, governed with change history; every CBS upgrade, patch or schema change triggers an automated before-and-after comparison of the mapped population by customer, account and value | HSBC 6.1; Metro 6.3; Capital One 2.8 | 1 | M |
| L-08 | **Attribute-level data lineage** from CBS column to each scenario and screening input, with null rate, last validation date and test; a scenario with an unmapped or failing input is marked "not operating as intended" (extends B-05) | Deutsche Fed 2023 2.5; UBSFS 2.6; Canaccord 2.7 | 1 | M |
| L-09 | **Single customer key and orphan metric**: one identity across CBS modules, trade, cards and payments; share of transactions and value on accounts not linked to a KYC record is a daily metric with a limit; such transactions alert by default | Deutsche 2.5; HSBC 6.1 | 1 | M |
| L-10 | **Unknown is not low**: an unknown or missing country, counterparty, address, name or rating assigns the highest tier of that field; the counts are reported | UBSFS 2.6; Binance 8.3; Nationwide 6.7 | 1 | M |
| L-11 | **Independent recomputation of reportable events** (cash above the CBI threshold, cross-border transfers, freeze-related events) from the CBS by channel and code, reconciled daily with the reports submitted to goAML and the FIU acknowledgements; a shortfall opens an incident the same day, including cash-in-transit and bank-to-bank cash (extends E-06) | CBA 5.2; Westpac 5.1; Capital One 2.8; US Bancorp 2.2 | 1 | M |
| L-12 | **STR and cash-report pre-submission checks**: party roles (subject, victim, on-behalf-of), mandatory identifiers and the beneficial party of cash deposits are validated before submission (extends E-03) | US Bancorp CTR names 2.2; Capital One 2.8 | 1 | S |
| L-13 | **Scenario result retention**: each run stores triggering transactions, scores and parameters so a look-back can be run from stored results without reloading history | ING 4.3 | 1 | M |
| L-14 | **De-duplication and exclusion rules are reconciled**: records excluded by a de-duplication or "no monitoring needed" rule (internal transfers, bank-own entries) are counted, sampled, and shown separately so they cannot mask real rejections | HSBC SEPA 6.1; Metro 6.3 | 1 | M |
| L-15 | **Cash and product classification test**: the mapping layer compares teller, cash-in and cash-in-transit codes with the transaction type assigned and reports cash classified as non-cash | NatWest 6.2; HSBC 6.1; Capital One 2.8 | 1 | M |
| L-16 | **No cap on alerts**: no parameter limits the number of alerts per scenario per period; a flat-line detector flags any scenario that produces the same count for N consecutive days; surplus alerts are queued with an age clock | US Bancorp 2.2; ING 4.3 | 1 | M |
| L-17 | **Suppression is merge, not hide**: duplicate or repeat alerts are grouped into one case with every underlying alert visible; the engine offers no suppression of reviewable alerts; a suppression ledger shows counts and the productive rate from samples for any grouping rule | US Bancorp 90-day rule 2.2; HSBC 6.1; ING 4.3 | 1 | M |
| L-18 | **Threshold change governance**: every change records reason code (risk-driven or capacity-driven), before-and-after alert and below-the-line evidence, requester and a second approver outside operations; capacity-driven changes and scenario removals (with a redeploy date) appear in the board pack; no dashboard ranks staff or functions by alerts reduced | Commerzbank 3.3; TD 2.1; US Bancorp 2.2 | 1 | M |
| L-19 | **Below-the-line testing as a scheduled job** per scenario: a sample just under the threshold goes to analysts as calibration cases; the productive rate is reported and a rate above the bank's tolerance sets "threshold review required"; monthly sampling of closed "not suspicious" alerts | US Bancorp 2.2; TD 2.1; HSBC 6.1; ING 4.3 | 1 | M |
| L-20 | **Reachability and dead-rule test**: for each scenario and segment, the share of customers who could ever trigger it (including product limits and the multiple of peak activity needed) is computed; rules above a set multiple or with no alert in N months despite eligible volume are flagged | HSBC 6.1; TD Zelle 2.1 | 1 | M |
| L-21 | **Segmented thresholds from go-live**: by customer type, size, risk, peer group, product and region; a single threshold per product is flagged | TD 2.1; Nationwide 6.7; HSBC 6.1 | 1 | M |
| L-22 | **Scenario logic unit tests** shipped with each scenario (positive and negative cases) and run on every release and parameter change; a failing test blocks activation | HSBC structuring bug 6.1 | 1 | M |
| L-23 | **Mandatory scenario record**: purpose, owner, typologies covered, data inputs, expected alert volume range, success measure, last tuned, approver; activation refused without them | Nationwide 2016 review 6.7; Starling 6.4 | 1 | M |
| L-24 | **Recalibration clock and coverage review**: each scenario has a last-tuned date and interval (amber, then red at twice the interval); an annual review lists typology indicators not covered, against the CBI scenario catalogue (file 27) | HSBC 6.1; TD 2.1; Deutsche 2.5 | 1 | M |
| L-25 | **Typology circular register** (CBI, FIU, FATF, FinCEN, correspondents): each item mapped to scenarios with the date mapped | Block mixers 8.2; Banque Misr 9.1 | 1 | S |
| L-26 | **Client-level and related-party aggregation** for every aggregable scenario (all accounts, products and linked parties) | ING 4.3; Barclays 6.6 | 1 | M |
| L-27 | **Documented risk-ranked prioritisation** combining relative deviation, absolute value, customer risk, counterparty risk and typology, with a test case set (the EUR 100 versus EUR 99 million case) | ING 4.3 | 1 | S |
| L-28 | **Structured declared profile** (expected turnover, cash use, purpose, counterparties, income) with expected-versus-actual ratio alerts shown beside the declared figure | NatWest 6.2; Nationwide 6.7; Barclays 6.6 | 1 | M |
| L-29 | **Silence alarm**: baseline alert counts by day per scenario and per list; zero alerts or a fall beyond a set percentage for N days raises an incident | Starling 6.4; ING 4.3 | 1 | M |
| L-30 | **Independent validation workspace**: replay of each rule on the bank's history, below-the-line samples and stored results; validator role differs from configuration owner; a document-only validation is labelled "not tested" (extends B-05, W-11) | US Bancorp 2.2; Mashreq 3.5 | 1 | M |
| L-31 | **Two clocks** per alert: data latency (transaction time to alert creation) and handling time (alert to disposition), with limits and breach display | Mashreq 3.5 | 1 | M |
| L-32 | **Known-answer screening harness**: seeded names, aliases, transliterations (Arabic, Kurdish, Latin), entities and payments run daily against every list, matching mode and payment path, with dated pass-fail reports; failures raise incidents (extends W-10) | Starling 6.4; Interactive Brokers 9.4 | 1 | M |
| L-33 | **Screening scope matrix** by party type (customer, owner, signatory, employee, supplier, service provider, restricted or low-tier account), payment direction and field (name, address, vessel, free text), shown per list | Danske 4.1; Block 8.2; Starling 6.4 | 1 | M |
| L-34 | **Fail closed on missing screening fields**: a record without a screening-critical field is held or flagged, never passed; incomplete-record rate per source is a control metric (extends W-12) | Habib 3.4 | 1 | M |
| L-35 | **Exemption and good-guy governance**: pair-keyed entries, maker-checker, expiry, automatic re-test on any list or attribute change, size and growth metric, and a report of entries now matching a current designation (must be zero); customer-level exemption is not available (extends W-07) | Habib 3.4; Rabobank N.A. 2.3; HSBC 6.1 | 1 | M |
| L-36 | **Zero-tolerance categories**: matches to designated, terrorist or prohibited-jurisdiction lists have no minimum score, materiality or exposure threshold; configurations that add one are rejected | Block 8.2 | 1 | M |
| L-37 | **Customer location evidence profile**: KYC address, phone prefixes, instruction channel (fax, e-mail, branch, online), login country and IP, device and beneficial owner nationality feed screening and risk; geofence list completeness and a user-style test are recorded; coverage by evidence type is reported | Standard Chartered 3.1; Swedbank Latvia 4.2; Interactive Brokers 9.4; Paxful 8.4 | 1 (2 inline) | M |
| L-38 | **Message-integrity analytics**: rate of blank, "." or generic ordering and beneficiary fields, share of cover payments, manual overtyping, repair-queue diversions and routing or hold rules that single out a named party, by operator, branch, customer and correspondent; inline enforcement in phase 2 (extends W-12) | Standard Chartered 3.1; BNP 3.2; Commerzbank 3.3; Mashreq 3.5; HSBC 2.4 | 1 and 2 | M |
| L-39 | **Rejected, returned and held payments**: a register; a resubmission detector (same or near-same beneficiary, amount, reference, or different correspondent or user); four-eyes release with reason code; each reject opens a customer case and a look-back for similar payments; replies to correspondents are drafted from system facts and logged | Swedbank Latvia 4.2; BNP 3.2; Standard Chartered 3.1 | 1 and 2 | M |
| L-40 | **Evasion-language search** in KYC notes, payment text and messages (Arabic and English) for terms such as sanction, embargo, OFAC, "do not mention", code names | BNP 3.2; Standard Chartered 3.1 | 1 | S |
| L-41 | **Designation look-back**: on any new designation, list historical flows with the party (before and after the effective date), connected accounts and owners, total value, and time from publication to re-screen and freeze (extends U-05, U-06) | Banque Misr UAE 9.1 | 1 | M |
| L-42 | **Policy-to-rule map**: each prohibition and requirement in the bank's policy and in the CBI controls maps to a named rule with a test date; unmapped statements are gaps; external legal opinions are stored with the facts supplied and an expiry and cannot disable a rule | BNP 3.2; Mashreq 3.5; Binance 8.3 | 1 | S |
| L-43 | **Risk rating completeness and re-rating triggers**: customers with no computed rating are treated as high until rated; missing-input share by rating factor; automatic reopening on law-enforcement requests, court orders, freezes, adverse media, business-model change and large unexplained inflows; rating changes intensity, never coverage | Nationwide 6.7; HSBC 2.4; Barclays 6.6; US Bancorp 2.2 | 1 | M |
| L-44 | **KYC completeness score** per customer (purpose and nature, expected activity, source of funds, owners, verified address, occupation); scenarios depending on a missing field mark the customer "degraded" and apply stricter defaults; completeness by segment and branch to the board | Monzo 6.5; Nationwide 6.7 | 1 | M |
| L-45 | **Review calendar with ageing**: overdue periodic, trigger and EDD reviews by age and risk; reviews that cannot complete (customer will not supply data) move to an exit-candidate queue with a deadline | Standard Chartered 3.1; Coinbase 8.1 | 1 | M |
| L-46 | **Authority-request and internal-report linkage**: FIU and authority requests, court orders, branch and teller internal reports are linked to the customer, accounts and alerts, shown to the closing analyst, trigger re-rating, and escalate when undecided after N days; a search lists all customers who transacted with a counterparty under investigation (extends E-01) | Barclays 6.6; NatWest 6.2; TD 2.1 | 1 | M |
| L-47 | **Explanations as claims**: customer or relationship-manager explanations are stored with source, date, corroboration status and expiry; an alert cannot be closed on the explanation alone | Rabobank N.A. 2.3; Capital One 2.8; Monzo 6.5 | 1 | M |
| L-48 | **Identity and address plausibility and duplicate detection**: PO boxes, landmarks, addresses outside the country of operation, shared contact data, and matches to previously exited or refused customers (including by family number) | Monzo 6.5 | 1 | S |
| L-49 | **Segment and rating verified from data**: segment membership and expected risk are recomputed from observed turnover, products and counterparties; disagreement with the declared segment alerts | ING 4.3 | 1 | S |
| L-50 | **Identity-less and low-tier account register** with stated limits, products and counterparties per tier; accounts lacking screening fields listed by value; no cross-border use; tier-behaviour monitoring | Binance 8.3 | 1 | M |
| L-51 | **Near-threshold scenarios** on the bank's own limits (SDD ceiling, cash reporting threshold, FX and tier limits) and on cash-in followed by transfer-out | Binance 8.3; CBA 5.2; Rabobank N.A. 2.3 | 1 | M |
| L-52 | **Queue ageing and capacity dashboard**: alerts per analyst, overdue buckets against the bank's service level, alert-to-case and alert-to-STR medians, 90th percentile and maximum, volume elasticity forecast (alerts per 1,000 transactions against growth), notification task for the MLRO when a limit is crossed with a record of supervisor notification | Coinbase 8.1; Block 8.2; HSBC 2.4; N26 7.1 | 1 | M |
| L-53 | **Surge and outsourced staffing controls**: time-boxed roles, tracked training before queue access, higher QC sampling for new reviewers, reviewer-level failure rates, bulk reopen of a reviewer's cleared alerts by date range | Coinbase 8.1 | 1 | M |
| L-54 | **Enforced four-eyes and structured disposition**: the same user cannot perform level 1 and level 2 on one alert; closure requires fields (relationship between parties, expected versus actual, source of funds evidence, counterparty bank and country, decision, rationale) with a generic-text check; separate QC queue with its own age | Mashreq 3.5; ING 4.3; Deutsche 2.5 | 1 | M |
| L-55 | **System-generated review evidence**: who opened which items, when, for how long and with what decision is recorded by the tool and cannot be edited or uploaded; an "unreviewed items" report lists any queue or report without recorded review in the period; policy documents carry system version history | Canaccord 2.7; US Bancorp 2.2 | 1 | M |
| L-56 | **Board and supervisor pack generated from system facts** (coverage, reconciliation, parameter changes, backlog, exemptions, tests); any manual edit is shown with author and time (extends B-11) | US Bancorp 2.2; TD 2.1 | 1 | M |
| L-57 | **Decision register** for risk acceptances, exclusions, exemptions, exits and reinstatements, with approver, rationale, attendees and expiry; issue and risk logs cannot have entries removed without a visible history | BNP 3.2; HSBC USA 2.4; Metro 6.3 | 1 | M |
| L-58 | **Findings register with independent validation**: findings from the CBI, auditors, validators and correspondents logged once, themed, owned, closed only after a test accepted by an independent function; recurrence across cycles counted and shown; supervisory restrictions tracked as controls with a daily breach test | NBP 3.6; Deutsche 2.5; Starling 6.4; UBSFS 2.6 | 1 | M |
| L-59 | **Warnings register**: concerns raised by staff, audit, IT and external signals (regulators, correspondents, foreign FIUs, press) with action, owner and age; open items beyond a limit go to the board committee | Standard Chartered 3.1; Danske 4.1; Metro 6.3 | 1 | S |
| L-60 | **Control evidence pack**: one export assembling all "evidence" items of section 10 for a date range, usable for the CBI, auditors and the correspondent questionnaires (extends W-06) | Whole file | 1 | M |
| L-61 | **Control status computed from test evidence**: a control is green only if its latest test is in date and passed; self-assessment cannot override | Danske 4.1 | 1 | M |
| L-62 | **Group request workbench and unit ledger**: one search over customers, owners, signatories, counterparties, payment text and documents across all units and systems in the data set; recorded scope, exclusions with reason and approver, per-unit counts; units not connected shown on the cover sheet (extends E-01) | Swedbank 2026 9.2; Danske 4.1; Commerzbank 3.3 | 1 | M |
| L-63 | **Omnibus, sub-account and nested relationship register**: counterparties (banks, exchange houses, PSPs, agents) that give their own customers access, with sub-account counts, flows, client-list status and last attestation; matched in-and-out pass-through detection across a one to three day window; "opaque" label and cap when no client data (extends B-12, W-22, W-25) | BNP 3.2; Binance 8.3; Danske 4.1; HSBC 6.1 | 1 | M |
| L-64 | **Assertion-versus-data cards** for correspondents and partners: what they stated, what the bank's data show, date of last independent test; user-style control tests recorded | Paxos 9.3; Binance 8.3 | 1 | S |
| L-65 | **Typology exposure index** per bank, branch and correspondent: value and count of flows matching each published typology, ratio to assets and to total flows, monthly trend; respondent view of what correspondents can compute (extends W-22, U-08) | Banque Misr UAE 9.1 | 1 | M |
| L-66 | **Front-company profile scoring** with stated reasons (third-country registration, shared address, director or signatory, commodity trading, in-and-out flows, no footprint, exchange-house counterparties) | Banque Misr UAE 9.1; MBaer 9.1 | 1 | S |
| L-67 | **Staff and branch behaviour analytics**: staff-linked accounts and accounts they manage, teller and branch patterns around cash-report errors and repeated explanations, conflict rules | TD 2.1; NatWest 6.2; Danske 4.1 | 1 | S |
| L-68 | **Language tags and required summaries** for documents in the evidence vault that are not in the working language of the reviewing unit (Arabic, English, Kurdish) | Danske 4.1 | 1 | C |

**Count:** 68 requirements (L-01 to L-68): 56 must-have, 11 should-have, 1 could-have. All apply to phase 1; three (L-37, L-38, L-39) also need phase 2 for inline enforcement.

## 13. Sources

All addresses were retrieved on 6 October 2026 (the sandbox clock). Grade: A primary document read, B official summary or press release, C secondary. Downloaded copies are in the session scratchpad
(`.../scratchpad/src/enf/`). Where a justice.gov, AUSTRAC, AustLII, Federal Court, Bank of Lithuania, UAE central bank, Swedish FSA, SEC or Federal Register web address was blocked, the entry says what was used instead.

### 13.1 United States: DOJ, Treasury, FinCEN, OCC, Federal Reserve, OFAC

| Label | URL | Grade |
|---|---|---|
| TD Bank N.A. plea agreement with statement of facts (Gibson Dunn copy of the court filing, 10 Oct 2024) | https://gibsondunn.com/wp-content/uploads/2025/02/TD-Bank-N.A.-plea-agreement-TD-Bank-1-of-2.pdf | A |
| Deputy Treasury Secretary remarks on TD Bank (10 Oct 2024) | https://home.treasury.gov/news/press-releases/jy2640 (copy: https://fraser.stlouisfed.org/files/docs/publications/treaspr/2024/2024-10-10_jy2640.pdf) | B |
| Federal Reserve press release on TD Bank (10 Oct 2024) | https://www.federalreserve.gov/newsevents/pressreleases/enforcement20241010a.htm | B |
| Latham and Watkins client alert No. 3308 on the TD Bank resolution (28 Oct 2024) | https://lw.com/admin/upload/SiteAttachments/Landmark-Bank-Secrecy-Act-Resolution-Signals-Increasing-Focus-on-Banks-Anti-Money-Laundering-Compliance.pdf | C |
| FinCEN assessment of civil money penalty, U.S. Bank N.A. (Feb 2018) | https://www.fincen.gov/sites/default/files/enforcement_action/2020-02-11/FinCEN%20U.S.%20Bank%20-%20Assesment%20-%20FinCEN%20review%202.14.18%20Final.pdf | A |
| Paul Weiss memo on U.S. Bancorp (21 Feb 2018) | https://www.paulweiss.com/insights/client-memos/us-bancorp-enters-into-deferred-prosecution-agreement-and-related-resolutions-and-agrees-to-pay-613-million-for-bsaaml-failures | C |
| Paul Weiss memo on Rabobank N.A. (13 Feb 2018) | https://www.paulweiss.com/insights/client-memos/rabobank-pleads-guilty-to-obstructing-occ-supervision-and-agrees-to-pay-368-million-for-permitting-and-concealing-bsaaml-failures | C |
| DOJ press release text on Rabobank N.A. (ICE copy) | https://www.ice.gov/news/releases/rabobank-national-association-pleads-guilty-agrees-pay-369-million | B |
| DOJ press release text on Rabobank N.A. (Mondo Visione copy) | https://mondovisione.com/media-and-resources/news/us-department-of-justice-rabobank-na-pleads-guilty-agrees-to-pay-over-360-mil-201828/ | B |
| Debevoise 2018 mid-year AML review (summarises the Rabobank plea agreement and OCC notice of charges) | https://www.debevoise.com/-/media/files/insights/publications/2018/07/20180719_2018_mid_year_anti_money_laundering_review_and_outlook.pdf | C |
| OCC civil money penalty order, HSBC Bank USA (Dec 2012) | https://occ.gov/static/enforcement-actions/ea2012-262.pdf | A |
| DOJ EDNY press release, HSBC deferred prosecution agreement (11 Dec 2012), archive copy | https://www.justice.gov/archive/usao/nye/pr/2012/2012dec11.html | B |
| Senate PSI hearing record including the report "U.S. Vulnerabilities to Money Laundering, Drugs, and Terrorist Financing: HSBC Case History" (July 2012) | https://www.govinfo.gov/content/pkg/CHRG-112shrg76061/pdf/CHRG-112shrg76061.pdf | A |
| HSBC 6-K exhibits (OFAC settlement agreement; OCC consent order) read through a text-extraction tool | https://www.sec.gov/Archives/edgar/data/0001089113/000119312512499974/d451985dex997.htm and .../d451985dex999.htm | B |
| FinCEN consent order No. 2026-02, UBS Financial Services Inc. | https://www.fincen.gov/system/files/2026-07/UBS-Consent-Order.pdf | A |
| FinCEN consent order No. 2026-01, Canaccord Genuity LLC | https://www.fincen.gov/system/files/2026-03/Canaccord-Consent-Order-No-2026-01.pdf | A |
| FinCEN assessment, Capital One N.A. (15 Jan 2021) | https://www.fincen.gov/system/files/enforcement_action/2023-04-05/Assessment_CONA_508_0.pdf | A |
| FinCEN consent order No. 2023-04, Binance Holdings Ltd (21 Nov 2023) | https://www.fincen.gov/system/files/enforcement_action/2023-11-21/FinCEN_Consent_Order_2023-04_FINAL508.pdf | A |
| FinCEN consent order, Paxful (listed 9 Dec 2025) | https://www.fincen.gov/system/files/2025-12/PaxfulConsentOrder.pdf | A |
| FinCEN enforcement actions list (pages 0 and 1) | https://www.fincen.gov/news-room/enforcement-actions and https://www.fincen.gov/news-room/enforcement-actions?page=1 | B |
| FinCEN section 311 and 9714 special measures page | https://www.fincen.gov/resources/statutes-and-regulations/311-and-9714-special-measures | B |
| FinCEN NPRM, Banque Misr UAE (91 FR 56085, 1 Sep 2026), govinfo PDF; metadata from the Federal Register API | https://www.govinfo.gov/content/pkg/FR-2026-09-01/pdf/2026-17871.pdf ; https://www.federalregister.gov/api/v1/documents/2026-17871.json | A |
| FinCEN NPRM, MBaer Merchant Bank AG (Federal Register 2 Mar 2026) | https://www.govinfo.gov/content/pkg/FR-2026-03-02/pdf/2026-04033.pdf | A (summary read) |
| FinCEN orders and rules, CIBanco (30 Jun 2025) and Huione Group (16 Oct 2025), abstracts | https://www.govinfo.gov/content/pkg/FR-2025-06-30/pdf/2025-11993.pdf ; https://www.govinfo.gov/content/pkg/FR-2025-10-16/pdf/2025-19571.pdf | B |
| OFAC settlement agreement, Standard Chartered Bank (10 Dec 2012) and enforcement information | https://ofac.treasury.gov/media/13726/download?inline ; https://ofac.treasury.gov/media/13801/download?inline (page: https://ofac.treasury.gov/recent-actions/20121210) | A |
| OFAC settlement agreement, Standard Chartered Bank (9 Apr 2019) and enforcement information | https://ofac.treasury.gov/media/13921/download?inline ; https://ofac.treasury.gov/media/26286/download?inline (page: https://ofac.treasury.gov/recent-actions/20190409) | A |
| Federal Reserve press release and order, Standard Chartered (9 Apr 2019) | https://www.federalreserve.gov/newsevents/pressreleases/enforcement20190409a.htm ; https://www.federalreserve.gov/newsevents/pressreleases/files/enf20190409a1.pdf | A |
| OFAC settlement agreement and enforcement information, BNP Paribas SA (30 Jun 2014) | https://ofac.treasury.gov/media/13521/download?inline ; https://ofac.treasury.gov/media/13516/download?inline | A |
| OFAC enforcement release, Swedbank Latvia AS (20 Jun 2023) | https://ofac.treasury.gov/media/931911/download?inline | A |
| OFAC enforcement releases, Interactive Brokers LLC (15 Jul 2025) and ShapeShift AG (22 Sep 2025) | https://ofac.treasury.gov/media/934501/download?inline ; https://ofac.treasury.gov/media/934641/download?inline | A |
| OFAC enforcement lists 2023 and 2025; civil penalties index with 2026 entries | https://ofac.treasury.gov/civil-penalties-and-enforcement-information/2023-enforcement-information ; .../2025-enforcement-information ; https://ofac.treasury.gov/civil-penalties-and-enforcement-information | B |
| Federal Reserve press release and orders, Deutsche Bank (19 Jul 2023) | https://www.federalreserve.gov/newsevents/pressreleases/enforcement20230719a.htm ; https://www.federalreserve.gov/newsevents/pressreleases/files/enf20230719a1.pdf | A |

### 13.2 New York DFS consent orders and press releases

| Label | URL | Grade |
|---|---|---|
| NYDFS enforcement actions list (underscore path) | https://www.dfs.ny.gov/industry_guidance/enforcement_actions | B |
| Deutsche Bank mirror trades, press release (30 Jan 2017) | https://www.dfs.ny.gov/reports_and_publications/press_releases/pr1701301 | B |
| Deutsche Bank (private-banking client, Danske Estonia, FBME) press release and consent order (7 Jul 2020) | https://www.dfs.ny.gov/reports_and_publications/press_releases/pr202007071 ; https://www.dfs.ny.gov/industry_guidance/enforcement_discipline/ea20200706_deutsche_bank | A |
| BNP Paribas press release (30 Jun 2014) | https://www.dfs.ny.gov/reports_and_publications/press_releases/pr1406301 | B |
| Commerzbank press release and consent order (12 Mar 2015) | https://www.dfs.ny.gov/reports_and_publications/press_releases/pr1503121 ; https://www.dfs.ny.gov/system/files/documents/2020/04/ea150312_commerzbank.pdf | A |
| Habib Bank press release (7 Sep 2017); 2015 consent order | https://www.dfs.ny.gov/reports_and_publications/press_releases/pr1709071 ; https://www.dfs.ny.gov/system/files/documents/2020/04/ea151215_habib.pdf | B (press release), A (2015 order, generic) |
| Mashreqbank consent orders (10 Oct 2018 and 26 Oct 2021) | https://www.dfs.ny.gov/system/files/documents/2020/03/ea181010_mashreqbank.pdf ; https://www.dfs.ny.gov/industry_guidance/enforcement_discipline/ea20211026_mashreqbank_psc | A |
| National Bank of Pakistan consent order (16 Feb 2022) | https://www.dfs.ny.gov/industry_guidance/enforcement_discipline/ea20220216_co_nbp | A |
| Coinbase consent order (4 Jan 2023) | https://www.dfs.ny.gov/industry_guidance/enforcement_discipline/ea20230104_coinbase | A |
| Block, Inc. consent order (10 Apr 2025) | https://www.dfs.ny.gov/industry-guidance/enforcement-discipline/ea20250410-block | A |
| Paxos Trust Company consent order (7 Aug 2025) | https://www.dfs.ny.gov/industry-guidance/enforcement-discipline/ea20250807-co-paxos-trust-company | A |
| Swedbank AB and New York Branch consent order (16 Jul 2026) | https://www.dfs.ny.gov/industry-guidance/enforcement-discipline/ea20260716-swedbank | A |

The DFS site blocks direct download; each PDF was obtained through the page-fetch tool, which saved the binary file, and read with pdftotext. Habib's 2017 consent order (https://www.dfs.ny.gov/system/files/documents/2020/03/ea170907_habib.pdf) and Deutsche Bank's 2017 order
(https://www.dfs.ny.gov/system/files/documents/2020/03/ea170130_deustche_bank.pdf) exceed the tool's 10 MB limit and were not read.

### 13.3 United Kingdom (FCA)

| Label | URL | Grade |
|---|---|---|
| Deutsche Bank AG final notice (30 Jan 2017) | https://www.fca.org.uk/publication/final-notices/deutsche-bank-2017.pdf | A |
| Standard Chartered Bank decision notice (5 Feb 2019) and press release | https://www.fca.org.uk/publication/decision-notices/standard-chartered-bank-2019.pdf ; https://www.fca.org.uk/news/press-releases/fca-fines-standard-chartered-bank-102-2-million-poor-aml-controls | A |
| HSBC Bank plc decision notice (14 Dec 2021) | https://www.fca.org.uk/publication/decision-notices/hsbc-bank-plc.pdf | A |
| NatWest prosecution press release (13 Dec 2021) | https://www.fca.org.uk/news/press-releases/natwest-fined-264.8million-anti-money-laundering-failures | B |
| Metro Bank final notice (12 Nov 2024) | https://www.fca.org.uk/publication/final-notices/metro-bank-plc-2024.pdf | A |
| Starling Bank final notice (27 Sep 2024) | https://www.fca.org.uk/publication/final-notices/starling-bank-limited-2024.pdf | A |
| Monzo Bank final notice (7 Jul 2025) | https://www.fca.org.uk/publication/final-notices/monzo-bank-limited.pdf | A |
| Barclays Bank plc and Barclays Bank UK plc final notices (14 Jul 2025) | https://www.fca.org.uk/publication/final-notices/barclays-bank-plc-2025.pdf ; https://www.fca.org.uk/publication/final-notices/barclays-bank-uk-plc-2025.pdf | A |
| Nationwide Building Society final notice (11 Dec 2025) | https://www.fca.org.uk/publication/final-notices/nationwide-building-society-2025.pdf | A |
| Dinosaur Merchant Bank final notice (24 Mar 2026) | https://www.fca.org.uk/publication/final-notices/dinosaur-merchant-bank-limited-2026.pdf | A |
| Mako Financial Markets Partnership LLP and Arian Financial LLP final notices (2025; introduction read) | https://www.fca.org.uk/publication/final-notices/mako-financial-markets-partnership-llp-2025.pdf ; https://www.fca.org.uk/publication/final-notices/arian-financial-llp-2025.pdf | A (partly) |
| FCA fines lists 2021, 2025, 2026 | https://www.fca.org.uk/news/news-stories/2021-fines ; .../2025-fines ; .../2026-fines | B |

### 13.4 Europe, Australia and secondary sources

| Label | URL | Grade |
|---|---|---|
| Danske Bank, Bruun and Hjejle report on the Non-Resident Portfolio of the Estonian branch (19 Sep 2018), archive copy | https://web.archive.org/web/20180920145939/https://danskebank.com/-/media/danske-bank-com/file-cloud/2018/9/report-on-the-non-resident-portfolio-at-danske-banks-estonian-branch-.-la=en.pdf | A |
| Danske Bank company announcement on the resolution with US and Danish authorities (13 Dec 2022) | https://danskebank.com/news-and-insights/news-archive/company-announcements/2022/ca13122022 | B |
| Dutch Public Prosecution Service press release on ING (4 Sep 2018) | https://www.om.nl/actueel/nieuws/2018/09/04/ing-betaalt-775-miljoen-vanwege-ernstige-nalatigheden-bij-voorkomen-witwassen | B |
| Statement of facts "Onderzoek Houston" on ING (Dutch) | https://www.om.nl/site/binaries/site-content/collections/documents/hoge-transacties/feitenrelaas/map/feitenrelaas-ing/Feitenrelaas+onderzoek+Houston+%28ING+Bank+NV%29+.pdf | A |
| OM "hoge transacties" topic page (lists the September 2026 Credit Suisse/UBS settlement) | https://www.om.nl/onderwerpen/h/hoge-transacties | B |
| ABC News, AUSTRAC allegations against Westpac (20 Nov 2019) | https://www.abc.net.au/news/2019-11-20/westpac-to-face-fines-anti-money-laundering-terrorism-breaches/11720474 | C |
| ABC News, AUSTRAC allegations against Commonwealth Bank (3 Aug 2017) | http://www.abc.net.au/news/2017-08-03/cba-risks-massive-fines-over-law-breaches/8770992 | C |
| Wikipedia pages used as secondary summaries: Danske Bank scandal, Swedbank, Westpac, Commonwealth Bank, AUSTRAC, N26, Revolut, OKX, ABN AMRO (Dutch) | https://en.wikipedia.org/wiki/Danske_Bank_money_laundering_scandal ; .../Swedbank ; .../Westpac ; .../Commonwealth_Bank ; .../AUSTRAC ; .../N26 ; .../Revolut ; .../OKX ; https://nl.wikipedia.org/wiki/ABN_AMRO | C |
| BaFin measure page, N26 Bank GmbH (growth cap; published 9 Nov 2021) | https://www.bafin.de/SharedDocs/Veroeffentlichungen/DE/Massnahmen/60b_KWG_84_WpIG_und_57_GwG/meldung_211109_60b_N26.html | B |
| BaFin measure page, N26 Bank AG (fine of EUR 9.2 million; 21 May 2024) | https://www.bafin.de/SharedDocs/Veroeffentlichungen/DE/Massnahmen/60b_KWG_84_WpIG_und_57_GwG/meldung_240521_N26_Bank_AG.html | B |
| German Wikipedia, N26 (Direktbank), used for the dates of the cap changes | https://de.wikipedia.org/wiki/N26_(Direktbank) | C |
| BaFin home page (used to check recent items; no N26 item was listed) | https://www.bafin.de/EN/Homepage/homepage_node.html | B |

### 13.5 Not reached, with the reason

| Item | Reason |
|---|---|
| DOJ press releases and plea documents (Rabobank N.A., HSBC DPA, Danske, OKX, Binance DOJ, Standard Chartered DPA) | justice.gov returns an interstitial or 404 to automated requests |
| AUSTRAC statements of claim and agreed facts; Federal Court judgments (CBA [2018] FCA 930, Westpac [2020] FCA 1538) | austrac.gov.au does not respond; AustLII and the Federal Court site return 403 |
| Dutch OM statement of facts on ABN AMRO (19 Apr 2021) | page addresses could not be found; OM search is script-driven |
| BaFin press releases on N26 (2025 measures) | not located; the 2021 and 2024 measure pages were reached through links in German Wikipedia |
| Bank of Lithuania release on Revolut (2025) | lb.lt behind a challenge page |
| Central Bank of the UAE enforcement pages | HTTP 403 |
| Swedish FSA (Finansinspektionen) Swedbank 2020 decision | addresses returned 404 |
| SEC filings (EDGAR) by curl | blocked as an undeclared automated tool (the HSBC 6-K exhibits were read through the page-fetch tool) |
| NYDFS Standard Chartered 2012 and 2019 orders, Habib 2017 order, Deutsche 2017 order | not located or above the 10 MB limit |
| Web search | the session's search budget (200 calls) was exhausted before the 2025-2026 sweep, so cases after about October 2025 come from regulators' own listing pages only |

