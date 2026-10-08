<!-- AKR-TMS plan: source text for the PDF. Converted with python-markdown (tables, attr_list, md_in_html) and printed
     with Chromium. Sections marked TODO are written after the remaining research lands. -->

<section class="cover">
<div class="bricks"></div>
<div class="inner">
<div class="lockup"><img src="assets/logo-symbol.svg" alt=""><div><div class="word"><b>AKR</b><span>-TMS</span></div><div class="ar">نظام مراقبة المعاملات ومخاطر العملاء</div></div></div>
<div class="rule"></div>
<h1>An AML and KYC system <em>designed for Iraqi banks</em></h1>
<div class="sub">A plan for transaction monitoring, sanctions screening and customer risk that meets the Central Bank of Iraq's rules, gives foreign correspondents the evidence they ask for, and never touches the core banking system.</div>
<div class="hero"><img src="images/final/01-command-centre.png" alt="AKR-TMS command centre"></div>
<div class="meta"><span>For bank leadership and compliance heads</span><span>October 2026</span></div>
</div>
</section>

## 1. Summary {#summary}

<div class="stats4">
<div class="stat"><div class="n">19 June 2026</div><div class="l">Iraq returns to the FATF grey list, with an eight-point action plan</div></div>
<div class="stat"><div class="n">32 banks</div><div class="l">on the CBI's February 2026 lists: 28 barred from dollar dealings, 4 blacklisted</div></div>
<div class="stat"><div class="n">About 6</div><div class="l">suspicious transaction reports per bank per year (2024)</div></div>
<div class="stat"><div class="n">86+</div><div class="l">CBI scenarios every bank must run as a minimum (2025 controls)</div></div>
</div>

<p class="lead">Iraqi banks must now prove their anti-money laundering controls to people outside the bank: the Central
Bank of Iraq, the FATF's reviewers and, above all, the foreign correspondent banks that decide whether an Iraqi bank
can move dollars at all. Having a monitoring system is no longer enough. The system has to find what matters in Iraqi
data, and show that it did.</p>

AKR-TMS is an anti-money laundering (AML) and know-your-customer (KYC) system designed for Iraqi banks. It monitors
transactions, screens customers and payments against sanctions lists, rates customer risk and runs KYC reviews,
supports investigations, prepares reports for the AML/CFT Office's goAML system, and keeps the evidence of every step. It
reads the bank's core banking database through a read-only connection and never changes it.

What makes it different:

- **Made for Iraq.** The CBI's full scenario catalogue, Iraq's national list and Arabic name matching come built in,
  not as add-ons to a Western product.
- **Read-only, and complete.** No writes to the core banking system, and a daily reconciliation that proves every
  record was seen.
- **Evidence by default.** Packs for the CBI, correspondents, validators and the board come from live data in
  minutes.
- **Explainable, with people in charge.** Rules, models and the AI assistant each have their own controls; a named
  person makes every decision.
- **One product for every Iraqi bank.** Conventional and Islamic, private and state-owned, with regime packs for the
  foreign rules each bank's correspondents apply.

The plan runs in phases: six to eight weeks of discovery and data mapping, a core system live at the first bank in
about nine months after a parallel run beside its current system, then checks on payments before release and
validated machine-learning models over the following six months. Further banks follow in three to five months each.

## 2. Why now: the position Iraqi banks are in {#why-now}

### Back on the grey list

On 19 June 2026 the Financial Action Task Force (FATF) put Iraq back on its list of jurisdictions under increased
monitoring, the "grey list" that Iraq left in 2018. Iraq's action plan has eight items. Four of them land directly on
the monitoring systems of Iraqi banks:

- more suspicious transaction reports, and better ones;
- detecting informal money transfer (hawala) activity that runs through bank accounts;
- stronger oversight of who really owns and controls companies;
- stopping the evasion of sanctions on weapons proliferation.

### Dollar access now depends on each bank's own controls

Iraq's oil revenue is held at the Federal Reserve Bank of New York, so the United States decides whether Iraqi banks
can obtain and move dollars. Since 2023 Iraqi banks have been cut off from dollar transactions in waves: 14 banks in
July 2023, 8 in January 2024 and 5 more reported in February 2025. In 2024 the US Treasury went further and designated
one Iraqi bank a "primary money laundering concern", which closes the US financial system to it entirely. The CBI's
own lists of February 2026 name 28 banks that may not deal in dollars, directly or indirectly, and 4 that are
blacklisted.

The way back now runs through compliance. In July 2026 the CBI announced an understanding with the US Treasury: banks
that meet its compliance and governance standards and complete the first phase of its reform and re-licensing
programme return first to correspondent banking in other currencies, and later regain dollar dealings. Seven banks
qualified for the first step. A bank that wants to be on that path has to show its controls work.

At the end of 2024 the Central Bank of Iraq (CBI) closed the electronic platform through which it had vetted dollar
transfers. Since then each bank deals directly with its own foreign correspondent banks, in Jordan, the UAE, Kuwait,
Europe and the United States. Each correspondent applies its own country's rules to its Iraqi customers, and each can
end the relationship.

<div class="timeline" markdown="1">

| When | What happened |
|---|---|
| July 2023 | US bars 14 Iraqi banks from dollar transactions |
| January 2024 | 8 more banks barred after Iran sanctions concerns |
| July 2024 | US Treasury final rule against an Iraqi bank as a "primary money laundering concern" |
| End of 2024 | CBI dollar platform closes; banks must deal with correspondents directly |
| January 2025 | CBI issues detailed AML supervisory controls with explicit system requirements |
| February 2025 | 5 more banks barred, most of them Islamic banks |
| September 2025 | UN sanctions on Iran restored; US designates four more Iraqi armed groups as terrorist organisations |
| February 2026 | CBI lists 28 banks barred from dollar dealings and 4 blacklisted |
| June 2026 | FATF grey-lists Iraq |
| July 2026 | CBI and US Treasury agree a way back for banks that pass the CBI's reform programme |
| August 2026 | CBI circulars on behavioural indicators and politically exposed persons |
| September 2026 | Deadline for each private bank's external AML assessment under the CBI's reform |
| July 2027 | New EU anti-money laundering regulation applies to every EU correspondent |

</div>

### The bar has been raised at home

The CBI's supervisory controls of January 2025 are unusually specific about what an AML system must do. They require
real-time monitoring of account activity against what the customer declared; the CBI's own catalogue of more than 86
scenarios as a minimum; artificial intelligence and machine learning to find suspicious activity; holding a transaction
that matches a scenario until the money laundering reporting officer (MLRO) clears it; screening against the UN,
Iraqi, US, EU and UK lists, refreshed at least every 12 hours, with search in Arabic and English; a liaison officer in
every branch with a window into the system; and AML tools verified by a CBI-approved firm. The August 2026 circulars
added daily news screening of politically exposed persons and their relatives to the second degree.

The CBI's banking reform, in force since September 2025, makes that verification a test of survival. Every private bank
that chose to continue had to submit, by 30 September 2026, an assessment of its anti-money laundering controls by an
independent external auditor approved by the CBI. Gaps must be fixed and the file resubmitted within six months; a bank
that cannot fix them may be moved to the merger or liquidation path. The auditor tests the system itself: its rule
logic and alert thresholds, user permissions, the integrity of its audit trail, its links between customer checks,
monitoring and screening, and whether the data behind each report can be traced.

### Execution is the gap

The 2024 mutual evaluation of Iraq found that most of the rules are in place but that they do not yet work well enough:
it rated the effectiveness of banks' preventive measures "moderate" and of proliferation sanctions "low". Every Iraqi
bank already runs an automated monitoring system with the CBI's standard scenarios, yet none of the banks interviewed
had written a single scenario of its own, and large numbers of alerts were archived every day against very few
reports. Bank reports to the AML/CFT Office rose from 82 in 2022 to 672 in 2023 and 451 in 2024; that is still about
six reports per bank per year. The quality is there (the Office graded 90 percent of bank reports "good" in 2022). The
volume, the coverage and the evidence are not.

### Correspondents have become a second supervisor

Every foreign regime reaches an Iraqi bank through its correspondents:

| | What it means for an Iraqi bank |
|---|---|
| **United States** | Dollar clearing depends on screening against US sanctions lists, including companies owned 50 percent or more by listed persons, and on coverage of Iran-related and armed-group typologies. Sanctions records must be kept for 10 years |
| **European Union** | From July 2027 every EU bank applies one regulation that requires a record of every assessment, human involvement in automated decisions, and answers to authorities within days. A new EU authority can single out an individual non-EU bank whose controls fall short; EU banks must then stop new business with it and end existing relationships unless they can show the risk is covered |
| **United Kingdom** | Iraq counts as a high-risk country. UK banks no longer have to apply enhanced checks to every Iraqi relationship automatically, so the quality of the Iraqi bank's own controls decides how it is treated |
| **UAE and the Gulf** | UAE banks follow detailed guidance on monitoring and screening (daily re-screening, data testing, documented tuning) and expect the same from their partners. Kuwait and Lebanon are grey-listed too, so their banks pass the pressure on |

The common thread is evidence. A correspondent cannot visit the bank, does not read Arabic, and will not take the bank's
word for it. It wants to see which lists are screened and how fast they are updated, which scenarios run and how they
were tested, how many alerts are waiting and for how long, and who approved each change.

<div class="callout" markdown="1">

**What this means.** An Iraqi bank's AML system is no longer a back-office tool. It decides whether the bank can do
business in dollars. It has to do three things at once: meet the CBI's explicit requirements; find what matters in
Iraqi data, which is largely in Arabic; and prove all of it to people outside the bank.

</div>

## 3. What the rules require, and how they converge {#rules .nobreak}

### Iraq first

Iraqi law and the CBI's instructions always come first. The research behind this plan read the AML/CFT Law No. 39 of
2015, the CDD instructions of 2023, the CBI supervisory controls of 2025 and the CBI circulars of August 2026 in full.
Their requirements for an AML system, condensed:

| Area | What the CBI requires |
|---|---|
| Monitoring | Real-time monitoring of all account activity against declared activity; the full CBI scenario catalogue as a minimum, plus the bank's own scenarios; AI and machine-learning analytics |
| Holding transactions | A transaction matching a scenario is held until the MLRO clears it |
| Customer risk | High, medium and low ratings (up to five levels) on customer, product, channel and geography; re-rating on events; review at least every two years; KYC refresh at least yearly |
| Screening | UN, Iraqi national list, US, EU and UK lists, with World-Check named as the data source; all parties including beneficial owners; refresh at least every 12 hours; Arabic and English search |
| PEPs | Relatives to the second degree and close associates; daily local and regional news screening; status kept after leaving office |
| Wire transfers | Complete originator and beneficiary data including purpose; incomplete incoming transfers detected and rejected if not fixed |
| Branches | A liaison officer in every branch with a branch window in the AML system; monthly report by the 10th |
| Reporting | STRs through goAML, including attempted transactions and any amount, with reasons; written decisions, also when the MLRO decides not to report |
| Records | At least five years after the relationship ends, and seven years of AML data for the reform's external assessment; transactions can be reconstructed; off-site backup |
| Assurance | AML tooling verified by a CBI-approved firm; three lines of defence; reports to the board and audit committee |

### International standards

The FATF's 40 Recommendations (revised to October 2025) set the global baseline: customer due diligence, beneficial
ownership, politically exposed persons, payment transparency, suspicious transaction reporting and the freezing of
terrorist and proliferation funds "without delay", which the FATF defines as within hours. The UN Security Council's
consolidated list, including the Iran sanctions restored in September 2025, applies to every Iraqi bank. The Basel
Committee, the Wolfsberg Group of global banks and the Egmont Group of financial intelligence units add detailed
guidance on monitoring, screening and reporting that correspondents use as their checklist.

### Foreign rules converge on one specification

US, EU, UK and UAE rules were written separately and years apart, yet they now describe almost the same screening and
monitoring system: complete and current lists checked against the official source; every party and identifier
screened, in the original script and in transliteration; ownership and control taken into account; matching tested
before and after every change; re-screening on every list change; backlogs under control; and every decision recorded
and explainable. AKR-TMS is built to the union of these rules, so that one system satisfies the CBI and every
correspondent at once, and each bank switches on the foreign "regime packs" that apply to it.

<div class="callout" markdown="1">

**The research behind this plan** reviewed Iraqi law and CBI instructions, the FATF standards, UN sanctions, the
Basel, Wolfsberg and Egmont guidance, US, EU, UK and Gulf regimes, more than 50 AML and KYC suppliers, and about 40
enforcement cases. It produced more than 1,200 specific product requirements, each traced to its source.

</div>

## 4. What the market offers, and where it falls short {#market .nobreak}

<p class="lead">More than 50 AML and KYC suppliers were reviewed, from the global suites and the core banking vendors
to this year's AI start-ups and the regional suppliers already working in Iraq. The best of them do many things well,
and AKR-TMS adopts those as its minimum. None documents the whole of what an Iraqi bank needs.</p>

<div class="market" markdown="1">

| Kind of supplier | Examples | Strong at | Weak for an Iraqi bank |
|---|---|---|---|
| Global AML suites | Oracle, SAS, NICE Actimize, SymphonyAI | Breadth, rule libraries, case management | Cost, heavy data loading, gaps in Arabic and goAML |
| AI detection specialists | Quantexa, Hawk, Feedzai, Tookitaki | Network analysis; models that rank alerts | Mostly foreign cloud or big-data stacks; no Arabic |
| Investigation automation | Silent Eight, Napier, Lucinity, Unit21, Flagright | Faster alert review; modern case tools | Mostly cloud; more autonomy than Iraqi rules allow |
| Screening engines and list data | LexisNexis, Eastnets, FinScan, Swift, LSEG World-Check, Dow Jones | Sanctions and PEP lists; payment screening | Arabic and Iraqi names handled poorly; no data supplier publishes its Iraqi coverage |
| KYC and onboarding platforms | Fenergo, Pega, Moody's, Encompass, Uqudo | Client lifecycle workflows; identity checks | Cloud-only, and built to own the customer record with write access; only one documents the Iraqi national card |
| AML modules of core banking vendors | Temenos, Azentio (iMAL), Capital Banking Solutions, Oracle (FLEXCUBE) | A ready link to their own core system; one contract | End-of-day feeds; no published Arabic matching or full CBI pack. The core system most Iraqi banks run has no AML module |
| Regional suppliers | Pio-Tech (Jordan), Eastnets (UAE) | Iraqi or regional customers; goAML and customer-risk modules; Arabic-speaking teams | No published accuracy or prices; detection depth unclear |

</div>

**What Iraqi banks run today.** Every Iraqi bank already runs a monitoring system (section 2), so AKR-TMS replaces
or sits beside an existing tool rather than filling a gap. Banks' annual reports name their tools, often several per
bank: FICO Siron for monitoring at two of the largest private banks, Firco for payment screening at two banks, core
banking vendors' AML modules at four banks, and goAML, customer-risk and scenario modules from Pio-Tech, a Jordanian
supplier that lists 29 banks in Iraq among its references. None of the largest global suites has an Iraqi customer.
Auditors at two banks still describe the CBI's minimum as 25 scenarios; the 2025 catalogue numbers 86.

### What the best systems get right

- **Rules and models together.** Rules cover known patterns and legal duties, models rank the alerts and spot unusual
  behaviour, and every alert shows its reasons.
- **Peer groups and tested thresholds.** Customers are compared with similar customers, thresholds are set per group,
  and a sample of what falls just below a threshold is reviewed.
- **The customer, not the transaction.** Alerts are gathered per customer and per network, so an analyst investigates
  a person or a group, not a list of payments.
- **Changes tested before they go live.** The best products replay every rule change on past data and show the
  difference before anyone approves it.
- **Screening that explains itself.** Scores are broken down field by field, and a decision on a false match is
  remembered so that nobody reviews the same pair twice.
- **Reporting as a workflow.** Draft, approve, file, acknowledge and archive, with validation before submission.
- **One product, many banks.** SAS serves 120 Nordic banks through a shared service: one standard product, with a
  separate layer for each bank's settings.

### Where they fall short for an Iraqi bank

| What an Iraqi bank needs | What the market offers |
|---|---|
| A system in the bank's own data centre | Most new products, and the newest AI features of the large suites, run only in foreign clouds |
| Arabic names matched properly, Iraqi naming included | Arabic treated as an edge case. Oracle's on-premises name matcher states that Arabic characters "will not match"; no vendor documents handling of the mother's name, tribe or Kurdish spellings |
| The CBI's scenario catalogue | Global vendors ship their own libraries; the systems in Iraq that carry CBI scenarios do not show which ones, or whether the set is current |
| Reports to the AML/CFT Office through goAML | One global suite documents a goAML module, but not for Iraq; a regional module runs at two Iraqi banks, with no published test results |
| Islamic contracts monitored as contracts | No vendor documents murabaha, ijara, musharaka or mudaraba |
| A read-only link with proof of completeness | Heavy data loading through several staging databases; completeness rarely measured |
| Rules that can change quickly | TD Bank added no new scenario between 2014 and 2022 |
| A price an Iraqi bank can pay, and local support | No published prices; global suites meter licences by volume and have no office in Iraq |
| Continuity | NICE Actimize is being sold; FICO and BAE sold their AML products; start-ups are being bought |

<div class="callout" markdown="1">

**Claims and evidence.** Vendors advertise that AI clears 75 to 98 percent of alerts automatically and removes 30 to 99
percent of false positives. The only figure compiled by a regulator from a bank in production is 7 percent of alerts
closed automatically, with sampling and the option to reopen (Hong Kong Monetary Authority, 2023). AKR-TMS promises only
what it can show on the bank's own data before a contract is signed: a replay of twelve months of the bank's history,
compared alert by alert with the system the bank already runs.

</div>

### A legal check comes first

Iraq's 2022 law against normalisation with Israel bans economic dealings with Israeli companies, with severe penalties.
NICE Actimize, one of the largest AML vendors, and ThetaRay, an AI detection vendor, are headquartered in Israel. Banks
should take legal advice before considering either. US vendors carry a different risk: their own sanctions policies can
end licences and support for a bank that becomes subject to US measures. AKR-TMS applies the same legal check to every
component and data feed it uses.

### Where AKR-TMS fits

AKR-TMS is built for the gap: in the bank's own data centre, read-only, Arabic first, with the CBI catalogue, goAML
reporting and Islamic contracts built in, a published price, and continuity guaranteed by source-code escrow and open
export of all data. Where a bank already runs a screening engine it trusts, AKR-TMS imports that engine's alerts and
adds what is missing, rather than demanding a replacement.


## 5. Lessons from enforcement {#lessons .nobreak}

<p class="lead">About 40 enforcement actions against banks and payment firms were studied, most of them from the
regulators' own notices. The cause was seldom a weak algorithm. Banks were fined because the system was not given all
the data, because alerts were limited to fit the staff, because nobody tested whether it worked, and because the
records could not be trusted.</p>

<div class="lessons" markdown="1">

| What went wrong | Where it happened | What AKR-TMS does |
|---|---|---|
| The system did not see all the data | TD Bank left 92 percent of its transaction volume, over USD 18 trillion, unmonitored (US, 2024). Metro Bank's monitoring, fed from a copy of its core banking data, missed 60 million transactions worth GBP 51 billion (UK, 2024) | Reconciles what it received against the core banking database every day, table by table, and shows coverage on the front page |
| Alerts were limited to fit the staff | US Bancorp capped alerts to match its staffing; ING's system stopped monitoring some categories after three alerts a day; by 2015 HSBC UK's rules suppressed two-thirds of its alerts | No caps and no hidden filters. Workload is managed in the queue; a threshold changes only with tested evidence and approval from outside operations |
| Nobody tested whether it worked | Starling screened customers against 39 of 3,088 listed names for six years (UK, 2024) | Daily tests with known answers on every list and scenario; a scenario that goes silent raises an alarm |
| Data the bank held was never used | Standard Chartered held fax numbers and IP addresses pointing to Iran; Swedbank Latvia saw IP addresses in Crimea | Addresses, phone prefixes and login locations feed screening and customer risk |
| Exemption lists switched controls off | Habib Bank's "good guy" list let at least USD 250 million through unscreened; Rabobank's "verified" customers, exempt from review, grew from fewer than 10 in 2009 to over 1,000 by 2012 | Exemptions are approved by two people, expire, and are re-tested whenever a list changes |
| Declared activity was never checked | NatWest onboarded a jewellery business expected to handle no cash and never compared that with the cash it deposited | Expected activity from KYC is compared with actual activity in every customer review and scenario |
| Records were edited or withheld | Metro Bank (edited minutes), US Bancorp (altered presentations), Swedbank (narrowed answers to its New York regulator; USD 50 million, July 2026) | Logs, parameter changes and board packs are generated by the system with full history, and cannot be altered |
| Backlogs grew with the business | Coinbase's unreviewed alerts passed 100,000 by the end of 2021 after rapid growth | Queue clocks, ageing alarms and volume forecasts on the command centre |

</div>

<div class="callout" markdown="1">

**Closest to home.** On 1 September 2026 the US Treasury proposed cutting the UAE branches of Egypt's state-owned
Banque Misr off from US correspondent banking. Its case rests on flows alone: 103 potential Iranian front companies that
moved about USD 1.8 billion through the branches, set against their size. A bank can compute the same ratios from its
own data. AKR-TMS gives each bank this view of itself before its correspondents compute it.

</div>


## 6. Design principles {#principles .nobreak}

<p class="lead">Ten principles follow from the rules, from what existing systems do well and badly, and from the cases in
which banks were fined. Each one shapes something a user can see in the product.</p>

<div class="principles" markdown="1">

1. **Iraq first.** Built around Iraqi law, the CBI's scenario catalogue, Iraqi lists, Arabic names and dinar amounts.
   Foreign rules are added on top as regime packs, never the other way round.
2. **Read-only, and provably so.** AKR-TMS never writes to the core banking system and never calls its application.
   The bank's own database permissions guarantee it; the bank's own audit log proves it.
3. **Complete, and able to show it.** Every day AKR-TMS reconciles what it received against the core banking system,
   table by table, counts and totals. A scenario that cannot run because data are missing is reported, never skipped
   in silence.
4. **Arabic is the original.** Names are matched in Arabic script and in every common transliteration. The interface
   is designed right to left first, with Hijri and Gregorian dates and both styles of digits.
5. **No black boxes.** Every alert shows its reasons, every risk score its factors, every rule a plain-language
   description generated from its settings.
6. **People decide.** The system finds, ranks, links and drafts. A named person decides, and a second person confirms
   the decisions that matter.
7. **Every decision is recorded,** including the decision not to report, with its reasons. Records are kept as long
   as the strictest applicable rule requires, and cannot be altered.
8. **Evidence is a product.** The packs that the CBI, correspondents, validators and the board ask for come from live
   data in minutes, not from weeks of manual assembly.
9. **Change is governed.** Rules and models change only through testing, approval and versioning, and every change
   can be rolled back.
10. **Proportionate.** Effort follows risk. Low-risk customers are not buried in checks, and the system does not
    demand what no rule requires.

</div>

## 7. The system, module by module {#modules}

<p class="lead">AKR-TMS has six working modules on a shared foundation of governance, evidence and audit. All of them
work from one picture of each customer, built from the core banking data, so an analyst never has to reconcile what
the screening system, the monitoring system and the KYC file each say about the same person.</p>

<div class="sysmap">
<div class="col">
<div class="h">Data in, read-only</div>
<div class="box">Core banking database<span>customers, accounts, transactions</span></div>
<div class="box">Payment records<span>SWIFT, RTGS, ACH, cards</span></div>
<div class="box">Sanctions and PEP lists<span>UN, Iraq, OFAC, EU, UK</span></div>
<div class="box">News and adverse media<span>local and regional sources</span></div>
</div>
<div class="arrow"></div>
<div class="col core">
<div class="h">AKR-TMS</div>
<div class="box m">Command centre</div>
<div class="box m">Customer risk and KYC</div>
<div class="box m">Screening</div>
<div class="box m">Transaction monitoring</div>
<div class="box m">Investigation and cases</div>
<div class="box m">Reporting through goAML</div>
<div class="box base">Governance, evidence vault and audit trail</div>
</div>
<div class="arrow"></div>
<div class="col">
<div class="h">People and outputs</div>
<div class="box">MLRO, analysts and branch liaison officers<span>one queue, one case per customer</span></div>
<div class="box">Operations team<span>freeze, hold and KYC tasks</span></div>
<div class="box">AML/CFT Office<span>STRs and CTRs through goAML</span></div>
<div class="box">CBI, correspondents, auditors, board<span>evidence packs from live data</span></div>
</div>
</div>

### Command centre

The command centre is the MLRO's daily view, and the first screen a supervisor or a correspondent is shown. It answers
five questions at a glance: is the data complete, are the lists current, how large and how old is the backlog, what is
due to whom, and how is customer risk spread across the bank.

- Data coverage and list freshness sit at the top, because every other number depends on them.
- The backlog is shown by queue and by age, with service levels per alert type and a surge mode for the day a large
  list update lands.
- Every deadline is in one place: CTR batches, the branch liaison officers' monthly reports, correspondent requests,
  periodic reviews and annual tests.
- Each branch liaison officer gets a branch window, as the CBI requires, limited to that branch's customers and tasks.

<figure>
<img src="images/final/01-command-centre.png" alt="AKR-TMS command centre">
<figcaption><b>Figure 1. Command centre.</b> Coverage, list freshness, backlog age, deadlines and risk distribution on
one screen; the correspondent evidence pack is generated from the same live data. All names and figures in the
mockups are fictional.</figcaption>
</figure>

### Customer risk and KYC

Customer risk is calculated, not typed in. AKR-TMS rates every customer on the four risk elements the CBI prescribes
(customer, product and service, delivery channel, geography) on up to five levels, and re-rates on events instead of
waiting for the next review: a list match, a sharp change in activity, an adverse news item, a new owner.

- Each score shows the factors behind it, so the rating can be explained to an auditor, a correspondent or the
  customer's relationship manager.
- Reviews are scheduled by risk and by the CBI's minimum intervals; overdue reviews escalate automatically.
- Enhanced due diligence is a checklist with the evidence attached: source of funds, beneficial owners, senior
  management approval, site visit.
- Ownership is drawn as a tree, with owners at 20 percent or more (the Iraqi threshold) highlighted.
- Documents go into an evidence vault that locks them for the retention period.
- Onboarding stays in the core banking system. AKR-TMS reads what was captured there and flags what is missing, such
  as an occupation, the purpose of an account or a company's owners.

<figure>
<img src="images/final/04-customer-risk.png" alt="Customer risk profile">
<figcaption><b>Figure 2. Customer risk profile.</b> A score of 72 broken down into its factors, with the enhanced due
diligence checklist, the ownership tree and the locked evidence beside it.</figcaption>
</figure>

### Screening

Screening is where Iraqi banks are most exposed, and where imported systems are weakest. Most names on an Iraqi bank's
books are Arabic, and so are many names on the lists that matter most to it. The same name can be written in Latin
letters a dozen ways (Mustafa, Moustafa, Mustapha), and a full Iraqi name chains a person's own name with the
father's, the grandfather's and often a family or tribal name.

- Names are compared in Arabic script and in transliteration, name part by name part. Very common tribal names and
  titles such as Sayyid, Sheikh and Hajji count for less. Kurdish names are covered too. The mother's name, which
  Iraq's national list records for almost every entry, helps confirm or rule out a match; no engine on the market
  documents using it.
- The lists are the UN, Iraq's national list, OFAC (including companies owned 50 percent or more by listed persons),
  the EU and the UK. Each is checked against the official source on every load and refreshed well inside the CBI's
  12-hour limit. Commercial data, including World-Check, which the CBI's controls name, connects through the bank's
  own licence.
- Every customer, beneficial owner, counterparty and payment party is re-screened whenever a list changes, not only
  at onboarding.
- When an analyst clears a match as the wrong person, the decision is remembered for that pair of records and
  reopened automatically if either record changes.
- A true match creates freeze and report tasks with their legal deadlines, such as the first report to Iraq's
  freezing committee within 48 hours. AKR-TMS then confirms from the core banking data that the freeze is in place,
  and flags any money that leaves a frozen account afterwards, fees included. In September 2026 the UK sanctions
  authority fined a global bank's London branch partly for fees taken from frozen accounts.
- A built-in test harness measures matching quality before and after every change, as US, EU, UK and UAE rules now
  expect.

<figure>
<img src="images/final/03-screening.png" alt="Screening match review">
<figcaption><b>Figure 3. Screening review.</b> The customer and the list entry are compared in Arabic script and in
transliteration, name part by name part. The analyst escalates this potential match with a recorded reason; a pair is
remembered only when it is cleared as the wrong person.</figcaption>
</figure>

### Transaction monitoring and the scenario studio

AKR-TMS ships with the CBI's full scenario catalogue: 86 numbered scenarios plus the product indicators, each mapped
to the data it needs and to the CBI article it implements. Its library holds 161 scenarios in 18 families. Beyond
the catalogue they cover the risks named in Iraq's action plan and by foreign authorities (hawala through bank
accounts, sanctions evasion, front companies, trade-based laundering, misuse of dollar access), Islamic products, and
public funds and payroll: corruption is the threat Iraq's national risk assessment ranks first, yet the CBI catalogue
has no scenario for it. As the CBI's controls require, a bank adds such scenarios in coordination with the AML/CFT
Office.

- Each scenario shows whether the bank's data can support it. Only about a quarter of the CBI's 173 scenarios and
  indicators can run on the core system's account, transaction and customer data alone; the rest need other modules
  such as cards or trade finance, payment messages, outside sources, or what branch staff observe at the counter,
  recorded through the branch window in AKR-TMS.
- Scenarios run per customer segment with their own thresholds, so a supermarket chain is not judged like a student.
- In the scenario studio the compliance team changes a threshold, reads a plain-language description of what the
  scenario will now do, and tests the change on months of past data before anyone approves it. The test includes a
  review of a sample of cases that would no longer alert.
- Every change goes through a change request with a second-line review and the MLRO's approval, and every version is
  kept.
- Alerts about the same customer are grouped into one case, so analysts investigate customers, not alerts.
- Monitoring runs continuously on data that are minutes old. Holding a payment before it leaves needs the optional
  phase-2 connection described in section 8.

<figure>
<img src="images/final/05-scenario-studio.png" alt="Scenario studio">
<figcaption><b>Figure 4. Scenario studio.</b> A proposed change to CBI scenario 1.4 tested on six months of data:
alerts fall from 412 to 298, and those that led to a report or enhanced due diligence from 31 to 30. The reason is
recorded as risk-based, not workload, and the change now waits for the MLRO's approval.</figcaption>
</figure>

### Investigation and network analysis

An analyst opens a case, not a pile of alerts. The case shows why it was raised, how money moved in and out against
what the customer declared, who the counterparties are, and what earlier alerts and reviews found.

- An AI assistant drafts a summary from the case data, labelled as a draft and linked to the transactions it relies
  on. The analyst edits it and decides.
- The network view links accounts through transfers, shared phone numbers, shared addresses and family records. That
  is how mule rings and hawala networks give themselves away.
- Closures and escalations follow a four-eyes rule, and every action is time-stamped in the case log.
- Requests for information to branches are tracked against deadlines, without alerting the customer.

<figure>
<img src="images/final/02-investigation.png" alt="Case workspace">
<figcaption><b>Figure 5. Case workspace.</b> Three alerts on one customer grouped into one case: reasons, money in and
out against declared turnover, counterparties, a draft summary awaiting review, and the four-eyes decision.</figcaption>
</figure>

<figure>
<img src="images/final/06-network.png" alt="Network view">
<figcaption><b>Figure 6. Network view.</b> Nine accounts linked by transfers, a shared phone number, a shared address
and a family record, all forwarding money to two exchange companies: one case for the whole cluster.</figcaption>
</figure>

### Reporting to the AML/CFT Office

Reports go to the AML/CFT Office through goAML, the reporting system the Iraqi financial intelligence unit uses.
AKR-TMS builds each report from its case: subjects, accounts and transactions are filled from the core banking data,
the narrative is drafted for the analyst to edit, and the report is checked against the goAML schema the Office
publishes, including Iraqi rules such as the mother's name of each person reported, before the MLRO signs it. A named
officer uploads the approved file and AKR-TMS records the Office's reference; it never holds a goAML password.

- STRs cover attempted as well as completed transactions, with indicators linked to the CBI scenarios.
- Unusual transaction reports, which the Office introduced in 2026 for activity that is unusual but not yet
  suspicious, are a third outcome besides an STR and no report. The customer is then watched more closely, and the
  case can still become an STR.
- Each draft is checked against the criteria the Office uses to grade every report it receives; it returns poor
  reports to the bank.
- CTRs and the other periodic reports are produced on schedule.
- The MLRO's written decision is recorded either way, including a decision not to report.
- Follow-up requests from the AML/CFT Office are tracked against the original report.

<figure>
<img src="images/final/07-goaml-report.png" alt="goAML report builder">
<figcaption><b>Figure 7. Report builder.</b> An STR assembled from the case, validated against goAML's rules and
waiting for the MLRO's sign-off. The narrative was drafted by the assistant and edited by the analyst.</figcaption>
</figure>

### Governance and evidence packs

Everything that changes how AKR-TMS decides (a threshold, a scenario, a list source, a model) has an owner, a
version, test results and an approval. The same records feed four evidence packs, one per audience:

| Audience | What its pack shows |
|---|---|
| CBI and AML/CFT Office | Coverage of the CBI catalogue, list freshness, backlog and timeliness, reports filed, branch liaison reports |
| Correspondent banks | Lists screened and update times, screening test results, scenario coverage, tuning log, backlog age, answers to their due diligence questionnaires |
| Independent validator, CBI-approved firm | Inventory of rules and models, test results, change history, data reconciliation, model documentation |
| Board and audit committee | Risk distribution and trends, overdue items, key changes and approvals, the annual certification |

## 8. How it connects to the bank {#connection}

<p class="lead">AKR-TMS reads the core banking database and nothing else. It never writes to it and never calls the
core banking application or its interfaces. That one rule makes AKR-TMS easy for a bank's IT and security teams to
approve, and independent of which core banking system the bank runs.</p>

<div class="conn">
<svg viewBox="0 0 760 290" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="How AKR-TMS connects to the bank">
<defs>
<marker id="ah" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#0F2147"/></marker>
<marker id="ahb" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#1E4FA0"/></marker>
</defs>
<text x="295" y="22" text-anchor="middle" class="lbl">Data flows one way: read-only</text>
<rect x="10" y="40" width="160" height="70" rx="8" class="bx"/>
<text x="90" y="70" text-anchor="middle" class="t">Core banking system</text>
<text x="90" y="90" text-anchor="middle" class="s">the bank's database</text>
<rect x="215" y="40" width="160" height="70" rx="8" class="bx clay"/>
<text x="295" y="70" text-anchor="middle" class="t">Read-only replica</text>
<text x="295" y="90" text-anchor="middle" class="s">or bank-owned views</text>
<rect x="420" y="30" width="170" height="90" rx="10" class="dark"/>
<text x="505" y="64" text-anchor="middle" class="tw">AKR-TMS</text>
<text x="505" y="84" text-anchor="middle" class="sw">monitors, screens, rates</text>
<text x="505" y="101" text-anchor="middle" class="sw">on the bank's premises</text>
<rect x="630" y="40" width="122" height="70" rx="8" class="bx dash"/>
<text x="691" y="66" text-anchor="middle" class="t">Payment gateway</text>
<text x="691" y="84" text-anchor="middle" class="s">SWIFT, RTGS, ACH</text>
<text x="691" y="100" text-anchor="middle" class="p2">Phase 2</text>
<rect x="215" y="190" width="160" height="70" rx="8" class="bx"/>
<text x="295" y="220" text-anchor="middle" class="t">Operations team</text>
<text x="295" y="240" text-anchor="middle" class="s">acts in core banking</text>
<line x1="170" y1="75" x2="213" y2="75" class="ln" marker-end="url(#ah)"/>
<line x1="375" y1="75" x2="418" y2="75" class="ln" marker-end="url(#ah)"/>
<line x1="592" y1="75" x2="628" y2="75" class="ln blue" marker-start="url(#ahb)" marker-end="url(#ahb)"/>
<path d="M505,120 V225 H377" class="ln" fill="none" marker-end="url(#ah)"/>
<text x="441" y="245" text-anchor="middle" class="s">freeze, hold and</text>
<text x="441" y="259" text-anchor="middle" class="s">KYC tasks</text>
<path d="M215,225 H90 V112" class="ln" fill="none" marker-end="url(#ah)"/>
<text x="152" y="245" text-anchor="middle" class="s">staff take</text>
<text x="152" y="259" text-anchor="middle" class="s">the action</text>
<text x="610" y="150" text-anchor="middle" class="s">checks each payment</text>
<text x="610" y="164" text-anchor="middle" class="s">before release</text>
</svg>
<div class="cap"><b>How AKR-TMS connects.</b> Data flows one way, from the core banking database to AKR-TMS. Actions
such as freezes go to the bank's operations team as tasks; AKR-TMS then confirms from the replica that they were
carried out. In phase 2 the payment gateway can ask AKR-TMS about each payment before releasing it.</div>
</div>

### What "read-only" means

- **It never changes data.** The database account AKR-TMS uses can only read. The bank's own database enforces this,
  and the bank's own audit log records every query.
- **It adds no load where it matters.** AKR-TMS reads from a replica, or from views the bank controls. Where no
  replica exists, it works within a load budget the bank signs off, and the bank can switch the connection off at any
  moment.
- **It sees everything.** Every day AKR-TMS reconciles what it received against the core banking system: record
  counts and totals per table, tied back to the general ledger. Any gap raises an alert, and each day's result is
  signed and kept as a coverage certificate.

### One product, any core banking system

Iraqi banks run several core banking systems: international products, regional ones and systems built in house. Some
older systems cannot be read through a database connection at all. AKR-TMS translates each bank's tables into one
standard model of customers, accounts and transactions through a mapping layer. The mapping is configuration, owned by
the bank and versioned like any other change, so connecting a new bank takes weeks of mapping rather than months of
custom development. Where a database connection is impossible, the bank delivers files to an agreed landing area
instead, with counts and checksums that still prove completeness. Payment messages are read from the payment systems'
own records, because a core banking system often holds only a summary of them.

<figure>
<img src="images/final/08-data-connection.png" alt="Data connection health">
<figcaption><b>Figure 8. Data connection.</b> The read-only status, replica delay, daily reconciliation of every table
against the core banking system, and the data-quality issues that affect monitoring.</figcaption>
</figure>

### Acting without writing

When a customer must be frozen or a payment held, AKR-TMS does not do it itself. It creates a task for the bank's
operations team, who act in the core banking system as they do today. AKR-TMS then checks the core banking data to
confirm the action was taken, and keeps watching: any debit from a frozen account after the freeze, including bank
fees, is flagged at once.

### Phase 2: checking payments before they leave

The CBI requires a transaction that matches a scenario to be held until the MLRO clears it. For payments, that can
only happen before release, inside the payment flow. AKR-TMS will offer an optional connection that the bank's
payment gateway calls before it releases a SWIFT, RTGS or ACH payment. AKR-TMS screens and scores the payment and
answers release, hold or reject within the time the payment system allows; the gateway then acts on the answer. The
bank decides, rail by rail, what happens if no answer comes in time. The connection starts in shadow mode, then
advises, and only then enforces. Until it is in place, AKR-TMS monitors minutes after the event, and records that
real-time holds depend on the bank's existing payment filter.

### Security

- Installed in the bank's own data centre; customer data never leave the bank.
- Encrypted connections, secrets kept in a vault, accounts with the least privilege they need, a separate network
  zone and no outbound internet access by default; list updates arrive through a controlled channel.
- No on-demand look-ups of individual customers in the core banking database, so its logs cannot reveal who is under
  investigation.
- Evidence kept in write-once storage; every user action logged.
- Access by role: branch staff see the reasons for an alert, not the thresholds that produced it.

<div class="dark-box" markdown="1">

**Under the hood.** AKR-TMS uses a modern open-source stack: services written in Rust, PostgreSQL databases,
write-once document storage, a secrets vault, single sign-on and a web interface built in React, running on a small
Kubernetes cluster with full monitoring and logging. No part of it needs a foreign vendor's licence, and it runs
without internet access.

</div>

## 9. AI, used responsibly {#ai .nobreak}

<p class="lead">The CBI requires AML systems to use artificial intelligence and machine learning. Correspondents and the
new EU rules require that people, not machines, make the decisions, and that every automated step can be explained.
AKR-TMS meets both by putting each technique on one of three tracks, each with its own controls.</p>

| Track | What it covers | How it is controlled |
|---|---|---|
| **Rules** | CBI scenarios, the bank's own scenarios, screening settings | Readable in plain language; tested on past data before approval; versioned and reversible. Rules are configuration, not models |
| **Models** | Anomaly detection, alert prioritisation, network risk, calibration of risk scores | Documented; validated before use by someone independent of the builders; monitored monthly for drift; explained alert by alert; can be suspended at once, with automatic fallback to rules |
| **AI assistant** | Case summaries, report narrative drafts, Arabic-English translation, search across case documents | Drafts only, always labelled; every statement linked to its source; a named person edits and decides; runs inside the bank; never decides an alert or a report |

- **Starter models, then the bank's own.** A new bank has few confirmed cases to learn from. AKR-TMS starts with
  models calibrated elsewhere, labelled as temporary, and builds the bank's own models from its decisions from the
  first day, under a recalibration plan with a target date.
- **Fair by design.** Iraqi names can reveal tribe, sect and ethnicity; these never enter a model. Nationality and
  geography are used only as documented risk factors that never trigger action on their own, and outcomes are
  compared across groups every month.
- **Built against automation bias.** The interface shows reasons before scores, requires a written rationale, and
  tracks how often analysts override the system. An unusual override rate triggers a review of the model.
- **Built to the strictest standard in the region.** The CBI has not yet issued rules on AI or model risk. AKR-TMS
  follows the UAE Central Bank's Model Management Standards, which cover AML models explicitly, together with UK,
  Singapore and US guidance, so it is ready for whatever the CBI-approved assessors and the correspondents test.

## 10. One product for many Iraqi banks {#banks .nobreak}

<p class="lead">Iraqi banks differ widely: state banks with millions of customers and large branch networks, private
banks of every size, Islamic banks and Islamic windows, banks headquartered in the Kurdistan Region, and branches of
foreign banks. AKR-TMS is one product, configured for each bank and never customised.</p>

<div class="grid-2" markdown="1">
<div class="card" markdown="1">

#### Islamic banks

Murabaha, ijara, musharaka, mudaraba, salam, istisna'a and qard hasan each carry their own laundering risks: commodity
trades that never take place, sale-and-buyback structures, investment accounts used to place cash, charity and zakat
funds diverted to terrorism. AKR-TMS reads Islamic contracts as contracts, not as loans, and adds a scenario pack for
them. Of the 28 banks on the CBI's February 2026 list of banks barred from dollar dealings, 17 are Islamic banks.

</div>
<div class="card" markdown="1">

#### State banks

Large customer bases, wide branch networks and older systems. The file landing route connects systems that cannot be
read through a database, and the branch window supports a liaison officer in every branch.

</div>
<div class="card" markdown="1">

#### Kurdistan Region

Kurdish names are matched alongside Arabic ones. A Kurdish (Sorani) interface is planned next to Arabic and English.

</div>
<div class="card" markdown="1">

#### Regime packs

Iraq is always on. Packs for the US, the EU, the UK, the UAE, Saudi Arabia and Jordan add each regime's lists, rules,
retention periods and evidence formats. A bank switches on the packs its correspondents and owners require. Where two
rules differ, the stricter one applies unless the bank's policy records a reason otherwise.

</div>
</div>

Each bank gets its own installation in its own data centre, and no data are shared between banks. Product updates,
list feeds and new scenarios arrive as signed packages, which the bank tests and approves before use.

## 11. Roadmap {#roadmap}

<p class="lead">AKR-TMS is delivered in phases, and each phase leaves the bank better off on its own. Durations are
indicative for the first bank; they depend most on how fast database access is approved.</p>

<div class="gantt">
<div class="g-head"><span style="left:0%">Month 0</span><span style="left:16.67%">3</span><span style="left:33.33%">6</span><span style="left:50%">9</span><span style="left:66.67%">12</span><span style="left:83.33%">15</span><span style="left:100%">18</span></div>
<div class="g-row"><div class="g-label">Phase 0</div><div class="g-track"><div class="g-bar p0" style="left:0%;width:11.1%">Discovery</div></div></div>
<div class="g-row"><div class="g-label">Phase 1</div><div class="g-track"><div class="g-bar p1" style="left:11.1%;width:27.8%">Build and configure</div><div class="g-bar p1r" style="left:38.9%;width:11.1%">Parallel run</div></div></div>
<div class="g-row"><div class="g-label">Phase 2</div><div class="g-track"><div class="g-bar p2" style="left:50%;width:33.3%">Pre-release checks, models, assistant</div></div></div>
<div class="g-row"><div class="g-label">Phase 3</div><div class="g-track"><div class="g-bar p3" style="left:83.3%;width:16.7%">More banks</div></div></div>
</div>

| Phase | What it delivers |
|---|---|
| **0. Discovery** (6 to 8 weeks) | Inventory of the core banking and payment systems; read-only access approved; data mapped; data-quality baseline; alignment with the bank's own risk assessment |
| **1. Core system** (first bank live in about 9 months) | Screening with Arabic matching; the CBI scenario catalogue and the Iraq risk pack; customer risk rating and KYC reviews; cases, investigation and network view; goAML reporting; command centre, branch window and evidence vault; daily reconciliation. Ends with two to three months of running beside the bank's current system, comparing every alert |
| **2. Before release, and smarter** (about 6 months more) | Pre-release checks for payments; models in shadow mode, then live after validation; the AI assistant for summaries and narratives; correspondent and validator packs |
| **3. Scale** (ongoing) | Further banks in three to five months each; more regime packs; Islamic pack updates; Kurdish interface |

## 12. What success looks like {#success .nobreak}

<div class="kpis">
<div class="stat"><div class="n">100%</div><div class="l">of core banking tables reconciled every day, and every gap explained</div></div>
<div class="stat"><div class="n">1 hour</div><div class="l">at most from a list update to a full re-screen, against the CBI's 12-hour limit</div></div>
<div class="stat"><div class="n">0</div><div class="l">high-risk customer reviews overdue</div></div>
<div class="stat"><div class="n">0</div><div class="l">alerts older than 30 days without a recorded reason</div></div>
<div class="stat"><div class="n">Rising</div><div class="l">share of alerts that lead to a report or enhanced due diligence</div></div>
<div class="stat"><div class="n">Minutes</div><div class="l">to produce an evidence pack for the CBI or a correspondent</div></div>
<div class="stat"><div class="n">First time</div><div class="l">pass of the CBI-approved firm's verification of the AML system</div></div>
<div class="stat"><div class="n">Weeks</div><div class="l">of mapping, not months of development, to connect a new bank</div></div>
</div>

AKR-TMS sets no target for the number of reports. Better detection will produce more reports, and Iraq's action plan
asks for them, but a report filed to meet a quota is only noise for the AML/CFT Office.


## Appendix A. Glossary {#glossary}

<div class="glossary" markdown="1">

| Term | Meaning | Arabic |
|---|---|---|
| AML/CFT | Anti-money laundering and countering the financing of terrorism | <span class="ar">مكافحة غسل الأموال وتمويل الإرهاب</span> |
| KYC, CDD | Know your customer; customer due diligence: identifying customers and understanding their activity | <span class="ar">اعرف عميلك، العناية الواجبة تجاه العملاء</span> |
| EDD | Enhanced due diligence: extra checks for higher-risk customers | <span class="ar">العناية الواجبة المعززة</span> |
| Beneficial owner | The person who ultimately owns or controls a customer | <span class="ar">المستفيد الحقيقي</span> |
| PEP | Politically exposed person: someone with a prominent public function, and their family and close associates | <span class="ar">الشخص المعرض سياسياً</span> |
| Risk-based approach | More effort where the risk is higher, less where it is lower | <span class="ar">النهج القائم على المخاطر</span> |
| Transaction monitoring | Checking account activity for signs of laundering or terrorist financing | <span class="ar">مراقبة العمليات</span> |
| Scenario | A rule that describes a suspicious pattern and raises an alert when it occurs | <span class="ar">سيناريو</span> |
| Alert, case | An alert is one signal; a case groups the alerts about one customer for investigation | <span class="ar">تنبيه، حالة</span> |
| False positive | An alert or list match that turns out not to be suspicious | <span class="ar">إنذار كاذب</span> |
| Screening | Comparing customers and payments against sanctions and PEP lists | <span class="ar">الفحص على القوائم</span> |
| Asset freeze | Blocking the funds of a listed person without delay | <span class="ar">تجميد الأموال</span> |
| STR | Suspicious transaction report to the financial intelligence unit | <span class="ar">تقرير عن معاملة مشبوهة</span> |
| CTR | Report of cash transactions above a threshold (USD 10,000 or its equivalent in Iraq) | <span class="ar">تقرير المعاملات النقدية</span> |
| goAML | Reporting software from the UN Office on Drugs and Crime, used by the Iraqi financial intelligence unit | goAML |
| AML/CFT Office | Iraq's financial intelligence unit, which receives STRs and CTRs | <span class="ar">مكتب مكافحة غسل الأموال وتمويل الإرهاب</span> |
| MLRO | Money laundering reporting officer: the bank officer who decides what is reported | <span class="ar">مسؤول الإبلاغ عن غسل الأموال</span> |
| Tipping off | Telling a customer that they are suspected or reported; prohibited | <span class="ar">تنبيه العميل</span> |
| Hawala | Informal money transfer outside the licensed system | <span class="ar">الحوالة غير الرسمية</span> |
| Structuring | Splitting transactions to stay below reporting thresholds | <span class="ar">تجزئة المعاملات</span> |
| Money mule | A person whose account moves other people's illicit money | <span class="ar">وسيط نقل الأموال</span> |
| Correspondent bank | A foreign bank that holds accounts for an Iraqi bank and makes payments on its behalf | <span class="ar">المصرف المراسل</span> |
| Grey list | The FATF's list of jurisdictions under increased monitoring | <span class="ar">القائمة الرمادية</span> |
| Mutual evaluation | The peer review of a country's AML/CFT system by the FATF or a regional body | <span class="ar">التقييم المتبادل</span> |
| Core banking system | The bank's main system of record for customers, accounts and transactions | <span class="ar">النظام المصرفي الشامل</span> |
| Three lines of defence | Business units, compliance and risk, and internal audit, each with its own control role | <span class="ar">خطوط الدفاع الثلاثة</span> |
| Regime pack | An AKR-TMS component that adds one foreign regime's lists, rules and record-keeping | <span class="ar">حزمة الأنظمة الأجنبية</span> |

</div>

## Appendix B. The research behind this plan {#research .nobreak}

<!-- Counts refreshed from research/ by script on 2026-10-07 (unique ^| PREFIX-NN rows across all files, excluding FM, FL, TR, G and IR, which are failure modes, rating floors, triggers, stack gaps and interface rules); all wave-1 and wave-2 files final. -->

<p class="lead">The plan rests on the research files listed below, written in October 2026 and delivered with it. Each
file records its sources and ends with requirements for AKR-TMS, each traced to a law, a standard, a regulator's finding
or a market fact.</p>

<div class="rindex" markdown="1">
<div class="rcol" markdown="1">

#### Rules and standards

- **01** Iraq: AML/CFT law and CBI rules
- **02** Iraq: money laundering and terrorist financing risks
- **03** FATF standards
- **04** Basel, Wolfsberg, Egmont and UN sanctions
- **05** US, EU, UK and Gulf regimes
- **06** AML and KYC fundamentals, with an English-Arabic glossary

</div>
<div class="rcol" markdown="1">

#### The market

- **07** Vendor landscape overview
- **08** NICE Actimize
- **09** Oracle FCCM
- **10** SAS
- **11** SymphonyAI, FICO Siron, Nasdaq Verafin, Fiserv
- **12** AI detection specialists
- **13** Investigation automation and challengers
- **14** Screening engines
- **15** Risk data providers
- **16** KYC and client lifecycle platforms
- **17** AML modules of core banking systems
- **18** The Iraqi and regional market

</div>
<div class="rcol" markdown="1">

#### Lessons and design

- **20** Enforcement failures
- **21** Detection technology
- **22** Arabic, Kurdish and Iraqi name matching
- **23** Regulatory reporting and goAML
- **24** Islamic banking
- **25** Core banking integration
- **26** Customer risk rating and due diligence
- **27** Monitoring scenarios
- **28** Case management and workflow
- **29** Governance and model risk
- **30** Security and privacy
- **31** Analyst experience
- **32** Technology stack

</div>
</div>

<div class="reqcount" markdown="1">

| Research area | Files | Requirements |
|---|---|---|
| International standards: FATF, Basel, Wolfsberg, Egmont, UN sanctions | 03, 04 | 78 |
| Foreign regimes: US, EU, UK, Gulf | 05 | 61 |
| The market | 07-18 | 249 |
| Enforcement | 20 | 68 |
| Detection technology | 21 | 66 |
| Arabic, Kurdish and Iraqi name matching | 22 | 59 |
| Regulatory reporting and goAML | 23 | 84 |
| Islamic banking | 24 | 30 |
| Core banking integration | 25 | 65 |
| Customer risk rating and due diligence | 26 | 42 |
| Monitoring scenarios | 27 | 60 |
| Case management and workflow | 28 | 76 |
| Governance and model risk | 29 | 54 |
| Security and privacy | 30 | 102 |
| Analyst experience | 31 | 71 |
| Technology stack | 32 | 43 |
| **Total** | | **1,208** |

</div>

Iraqi law and the CBI's instructions (file 01) are the baseline for all of them; their requirements are summarised in
section 3.


