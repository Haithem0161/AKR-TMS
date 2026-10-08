# 27 — Transaction monitoring scenarios

Status: done (2026-10-07)

Part A (the CBI's own scenario catalogue) was transcribed from the source first because
it is the legal minimum every Iraqi bank's monitoring system must implement.

Contents of Part B (typologies, the AKR-TMS scenario library, CBI mapping, tuning, design), appended after Part A:

- B1 How to read Part B (ID scheme, basis tags, data-readiness and evidence grades)
- B2 Iraqi typologies and the evidence behind them (16 typologies, B2.1 to B2.16, each with sources and AKR-TMS hooks)
- B3 What a read-only replica can and cannot see
- B4 to B21 The scenario library by family, 161 scenarios (each family: what it detects, how it runs, hooks): CASH, XFER, TRADE,
  ISL (by reference to file 24), DIGI (e-wallets, cards, digital), LOAN, ACCT, PEP, PUB (public funds and payroll), CORR (correspondent,
  nested, exchange house, PSP), TF, PF (proliferation and sanctions evasion), INS (internal fraud and staff), DORM, CHQ, FX, OTH, OBS
  (branch observations)
- B22 Mapping the 173 CBI items (86 numbered scenarios plus 87 indicators) to library scenarios; what cannot be monitored from a replica
- B23 Coverage against the bank's own risk assessment
- B24 Tuning and testing methodology (initial thresholds, above/below-the-line, segmentation, back-testing, champion/challenger,
  scenario card, review cadence)
- B25 What the CBI assessor (standard D2) and foreign supervisors expect
- B26 Concrete design: catalogue, versioning, scenario studio workflow, performance metrics
- B27 What it means for AKR-TMS, with gaps and items not verified
- B28 Derived requirements (S-01 to S-60)
- B29 Sources

## Part A. The CBI scenario catalogue (AML/CFT/CPF supervisory controls, 8 Jan 2025, Chapter 6)

Source: CBI, "Supervisory controls for AML, CFT and counter-proliferation", 2025-01-08, Chapter 6
"Indicative indicators for identifying operations suspected of involving ML or TF", doc pp. 49-58
(https://cbi.iq/static/uploads/up/file-173666439114818.pdf). Translated from Arabic by reading the rendered
pages. Numbering follows the source.

**Legal status (chapter intro, p. 49):** these examples and scenarios are "the latest raised by the CBI and the
AML/CFT Office", reflecting new financial instruments and technology; **"these scenarios are the MINIMUM that
financial institutions must follow"** to apply due diligence suited to the Iraqi financial environment, and
**"your institutions may develop new scenarios under the supervision of, and in coordination with, the AML/CFT
Office."** Article 3 states the AML/CFT Office's general scenarios are to be adopted **"as a minimum for the
digital systems you rely on"**.

(The 2024 MER described a 34-scenario set raised to 56 for banks; this 2025 catalogue is larger: 21 cash + 16
transfer + 49 general = 86 numbered scenarios before the product-specific articles 4-14.)

### Article 1 — Cash operations (21)

1. Large cash deposits made unusually by a natural or legal person, inconsistent with their business or job
   activity and size.
2. Repeated small cash deposits meant not to attract staff attention, whose total over a period does not fit
   the customer's activity.
3. Use of multiple accounts to deposit cash totalling a very large amount in a very short time.
4. Repeated cash deposits at several branches of the same bank in a short period, by the account holder or
   through others.
5. Large deposits and withdrawals through ATMs to avoid contact with staff, especially if inconsistent with
   the customer's activity and usual income.
6. Large cash deposits and withdrawals on dormant or inactive accounts, relatively large withdrawals, or
   accounts receiving unexpected large amounts from abroad.
7. Repeated withdrawal of funds shortly after deposit without clear justification.
8. Customer takes back part of the cash to be deposited on learning that CDD is required.
9. Customer submits financial statements clearly different from similar companies in the same sector.
10. Repeated or large outgoing transfers funded in cash, inconsistent with activity and size.
11. Companies with relatively large activity submitting financial statements not audited by a licensed
    accountant.
12. Depositing or withdrawing cash in several stages so each amount is under the bank's threshold but the
    total exceeds it (structuring).
13. Customer suddenly repays a large debt with no clear, reasonable explanation of the source.
14. Customer exceeds their average withdrawal and deposit movements.
15. Customer exceeds their average outgoing and incoming transfers.
16. Cash withdrawal above the limit declared in the KYC form.
17. Monthly income above the expected income declared in the KYC form.
18. Customer unexpectedly transfers the value of credit facilities obtained to outside Iraq.
19. Repeated transfers to the same beneficiary within a set period.
20. Large cash purchases of bank cheques.
21. Receipt of large cheques endorsed by third parties to the customer's account.

### Article 2 — Transfers (16)

1. Large transfers abroad, or incoming transfers from abroad, with instructions to pay out in cash.
2. Repeated incoming transfers from different parties with no clear relationship to the customer (or outgoing
   from the customer to such parties).
3. Equal or near-equal transfers to several persons in different countries, or to one beneficiary across
   several accounts.
4. Large local transfers followed by transfers abroad in different currencies.
5. Deposits moved abroad directly, in one or several payments.
6. Large transfers from abroad into dormant accounts.
7. Repeated or large outgoing transfers funded in cash, inconsistent with activity and size.
8. Repeated transfers whose total over a period does not fit the customer's activity.
9. Customer uses the account as a pass-through to move funds between parties or accounts.
10. Outgoing or incoming transfers in near-identical value ranges.
11. Transfers to or from high-risk countries or regions, or parties on local (asset-freeze) or international
    sanctions lists.
12. Incoming transfers sent on in the same period or shortly after.
13. Transfers to or from PEPs and high-risk persons.
14. Identical personal details for several unrelated persons (address, phone, date of birth, passport no.).
15. Different documents presented on each transfer compared with the bank's records.
16. Refusal to present the passport of sender and recipient, which name screening against local and
    international lists depends on.

### Article 3 — General ML/TF scenarios prepared by the AML/CFT Office (49) — minimum for digital systems

1. Dormant account becomes active without reasonable cause.
2. Exchanging large quantities of small notes for large notes without clear reason.
3. Unusually high cash deposits withdrawn by cheque and moved to other accounts repeatedly without convincing
   reason, especially soon after deposit.
4. Not using letters of credit for foreign trade and insisting on cash transfers.
5. No logical reason for the customer's choice of branch.
6. Customer refuses to give information or documents required for CDD.
7. Customer refuses credit facilities (extra benefits) to avoid giving information that could reveal the real
   activity.
8. Several customers come to exchange dinars into various foreign currencies with no sign of need.
9. Customers who travel constantly to countries known for drug production or trafficking or illicit trade.
10. Walk-in customers making fixed deposits without opening accounts.
11. Suspicious behaviour: visible nervousness, sometimes threatening the employee to stop a suspicion report
    or ID verification.
12. Deposits containing counterfeit or heavily used, dirty, stored banknotes.
13. Customers holding several accounts their work does not need, especially with transactions involving
    unknown persons.
14. Large cheques endorsed by a third party to the customer with no clear relationship, or not consistent with
    the customer's profession.
15. Multiple transfers from abroad into one account, repeatedly just **below the USD 10,000** reporting
    threshold.
16. Transfers from banking-secrecy jurisdictions.
17. Persons or companies attracting large sums to invest in FX or tradeable instruments when deal size does
    not fit them.
18. Repeated buying or selling of securities without clear purpose.
19. Sudden early repayment of long-term loans without declaring the source; facilities secured by jewellery or
    real estate repaid before maturity.
20. Loans secured by assets owned by others with no clear relationship.
21. Several accounts tied to the same beneficiary or agent with the same data, addresses and job details,
    aimed at issuing payment cards for use abroad.
22. Open-source reports linking a person or entity to a past crime, especially human trafficking, migrant
    smuggling or international sanctions.
23. Attempts to open accounts with documents that look forged.
24. Young customer with unclear account purpose, source of funds or transaction values.
25. A third party translates or directs the account holder during withdrawals or deposits, especially at peak
    times; a third party always holds the customer's ID.
26. Lifestyle not justified by business; profits or deposits far above peers in the same profession.
27. Dealings with persons or entities about whom the bank previously filed STRs.
28. Transactions by women, especially young, inconsistent with their activity, repeatedly sending or receiving
    money via the bank or agents (Western Union, MoneyGram) inconsistent with activity abroad.
29. Transactions of some legal persons (e.g. beauty salons) inconsistent with their activity, or foreigners
    operating those accounts.
30. Negative open-source information about sexual exploitation, marriage of minors or similar.
31. Cash deposits to or from doctors, nurses, medical and hospital staff disproportionate to their work.
32. Unjustified dealings between charities (or their staff) and parties unrelated to humanitarian work.
33. Several accounts with several signatories who have no clear relationship.
34. Beneficial owner belongs to countries or regions known for crime, or high-risk countries.
35. Customer changes the place of payment named in the LC or invoice.
36. Suspected under- or over-invoicing.
37. Several credit cards issued to the same person in a short time.
38. Customer holding a senior government post withdraws all IQD balances and deposits them in USD.
39. ATM withdrawals in a high-risk area far from the customer's address (card may not be in the customer's
    hands; customer may not be the real beneficiary).
40. Suspected illegal FX trading: card funded with IQD cash, then USD cash withdrawals from ATMs abroad.
41. Repeated card purchases of large USD amounts abroad.
42. Cash placed in a term deposit out of proportion to the customer's young age and unclear activity.
43. Purpose of a transfer from a customer to an account at another local bank cannot be verified, nor the
    relationship.
44. Repeated deposits for third parties by bank employees with no stated relationship.
45. Housewife of unknown activity repeatedly receiving small cash transfers from several people in several
    countries with no clear relationship.
46. Suspected use of credit cards for virtual-currency speculation, with deposits onto the card from unknown
    parties.
47. Single-premium insurance policy for a large amount, partly withdrawn soon after, inconsistent with
    activity.
48. Social-media worker receiving fragmented, continuous cash transfers from abroad below the threshold.
49. Large cash deposit backed by recent deferred-dowry (divorce) documents, followed by fragmented transfers to
    unrelated persons.

### Articles 4-14 — product and behaviour indicators (pp. 53-58)

**Article 4 — Documentary letters of credit (9):** (1) import/export inconsistent in type or value with the
customer's business; (2) large gap between goods value in the LC and real value; (3) request to change the LC
beneficiary before payment without clear reason; (4) many LCs out of line with activity; (5) LCs against
financial guarantees out of line with activity size and history; (6) unusual payment terms or payment to
foreign parties unrelated to the LC; (7) LC beneficiary or shipping company owned by the customer; (8) amounts
in submitted LC documents differ from originals; (9) place of payment changed to an account in a country other
than the beneficiary's.

**Article 5 — Letters of guarantee (4):** (1) many LGs out of line with activity; (2) LGs against financial
guarantees out of line with activity and history; (3) beneficiary calls the LG soon after issue without clear
reason; (4) LGs issued without underlying project contracts.

**Article 6 — Credit facilities (10):** (1) loans secured by others' assets with no clear link; (2) facilities
against guarantees from a bank abroad without clear reason; (3) borrower asks to move the loan quickly to banks
abroad; (4) unexpected early repayment of distressed debt by the customer or third parties; (5) buying CDs from
time to time and later pledging them for facilities; (6) facilities against blocked deposits of a company or
subsidiaries abroad, especially in drug-producing countries; (7) facilities secured by cash deposits;
(8) sudden repayment of a large debt with no clear source; (9) facility refused because of doubts about the
guarantees; (10) unaudited financial statements.

**Article 7 — Digital banking: internet, mobile, online payments (10):** (1) many small digital inflows then
large digital transfers abroad; (2) regular large digital payments from countries known for drugs, arms or
human trafficking; (3) repeated online transfers between the customer's own accounts for no reason;
(4) regular large digital deposits or receipts from other high-risk countries; (5) online account opening with
forged or misleading documents to get preferential services; (6) digital channels used for repeated transfers
to different people without reason; (7) sending/receiving transfers while logged in from high-risk countries;
(8) changing **IP addresses** or using technical means to hide tracking; (9) funds moved from several prepaid
accounts to one person or entity; (10) services used in many locations, regions and currencies.

**Article 8 — Payment cards: credit, debit, prepaid (5):** (1) repeatedly using the full card limit then
paying it all off; (2) repeatedly hitting the daily cash-withdrawal maximum; (3) account funded constantly with
few or no transactions, then withdrawn; (4) unjustified transfers between the customer's accounts or with
authorised persons; (5) ATM cash deposit in one currency followed by ATM withdrawals in another.

**Article 9 — Foreign exchange (6):** (1) repeated FX buying or selling whose total does not fit the activity;
(2) counterparty located in drug-producing countries; (3) repeated, unexpected, relatively large domestic or
foreign cash transfers in foreign currency; (4) refusal to explain the source of foreign currency deposited;
(5) chains of FX transactions to hide the real beneficiary; (6) FX deposits inconsistent with the source of
funds and income declared in the KYC form.

**Article 10 — Safe-deposit lockers (4):** (1) unusually frequent visits; (2) non-residents of the area keeping
lockers without reason, especially if available near them; (3) heavy locker use suggesting cash storage;
(4) renting many lockers without reason.

**Article 11 — Securities trading and settlement (4 distinct):** (1) huge amounts without basic knowledge of
securities risk; (2) cash deposited to buy "long-term" securities, sold and withdrawn shortly after;
(3) account funded constantly with few transactions, then withdrawn; (4) unjustified transfers between
accounts.

**Article 12 — Customer behaviour (15):** (1) avoids staff (constant ATM use, evades officers); (2) anxiety or
confusion during the transaction; (3) many accounts without reason; (4) persistent questions about AML rules
and records, to learn how to avoid them; (5) asks for excessive secrecy; (6) cancels when staff ask for missing
information; (7) reluctance once told details must be reported; (8) abnormal behaviour (ignores high interest
on a large balance, ignores basic facts of the deal); (9) refuses ID documents; (10) student requesting large,
unusual transfers or FX; (11) customer controlled by another person, unaware of what they are doing, or an
elderly person accompanied by an unrelated person; (12) gifts or bribes to staff, pressure not to verify ID;
(13) phone numbers disconnected or not working; (14) refuses to disclose business details or documents;
(15) permanent address outside the branch's area.

**Article 13 — Other indicators (11):** (1) several unrelated signatories on one account, especially foreign
nationals; (2) using accounts belonging to others; (3) beneficial owner from an area known for crime;
(4) accounts used to receive and pay out large sums unrelated to the holder's activity; (5) accounts that
receive many cash deposits or transfers and are then closed or go dormant; (6) sudden luxury out of line with
economic status; (7) criminal record of the suspect, BO or a party; (8) purchase of high-value real estate,
vehicles, jewellery; (9) proven document forgery; (10) a party under investigation abroad; (11) fictitious
contracts.

**Article 14 — Employee behaviour and Know-Your-Employee (KYE).**
Indicators of staff involvement: (1) sudden rise in lifestyle and spending vs salary; (2) loosely bypassing
controls; (3) helping execute transactions where the beneficiary or counterparty is not fully known;
(4) exaggerating a customer's credibility and means in reports to management; (5) staying after hours without
reason; (6) clear neglect of duties; (7) assets out of line with salary; (8) dealing with persons known for lack
of integrity; (9) breaching confidentiality, leaking documents.
KYE obligations on banks: apply KYE to detect internal collusion; a KYE form, with compliance tracking it and
reporting results in the **semi-annual report to the CBI**; **mandatory two consecutive weeks of leave per year
for all staff** including managers and control staff, without pay deductions; specialised compliance / AML /
anti-corruption / fraud training for all staff (not only managers) at no cost to them; **rotate branch managers
at most every 5 years** (no more than 3-5 years at the same branch).

### Design implications for AKR-TMS (from Part A)

- Ship the full CBI catalogue as a versioned "CBI baseline pack", each scenario mapped to its article number so
  examiners can trace coverage (coverage matrix report: scenario -> rule(s) -> alerts -> outcomes).
- Many scenarios depend on **KYC-declared values** (expected cash withdrawals, monthly income, declared source
  of funds): the customer profile must capture declared expectations and compare actuals continuously
  (Art. 1 nos. 16-17, Art. 9 no. 6).
- Some scenarios are **observational** (nervousness, third party holding the customer's ID, coercion, elderly
  customer accompanied, bribery attempts, forged documents). These cannot come from the database; they need the
  **branch observation log** (front-line staff record what they saw) which then combines with data patterns.
- Some need **external data** the CBS may not hold: open-source/adverse media (Art. 3 nos. 22, 30), IP address
  and geolocation of digital sessions (Art. 7 nos. 7-8), ATM location vs home address (Art. 3 no. 39), customs
  data vs LC/invoice values (Art. 4, Art. 3 no. 36). Mark each scenario with the data it needs and show the bank
  which ones its data feed can and cannot support (data-readiness view).
- **Insider risk**: staff are bank customers too, so the CBS holds their accounts. Scenario 3/44 (employee
  deposits for third parties) and the KYE indicators support an employee-risk view: salary vs inflows, deposits
  made by staff into unrelated accounts, overrides by user ID, mandatory-leave tracking (if HR data is
  provided). Handle with strict access control (only compliance head) to avoid misuse.
- **Trade finance** scenarios (LC/LG) require trade data (goods, values, ports, shipping companies), often in a
  separate trade-finance module of the CBS; mapping layer must include it.
- Iraqi specifics a foreign vendor would miss: deferred dowry (mahr) documents as cash justification (Art. 3
  no. 49), family card as ID, Western Union/MoneyGram agent flows, IQD-to-USD conversion by senior officials
  (no. 38), card-funded illicit FX (nos. 40-41), social-media workers receiving fragmented transfers (no. 48).

---

## Part B. Typologies, the AKR-TMS scenario library, CBI mapping, tuning and design (research of 2026-10-07)

Part A is the legal floor: the CBI's own 86 numbered scenarios and its product and behaviour indicators. Part B (1) records which
laundering and terrorist-financing typologies the evidence says matter in Iraq, (2) defines an AKR-TMS scenario library that implements
the CBI floor and extends it, (3) maps every CBI scenario to the library and says which ones a read-only replica cannot monitor,
(4) sets out how thresholds are chosen without history and then tuned, tested and reviewed, and (5) proposes the catalogue, version and
scenario-studio design. Evidence, not opinion: every typology cites a source; every numeric parameter says where it comes from.

**What was not possible.** The web-search tool was exhausted, so sources were reached by direct download, the UN document system, the
Federal Register, Treasury pages, the Wayback Machine and the CDX listing. Items I could not verify are marked "Unverified". Several
CBI documents are scans; the Arabic readings of the 2016 FX procedures and of the 2024 fraud and corruption guide are mine.

### B1. How to read Part B

**Scenario IDs.** Library scenarios are named SCN-<FAMILY>-NN. The families are:

| Family code | Family | Main CBI anchors (Part A) |
|---|---|---|
| CASH | Cash deposits, withdrawals, structuring, denominations | Art. 1 (cash); Art. 3 nos. 2, 3, 12, 44 |
| XFER | Transfers and wires, domestic and cross-border | Art. 2 (transfers); Art. 3 nos. 15, 16, 43, 45, 48 |
| TRADE | Trade finance, import payments, dollar access | Art. 4 (LCs); Art. 5 (LGs); Art. 3 nos. 4, 35, 36 |
| ISL | Islamic products (by reference to file 24 section 8) | CBI 2018 Islamic controls; Art. 4 and Art. 6 by analogy |
| DIGI | E-wallets, cards, digital channels, mule networks | Art. 7 (digital); Art. 8 (cards); Art. 3 nos. 21, 37, 39-41, 46 |
| LOAN | Loans, guarantees and early repayment | Art. 6 (credit); Art. 1 nos. 13, 18; Art. 3 nos. 19, 20 |
| ACCT | Accounts, profile deviation and behaviour change | Art. 1 nos. 14-17; Art. 3 nos. 13, 24, 26, 33; Art. 13 |
| PEP | PEPs and other high-risk customers | Art. 2 no. 13; Art. 3 nos. 34, 38; CBI PEP circular of 26 Aug 2026 |
| PUB | Public funds, payroll and government contracts (added: the corruption typology is ranked first in Iraq) | Art. 13; Art. 14; CBI fraud guide 2024 annex 2 |
| CORR | Correspondent, nested, exchange-house and PSP activity | Chapter 10 of the 2025 controls; Art. 2 no. 11; Art. 9 |
| TF | Terrorist financing | Art. 3 no. 32; circular of 26 Aug 2026 group 7 |
| PF | Proliferation financing and sanctions evasion | Chapter 9 of the 2025 controls; Art. 2 no. 11 |
| INS | Internal fraud, staff accounts, know-your-employee | Art. 14; CBI fraud guide 2024; Art. 3 no. 44 |
| DORM | Dormant accounts | Art. 3 no. 1; Art. 1 no. 6; Art. 2 no. 6; chapter 15 |
| CHQ | Cheques | Art. 1 nos. 20, 21; Art. 3 nos. 3, 14 |
| FX | Currency exchange and dollarisation | Art. 9 (FX); Art. 3 nos. 8, 38 |
| OTH | Lockers, securities, insurance, term deposits | Art. 10, Art. 11; Art. 3 nos. 10, 18, 42, 47 |
| OBS | Branch observations and behavioural indicators (hybrid alerts) | Art. 12; circular of 26 Aug 2026; Art. 3 nos. 6, 11, 25 |

**Library size.** 161 scenarios: CASH 14, XFER 13, TRADE 13, ISL 8 (bundles of the 57 IS scenarios of file 24), DIGI 15, LOAN 8, ACCT 13, PEP 9,
PUB 7, CORR 8, TF 7, PF 9, INS 9, DORM 6, CHQ 6, FX 4, OTH 4, OBS 8. Of these, 99 implement at least one CBI item and 62 extend the CBI floor (B22).
Bank-authored scenarios use IDs 901 to 999 in each family (B26.1).

**Typology tags.** Each scenario's name cell ends with the typologies it is built to catch, numbered as in the B2 table (1 to 16) plus five cross-cutting
patterns: 17 layering and integration through credit, securities, insurance and lockers; 18 insider and staff abuse; 19 dormant, abandoned and low-tier
accounts; 20 customer behaviour and avoidance of due diligence; 21 profile deviation (expected versus actual activity). The reverse index is at the end of B2.

**Fields recorded for every scenario.** Each family section has two tables. Table 1 says what the scenario detects: ID, name and
typology, the CBI link (article and number, and whether the scenario implements, extends or is outside the catalogue), the logic in plain
language and the canonical-model fields it reads (file 25 section 13.2). Table 2 says how it runs: starting parameters with the basis for
each, segmentation, look-back window, mode, expected false-positive drivers and the evidence an analyst needs.

**Basis tags for parameters.**

| Tag | Meaning |
|---|---|
| [CBI] | A number written in a CBI or AML/CFT Council rule (for example USD 10,000 reporting, IQD 1 million simplified due diligence, six months for dormancy) |
| [CBI-2016] | A number from the CBI's 2016 FX procedures (read from the scan; current status not verified) |
| [MER] | A number reported in the 2024 MER (for example USD 380,000 real-estate rule, exchange-company transfer threshold of about USD 3,000) |
| [PEER] | A number or method printed in a foreign supervisor's text (for example MAS: pass-through within plus or minus 10 percent) |
| [KYC] | A multiple or ratio of the customer's own declared profile (expected income, cash limit, turnover) |
| [PCT] | A percentile of the customer's segment history, used after a bootstrap period (B24.2) |
| [DP] | A design proposal: a cautious starting value chosen by this research, to be replaced by [PCT] or [KYC] within 90 to 180 days |

**Conventions.** Amounts are in US dollar equivalents at the daily CBI rate from the rate table; the MER's own conversions imply about
IQD 1,310 to 1,320 per USD (IQD 3.7 billion = USD 2.82 million, Box 4.8; IQD 500 million = USD 380,000, MER paragraph 195), so USD 10,000 is
about IQD 13.1 million, USD 5,000 about IQD 6.5 million and USD 3,000 about IQD 3.9 million. IQD and USD are separate dimensions in
every parameter (file 21 section 2b). Windows are rolling unless stated. "Segment" means customer segment times currency times
risk tier (file 21 section 3b). Mode: NRT = near-real-time per event (minutes after CBS commit, file 25 section 9.2 class F1); EOD =
batch after the CBS end of day (class F3); WK/MO = periodic. "P2" marks scenarios whose precision is high enough to be candidates
for the phase-2 pre-release hold (CBI chapter 5 article 5: stop a transaction that matches a scenario until the MLRO clears it).

**Data-readiness grades** (used in B3 and B22): **A** operable from fields every CBS holds; **B** degraded or dependent on an optional
module (payment messages, trade module, teller log, card switch); **C** needs data outside the CBS (digital-channel logs, HR, customs,
registries, adverse media, authority requests); **D** observational, only a person can supply it (branch observation log).

**Evidence grades for typologies.** [P] official document read in this session (regulator, FIU, UN, Treasury, Federal Register);
[S] reputable secondary source or Wikipedia citing it; [N] analysis by this research, to be tested with bank data. Source tags in
square brackets point to section B24.

### B2. Iraqi typologies and the evidence behind them

The 2024 MER rates ML risk "moderate to high" and TF risk "high" (MER paragraph 56). It names "the banking, exchange and real estate
sectors" as "among the most prominent sectors that are misused for ML" (paragraph 44) and describes the banking pattern directly:
"large cash deposits and transfers abroad, as the nature of money laundering patterns has been characterized by external flows in
addition to the factor of high use of cash in Iraq" (paragraph 627). It also says the NRA is "the first of its kind" and that
information on methods "is still not comprehensive" (paragraph 48). The typologies below therefore draw on the MER, on US Treasury and
FinCEN findings about Iraqi banks, on the UN Monitoring Team, on FATF and Egmont reports, and on the CBI's own documents. Individuals on
sanctions lists are not named in this file.

| # | Typology | Strongest evidence (grade) | Visible in a bank's data as |
|---|---|---|---|
| 1 | Dollar and currency-auction abuse, over-invoiced and fictitious imports | FinCEN Al-Huda final rule 2024 [P]; MER para 195 [P]; CBI 2016 FX procedures [P] | New accounts or companies buying USD; IQD funding by cheque or cash just before a purchase; reused or dead persons' IDs; counterfeit IQD; import payments without customs entries |
| 2 | Front companies | FinCEN notice on Banque Misr UAE 2026 (103 front companies) [P]; FinCEN FIN-2025-A002 [P]; OFAC 9 Oct 2025 [P] | Several companies sharing address, director, phone or incorporation date; sector mismatch between inflows and outflows; third-country free-zone counterparties |
| 3 | Hawala and unlicensed transfers through bank accounts | MER paras 43, 390-391, Box 4.3 [P]; FATF 2013 and 2026 [P]; UN S/2025/482 [P] | Collective accounts, many depositors then one wire, business accounts with no payroll or invoices, third-party cheques |
| 4 | Cash smuggling and couriers | MER paras 361-366, Boxes 3.28, 3.29, 4.8 [P]; OFAC 3 Jul 2025 [P] | Large cash withdrawals in border governorates, pilgrim and Umrah season cash, cash-intensive accounts of transfer offices |
| 5 | Trade-based laundering | FATF-Egmont 2020 and 2021 [P]; MER box 5.2 and para 57 [P] | Import payments above declared volume, goods outside the declared sector, third-party payers, LC amendments |
| 6 | Real estate | MER paras 44, 195, Boxes 3.18, 5.2 [P] | Cash then property, property in relatives' names, payments to developers above profile, payments to registrars |
| 7 | Corruption and embezzlement of public funds | MER paras 57, 58, Box 3.18, 5.2 [P]; Treasury 9 Oct 2025 [P]; tax-deposit theft 2022 [S] | State cheques to new companies then cash or abroad; payroll and pension manipulation; sub-contracting chains |
| 8 | Fraud proceeds | MER para 57 (2,481 cases) [P]; FATF/Interpol/Egmont 2023 annex A [P] | Many small inbound payments to one account, rapid cash-out, shared devices, new beneficiaries |
| 9 | Drug trafficking (crystal, Captagon) | MER paras 57, 58, Box 3.9 [P] | Small cash by many couriers consolidated to agents, transit through Kurdistan, no ML convictions |
| 10 | Terrorist financing (ISIL remnants, armed groups) | UN S/2025/482, S/2026/44, S/2026/651 [P]; MER Chapter 4 [P] | Small recurring transfers, exchange-office payouts, personal-account donations, NPO flows |
| 11 | Proliferation financing and Iran sanctions evasion | Treasury 3 Jul, 2 Sep, 9 Oct 2025 [P]; FinCEN FIN-2025-A002 [P]; CBI chapter 9 [P] | Oil-trade payments of doubtful origin, exchange-house chains, dual-use goods, USD cash leakage |
| 12 | Mule networks in e-wallets and cards | FATF/Interpol/Egmont 2023 [P]; UN S/2026/651 para 112 [P]; MER Box 4.8 [P] | New wallets with many senders, shared devices, card cash-out abroad, paid-for identity documents |
| 13 | Abuse of NPOs and charities | MER paras 51, 395, 412, Boxes 3.13, 4.5, 4.11, 4.12 [P]; FATF 2014 [P] | Foreign funding out of line with activity, fictitious salaries, personal accounts collecting donations |
| 14 | Zakat and religious donations | File 24 section 4 [P]; FATF 2026 box 4 [P]; MER para 409 [P] | Seasonal peaks, donation narratives, pooled collections |
| 15 | Kurdistan-specific flows | MER paras 52, 59, 147-149, 239, Box 3.9 [P]; file 01 section 9 [P] | Drug transit, human trafficking, salary localisation accounts, Kurdish-script names |
| 16 | Human trafficking and exploitation | MER paras 57, 148 [P]; CBI Art. 3 nos. 24, 28, 30, 42, 45, 49 [P] | Young customers with unexplained transfers, small remittances from many senders, deferred-dowry cash |

#### B2.1 Dollar and currency-auction abuse; over-invoiced and fictitious imports

*What it is.* Iraq sells oil for dollars held at the Federal Reserve Bank of New York, and importers bought dollars from the CBI through
the "dollar auction" (colloquially مزاد العملة, mazad al-'umla; the CBI's own 2016 text calls it the window for buying and selling foreign
currency, نافذة بيع وشراء العملة الأجنبية). FinCEN's final rule against Al-Huda Bank (89 FR 55051, 3 July 2024) describes it: "Iraqi businesses that import
goods into Iraq rely on wire transfers of USD from the CBI account at the Federal Reserve Bank of New York (FRBNY), a process known as
the wire auction", while the "cash auction" is "bulk USD banknote shipments to Iraq which the CBI sells to exchange houses and banks in
return for Iraqi dinar" [FR-2024-14415, footnote 7]. FinCEN assessed that the bank abused the wire auction "through numerous money
laundering typologies, including use of fraudulent documentation to obscure the ultimate beneficiaries of the transactions", and listed
"fraudulent documentation, fake deposits, identity documents of the deceased, fake companies, and counterfeit IQD". Specific schemes
quoted: false documents to justify transfers of "over $6 billion" in 2012-2014 (FinCEN cites media reporting, not its own count);
"fake checks" deposited "to make the balance seem higher" before purchasing USD, after which the cheque "bounced" and was written
off; use of "identification documents of deceased individuals", and payments to living people for their IDs, "to circumvent limits on
currency purchases"; counterfeit IQD "printed in Iran" funnelled "through Iraqi businesses" and exchanged for dollars; and "falsely
claimed imports into Iraq that did not exist worth billions of dollars". These are FinCEN's allegations about one bank, not court
findings. The MER's trade-based case is the same pattern at customer level: a company and its director sent USD 21 million abroad in two
years "for importing chemical materials", could not prove the source of funds, part of which came from "checks issued by one of the state
departments", and "customs permits do not correspond to the volume of the transferred amounts" (MER box 5.2).
*Controls on the books.* CBI 2016 FX procedures (letter of 31 July 2016; scan): window sales of USD capped at USD 3,000 per person
(instruction 9/3/176 of 24 May 2016); due diligence on source of funds for purchases above USD 3,000; no participation in the
window until six months after account opening; funds for purchase deposited seven days before the sale; the remittance procedures apply
to outgoing electronic remittances up to USD 150,000 and at most 10 a day for different persons "without splitting the original
transactions" [CBI-FX-2016]. The CBI's electronic platform for trade remittances was built "to ensure that this product is not
exploited for transferring criminal proceeds (such as from corruption) through the use of fake invoices" (MER paragraph 195); the
platform ended at the end of 2024 and each bank now answers to its own correspondents (file 01 section 4).
*Data signature.* New account or company buys dollars; funding by cheque or cash in the days before; many applicants sharing address,
phone, director or introducer; one ID used by several "customers"; deceased customers' IDs; counterfeit-note finds; import payments with
no customs entry; payments to third-country accounts unrelated to the goods' origin.
AKR-TMS hooks: SCN-TRADE-01, -02, -03, -10; SCN-ACCT-11 (reused IDs and IDs of dead persons); SCN-CHQ-04 (fake-cheque funding); SCN-CASH-07
(counterfeit IQD); SCN-FX-01; SCN-XFER-08 (shared identifiers); the bank-level mirror of these flows is SCN-CORR-07 (typology exposure index).

#### B2.2 Front companies (Banque Misr UAE: 103 front companies)

*Evidence.* FinCEN's proposed section 311 action against Banque Misr UAE (Federal Register notice, September 2026; file 20 section 9.1):
"FinCEN identified 103 potential Iranian shadow banking front companies transacting approximately USD 1.8 billion, using accounts with
Banque Misr UAE from January 2024 to June 2026. This includes approximately USD 520 million over the most recent 12-month period";
"Front companies are predominantly registered in third-country jurisdictions such as the UAE and ... Hong Kong to obscure beneficial
ownership". FinCEN's method is itself a monitoring result: it looked at the ratio of flows matching a typology to the bank's assets.
FinCEN advisory FIN-2025-A002 (6 June 2025) lists the red flags a bank can test (verbatim from the advisory): "Multiple companies
incorporated at roughly the same time share counterparties, addresses, owners, or name similarities and demonstrate similar transaction
profiles. Such companies may have little to no web presence and transact in large, recurring amounts"; "customers that primarily receive
funds from commodities trading companies, but send funds primarily to electronics suppliers"; "A customer makes transactions that move
through multiple exchange houses and/or trading companies, adding additional fees and costs as the transactions progress"; business
addresses "that are residential or co-located with other companies, especially companies that have been previously sanctioned"
[FINCEN-2025-A002]. The Treasury action of 9 October 2025 designated a state-owned Iraqi conglomerate that "uses a sub-contracting
method to divert funds from Iraqi government contracts" and an agricultural "commercial front" for it, and three bank executives for
"procuring U.S. dollars and laundering funds using counterfeit documents and receipts to avoid government scrutiny" [TSY-SB0277]. The
MER adds the domestic version: "preparing licenses for importing goods under different names not active in trade and import, thus
concealing the identity of the beneficial owner" (paragraph 57), and rates limited-liability companies and foreign-company branches
"medium" for ML and "moderate-high" for TF (paragraph 49).
*Data signature.* Clusters of customers or counterparties with shared address, phone, director, signatory or incorporation month;
sector mismatch between inflow and outflow counterparties; residential addresses for trading companies; flows through exchange houses
and third-country free zones.
AKR-TMS hooks: SCN-PF-02, SCN-PF-03, SCN-PF-06, SCN-TRADE-11, SCN-TRADE-12, SCN-XFER-08, SCN-XFER-13, SCN-CORR-07; entity graph (file 21 section 6).

#### B2.3 Hawala and unlicensed money transfer through bank accounts

*Evidence.* The MER explains why (the informal system is hawala, حوالة, "transfer"): dollars "can be obtained only through the Central Bank of Iraq (CBI) and only for commercial
purposes", which "gave rise to an alternative currency exchange system through unlicensed companies that are also active in the field
of money transfers"; their speed and low cost "have contributed to criminals increasingly resorting to the informal sector, especially
since most transactions executed through this sector are in cash" (paragraph 43). The exchange sector is rated high risk partly because
of "1028 cases" linked to the unlicensed sector in 2018-2022 (MER paragraph 109), and "the exchange sector, especially unlicensed
ones, is considered one of the most important channels exploited for transferring funds" for ISIL (paragraph 391). The CBI referred 347
cases of unlicensed banking activity to the judiciary in the same period (paragraph 603). Detecting informal transfers is item 2 of
Iraq's FATF action plan of June 2026 (file 01 section 1). FATF's 2013 typology gives the account-level indicators a bank can compute
(section 3.6.4, verbatim extracts): "extensive use of collective accounts" (small sums deposited, often with the name in the reference
line, or large cash sums "at regular intervals before transfers aggregating all of the smaller amounts"); "money being transferred at
regular intervals to international locations such as Dubai"; an account "used as a temporary repository and the funds are transferred in
and out ... immediately"; "usage of third party accounts"; "wire transfers frequently sent by traders to foreign countries, which do not
seem to have any business connection"; transactions "beneath the identification, STR or CTR reporting threshold"; "business accounts
used to receive or disburse large sums of money but show virtually no normal business related activities such as payment of payrolls,
invoices"; "frequent deposits of third party checks"; "frequent deposits by multiple individuals into a single bank account, followed
by international wire transfers and/or international withdrawals through ATMs"; and a "sudden change in pattern ... from low value
international fund transfers to large value transfers" [FATF-2013]. FATF's September 2026 report on professional money laundering,
underground banking and hawala notes that networks "continue to rely on the regulated financial system to settle net balances",
through "shell companies, money-mule accounts, virtual IBANs (vIBANs)", that "third party money mules ... fragment large-sum criminal
cash into multiple small deposits ... avoiding the AML monitoring thresholds", that "cuckoo smurfing" deposits illicit cash into a genuine
remitter's account, that crowdfunding or donation campaigns can serve as "a bank-visible explanation", and that in the Middle East
settlement increasingly uses "virtual assets backed by gold" (paragraphs 73-76, 113-116) [FATF-2026-UB]. It also states that "jurisdictions
indicate a pressing need for behavioural indicators specific to underground banking" (paragraph 146), so the scenarios below are
analysis built on the 2013 list, not an agreed standard. The UN Monitoring Team adds for ISIL: "cloud hawala systems in which data is
stored in the cloud", "safe drop boxes where money is deposited at exchange offices for retrieval with a password/code", and female
couriers (S/2025/482 paragraph 101) [UN-S2025-482].
*Data signature.* See the FATF list above; plus accounts of exchange and transfer companies with cash funding and few foreign
counterparties (CORR family, B13).
AKR-TMS hooks: SCN-XFER-01, -03, -05, -13; SCN-CASH-02, -08; SCN-ACCT-04; SCN-CORR-02, -03; SCN-CHQ-02; SCN-TF-04.

#### B2.4 Cash smuggling and couriers

*Evidence.* Cross-border cash is a monitored channel in the MER: the General Authority of Customs confiscated USD 304 million of undeclared
incoming funds in 2018 and small amounts since (Table 3.41); false-declaration cases such as a traveller carrying Saudi riyals and
Bahraini dinars worth about USD 171,000 (Box 3.28); a vehicle stopped carrying IQD 3.7 billion (USD 2.82 million) traced to salary-
disbursement "e-payment offices" with "many unjustified transfers between one another" (Box 4.8). The Office's declaration analysis
"revealed indicators of criminals exploiting some pilgrims, Umrah performers and persons accredited by Hajj and Umrah companies for
cross-border money transportations" (paragraph 366). OFAC's 3 July 2025 action says employees of an Iraq-based oil terminal "smuggle
hard currency into Iran via cars and trucks, some of which carry millions of dollars" [TSY-SB0188]. The CBI controls on cash imports
(chapter 12) cover the legal side: prior approval for each programme of foreign-cash imports (file 01 section 2d).
*Data signature.* Large cash withdrawals followed by no deposits, in border governorates or by customers in oil, logistics and exchange
sectors; USD cash purchases or withdrawals above declared needs; cash-intensive accounts of transfer offices; seasonal withdrawals
before Hajj and Umrah travel.
AKR-TMS hooks: SCN-CASH-12, SCN-CORR-03, SCN-FX-03.

#### B2.5 Trade-based laundering

*Evidence.* FATF-Egmont (December 2020) describes over-invoicing, under-invoicing, multiple invoicing, short and over shipping and
phantom shipments, and lists business indicators: "rapid growth of newly formed companies"; "consistent and significant cash payments,
including those directed towards previously unknown third-parties"; "unnecessarily complicated and complex supply chains"; "previously
established companies specialising in one sector that unexpectedly pivot into an entirely unrelated sector"; companies "simultaneously
involved in more than one unrelated sector"; and warns that "general trading companies that trade in a multitude of commodities" raise
false positives [FATF-EGMONT-2020]. The March 2021 risk-indicator note adds structural indicators (shell companies, mass registration
addresses, "a notable lack of typical business activities, e.g. it lacks regular transactions in line with the number of stated
employees, transactions relating to operating costs, tax remittances", nominee owners "who manage multiple companies") and says "some
of the risk indicators require the cross-comparison of various data elements (e.g. financial transactions, customs data, and open
market prices) often held in external sources", so "the private sector will not observe all of the indicators" [FATF-TBML-2021]. MENAFATF's own regional work says the same: its 2014 trends update found that "cash and checks are listed as the most misused tools", banks and
exchange companies the institutions most misused, with precious-metal dealers, real-estate agents, car dealers, shell companies, cross-border cash and
"decrease and increase (inflation) of invoices" recurring, and its LC study lists "opening accounts then requesting immediately an LC", "customer
activity inconsistent with the type of goods imported" and "significant discrepancies between the value of goods in the invoice and their actual and
fair value in the market" [MENAFATF-2014]. Iraq's
authorities flag "falsifying import invoices" and the Office's STR categories include "value of invoices" (MER paragraph 57 and table
5.2.2). The CBI's LC indicators (Art. 4) and Art. 3 nos. 4, 35 and 36 are the catalogue's version; the Wolfsberg-ICC-BAFT trade
principles add amended, extended or unused LCs, third-party funding of margins and connected applicant and beneficiary (file 04 section
2g).
AKR-TMS hooks: SCN-TRADE-01 to SCN-TRADE-13.

#### B2.6 Real estate

*Evidence.* The NRA rates real estate "high" for ML vulnerability (MER paragraph 47). Rule: when a property is worth more than "USD
380,000", the price must be deposited through a bank after enhanced due diligence, and the Real Estate Registration Department completes the
transfer only after the bank's notice (MER paragraph 195; IQD 500 million). Cases: an embezzler bought "a property in her daughter's
name worth $7 million" and collected "$1.2 million" in rent (Box 3.18); a director's five upscale Baghdad properties worth IQD 8.2
billion were frozen (Box 5.2); ISIL networks invested in "real estate, fish farming, car trading" (paragraph 391).
*Data signature.* Cash or transfers into an account followed by a payment to a developer, seller or registrar; payments above profile to
car dealers, jewellers and developers; related parties paying for the same property; purchases by relatives of PEPs.
AKR-TMS hooks: SCN-ACCT-08, SCN-PEP-02, SCN-PEP-04, SCN-PEP-07, SCN-LOAN-06, SCN-ISL-04; the USD 380,000 rule becomes SCN-ACCT-08's hard rule.

#### B2.7 Corruption and embezzlement of public funds (ghost employees, salary diversion)

*Evidence.* Corruption (فساد) is ranked first among ML threats; "ghost employees" are called الموظفون الفضائيون (al-muwazzafun al-fada'iyyun) in
Iraqi usage. About USD 2.6 billion was confiscated in 2018-2022, there were 15 ML convictions, and
proceeds were "laundered locally and largely abroad" (MER paragraphs 57, 58). MER table 3.33 lists the instrumentalities as "manipulation of
salary slips" and "exploitation of position". Box 3.18: IQD 47 billion (about USD 35.8 million) embezzled from a government bank
"using fraudulent notifications, manipulating the accounts and lists of retirees, and fictitiously disbursing these funds", with employees
convicted as co-offenders. Box 5.2: funds from "checks issued by one of the state departments". In October 2022 the Federal Commission of
Integrity opened an investigation into about IQD 3.7 trillion (USD 2.5 billion) allegedly taken from the General Commission for Taxes'
account at a state bank and transferred "to 5 companies in 2021 and 2022" [S: Wikipedia citing The National and Iraq Business News; not
verified against a primary source]. The Treasury action of 9 October 2025 describes sub-contracting to divert government-contract funds
[TSY-SB0277]. NPO cases show "fictitious salaries" (Boxes 3.13, 4.11, 4.12). The CBI's own 2024 guide on fraud and corruption in banks
lists, in annex 2, bank-side forms: "adding fictitious and ghost employees and amending payroll data", paying overtime for hours not
worked, "accepting a gift or bribe to facilitate procedures contrary to the laws", and "manipulating customers' data in the bank's
databases" [CBI-FRAUD-2024]. Salary localisation (tawtin, توطين الرواتب) pushes millions of public salaries into bank accounts (file 01 section 9), so
ghost-employee clusters become visible in bank data for the first time.
*Data signature.* Payroll accounts at one employer sharing phone, address, device or card collector; salary credited and cashed out by
a third party; state-account cheques or transfers to new companies followed by cash or foreign transfers; contractor-to-sub-contractor
chains; retiree accounts with duplicated identifiers.
AKR-TMS hooks: SCN-PUB-01 to SCN-PUB-07, SCN-PEP-01 to SCN-PEP-04, SCN-CHQ-02, SCN-INS-07.

#### B2.8 Fraud proceeds

*Evidence.* The MER records 2,481 fraud cases and USD 65 million confiscated, with fraud "through social media, whether in the field
of remittances or the creation of unlicensed companies for the purpose of providing loans and investment opportunities" (paragraph 57);
the Office received 92 fraud STRs in 2018-2022, 28 of them in 2022 (table 5.2.2). The MER also says the authorities claim no cybercrime complaints at all,
which it doubts (paragraph 59). The FATF, Interpol and Egmont report of 2023 gives account-level indicators in its annex A, among them:
"rapid or immediate cash withdrawals or transfers of large amounts following the receipt of a funds transfer in order to empty the
account"; "small payment to a beneficiary, which once successfully completed, is rapidly followed by larger value payments to the same
beneficiary"; "frequent changes of contact details"; "a single static IP address or electronic device associated with multiple accounts of
various account holders"; "multiple IP addresses ... associated with a single online account"; "transactions conducted with device time
zone mismatch"; "the user ... being instructed or coached during the transaction"; and "accounts relating to entities who could be
expected that they are no longer active in the jurisdiction (e.g., overseas students' account sold ...)" [FATF-CEF-2023].
AKR-TMS hooks: SCN-DIGI-01, -02, -11, -12, -13, -15; SCN-ACCT-11; SCN-XFER-01.

#### B2.9 Drug trafficking (crystal and Captagon)

*Evidence.* Iraq is "a drug transit country"; 17 tons were seized and about USD 60 million confiscated in 2018-2022, with "no
convictions for ML" (paragraph 57). The MER describes roles: "the supplier and agent play the primary role"; the distributor "purchas[es]
drugs from the agent and sell[s] them to the consumer through the courier"; "the most prominent narcotic substances traded locally
(crystal and Captagon) are characterized by their low prices". Drugs enter "through the Kurdistan Region" in a joint Iraqi-Syrian case
(Box 3.9). The CBI catalogue names constant travel to drug-producing countries (Art. 3 no. 9) and counterparties in drug-producing
countries (Art. 9 no. 2). I found no Iraq-specific laundering study for Captagon; the UK and US actions on Syrian Captagon networks
(Wikipedia summary) are not Iraq-specific, so the signature below is analysis [N].
*Data signature (analysis).* Many small cash deposits by different depositors into agent accounts, consolidated transfers to suppliers or
border-area counterparties, cash-intensive "shops" with turnover far above peers, vehicles and property bought in relatives' names.
AKR-TMS hooks: SCN-CASH-02, -03, -08; SCN-XFER-01; SCN-ACCT-08; SCN-FX-03; SCN-OBS-03.

#### B2.10 Terrorist financing: ISIL remnants and armed groups

*Evidence (ISIL).* The UN Monitoring Team's 38th report (S/2026/651, 10 August 2026): "ISIL (Da'esh) in Iraq was at its weakest", with a
strength "in the low hundreds of fighters"; in desert areas it was "generating an estimated $500,000 annually through extortion and
illegal taxation of shepherds and farmers", "enough to sustain basic operations, including payments to fighters and detained families"
(paragraphs 67-71); 5,704 ISIL-affiliated detainees were transferred to Iraq and 15,000-20,000 people from the closed Hawl camp are
unaccounted for (paragraph 57). The 37th report (S/2026/44) records operations "in Anbar, Diyala and Salah al-Din" and footholds "in the
Hamrin Mountains and western al-Anbar", and a financial network spanning Cote d'Ivoire, Madagascar and Togo that Iraq disrupted
(paragraphs 63-66, 109). The 36th (S/2025/482) gives the money pattern: salaries "reduced to $50-$70 per ISIL (Da'esh) fighter per month, with $35
per family", moved by hawala, cash, female couriers, cloud hawala and "safe drop boxes" at exchange offices (paragraphs 101-102). The
MER's cases: oil-product smuggling and "taxes imposed ... on shop owners and factories" (Box 4.2); a transfer office used to forward
funds from Europe through "a globally renowned money transfer company" with passports obtained by deception (Box 4.3); an exchange
office paying salaries through a card system in Anbar and "designated by the US Treasury Department" (Box 4.4); e-payment offices with
mismatching statements (Box 4.8); an exchange company with "records of funds transfers buried underground" (Box 4.10). Outflows from
Iraq for ISIL fell from USD 870,800 in 2018 to USD 196,000 in 2022 (Table 4.2).
*Evidence (other groups).* The MER says groups other than ISIL are mostly self-financed in cash from "personal donations out of
salaries", theft and ransom (paragraph 386); Box 4.1 shows a rocket attack with no financial transactions at all. On 17 September 2025
the US designated four further Iraqi militia groups as foreign terrorist organisations (file 05 section 1e), and OFAC's October 2025
action names two of the larger ones as clients of bank executives (above), and on 17 April 2026 OFAC designated seven commanders of four of them,
saying these groups are "siphoning Iraq's wealth to finance their terrorist activities" [TSY-SB0458]. Banks should therefore expect sanctions-evasion and
corruption patterns around these groups more than classic TF micro-flows (B2.11).
*What a bank can see.* Small recurring amounts to or from conflict-affected districts; many senders to one beneficiary; payouts from
exchange offices; "donations" to personal accounts (CBI circular of 26 August 2026, TF group); NPO accounts with foreign funding
inconsistent with the registered activity; no signal at all for self-financed attacks (Box 4.1), which must be stated openly.
AKR-TMS hooks: SCN-TF-01 to SCN-TF-07, SCN-CORR-02, -03, SCN-DIGI-09, SCN-OBS-06.

#### B2.11 Proliferation financing and Iran sanctions evasion (oil, dollar access, exchange houses)

*Evidence.* OFAC, 3 July 2025: a network "selling Iranian oil falsely declared as Iraqi oil since at least 2020", with ship-to-ship
transfers and blending, sold "to Western buyers via Iraq or the United Arab Emirates (UAE) as purely Iraqi oil using forged
documentation", with payments of kickbacks "in exchange for forged vouchers" [TSY-SB0188]. OFAC, 2 September 2025: another network that
"operates primarily by covertly blending Iranian oil with Iraqi oil" worth "around $300 million" a year by conservative estimate, using
vessel obfuscation including "AIS spoofing" and Marshall Islands shell owners [TSY-SB0233]. FinCEN FIN-2025-A002 (above) names exchange
houses and trading companies near Iran and gives oil red flags: documents "inconsistent with information contained in maritime database
entries", vessels with "recent or multiple name or flag changes", IMO numbers that "belong to a different vessel or to vessels that have
previously been scrapped", "bills of lading or shipping invoices, with no consignees". The CBI's chapter 9 PF indicators (file 01
section 2d) list dual-use goods, shared directors or addresses between trading companies, indirect payments, "declared value below
shipping cost", "vessel on a sanctions list", and sudden changes of board members or signatories. The MER rates PF sanctions implementation
(IO11) Low and notes the 1,480 km border and religious tourism with Iran (paragraph 57); the FATF action plan lists PF sanctions
evasion (file 01 section 1). UN sanctions on Iran were restored on 27 September 2025, with 43 individuals and 78 entities on the restored 1737 list (file 04 section 4b).
*Data signature.* Payments in the oil and petrochemical trade with doubtful origin; counterparties in UAE free zones and Hong Kong;
exchange-house chains; shared directors and addresses; goods and destinations out of line; USD cash withdrawals in border areas.
AKR-TMS hooks: SCN-PF-01 to SCN-PF-09, SCN-TRADE-07, -12, SCN-CORR-01, -05, -07.

#### B2.12 Mule networks in e-wallets and cards

*Evidence.* The FATF annex above and the UN note that Al-Shabaab "exploited unwitting third parties by paying them for identity
documents subsequently used to open bank accounts and conduct financial transfers on the group's behalf" (S/2026/651 paragraph 112)
describe the mechanism; the CBI circular of 26 August 2026 names the Iraqi version: "the same person accompanies several customers at
different times for similar transactions" (a money-mule "herder" pattern). The CBI catalogue has several card and wallet indicators
(Art. 3 nos. 21, 37, 39-41, 46; Art. 7 nos. 1, 3, 6-10; Art. 8 nos. 1-5). The CBI banned the use of cards and e-wallets for crypto
speculation (warning of November 2022, decision of March 2023; file 01 section 2b). Iraq has 17 e-payment companies, only four of which
offer mobile payments, and bank accounts carry their flows (MER paragraph 109).
*Data signature.* New wallet or account with many small inbound payments from unrelated senders then rapid cash-out; many accounts per
device, phone or address; first-time large transfers after a device or phone change; foreign ATM cash-outs in USD funded by IQD cash.
AKR-TMS hooks: SCN-DIGI-01 to SCN-DIGI-15, SCN-XFER-08, SCN-OBS-03.

#### B2.13 Abuse of NPOs and charities

*Evidence.* MER: 16 NPOs misused for TF in 2018-2022, "most ... received suspicious foreign funding or were active in conflict areas that
fell under the control of ISIL" (paragraph 51); the NGOs Department filed 11 STRs, all referred to prosecution (paragraph 395); the
Office's "Manual on Indicators of suspected ML and TF related to NPOs Transactions" "requires updating" (paragraph 412). Box 4.12:
"substantial amounts were transferred to the account of the NPO from five external organizations, which were not in line with its stated
activities", "fake loans", "fake salaries", "amounts for delegation purposes to members", and a hit on the CBI blacklist. Box 3.13 and Box
4.11: "fictitious salaries to members of the Board". FATF's 2014 typology groups NPO abuse into diversion of funds (by insiders and
outsiders), affiliation with a terrorist entity, abuse of programming, support for recruitment and false representation or sham NPOs
[FATF-NPO-2014]. The CBI circular lists "using personal accounts to receive donations for 'charitable or humanitarian' purposes without
a licence or institutional framework".
AKR-TMS hooks: SCN-TF-02, SCN-TF-03, SCN-ISL-07 (file 24 IS-C pack).

#### B2.14 Zakat and religious donations

*Evidence.* File 24 section 4 sets out the flows (zakat, زكاة; sadaqa, صدقة; waqf, وقف; the bank's own charity fund) and the seasonality: Ramadan, Eid,
Muharram and Arbaeen peaks. The MER notes ISIL's financial staff included collectors of zakat (paragraph 409). FATF's 2026 report shows
donation campaigns used to dress up settlement between underground bankers (box 4). The TF risk is real but narrow; the main risk for a
monitoring system is false positives in the religious calendar. Evidence on abuse of religious donations in Iraq specifically is thin; I
found no regulator or FIU document that quantifies it (Unverified).
AKR-TMS hooks: SCN-ISL-07, SCN-TF-02, calendar baselines (file 21 section 3c; file 24 section 4e).

#### B2.15 Kurdistan-specific flows

*Evidence.* The MER says understanding of ML risk at sub-national level, "specifically in the Kurdistan Region", is limited (paragraph 52), and
that the NRA "considered, to a limited extent, the particularity of the Kurdistan Region" (paragraph 59). Facts: 361 of 743 human-
trafficking investigations (48 percent) were in the Region (paragraph 57); drug transport into Iraq "takes place through the Kurdistan
Region" in the joint case of Box 3.9; the Office received 671 reports from FIs with a presence in the Region in 2018-2022 (paragraph 239);
the Region "is characterized by the development of communications infrastructure and other technological services" (paragraph 172) and
runs its own, stricter NPO supervision (paragraph 412). Public salaries moved to bank accounts in 2025-2026 (MyAccount, file 01 section 9).
One Erbil-based Islamic bank was on the CBI's dollar-restricted list (file 01 section 4).
*Data signature.* Branch region flag; Kurdish-script names (file 22); salary-localisation accounts; drug-transit counterparties; human-
trafficking indicators in small remittances to young customers.
AKR-TMS hooks: region as a segment and a geography factor in every family; SCN-PUB-01, SCN-XFER-11, SCN-CASH-03, SCN-DIGI-01; name
normalisation for Kurdish letters.

#### B2.16 Human trafficking and exploitation

*Evidence.* The MER ranks the offence eleventh by proceeds but counts 743 investigations (prostitution 371, forced child labour 131,
begging 108, foreign-labour trafficking 51, forced marriage 51, organ selling 31) and says credible sources see it as a major threat,
especially in the Kurdistan Region (paragraphs 57, 59, 148). The CBI catalogue has six indicators that relate to it: young customers
with unclear purpose (Art. 3 no. 24), repeated small transfers via agents (nos. 28, 45), negative open-source information on
sexual exploitation or marriage of minors (no. 30), term deposits out of proportion to age (no. 42), and deferred-dowry cash followed by
fragmented transfers (no. 49). The MER mentions bank guidance with human-trafficking indicators (file 01 section 2). The gendered wording of
nos. 28 and 45 is a design risk: AKR-TMS implements them on profile mismatch, never on gender (B3, B10 and file 29 section 9i).
AKR-TMS hooks: SCN-ACCT-09, SCN-XFER-11, SCN-CASH-14, SCN-OBS-03.

**What B2 shows.** The best-evidenced Iraqi typologies are the ones foreign authorities have written down about Iraqi banks (dollar
access, front companies, Iran oil, militia-linked companies, bank insiders), followed by the MER's corruption, drug, TF and NPO cases.
Real estate, Kurdistan flows, zakat abuse and Captagon laundering rest on thin public evidence, so their scenarios carry the [N] grade and
must be validated on each bank's own cases.

#### B2.17 Typology-to-scenario index

The table lists, for each typology, the library scenarios built to catch it (family and scenario numbers, for example CASH 01, 02 means SCN-CASH-01 and SCN-CASH-02).

| Typology | Scenarios (family and numbers) | Count |
|---|---|---|
| 1 Dollar and currency-auction abuse; over-invoiced and fictitious imports | CASH 07; XFER 04, 07; TRADE 01, 02, 10; ISL 01, 02; DIGI 07; ACCT 11; PEP 03; CHQ 04; FX 01, 02 | 14 |
| 2 Front companies | XFER 08, 13; TRADE 05, 09, 11, 12; ACCT 04, 13; PEP 04; PUB 05; CORR 07; PF 02, 03, 09 | 14 |
| 3 Hawala and unlicensed transfers through bank accounts | CASH 01, 02, 03, 05, 06, 08, 10, 14; XFER 01, 02, 03, 04, 05, 09, 11, 12, 13; TRADE 08; LOAN 03; PEP 07; CORR 01, 02, 03, 08; TF 04, 05; PF 06; CHQ 01, 02, 03, 06; FX 02 | 32 |
| 4 Cash smuggling and couriers | CASH 06, 12; CORR 03; FX 03; OTH 01 | 5 |
| 5 Trade-based laundering | TRADE 01, 03, 04, 05, 06, 07, 08, 09, 10, 11, 13; ISL 01, 02 | 13 |
| 6 Real estate | ISL 04; LOAN 06; ACCT 08; PEP 07 | 4 |
| 7 Corruption and embezzlement of public funds | CASH 01, 09; TRADE 06; ISL 08; ACCT 08; PEP 01, 02, 03, 04, 05, 06, 08; PUB 01, 02, 03, 04, 05, 06, 07; CHQ 02 | 20 |
| 8 Fraud proceeds | CASH 04, 05, 10; XFER 01, 05; DIGI 01, 02, 03, 10, 11, 12, 15; ACCT 05, 07; CHQ 05 | 15 |
| 9 Drug trafficking | CASH 01, 02, 03, 07, 08, 09; XFER 06; DIGI 14; CHQ 01; FX 03; OBS 05 | 11 |
| 10 Terrorist financing | XFER 02, 06; ISL 07; DIGI 09, 10; PEP 08, 09; CORR 02, 03; TF 01, 02, 03, 04, 05, 06, 07; OBS 06 | 17 |
| 11 Proliferation financing and Iran sanctions evasion | CASH 12; XFER 06, 07; TRADE 03, 12; DIGI 03; PEP 09; CORR 01, 04, 05, 06, 07; TF 06; PF 01, 02, 03, 04, 05, 06, 07, 08, 09 | 22 |
| 12 Mule networks in e-wallets and cards | CASH 03, 04, 05, 08; XFER 01, 03, 08; DIGI 01, 02, 04, 05, 07, 08, 12, 14; ACCT 04, 05, 06, 11; CORR 08; OBS 03 | 21 |
| 13 Abuse of NPOs and charities | ISL 07; TF 02, 03 | 3 |
| 14 Zakat and religious donations | ISL 07; TF 02 | 2 |
| 15 Kurdistan-specific flows | PUB 01 | 1 |
| 16 Human trafficking and exploitation | CASH 14; XFER 11; ACCT 09; OBS 03 | 4 |
| 17 Layering and integration through credit, securities, insurance and lockers | ISL 03, 05, 06, 08; LOAN 01, 02, 03, 04, 05, 06; OTH 01, 02, 03, 04 | 14 |
| 18 Insider and staff abuse | INS 01, 02, 03, 04, 05, 06, 07, 08, 09; DORM 06; OBS 04 | 11 |
| 19 Dormant, abandoned and low-tier accounts | ACCT 10; DORM 01, 02, 03, 04, 05, 06 | 7 |
| 20 Customer behaviour and avoidance of due diligence | CASH 13; XFER 10; LOAN 07; ACCT 07, 12; OBS 01, 02, 03, 04, 05, 06, 07, 08 | 13 |
| 21 Profile deviation (expected versus actual) | CASH 11; DIGI 06, 13; LOAN 08; ACCT 01, 02, 03, 08, 09, 13; PEP 06; PUB 07; FX 04 | 13 |

Every scenario has at least one typology tag; 161 scenarios carry 256 tags in all.

Typologies 13 to 16 have few dedicated scenarios because the public evidence is thin (B2). Kurdistan (15) is handled as a region segment and a geography factor in every
family, plus the payroll cluster scenario, rather than as a separate scenario family; a bank with branches in the Region should add its own region-specific
scenarios in the bank extension namespace once it has cases.

### B3. What a read-only replica can and cannot see

AKR-TMS reads the bank's CBS database only, through a read-only connection (file 25 section 1). That is enough for the ledger,
accounts, customers and KYC fields, and not enough for everything a scenario would like to know. The table grades each data domain; the
grade is shown next to every scenario and drives the data-readiness view that file 27 Part A already asks for ("show the bank which ones
its data feed can and cannot support").

| Data domain | Where it usually lives | Reachable by a read-only database link? | Grade | Scenarios that depend on it |
|---|---|---|---|---|
| Ledger lines, balances, account and customer master, signatories, holds, status history | CBS core tables | Yes, in minutes (polling or CDC; file 25 sections 2-9) | A | Most CASH, XFER, ACCT, DORM, CHQ, PUB |
| KYC-declared values (expected income, cash limit, turnover, source of funds, occupation) | CBS KYC form fields, often free text or empty | Yes, but completeness varies; a missing value must not read as "low" (file 20 L-10, L-44) | A or B | Art. 1 nos. 14-17 and every [KYC]-based parameter |
| Counterparty, narrative, purpose of transfer | Ledger narrative; transfer form (the CBI requires purpose on every outgoing transfer) | Partly: structured fields if the CBS keeps them, otherwise narrative text to parse | B | XFER, TRADE, TF |
| Teller module: cash denominations, depositor identity, cancelled or reversed entries, operator and authoriser IDs | CBS teller and audit tables | Depends on the CBS; many keep reversal and operator IDs, few keep depositor ID and denominations | B | CASH-07, -08, -13; INS; OBS-07 |
| Payment messages (SWIFT MT or MX, RTGS, ACH, switch) | Gateway or message tables; the SWIFT interface database itself is not read (file 25 section 12.4) | Via gateway views or the bank's message export | B | XFER-08; CORR-05 to -07; PF-04 |
| Trade module: LC, LG, collection, goods, HS code, vessel, ports, amendments | CBS trade module | Yes where a module exists; field quality varies | B | TRADE, PF-04, PF-05, LOAN |
| Cards and ATM: authorisations, MCC, terminal, country | Card processor or national switch database | Only if the bank grants a view | B or C | DIGI-05 to -10 |
| Digital-channel logs: device ID, IP, geolocation, login events | Mobile and internet banking databases | Optional read-only link; often a separate system | C | DIGI-02, -03, -11; PF-09 |
| Staff, HR, leave, rotation, relatives of staff | HR system | Not in the CBS; uploaded file or optional link | C | INS-05, -06, -07 |
| Customs declarations, company registry (CRD), real-estate registry, NGO registry, civil registry (deaths), FCOI declarations | State bodies | Files or APIs, some through the AML/CFT Office | C | TRADE-01, -12; PEP-07; PUB-06; TF-03; TRADE-04 |
| Sanctions, PEP and adverse-media lists | Commercial and official feeds | Ingested by AKR-TMS (files 14, 22) | C | PF, PEP, TF screening-linked scenarios |
| Branch observation log (what the teller saw) | Created by AKR-TMS (file 31, file 28) | Not applicable: new data, entered by people | D until the log is live | OBS family and Art. 3 nos. 6, 11, 25; Art. 12 |
| Authority and FIU requests, internal reports | Created or imported in AKR-TMS (file 28) | Same | C | OBS-06; TF-07; PEP-08 |

**What a read-only replica cannot do or see.**

1. **Stop anything.** The CBI wants a transaction that matches a scenario held until the MLRO clears it (chapter 5, article 5). Phase 1
   detects within minutes and cannot interdict; the optional phase-2 pre-release API can (file 25 section 15). Scenarios suited to a hold
   are marked P2 below: hard rules with a CBI page behind them, sanctions and designation hits, incomplete originator data, prohibited
   Islamic structures, FX eligibility rules.
2. **See flows that never touch the bank.** Hawala settlement, cash carried by couriers, offsetting through trade and gold are visible only
   at their edges: the cash that was deposited, the transfer that was ordered, the account that was used as a collection point.
3. **See other banks.** A customer's accounts elsewhere are invisible; the pass-through pattern shows only its own half.
4. **Verify physical reality**: whether goods exist, whether the customer is nervous, whether a companion holds the ID. These need customs
   data, field visits, or the observation log.
5. **Know the true owner** where the CBS holds only what the customer declared; the MER says banks "rely mainly on the customer's
   statement" for beneficial ownership (file 01 section 2). Ownership graphs help only inside the bank's own data.
6. **Tell a reversal from a real event, or a late entry from a missing one** unless the status lifecycle and the late-data rules of file
   25 sections 6-7 are implemented: scenarios must count posted events, net reversals, re-evaluate at the settled time and amend alerts
   with an explanation.
7. **Use protected attributes.** Gender, religion, ethnicity and nationality must not drive a threshold. Several CBI indicators are written
   in terms of women or young customers (Art. 3 nos. 24, 28, 42, 45); AKR-TMS implements them as profile-mismatch scenarios that use age
   band and declared occupation only where the CBI text names them, with a fairness report (file 29 section 9i).

**Silent-failure guard.** Every scenario declares the canonical fields it reads. After mapping, the coverage matrix of file 25 section 13.6
marks it operable, degraded (some field missing, loss stated) or blocked. The daily reconciliation of file 20 (L-04 to L-09) and the
reachability and dead-rule tests (L-20) apply to every scenario in this library: TD Bank and Metro Bank both failed because transaction
types never reached the scenarios (file 20 sections 2.1 and 6.3).

AKR-TMS hooks:
- A **data-readiness view per scenario and per bank** (grades A to D, with the missing field named and the vendor ask written out).
- A **scenario-input registry**: field, mapping version, null rate, last validation date; a scenario whose input drops below its minimum
  completeness is shown "not operating as intended" on the compliance dashboard.
- **KYC-completeness handling**: where a [KYC] parameter has no declared value, the scenario falls back to the [PCT] segment threshold and
  flags the customer "degraded" (file 20 L-44), never to a high threshold.
- **Phase-2 candidate flag** on each scenario so the bank can see which rules it could enforce before release once the API is installed.

### B4. Family CASH: cash deposits, withdrawals, structuring and denominations

Cash is the NRA's basic method ("cash was largely used when executing financial transactions", MER paragraph 48) and the CBI's article 1
has 21 scenarios. Library rule: every cash scenario aggregates by customer and by linked parties (family number, shared phone, shared
address, same depositor), never by single transaction, because a rule that looks at single deposits fails against structuring (HSBC, file
20 FM-08). Reporting at USD 10,000 (cash transaction reports to the Office) is a separate automatic duty; CASH scenarios start below it.

**Table 1: what each CASH scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-CASH-01 | Large cash deposit against profile (cash placement). Typologies: 3, 7, 9. | Implements Art.1 nos. 1, 14. Extends Art.1 no. 17; Art.3 nos. 26, 31 | A cash deposit, or the same-day total of cash deposits across all accounts of the customer and linked parties, exceeds both an absolute floor and a profile ceiling | transaction (cash flag, amount_usd, booking_date), customer expected profile, party_relationship, segment |
| SCN-CASH-02 | Structuring: repeated cash deposits just under the limit (placement, smurfing). Typologies: 3, 9. | Implements Art.1 nos. 2, 12. Extends Art.1 no. 4. Also: circular of 26 Aug 2026 group 1 | At least N cash deposits in W days, each below the reporting or customer-usual size, whose total exceeds the reporting limit or the profile; counted across accounts, branches, depositors and linked parties; severity rises after a limit enquiry logged by staff | transaction, cash_event, party_relationship, branch, observation log (optional) |
| SCN-CASH-03 | Cash accumulation across branches, accounts or depositors (placement through many hands). Typologies: 3, 9, 12. | Implements Art.1 nos. 3, 4; Art.3 no. 5; Art.12 no. 15. Extends Art.3 no. 13 | Cash reaching one customer or group through at least 3 branches, 3 accounts or 3 unrelated depositors within a short window; branch far from the customer's home and work | transaction, branch (governorate), party_contact (address), account_party_role |
| SCN-CASH-04 | ATM cash: limit-hugging and avoidance of staff. Typologies: 8, 12. | Implements Art.1 no. 5; Art.12 no. 1 | ATM cash withdrawals at or near the daily card maximum on many days, or repeated cash-machine deposits above profile, with no branch visits | card, card_transaction, channel, account |
| SCN-CASH-05 | Rapid cash turnover (layering by speed). Typologies: 3, 8, 12. | Implements Art.1 no. 7. Extends Art.3 no. 3 | At least 80 percent of a cash deposit leaves the account within 48 hours, repeated | transaction (sequence), balance_snapshot |
| SCN-CASH-06 | Cash-funded outward transfer, including walk-ins (placement then movement). Typologies: 3, 4. | Implements Art.1 no. 10; Art.2 nos. 1, 7. Extends Art.3 no. 3 | A cash deposit by the holder or a third party followed within 24 hours by an outward domestic or foreign transfer, or a walk-in customer paying cash for a transfer; linked occasional transactions summed | transaction, payment_message, party (occasional), cash_event |
| SCN-CASH-07 | Denomination and counterfeit anomalies (placement of illicit notes; counterfeit IQD). Typologies: 1, 9. | Implements Art.3 nos. 2, 12. Also: circular group 3 | Small notes exchanged for large, deposits dominated by worn or stored notes, any counterfeit find, in a customer whose sector does not explain it | cash_event (denominations, counterfeit flag) |
| SCN-CASH-08 | Third-party cash depositors (use of others' accounts, mules). Typologies: 3, 9, 12. | Implements Art.3 no. 44. Extends Art.13 no. 2 | Share of deposits made by persons other than the holder or signatories above the segment norm, or many distinct unrelated depositors | cash_event (depositor ID), operator_id, account_party_role |
| SCN-CASH-09 | Cash intensity mismatch (cash for a non-cash job). Typologies: 7, 9. | Implements Art.1 no. 14; Art.3 no. 31. Extends Art.3 nos. 26, 29 | Cash share of inflows far above the segment norm or the declared profile where the occupation is not cash-intensive | transaction, customer (occupation or ISIC), expected profile |
| SCN-CASH-10 | Cash-out after an inbound transfer (pass-through to cash). Typologies: 3, 8. | Implements Art.2 no. 1. Extends Art.1 no. 7; Art.2 no. 12 | An incoming domestic or foreign transfer followed within 48 hours by a cash withdrawal of most of it, especially where the instruction said "pay in cash" | transaction, payment_message (instruction, originator) |
| SCN-CASH-11 | Cash withdrawal above declared limit or history. Typologies: 21. | Implements Art.1 no. 16. Extends Art.1 no. 14 | Cash withdrawals above the KYC-declared limit or far above the customer's own recent maximum | transaction, customer expected profile |
| SCN-CASH-12 | Large cash withdrawals in border governorates and in oil, logistics and exchange sectors (cash smuggling, USD leakage). Typologies: 4, 11. | Extends Art.1 no. 14; Art.3 no. 39. Also: 2025 controls ch. 2 art. 2 (branches in border areas) | Large cash withdrawals with no refill or card spending by customers of border-area branches or in sectors tied to cross-border trade | transaction, branch (border flag), customer (ISIC), card_transaction |
| SCN-CASH-13 | Reduction or cancellation after a CDD prompt (restructuring). Typologies: 20. | Implements Art.1 nos. 8, 12. Also: circular groups 1 and 6 | A cash transaction is cancelled, split or reduced after the teller asks for ID or source of funds, or the customer returns with a smaller or split amount | transaction_event (cancelled, amended), teller note or observation log |
| SCN-CASH-14 | Cash justified by a document, then fragmented transfers (deferred dowry, المهر المؤخر al-mahr al-mu'akhkhar; inheritance or sale papers). Typologies: 3, 16. | Implements Art.3 no. 49. Extends Art.3 nos. 3, 45 | A large cash deposit supported by a document (deferred dowry, inheritance, property sale) followed within days by many small transfers to unrelated persons | cash_event, kyc_document (source-of-cash document), transaction, counterparty |

**Table 2: how each CASH scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-CASH-01 | Floor USD 5,000 a day (half the USD 10,000 reporting anchor) [CBI-DP]; ceiling = larger of 40 percent of declared monthly cash turnover [KYC] and segment P99 of daily cash deposits [PCT after 90 days]; salaried and professional segments: 2 times median monthly salary credit [KYC]; PEP and EDD customers use half the floor | Salaried, pensioner, student, professional, small trader, wholesale trader, contractor, NPO; IQD and USD separate | Same day across accounts; also 7-day rolling. NRT, recomputed at EOD | Eid and Ramadan, harvest, school-fee and wedding seasons, documented property sales, trader weekly collections, salary arrears | Deposit slip, depositor ID, declared source of cash, declared turnover beside the amount, 12-month cash chart with seasonal band, other-branch deposits the same day |
| SCN-CASH-02 | At least 3 cash deposits in 7 days and total at least USD 10,000 [CBI anchor]; just-below band 70 to 99 percent of USD 10,000 twice in 14 days [DP]; or total above 150 percent of declared monthly cash [KYC] | All segments; thresholds per currency. Exclusion only by a documented profile (for example a cash retailer with a daily float) | 7 and 30 days; NRT and EOD | Traders who deposit daily below the float, several outlets of one company, charity collections, cash payroll | List of deposits with times, tellers, branches, depositor IDs; limit-enquiry flag; declared cash-in-hand business; whether a cash report was filed |
| SCN-CASH-03 | 3 or more branches in 3 days, or 3 or more unrelated depositor IDs in 30 days, total at least USD 5,000 [DP]; branch governorate differs from residence and work governorate [CBI: 2025 controls ch. 2 "branch far from home or work"] | Individuals and SMEs; documented multi-outlet companies whitelisted with expiry | 3 and 30 days; EOD | Head-office accounts fed by outlets, families depositing for one person, tuition payments | Map of deposits by branch and time, depositor identities and recorded relationships, distance from home |
| SCN-CASH-04 | At least 5 days in 10 at 90 percent or more of the card daily maximum [DP; basis Art.8 no. 2]; cash-machine deposits on 3 days in 7 above SCN-CASH-01 ceiling | Debit, prepaid and credit products; salary accounts excluded when withdrawals follow pay day (profile) | 10 and 30 days; EOD | Salary accounts emptied on pay day, low-limit products, travel, machine shortages | ATM IDs, places and times, card limit, balance path, branch visits in 90 days, CCTV request |
| SCN-CASH-05 | At least 80 percent of a deposit out within 48 hours [DP; basis file 21 section 16g, FATF cyber-fraud annex A]; minimum USD 3,000; at least 3 occurrences in 30 days | All; exclude cash-in-transit and PSP settlement accounts by profile | 48 hours and 30 days; NRT | Merchants paying suppliers, cash payroll, rent | Ledger path, balances before and after, destinations of the outflow |
| SCN-CASH-06 | Transfer at least USD 3,000 [MER: exchange companies' own threshold about USD 3,000]; linked occasional transactions at least USD 10,000 in 30 days [CBI: Instructions 1/2023 art. 2/4]; cash share of funding at least 70 percent [DP] | Individuals, SMEs, walk-ins reported separately | 24 hours and 30 days; NRT; P2 candidate for walk-in wires with incomplete data | Tuition and medical payments abroad, importers paying suppliers from cash sales | Deposit slip and depositor, transfer form with purpose, supplier invoice, ID copy |
| SCN-CASH-07 | Any counterfeit find: alert [DP]; lowest two denominations at least 60 percent of deposit value and total at least USD 3,000 for non-cash-intensive segments [DP] | Cash-intensive retailers and transport operators have their own bands | Same day and 30 days; NRT | Market traders, bus and taxi operators, charity boxes | Denomination breakdown, counterfeit report number, sector, packaging notes from the observation log |
| SCN-CASH-08 | At least 4 distinct unrelated depositor IDs in 30 days [DP]; third-party share above segment P95 [PCT], 50 percent floor during bootstrap | Individuals, NPOs, small traders | 30 days; EOD | Family members paying into one account, rent and tuition, company agents | Depositor IDs and relationship records, slips, depositor ID capture rate for the branch |
| SCN-CASH-09 | Cash share of 90-day inflows above 50 percent where segment P90 is below 10 percent [DP]; cash above 3 times median monthly salary for salaried staff [KYC] | By occupation or ISIC; cash-intensity table by sector, replaced by [PCT] | 90 days; weekly | Side businesses, rent, family support, sale proceeds | Occupation, employer, salary history, cash history, declared source |
| SCN-CASH-10 | Inbound at least USD 3,000 [DP]; at least 80 percent withdrawn in cash in 48 hours; foreign-origin inbound counts 1.5 times [DP] | Individuals, SMEs; exclude agreed settlement flows | 48 hours; NRT; P2 candidate when the instruction says cash payout | Remittances to families, emergencies, wages of foreign workers | Inbound message (originator, purpose), withdrawal slip, ID shown |
| SCN-CASH-11 | Above 100 percent of declared monthly cash-withdrawal limit [KYC]; or above 3 times the trailing 90-day P95 and at least USD 3,000 [DP] | All individuals and SMEs with a declared limit | Calendar month and 90 days; NRT | House purchase, treatment, wedding, Hajj | Declared limit, withdrawal history, ID, stated purpose |
| SCN-CASH-12 | At least USD 10,000 a week for non-cash-intensive customers and USD 50,000 for oil, logistics and exchange sectors with no refill or card spending [DP]; border list from the bank's own risk assessment (MER paragraph 41: six neighbours) | Border-governorate branches; exchange, e-payment and oil-service customers | 7 days; EOD | Cross-border traders, pilgrim season, contractor payroll | Customer profile, branch region, counterparties, cash source, prior deposits |
| SCN-CASH-13 | Any cancelled or reduced event of at least 50 percent after a CDD prompt flag, or 2 cancellations in 30 days [DP] | Teller channels; needs the transaction-event log | Same day and 30 days; NRT | Teller errors, wrong denomination counts | Event log, teller note, CCTV request |
| SCN-CASH-14 | Cash deposit of at least USD 10,000 [CBI anchor] with a source document recorded, then at least 5 transfers to at least 3 unrelated persons within 14 days totalling at least 70 percent of the deposit [DP] | Individuals | 14 days; EOD | Genuine family distribution of inheritance or sale proceeds | Document copy and issuer, court or notary reference, transfer list, relationships |

AKR-TMS hooks:
- **Cash feature library** (file 21 section 16g): cash share, cash velocity, third-party depositor share, branch dispersion, round-amount
  share, cash relative to declared turnover; each scenario above reads these features, so a new cash scenario is a configuration.
- **Linked-party aggregation service** (family number, phone, address, depositor) shared by CASH, XFER and PUB; it is the answer to the
  CBI's "through others" wording (Art. 1 no. 4) and to the Aug 2026 circular on companions who accompany several customers.
- **Seasonal calendar** (Eid, Ramadan, Muharram, Arbaeen, harvest, school fees, Hajj and Umrah): baselines are seasonal; a seasonal peak is
  not an alert until it passes the seasonal band (file 24 section 4e).
- **Depositor-ID capture rate** per branch shown on the dashboard; where it is low, SCN-CASH-08 runs degraded and says so.

### B5. Family XFER: transfers and wires, domestic and cross-border

The CBI's article 2 has 16 transfer scenarios; the MER describes the national pattern as "external flows" (paragraph 627) and the CBI's
2023 transfer system makes sure "no transfer can be processed unless all required data is complete" (file 01), which leaves accuracy,
pattern and relationship questions for the bank. Library rule: a transfer scenario reads the counterparty as a resolved party (file 25
`counterparty`, `payment_message_party`), not as a text string, and treats domestic and foreign transfers alike (Instructions 1/2023 art. 28).

**Table 1: what each XFER scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-XFER-01 | Many unrelated senders or recipients (collective accounts, mules, hawala collection). Typologies: 3, 8, 12. | Implements Art.2 no. 2; Art.3 nos. 45, 48. Extends Art.7 no. 6. Also: FATF 2013 hawala indicators 1 and 11 | An account receives from, or pays to, many counterparties with no prior relationship in a month, then consolidates the funds outward | transaction, counterparty (first-seen date), payment_message_party |
| SCN-XFER-02 | Equal or near-equal transfers to several beneficiaries (smurfed remittance, payroll to fighters, cuckoo smurfing). Typologies: 3, 10. | Implements Art.2 nos. 3, 10 | At least 3 transfers in 7 days with identical or near-identical amounts to at least 3 beneficiaries, including in different countries, or to one beneficiary across several accounts | transaction, counterparty, payment_message |
| SCN-XFER-03 | Pass-through account: matched inflow and outflow (layering). Typologies: 3, 12. | Implements Art.2 nos. 9, 12. Extends Art.13 no. 4. Also: MAS 2018 case study B | Inflows matched to outflows by amount within a tolerance and 72 hours, running balance near zero, repeated | transaction (sequence), balance_snapshot |
| SCN-XFER-04 | Local transfer then foreign transfer in another currency (layering across borders). Typologies: 1, 3. | Implements Art.2 nos. 4, 5 | A domestic inflow followed within days by an outward foreign transfer, in USD or another currency, of most of its value | transaction, payment_message, fx_deal |
| SCN-XFER-05 | Incoming foreign transfers just below USD 10,000 (structuring of remittances). Typologies: 3, 8. | Implements Art.3 nos. 15, 48. Extends Art.2 nos. 5, 8 | At least 3 incoming foreign transfers in 30 days each between 70 and 99.9 percent of USD 10,000, to one account or linked accounts | payment_message, party_relationship |
| SCN-XFER-06 | High-risk geography exposure. Typologies: 9, 10, 11. | Implements Art.2 no. 11; Art.3 nos. 16, 34; Art.7 nos. 2, 4. Extends Art.9 no. 2. Also: 2025 controls ch. 8 | Transfers to or from jurisdictions on the bank's versioned high-risk table (FATF black and grey lists, CBI-circulated lists, banking-secrecy, drug-producing and conflict-adjacent countries) or flagged regions inside Iraq | counterparty (country), payment_message, country_risk table |
| SCN-XFER-07 | Incomplete, inconsistent or substituted identity data on transfers. Typologies: 1, 11. | Implements Art.2 nos. 14, 15, 16. Also: 2025 controls ch. 5 art. 4; Instructions 1/2023 arts. 28-30; MER box 5.1 | Missing or placeholder originator and beneficiary fields; one ID with different names or dates; one person with two documents; different documents per transfer | payment_message_party, party_identifier |
| SCN-XFER-08 | Shared identifiers among unrelated customers or counterparties (mule rings, front companies). Typologies: 2, 12. | Implements Art.2 no. 14; Art.3 nos. 21, 33. Extends Art.12 no. 13 | Unrelated parties sharing phone, address, ID number, email, device or introducer; a cluster of at least 3 | party_contact, party_identifier, party_relationship |
| SCN-XFER-09 | Repeated transfers to one beneficiary with unverifiable purpose. Typologies: 3. | Implements Art.1 no. 19; Art.3 no. 43. Extends Art.2 no. 8 | At least 5 transfers in 30 days to one beneficiary (local bank or abroad) with no recognised relationship, or a purpose text that does not fit the relationship | transaction, counterparty, narrative, purpose code |
| SCN-XFER-10 | Dealings with parties previously reported. Typologies: 20. | Implements Art.3 no. 27 | Any transfer to or from a party that was the subject of an earlier internal report or STR at this bank, or on its internal watchlist; compliance-only visibility | internal report and STR register (file 28), counterparty |
| SCN-XFER-11 | Remittance-network receipts out of profile. Typologies: 3, 16. | Implements Art.3 nos. 28, 45 (without gender) | Repeated receipts via global remittance networks from several senders or countries, not explained by the customer's age band and occupation | transaction (remittance channel), counterparty, customer profile |
| SCN-XFER-12 | Walk-in and occasional customers: linked transactions. Typologies: 3. | Implements Art.2 no. 16. Also: 2025 controls art. 3 (SDD barred when linked transactions stay under a threshold); Instructions 1/2023 art. 2/4 | Linked occasional transfers reaching USD 10,000 within 30 days, or several just under the IQD 1 million simplified-due-diligence ceiling, by one ID, phone or companion | party (occasional), payment_message, party_identifier |
| SCN-XFER-13 | Business account without business activity (hawala settlement, front company). Typologies: 2, 3. | Implements Art.13 no. 4. Extends Art.3 no. 13. Also: FATF 2013 hawala indicator 8; FATF-Egmont 2021 structural indicators | A company or trader account with large turnover and no payroll, rent, utilities, tax or supplier-type payments, dominated by a few unrelated counterparties | transaction, counterparty classification, customer type |

**Table 2: how each XFER scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-XFER-01 | At least 8 distinct unrelated senders in 30 days for individuals, 15 for SMEs [DP]; at least 80 percent of senders new in 90 days; at least 70 percent of the inflow leaves within 7 days; total at least USD 2,000 | Individuals, sole traders, NPOs; exclude payroll accounts and documented collectors (rent, tuition) | 30 days; EOD | Crowd collections for weddings or funerals, online sellers, cooperative societies | Sender list with country and first-seen date, narratives, outflow destinations, declared occupation |
| SCN-XFER-02 | At least 3 transfers within 7 days with amounts within plus or minus 5 percent, to at least 3 beneficiaries; each at least USD 300 [DP; basis: the Office's STR quality form names "equal amounts transferred to several individuals in different countries", file 01 section 2] | All; exclude payroll batches and standing orders | 7 days; NRT | Family support sent to several relatives, salary of foreign workers | Transfer list, beneficiary relationships, countries, purpose texts |
| SCN-XFER-03 | Match tolerance plus or minus 10 percent within 72 hours [PEER: MAS 2018 case B]; matched pair at least USD 5,000 [DP]; at least 2 pairs in 30 days; monthly turnover at least 10 times average balance [DP] | Individuals, SMEs, corporates; exclude PSP, exchange and treasury accounts, which have their own scenario (CORR) | 72 hours and 30 days; NRT | Trading intermediaries with thin margins, agents, escrow-like accounts | Matched pairs with times and amounts, counterparties on both sides, balance path |
| SCN-XFER-04 | Inflow at least USD 5,000 equivalent; at least 70 percent sent abroad within 5 days in another currency; at least 2 times in 90 days [DP] | Individuals and SMEs | 5 and 90 days; EOD | Importers funding supplier payments, tuition and medical payments | Inflow source, outward message with purpose, goods or tuition documents |
| SCN-XFER-05 | Band 70 to 99.9 percent of USD 10,000 [DP; the number is written into Art.3 no. 15]; at least 3 in 30 days; aggregated across linked parties | All receivers of foreign transfers | 30 days; EOD | Fixed monthly remittances that happen to sit near the band | List with senders and countries, linked parties, profile of the receiver |
| SCN-XFER-06 | Sanctioned or embargoed corridor: any amount, hard rule, P2 candidate; other high-risk corridors: first-time corridor at least USD 1,000, or at least 3 transfers in 30 days, or at least 20 percent of the customer's flows [DP] | Geography table versioned with dates (FATF, CBI, UN, OFAC, EU, HM Treasury) | Event; NRT | Religious tourism and legitimate trade with neighbouring countries, students | Corridor history, purpose, counterparties, table version in force that day |
| SCN-XFER-07 | Any missing mandatory field: exception and P2 reject or hold [CBI: 2025 controls ch. 5 art. 4]; one ID with at least 2 names: alert; fields missing above 2 percent of a correspondent's or branch's messages [DP] | All messages | Event; NRT | Truncated names (35-character fields), Kurdish and Arabic spellings | Message fields, document copies, entity-resolution result |
| SCN-XFER-08 | Cluster of at least 3 customers sharing a phone, or at least 4 sharing an address, excluding landlords, branch and registry addresses [DP]; relatives (family number) shown but not alerted unless flows link them | All customers; company registry addresses excluded by list | Continuous graph; EOD | Families, employees of one company, shop-owner addresses | Graph with shared attributes, account openings, flows between members |
| SCN-XFER-09 | At least 5 transfers in 30 days to the same beneficiary with no relationship record; total at least USD 2,000 [DP] | Individuals and SMEs; exclude employees, landlords, utilities and suppliers on file | 30 days; EOD | New suppliers, informal landlords, family | Counterparty history, purpose texts, relationship record |
| SCN-XFER-10 | Any amount; no customer contact triggered by the alert itself | Restricted to compliance roles | Event; NRT | Common names matched by name only (use identifiers) | Prior report reference, identifier match, tipping-off reminder |
| SCN-XFER-11 | At least 5 receipts in 30 days and at least 3 distinct senders and total at least USD 1,500 [DP] | Remittance channels; age band and occupation only as profile context | 30 days; EOD | Expatriate families, students abroad | Sender list, countries, passports used (MER box 4.3), profile |
| SCN-XFER-12 | Linked occasional transfers at least USD 10,000 in 30 days [CBI]; at least 2 transfers in 14 days between 80 and 99 percent of IQD 1 million [CBI-DP] | Walk-in and occasional customers | 14 and 30 days; NRT | Remitting workers who send regularly | ID, phone and companion links, transfer forms |
| SCN-XFER-13 | No operating-cost debits (payroll, rent, utilities, tax, supplier types) for 90 days and monthly turnover at least USD 20,000 [DP] | Companies, sole traders | 90 days; WK | New businesses, holding companies, agents | Counterparty classes, payroll absence, owner links |

AKR-TMS hooks:
- **Resolved counterparties**: every transfer carries a resolved counterparty with first-seen date, country, bank and risk class so that
  "no relationship" and "new counterparty" are queries, not guesses.
- **Match engine** for pass-through (cumulative, tolerance-based, time-windowed) with unit tests that include the MAS case (USD 5 million in,
  out three days later, no alert in a sum-based design).
- **Country risk table** with FATF status, sanctions programmes, corruption score and border flags, versioned and effective-dated, so
  every XFER-06 alert shows which table version applied (file 01 section 2d implications).
- **Walk-in register** that aggregates occasional customers by ID, phone and companion across the day and the month.
- **P2 hooks**: SCN-XFER-06 (sanctioned corridor), SCN-XFER-07 (missing mandatory data) and SCN-XFER-12 are the first candidates for
  hold-until-cleared once the phase-2 API exists.

### B6. Family TRADE: trade finance, import payments and dollar access

Trade is where Iraq's two biggest external risks meet: the dollar-access abuse of B2.1 and the Iran-linked front-company flows of B2.2 and B2.11.
The CBI catalogue has 9 LC indicators (Art. 4), 4 guarantee indicators (Art. 5) and general scenarios on invoicing (Art. 3 nos. 4, 35,
36). Library rules: (1) compare onboarding knowledge with actual trade, as the Wolfsberg-ICC-BAFT principles ask ("an active exchange
between CDD information and transaction information", file 04 section 2g); (2) frame price checks as "manifestly unusual", not as a
pricing judgement; (3) treat any signal that needs customs or vessel data as grade C and say so; (4) the dollar-access rules are the CBI's own
2016 numbers, kept as configurable hard rules because their current status is not verified.

**Table 1: what each TRADE scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-TRADE-01 | Import payments above declared or customs volume (over-invoicing, phantom shipments). Typologies: 1, 5. | Implements Art.3 no. 36; Art.4 nos. 1, 2. Also: MER box 5.2 | Cumulative import payments and LC settlements exceed the declared annual import volume or the customs value, or have no customs entry | payment_message, trade_instrument, expected profile, customs data (optional) |
| SCN-TRADE-02 | Dollar purchase by a customer who does not meet the CBI preconditions (dollar-access abuse). Typologies: 1. | Extends Art.9 nos. 1, 3, 6; Art.3 no. 36. Also: CBI FX procedures 2016 | USD purchase or outward USD remittance by an account younger than six months, funded in the last seven days, above the per-person cap without source documents, above the remittance cap or count, or by several applicants sharing address, phone or director | transaction, fx_deal, payment_message, party_contact, account (open date) |
| SCN-TRADE-03 | Payment to a third-country or unrelated beneficiary, or a changed place of payment. Typologies: 5, 11. | Implements Art.4 nos. 6, 9; Art.3 no. 35 | Beneficiary or paying-agent country differs from the supplier's or the goods' origin, or the payment place or beneficiary is changed after issue | payment_message_party, trade_instrument |
| SCN-TRADE-04 | LC lifecycle anomalies. Typologies: 5. | Implements Art.4 nos. 3, 4, 8. Extends Art.3 no. 35. Also: Wolfsberg-ICC-BAFT principles | Repeated amendments or extensions, change of beneficiary, many cancelled or unused LCs, amounts in presented documents that differ from the originals | trade_instrument (amendments, presentations, settlements) |
| SCN-TRADE-05 | Connected applicant, beneficiary or shipper; third-party margin funding. Typologies: 2, 5. | Implements Art.4 nos. 5, 7 | Applicant and beneficiary or shipping company share an owner, address, phone or director; the margin is funded by a third party just before issue | trade_instrument parties, party_relationship, transaction |
| SCN-TRADE-06 | Guarantee anomalies (fictitious contracts, cash-backed guarantees, early calls). Typologies: 5, 7. | Implements Art.5 nos. 1, 2, 3, 4; Art.13 no. 11. Extends Art.6 no. 7 | Guarantees without an underlying contract on file, called soon after issue, backed by a third party's cash margin, or out of line with the applicant's size | trade_instrument (LG), kyc_document, facility |
| SCN-TRADE-07 | Price and value outliers (over- and under-invoicing). Typologies: 5. | Implements Art.3 no. 36; Art.4 no. 2. Also: CBI ch. 9 ("declared value below shipping cost"); file 24 IS-M3 | Unit price or declared value far from peer deals, customs or benchmark values, where goods code, quantity and value exist | trade_instrument (goods, HS, quantity, value), benchmark table |
| SCN-TRADE-08 | Cash transfers instead of letters of credit. Typologies: 3, 5. | Implements Art.3 no. 4 | A customer with an import profile pays suppliers almost only by open-account transfer, often cash-funded, where peers use LCs | trade_instrument, payment_message, cash_event |
| SCN-TRADE-09 | Goods inconsistent with the declared business. Typologies: 2, 5. | Implements Art.4 no. 1. Also: MER paragraph 57 (licences in names of people not active in trade); FATF-Egmont 2020 | Goods group outside the expected set for the customer's ISIC, or a first-time goods category above a value | trade_instrument (HS), customer (ISIC), sector-goods map |
| SCN-TRADE-10 | Payment without arrival evidence (phantom or short shipping). Typologies: 1, 5. | Extends Art.4 nos. 2, 8; Art.13 no. 11. Also: Wolfsberg principles; file 24 IS-M6 | Payment released with no transport document, arrival notice or customs entry within a period | trade_instrument (documents), customs data (optional) |
| SCN-TRADE-11 | New or fast-growing importers; companies incorporated together. Typologies: 2, 5. | Extends Art.3 nos. 13, 34. Also: FinCEN FIN-2025-A002; FATF-Egmont 2020 | A company under a year old with large import payments, or a cluster of companies incorporated within a month sharing address, director, signatory, phone or counterparties | customer (onboarding date), registry data, party_relationship, transaction |
| SCN-TRADE-12 | Free-zone and general-trading counterparties. Typologies: 2, 11. | Extends Art.3 nos. 34, 36. Also: FinCEN FIN-2025-A002; Banque Misr UAE notice | Large shares of outward payments to young general-trading companies in free zones with opaque ownership | counterparty (country, type, age, ownership), transaction |
| SCN-TRADE-13 | LC and guarantee volume out of line with activity. Typologies: 5. | Implements Art.4 no. 4; Art.5 no. 1. Extends Art.4 no. 5; Art.5 no. 2 | Issuance count or value in a quarter far above the customer's own history, capital or turnover, or backed by financial guarantees out of line | trade_instrument, facility, customer financials |

**Table 2: how each TRADE scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-TRADE-01 | More than 120 percent of declared annual import volume [KYC]; where customs data exist: payment to customs-value ratio above 1.2 or no customs entry within 90 days of payment [DP; basis MER box 5.2 "customs permits do not correspond to the volume"]; segment P99 by goods group and company age [PCT]; per-payment check from USD 50,000 | Importers by goods group (food, construction, electronics, vehicles, chemicals, pharmaceuticals, general trading); government-contract suppliers separately | Rolling 12 months, monthly; per-payment NRT | Price rises, exchange-rate moves, multi-year advance payments, re-exports | Invoice, bill of lading, customs entry, goods and HS code, declared volume, 12-month payment list |
| SCN-TRADE-02 | Account age under 6 months [CBI-2016]; funding deposited under 7 days before the sale [CBI-2016]; above USD 3,000 per person without source documents [CBI-2016]; remittance above USD 150,000 or more than 10 a day for different persons [CBI-2016]; at least 3 applicants sharing an address, phone or director [DP]. All values configurable because the 2016 rules' current status is not verified | Individuals and companies | Day and 7 days; NRT; P2 candidate | New legitimate importers, funding rhythm | Account opening date, funding trail, source documents, shared-attribute graph |
| SCN-TRADE-03 | Beneficiary or agent country differs from the supplier or origin country; or payment place or beneficiary changed after issue, any value at least USD 20,000 [DP] | Importers | Per payment; NRT; P2 candidate for post-issue changes | Triangular trade, regional distributors, freight forwarders paid directly | LC or invoice, amendment history, supplier registration, account country |
| SCN-TRADE-04 | At least 2 amendments in 90 days; extension beyond 90 days past original expiry; at least 50 percent of the customer's LCs cancelled or unused in 12 months; amendment changing beneficiary, payment place or amount by more than 20 percent [DP; basis Wolfsberg-ICC-BAFT: "constantly amended or extended" and "routinely cancelled or unused"] | LC customers | 90 days and 12 months; EOD | Genuine contract changes, shipment delays | Amendment log, reasons, presented documents compared with originals |
| SCN-TRADE-05 | Any shared owner, director, address or phone between applicant and beneficiary or shipper; margin funded at least 50 percent by a non-applicant within 3 days before issue [DP; basis Wolfsberg "just in time" credits] | LC and guarantee customers | At issue and at settlement; NRT | Group companies trading with each other at arm's length | Ownership chart, margin funding trail, related-party declaration |
| SCN-TRADE-06 | Guarantee with no underlying contract on file: exception; call within 30 days of issue; amount above 20 percent of applicant's annual turnover [DP] | Guarantee customers | At issue and on call; EOD | Bid bonds that are called early for administrative reasons | Contract copy, call letter, applicant financials, margin source |
| SCN-TRADE-07 | Unit price more than 30 percent from the peer median or customs or benchmark value [DP]; declared value below shipping cost [CBI ch. 9]; applied only when goods code, quantity and value exist; framed "manifestly unusual" [Wolfsberg] | Goods groups with enough peer deals (at least 30) | Weekly; WK | Quality or brand differences, volume discounts, inflation | Comparison table, benchmark source, documents |
| SCN-TRADE-08 | Open-account transfer share at least 90 percent of import value where the peer LC share is at least 40 percent [PCT]; cash-funded share at least 50 percent [DP] | Importers above USD 100,000 a year | 12 months; monthly | Suppliers that do not accept LCs, small buyers | Payment mix, funding sources, supplier list |
| SCN-TRADE-09 | Goods group outside the sector-goods map for the declared ISIC; first-time goods category above USD 50,000 [DP] | All importers; map maintained by the bank and reviewed yearly | Per payment and LC; EOD | Diversifying businesses, general traders | Declared activity, goods, licence, owner background |
| SCN-TRADE-10 | No transport document, arrival notice or customs entry within 60 days of payment [DP; basis Wolfsberg "missing transport documents"; murabaha possession rule] | LC and open-account payments above USD 50,000 | 60 days after payment; WK | Late customs data, services mislabelled as goods | Payment, documents on file, customs query result |
| SCN-TRADE-11 | Company under 12 months with cumulative import payments at least USD 250,000 [DP]; first LC or large import payment within 90 days of account opening [DP; basis MENAFATF 2014: "opening accounts then requesting immediately an LC"]; cluster of at least 3 companies incorporated within 30 days sharing address, director, signatory, phone or counterparties [FinCEN red flag] | New customers; registry extract needed for incorporation date | 12 months; monthly graph | Genuine start-ups, group restructurings | Registry extract, graph, owners, flows between cluster members |
| SCN-TRADE-12 | Share of outward payments to free-zone or general-trading counterparties under 24 months old above 30 percent [DP; basis FinCEN advisory and notice] | Importers and traders | Quarterly; WK | Normal regional trade through the UAE | Counterparty profile, ownership, address checks |
| SCN-TRADE-13 | Quarterly issuance above 3 times the customer's trailing 12-month quarterly average, or above 50 percent of annual turnover [DP] | LC and guarantee customers | 90 days; monthly | Large contracts won, seasonal tenders | Issuance history, contracts, financials |

AKR-TMS hooks:
- **Trade view**: one page per instrument with parties, goods, ports, vessel, amendments, margin funding, payments and documents, with
  the customer's declared trade profile beside it.
- **Optional data connectors** for customs declarations, vessel and port data and the company registry; each connector turns a grade C
  scenario into grade B and is shown as such in the data-readiness view.
- **CBI FX rule pack** (2016 numbers) as configurable hard rules with an effective date and a "status unverified" label until the bank confirms
  what applies.
- **Sector-goods map and peer-price tables** managed as governed reference data (owner, version, review date).

### B7. Family ISL: Islamic products (by reference to file 24 section 8)

File 24 section 8 defines 57 Islamic-product scenarios (the "IS pack": murabaha M1-M11, tawarruq and 'inah T1-T4, ijara I1-I5, mudaraba and
PSIA P1-P5, musharaka S1-S3, qard hasan Q1-Q7, salam and istisna' Z1-Z4, charity C1-C8, sukuk K1-K3, cross-cutting X1-X7), each with logic, data,
basis and phase. This section does not repeat them. It does three things: it bundles them into eight library scenarios so that the CBI
coverage report, the scenario studio and the parameter register treat them like any other scenario; it gives each bundle its CBI baseline
link and starting parameters; and it keeps the file 24 rule that **no Islamic scenario is a CBI minimum**: they are a bank extension to be
agreed with the AML/CFT Office, shipped optional, each labelled with its basis (CBI page, UN entry or analysis) and never presented as
evidence that Islamic contracts are riskier (file 24 I-30).

| ID | Bundle | IS pack members (file 24) | CBI baseline link | Basis and phase |
|---|---|---|---|---|
| SCN-ISL-01 | Murabaha: supplier is the customer or related; no possession evidence; round-trip of the supplier payment; supplier profile. Typologies: 1, 5. | M1, M2, M5, M6 | Extends Art.4 no. 7 (LC beneficiary owned by the customer); Art.13 no. 11 (fictitious contracts); Art.3 no. 13. Also: CBI 2018 items 6-2-1 to 6-2-3 | Hard rule M1 and M6 on a CBI 2018 page; phase 1 for M1 and M6, phase 2 for M2 and M5 |
| SCN-ISL-02 | Murabaha: price outlier, payment geography, early or third-party settlement, serial rescheduling, profile mismatch. Typologies: 1, 5. | M3, M4, M8, M9, M11 | Extends Art.3 no. 36; Art.6 nos. 4, 8; Art.12 no. 10 | Analysis plus CBI catalogue; phase 1 for M4, M8, M11 |
| SCN-ISL-03 | Prohibited structures: 'inah, organised tawarruq, foreign-currency murabaha, liquidity cycling, circular commodity chains. Typologies: 17. | T1, T2, T3, T4, M7 | Outside the CBI catalogue. Also: CBI 2018 (prohibitions); IIFA resolution 179 | CBI 2018 hard rules; phase 1; P2 candidate |
| SCN-ISL-04 | Ijara and real estate: inflated sale-and-leaseback, third-party buy-out, rent paid by others, asset encumbrance, early real-estate sale, churn. Typologies: 6. | I1, I2, I3, I4, I5, M10 | Extends Art.13 no. 8 (high-value real estate). Also: CBI 2018 item 16 (two-year rule) | CBI 2018 for I5; phase 1 for I2, I3, I5; phase 2 for I1, I4, M10 |
| SCN-ISL-05 | Profit-sharing accounts and partnership financing: profit allocation outliers, profit harvesting, third-party funding, fixed "profit", early-termination value transfer, capital source mismatch, accelerated buy-out. Typologies: 17. | P1-P5, S1-S3 | Extends Art.3 no. 17; Art.11 nos. 2, 3 (by analogy). Also: CBI 2018 mudaraba rules | CBI 2018 for P4; analysis for the rest; phase 1 for P2, P3, P4 |
| SCN-ISL-06 | Qard hasan hard rules: per-loan cap, recipient abroad, repeat loan, insider recipient, aggregate cap, third-party cash repayment. Typologies: 17. | Q1-Q7 | Outside the catalogue. Also: CBI 2018 pp. 58-59 | CBI 2018; phase 1; P2 candidate |
| SCN-ISL-07 | Charity, zakat and waqf: donations outside registered NPOs, NPO pass-through, transfers to unregistered or listed bodies, waqf flows against the deed, charity-fund concentration, seasonal excess, narrative mismatch, alias-aware screening. Typologies: 10, 13, 14. | C1-C8 | Extends Art.3 no. 32; Art.13 no. 4. Also: circular of 26 Aug 2026 group 7; FATF R.8 | Analysis and UN list entries; phase 1 except C6 |
| SCN-ISL-08 | Salam, istisna', sukuk and cross-cutting controls: prepayment without delivery, milestone payments to related contractors, off-market sukuk trades, Sharia non-compliance density, missing Sharia approval, staff-linked suppliers, inter-book transfers, contract-data completeness. Typologies: 7, 17. | Z1-Z4, K1-K3, X1-X7 | Extends Art.14 no. 8 (staff dealing with people lacking integrity); Art.13 no. 11. Also: Law 43 art. 8 | Mixed; phases 1 and 2 as in file 24 |

**Starting parameters (the bundles inherit file 24's logic and add these numbers).**

| ID | Start parameters (basis) | Window and mode |
|---|---|---|
| SCN-ISL-01 | Seller or supplier shares owner, signatory, address, phone, email, account or device with the customer: any [CBI 2018 item 6-2-3: void]; no invoice, delivery or title after 30 days [DP] | At contract and daily ageing; EOD |
| SCN-ISL-02 | Unit cost or margin more than 30 percent from peer deals [DP, as SCN-TRADE-07]; settlement funded by an unrelated payer, cash or a foreign account: any [DP] | Per deal and on settlement; EOD |
| SCN-ISL-03 | 'Inah: same asset sold and bought back within 30 days at a lower cash price [CBI 2018]; organised tawarruq: sale proceeds credited the same day as purchase [CBI 2018]; contract currency different from settlement currency: any [CBI 2018 p. 22 item 7] | At contract; NRT |
| SCN-ISL-04 | Property sold before two years after break-even without default [CBI 2018 item 16]; buy-out funded by a third party or cash: any [DP] | On event; EOD |
| SCN-ISL-05 | Fixed payout to a mudaraba account: any [CBI 2018]; deposit within 7 days before and withdrawal within 7 days after a profit run, at least twice or above USD 10,000 [DP] | On profit run; monthly |
| SCN-ISL-06 | Loan above IQD 10 million [CBI 2018]; recipient outside Iraq [CBI 2018]; new loan within 6 months of repayment or while another is unpaid [CBI 2018]; recipient is staff, board or relative [CBI 2018]; total above 10 percent of net profit after the investment account holders' share [CBI 2018] | At disbursement; NRT |
| SCN-ISL-07 | At least 10 donors in 30 days into a non-registered or personal account [DP]; NPO inflow equals outflow within 7 days [DP]; seasonal band by Hijri calendar [PCT] | 30 days and seasonal; EOD |
| SCN-ISL-08 | Salam delivery overdue by 30 days with no evidence [DP]; milestone payment above certified progress [DP]; share of deals missing supplier, asset or evidence fields above 5 percent as a control failure [DP] | Per deal; monthly |

AKR-TMS hooks:
- **Optional extension pack** with its own version and data-readiness view: scenarios whose deal, supplier-payment, asset or pool data the bank
  cannot supply show "not covered", not silently skipped (file 24 section 8 hooks).
- **Deal view** beside the account view (file 24 section 3 hooks), because value goes to suppliers and sellers, not to the customer.
- **Back-test before go-live** on the bank's own deals, and agree the extension with the AML/CFT Office before treating it as the bank's
  scenario set (file 24 section 8; Part A legal note: banks may develop new scenarios "in coordination with" the Office).
- **Arabic alert narratives** that use the contract's own terms so branch liaison officers and Sharia audit staff read them naturally.

### B8. Family DIGI: e-wallets, cards, digital channels and mule networks

The CBI's articles 7 and 8 list 15 digital and card indicators and several general scenarios (Art. 3 nos. 21, 37, 39-41, 46) add card-based
illicit FX and crypto speculation. Iraq has 17 e-payment companies, four with mobile payments (MER paragraph 109), card and wallet
rails that bank accounts fund and drain, and growing digital banks (file 01 section 2e). Library rules: (1) device and phone evidence
outweighs IP address, because IP sharing is weak evidence on shared mobile networks [N: expected, to be confirmed with each bank's channel
data]; (2) cards and wallets are monitored on the account that funds them, with the card or wallet record joined (file 25 section 12.6);
(3) channel logs are optional data (grade C): the scenarios say what they lose without them.

**Table 1: what each DIGI scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-DIGI-01 | Mule account: many small inbound payments then fast cash-out (fraud and drug proceeds, scams). Typologies: 8, 12. | Implements Art.7 no. 1; Art.8 no. 3. Extends Art.3 nos. 45, 48. Also: FATF/Interpol/Egmont 2023 annex A | A new account or wallet receives many small payments from unrelated senders and passes most of the value on or out within a day | account (open date), transaction, counterparty, channel |
| SCN-DIGI-02 | Shared device, phone or IP across unrelated customers (mule rings, ghost accounts). Typologies: 8, 12. | Implements Art.7 no. 8; Art.3 no. 21 | Several unrelated customers use one device, phone or network identity | party_contact, channel logs (device, IP) |
| SCN-DIGI-03 | Logins or transfers from high-risk countries, VPN or impossible travel. Typologies: 8, 11. | Implements Art.7 nos. 7, 8. Extends Art.7 no. 2 | Session or transfer from a country on the risk table, behind a VPN, or from two distant places within hours, for a customer declared resident in Iraq | channel logs (IP, geolocation), country_risk table |
| SCN-DIGI-04 | Prepaid and wallet funnel. Typologies: 12. | Implements Art.7 no. 9 | Funds from several prepaid or wallet accounts converge on one person or entity | transaction, counterparty, card |
| SCN-DIGI-05 | Account or card funded steadily with little use, then emptied. Typologies: 12. | Implements Art.8 no. 3; Art.11 no. 3 | A balance builds for weeks with almost no transactions, then most of it leaves in a day or two | balance_snapshot, transaction |
| SCN-DIGI-06 | Card limit exhaustion and full-repayment cycles; daily cash maximum. Typologies: 21. | Implements Art.8 nos. 1, 2 | Credit-card limit used fully then repaid in full, repeatedly; cash withdrawals at the daily maximum on many days | card, facility, card_transaction |
| SCN-DIGI-07 | Card-based illicit FX: IQD cash in, USD cash out abroad; large USD purchases abroad. Typologies: 1, 12. | Implements Art.3 nos. 40, 41; Art.8 no. 5 | Card or account funded with IQD cash, then USD cash withdrawals at foreign ATMs or large USD purchases abroad within days; cash deposited in one currency and withdrawn in another | card_transaction (country, currency), cash_event, transaction |
| SCN-DIGI-08 | Several cards or accounts with the same data for use abroad. Typologies: 12. | Implements Art.3 nos. 21, 37 | Several cards issued to one person in a short time, or several accounts of one beneficiary or agent with identical data, aimed at cards used abroad | card, party_contact, party_relationship |
| SCN-DIGI-09 | ATM withdrawals in a high-risk or distant area. Typologies: 10. | Implements Art.3 no. 39. Extends Art.7 no. 10. Also: circular of 26 Aug 2026 group 7 | Cash withdrawals in districts flagged high-risk, or far from the holder's home, with no local spending; card may not be in the holder's hands | card_transaction (terminal location), party_contact (address) |
| SCN-DIGI-10 | Crypto speculation through cards and wallets. Typologies: 8, 10. | Implements Art.3 no. 46. Also: CBI ban on cards and e-wallets for crypto (2022-2023) | Payments to crypto exchanges, crypto-related merchants or peer-to-peer crypto narratives; repeated top-ups from unknown parties followed by foreign merchant spend | card_transaction (MCC, merchant), counterparty, narrative |
| SCN-DIGI-11 | Account-takeover pattern. Typologies: 8. | Extends Art.7 nos. 5, 8. Also: FATF/Interpol/Egmont 2023; Wolfsberg digital lifecycle (file 04 section 2q) | A new device bound or contact data changed shortly before a first-time beneficiary and a large transfer | channel logs, party_contact (change history), transaction |
| SCN-DIGI-12 | Many new beneficiaries in a short time. Typologies: 8, 12. | Implements Art.7 no. 6. Extends Art.7 no. 1 | Many first-time beneficiaries added and paid within a day or a week | transaction, counterparty (first-seen), channel |
| SCN-DIGI-13 | Circular transfers between own or controlled accounts. Typologies: 21. | Implements Art.7 no. 3; Art.8 no. 4 | Repeated transfers among the customer's accounts, or A to B and back to A, with no balance purpose | transaction, account_party_role |
| SCN-DIGI-14 | Use in many locations, regions and currencies. Typologies: 9, 12. | Implements Art.7 no. 10. Extends Art.3 no. 9 (proxy: card use in drug-producing countries) | Services used from several countries or governorates and in several currencies in a week; repeated card or ATM use in countries on the drug-producing list, as a proxy for constant travel there | channel logs, card_transaction |
| SCN-DIGI-15 | Collection account for an investment or loan scam. Typologies: 8. | Implements Art.3 no. 17. Also: MER paragraph 57 (fraud through social media, unlicensed loan and investment companies) | Many payers send small sums with investment or loan narratives to one unlicensed account that forwards most of it to a few beneficiaries or cash | transaction, narrative, counterparty |

**Table 2: how each DIGI scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-DIGI-01 | Account or wallet age at most 30 days; at least 10 distinct senders in 7 days, each at most USD 200; at least 85 percent out within 24 hours of the last inbound [DP; basis Art.7 no. 1, FATF annex A] | Wallets, digital-bank and prepaid accounts, individuals | 7 days; NRT | Crowdfunding for a medical case, market vendors, online sellers | Sender list, device and phone evidence, time series, destinations, onboarding channel |
| SCN-DIGI-02 | At least 4 customers on one device in 30 days, or at least 3 on one phone number [DP]; IP sharing counted at a lower weight; households (family number) shown, not alerted | All digital customers; office static IPs and internet cafes excluded by list | 30 days; EOD | Families, shared household tablets, employees of one firm | Device graph, account openings, flows between members |
| SCN-DIGI-03 | Country on the risk table, VPN flag, or two countries within 2 hours, followed within 1 hour by a transfer of at least USD 1,000 [DP] | Customers declared resident in Iraq | Session; NRT | Travellers, students abroad, Iraqis working abroad | Session log, country table version, transfer, declared residence |
| SCN-DIGI-04 | At least 5 prepaid or wallet source accounts into one account in 30 days [DP] | Prepaid and wallet products | 30 days; EOD | Family pooling, event collections | Source list, relationships, destination use |
| SCN-DIGI-05 | At least 60 days with 3 or fewer transactions, then at least 90 percent withdrawn within 48 hours [DP; basis Art.8 no. 3] | Cards, wallets, savings-type accounts | 60 days; EOD | Savings for Hajj or marriage, gift cards | Balance path, funding sources, withdrawal destination |
| SCN-DIGI-06 | Utilisation at least 90 percent then full repayment within 5 days, at least 3 cycles in 90 days; cash maximum on at least 5 days in 10 [DP; basis Art.8 nos. 1, 2] | Credit and prepaid cards | 90 and 10 days; EOD | Revolving users who pay from salary | Utilisation and repayment series, repayment source |
| SCN-DIGI-07 | IQD cash funding then foreign USD cash withdrawals of at least USD 1,000 within 7 days; at least 5 foreign withdrawals in a day; USD purchases abroad at least USD 3,000 a day or on 3 days in 7 [DP; basis Art.3 nos. 40, 41] | Debit and prepaid cards, travellers flagged by declared travel | 7 days; NRT | Genuine travel, students, medical travel | Funding trail, foreign terminals, travel evidence, currency of funding |
| SCN-DIGI-08 | At least 3 cards in 90 days to one person [DP; basis Art.3 no. 37]; at least 3 accounts sharing beneficiary, agent, address and job data with cards used abroad [DP] | Card issuance | 90 days; EOD | Family members, staff cards for a company | Issuance log, shared attributes, foreign usage |
| SCN-DIGI-09 | At least 3 withdrawals in 7 days in a flagged district or in another governorate from the home address, with no local spending [DP]; district table from the bank's risk assessment and the UN reports (Anbar, Diyala, Salah al-Din, western districts) | Card holders | 7 days; EOD | Travel for work, pilgrims, displaced families | ATM list and map, home address, spending pattern, card status |
| SCN-DIGI-10 | Any payment to a crypto exchange or crypto MCC or peer-to-peer crypto narrative: alert [CBI ban, file 01 section 2b]; at least 3 foreign merchant payments after unknown-party top-ups [DP] | Cards and wallets | Event; NRT | Mislabelled merchants, gaming sites | Merchant data, narrative, top-up sources |
| SCN-DIGI-11 | New device or contact change within 24 hours before a first-time beneficiary and at least USD 1,000 or at least 80 percent of balance [DP; basis FATF 2023 annex A, Wolfsberg digital lifecycle] | Digital customers | 24 hours; NRT; P2 candidate (step-up check) | Lost phones, genuine upgrades | Change log, beneficiary history, session evidence, customer contact |
| SCN-DIGI-12 | At least 10 new beneficiaries in 24 hours or 25 in 7 days [DP; basis Art.7 no. 6] | Digital customers; payroll and bulk files excluded | 24 hours and 7 days; NRT | Payroll by individuals, event organisers | Beneficiary list, first-seen dates, narratives |
| SCN-DIGI-13 | At least 5 own-account transfers in 7 days, or a loop of 2 to 3 accounts within 5 days [DP; basis Art.7 no. 3] | All accounts | 7 days; EOD | Sweeps between own accounts, budgeting | Transfer graph, balances, purpose |
| SCN-DIGI-14 | At least 3 countries or 5 governorates or 3 currencies in 7 days [DP; basis Art.7 no. 10]; card or ATM use in a drug-producing country on at least 3 days in 90 [DP; proxy for Art.3 no. 9] | Digital customers | 7 days; EOD | Frequent travellers, border-area residents | Session map, currency use |
| SCN-DIGI-15 | At least 20 payers in 14 days with investment or loan narratives into one unlicensed account; at least 80 percent out to at most 3 beneficiaries or cash [DP; basis MER paragraph 57] | Individuals and small firms | 14 days; EOD | Rotating savings circles, cooperatives | Payer list, narratives, outflow, registry check of the company |

AKR-TMS hooks:
- **Device and phone graph** built from the channel database where the bank allows a read-only link, with an explicit grade C label and
  retention limits for device and location data (file 04 section 2q).
- **Wallet and card features**: account age, first-seen counterparties, balance-to-throughput ratios, foreign-terminal share, funding
  currency versus spend currency.
- **Onboarding-channel field** on the customer (branch with original ID, agent, eKYC) so mule and takeover cases can be reported by
  onboarding method ("follow the data", file 04 section 2q).
- **Crypto counterparty list**: exchange names, MCCs and wallet-provider narratives, maintained as reference data with a review date.

### B9. Family LOAN: loans, guarantees and early repayment

Credit is a layering route: a loan turns a deposit into a clean debt and early repayment turns dirty cash back into a bank asset. The CBI
catalogue has ten credit indicators (Art. 6) and six general or cash scenarios about loans (Art. 1 nos. 13, 18; Art. 3 nos. 7, 19, 20).
Library rule: loans are monitored by event on the facility record (disbursement, repayment, early settlement, collateral change), joined to
the payer's identity; Islamic financing is covered through the deal view and the ISL bundles (B7).

**Table 1: what each LOAN scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-LOAN-01 | Sudden or early repayment of a large debt with no explained source (integration through credit). Typologies: 17. | Implements Art.1 no. 13; Art.3 no. 19; Art.6 no. 8. Extends Art.6 no. 4 | A large repayment or early settlement, far ahead of schedule, funded by cash, a third party or a foreign source | facility (schedule, settlement), transaction (payer, source) |
| SCN-LOAN-02 | Repayment by third parties. Typologies: 17. | Implements Art.6 no. 4. Extends Art.1 no. 13 | Repayments paid by persons who are not the borrower, guarantor or a recorded relative, or one third party repaying several borrowers | facility, transaction (payer), party_relationship |
| SCN-LOAN-03 | Facility proceeds moved abroad fast. Typologies: 3, 17. | Implements Art.1 no. 18; Art.6 no. 3 | Most of a disbursement sent out of Iraq within days | facility (disbursement), payment_message |
| SCN-LOAN-04 | Collateral or guarantee owned by an unrelated third party. Typologies: 17. | Implements Art.3 no. 20; Art.6 nos. 1, 2 | Collateral or guarantor with no recorded link to the borrower, one guarantor across several unrelated borrowers, or a guarantee from a bank abroad with no clear reason | facility (collateral, guarantor), party_relationship |
| SCN-LOAN-05 | Cash-collateralised loans and loans against certificates or blocked deposits (loan-back). Typologies: 17. | Implements Art.6 nos. 5, 6, 7 | Cash deposit or certificate purchase followed by a loan against it, early liquidation to repay, or collateral held abroad in high-risk countries | facility, account_hold, counterparty (country) |
| SCN-LOAN-06 | Early repayment of facilities secured by jewellery or real estate. Typologies: 6, 17. | Implements Art.3 no. 19 (second part). Extends Art.6 no. 4 | Such a facility repaid long before maturity from cash, a third party or an unexplained source | facility (collateral type), transaction |
| SCN-LOAN-07 | Credit refused or declined to avoid disclosure. Typologies: 20. | Implements Art.3 no. 7; Art.6 no. 9 | A customer with idle balances repeatedly declines credit offers, or a facility is refused for doubts about guarantees; hybrid with the observation log | credit decision log, observation log, balance_snapshot |
| SCN-LOAN-08 | Financial statements out of line with peers, or unaudited, in the credit file. Typologies: 21. | Implements Art.1 nos. 9, 11; Art.6 no. 10 | A company with large activity or credit lacking audited statements, or ratios outside the sector band | kyc_document (financials), customer financials, facility |

**Table 2: how each LOAN scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-LOAN-01 | Repayment of at least 30 percent of the outstanding balance or at least USD 10,000 [CBI anchor], at least 6 months before maturity, funded by cash, a third party or a foreign transfer [DP] | Personal, SME, corporate and mortgage facilities | On event; NRT | Sale of an asset, inheritance, bonus, refinancing elsewhere | Source-of-funds documents, payer identity, schedule, asset sale evidence |
| SCN-LOAN-02 | Any payment of at least USD 3,000 by a non-recorded payer; one payer repaying at least 3 borrowers [DP] | All facilities; family payers recorded as relationships are excluded | On event; EOD | Relatives paying, employers paying staff loans | Payer identity and relationship, payment list across borrowers |
| SCN-LOAN-03 | At least 70 percent of a disbursement sent abroad within 7 days and at least USD 10,000 [DP] | Business and personal loans; purpose-specific (tuition, import) loans excluded when documents are on file | 7 days; EOD | Loans for supplier payments, medical treatment, tuition abroad | Loan purpose, outward message, documents |
| SCN-LOAN-04 | Any unrelated collateral owner or guarantor; one guarantor for at least 3 unrelated borrowers [DP] | Secured facilities | At origination and monthly; EOD | Business partners, employer guarantees | Collateral deeds, relationship record, guarantor's other exposures |
| SCN-LOAN-05 | Loan within 90 days of the cash deposit or certificate purchase it is secured on; early liquidation to repay; foreign blocked deposit of a related company [DP] | Loans against deposits and certificates | 90 days; EOD | Standard treasury or overdraft against deposits | Deposit trail, certificate history, collateral country |
| SCN-LOAN-06 | Repayment at least 6 months early from cash, a third party or an unexplained source [DP] | Jewellery and real-estate secured facilities | On event; EOD | Refinancing, sale of the pledged asset | Valuation, payer, source evidence |
| SCN-LOAN-07 | At least 2 declined offers in 12 months with balances above USD 10,000 [DP]; any "refused: doubtful guarantee" decision creates a customer flag [rule] | Customers with credit-offer history; needs credit decision log | 12 months; WK | Customers who dislike debt | Offer log, decision notes, balances, observation entries |
| SCN-LOAN-08 | Annual turnover at least USD 500,000 or credit limit at least USD 250,000 without audited statements [DP]; margin or turnover-to-assets ratio outside the segment 5th to 95th percentile [PCT] | Companies | At origination and annual review; MO | Start-ups, family firms with informal accounts | Statements, auditor, ratio table against peers |

AKR-TMS hooks:
- **Facility event stream** (disbursement, repayment, early settlement, collateral change, rescheduling) with the payer's identity on each
  repayment; where the CBS shows only the loan account, the payer comes from the cash or transfer record that funded it.
- **Peer ratio tables** for financial-statement checks, built from the bank's own borrowers and refreshed yearly.
- **Collateral and guarantor graph** shared with the entity graph, so one guarantor across many borrowers is a query.

### B10. Family ACCT: accounts, profile deviation and behaviour change

This family turns the CBI's central requirement into scenarios: compare "actual volumes with what the customer disclosed" (2025 controls,
chapter 5 article 5 item 5) and notice when behaviour changes. It also carries the account-structure indicators of Art. 3 and Art. 13.
Library rules: (1) every [KYC] threshold has a fallback to the segment percentile when the declared value is missing, and flags the customer
as "degraded" (file 20 L-44); (2) peer comparison uses robust statistics (median and MAD, or percentiles) because cash and trade amounts are
heavy-tailed (file 29 section 9c); (3) none of these scenarios uses gender, religion or nationality as an input.

**Table 1: what each ACCT scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-ACCT-01 | Income or turnover above the declared profile (expected versus actual). Typologies: 21. | Implements Art.1 nos. 15, 17; Art.2 no. 8; Art.9 no. 6. Extends Art.1 no. 14; Art.3 no. 26 | Monthly inflows or outflows exceed the KYC-declared income or turnover, or the customer's own trailing level | customer expected profile, transaction, balance_snapshot |
| SCN-ACCT-02 | Spike against the customer's own history. Typologies: 21. | Implements Art.1 nos. 14, 15 | Volume, value or count of movements far above the customer's own robust baseline | transaction history (at least 6 months) |
| SCN-ACCT-03 | Behaviour or segment migration. Typologies: 21. | Extends Art.1 no. 14. Also: Wolfsberg part I (file 04 section 2a); file 21 section 3c | A customer's behavioural cluster changes, for example a salaried account that starts to behave like a pass-through trader | features, segment assignments |
| SCN-ACCT-04 | Many accounts, branches or signatories without need. Typologies: 2, 12. | Implements Art.3 nos. 13, 33; Art.12 no. 3; Art.13 no. 1 | Several accounts opened in a short time, or several unrelated signatories (especially foreign nationals), with transactions involving unknown persons | account, account_party_role, party_relationship |
| SCN-ACCT-05 | Burst and bust: account used heavily then closed or abandoned. Typologies: 8, 12. | Implements Art.13 no. 5 | An account receives many deposits or transfers, then is closed or goes dormant soon after | account (status history), transaction |
| SCN-ACCT-06 | Accounts belonging to others, or controlled by an agent. Typologies: 12. | Implements Art.13 no. 2; Art.3 no. 25. Extends Art.12 no. 11 | One attorney, signatory or agent operating many unrelated accounts, or withdrawing most of an account's value for others | account_party_role, transaction (operator, presenter) |
| SCN-ACCT-07 | Profile edits around large movements. Typologies: 8, 20. | Implements Art.12 no. 13. Extends Art.12 no. 15. Also: circular group 6; Wolfsberg digital lifecycle triggers | Repeated changes to phone, address or signatory, an edit shortly before or after a large transaction, or a contact number reported disconnected by the bank's messaging gateway | party_contact (history), account_party_role, cbs_user_activity |
| SCN-ACCT-08 | Lifestyle and high-value purchases beyond means. Typologies: 6, 7, 21. | Implements Art.13 nos. 6, 8; Art.3 no. 26 | Payments to car dealers, jewellers, developers and property sellers far above declared income | transaction, counterparty classification (DNFBP), expected profile |
| SCN-ACCT-09 | Young, student or low-income profile with large or unusual flows. Typologies: 16, 21. | Implements Art.3 nos. 24, 42; Art.12 no. 10 | Large flows, foreign exchange, transfers or term deposits by a customer whose age band or occupation does not explain them | customer (age band, occupation), transaction, product |
| SCN-ACCT-10 | Low-tier and simplified-due-diligence accounts near their limits. Typologies: 19. | Outside the Part A catalogue. Also: 2025 controls chapter 5 article 3 (SDD barred if linked transactions stay under the limit) | Monthly flows hugging the tier limit, or several low-tier accounts per ID | account (tier), transaction, party_identifier |
| SCN-ACCT-11 | Identity reuse and documents of dead persons. Typologies: 1, 12. | Implements Art.3 no. 23; Art.7 no. 5; Art.13 no. 9. Extends Art.2 nos. 14, 15. Also: FinCEN Al-Huda final rule | One ID with several names or birth dates, one document behind several customers, documents proven forged, or an ID flagged as deceased before the transaction date (attempts that were refused are in the KYC module and the observation log) | party_identifier, civil registry (optional), eKYC records |
| SCN-ACCT-12 | Active accounts with incomplete or refused CDD. Typologies: 20. | Implements Art.3 no. 6; Art.12 nos. 9, 14 | Accounts under delayed or refused verification that keep transacting above the allowed limits | customer (CDD status), transaction |
| SCN-ACCT-13 | Legal person inconsistent with its sector, or run by outsiders. Typologies: 2, 21. | Implements Art.3 no. 29. Extends Art.13 no. 1 | Turnover far above sector peers, or an account operated by a signatory or foreign operator with no ownership link | customer (ISIC), account_party_role, transaction |

**Table 2: how each ACCT scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-ACCT-01 | Monthly inflows above 150 percent of declared income or turnover and at least USD 2,000 above it [KYC]; or above 3 times the customer's 12-month median [DP]; missing declared value: fallback to segment P99 [PCT] | By segment and currency; PEP and EDD customers use 120 percent | Calendar month and 90 days; EOD | Bonuses, arrears, sale proceeds, seasonal income | Declared profile beside actuals, 12-month chart, inflow sources |
| SCN-ACCT-02 | Robust z-score (median and MAD) above 4 on monthly inflow, outflow or count, and at least USD 2,000 above baseline; at least 6 months of history [DP] | Customers with history; new customers use [PCT] segment baselines | Monthly and 3 months; EOD | One-off events, salary changes | Baseline chart, drivers of the spike, counterparties |
| SCN-ACCT-03 | Cluster change confirmed in 2 consecutive months [DP]; clusters rebuilt quarterly in shadow mode (file 21 section 3d) | Segments with at least 200 customers (file 21) | Monthly; MO | Job change, new business | Old and new cluster profiles, feature changes |
| SCN-ACCT-04 | At least 4 accounts opened in 90 days; at least 3 signatories with no recorded relationship, extra weight for foreign nationals [DP] | Individuals and companies | 90 days; EOD | Large corporates, holding structures | Account list, signatory relationships, flows between accounts |
| SCN-ACCT-05 | Account closed or inactive for at least 60 days within 90 days after turnover of at least USD 10,000 and at least 5 times declared [DP] | All | 90 days; EOD | Planned closure, relocation | Turnover, closure reason, remaining counterparties |
| SCN-ACCT-06 | One attorney or agent on at least 5 unrelated accounts; or at least 80 percent of account value withdrawn by an agent across at least 3 customers [DP] | All; documented family and company agents flagged and reviewed, not suppressed | 90 days; EOD | Lawyers, family agents, company representatives | Agent ID, powers of attorney, account list, presenter records |
| SCN-ACCT-07 | At least 3 contact or signatory edits in 30 days; any edit within 7 days before or after a transaction of at least USD 5,000 [DP] | All; branch and operator recorded | 30 days; EOD | Moves, lost phones, company reorganisations | Change log with operator, transaction, verification evidence |
| SCN-ACCT-08 | Payments in 90 days to car dealers, jewellers and developers above 12 times declared monthly income [KYC], or above USD 20,000 for salaried staff [DP]; property payments above USD 380,000 or IQD 500 million must have bank-channel and EDD evidence [MER paragraph 195] | By occupation; DNFBP counterparty list | 90 days; EOD | Savings spent on a house, weddings, family gifts | Counterparty class, invoices, source of funds, declared assets |
| SCN-ACCT-09 | Age 25 or under, or occupation student or unemployed, with monthly flows above 5 times the segment median and at least USD 2,000; term deposit of at least USD 10,000 by age 25 or under [DP] | Age band and occupation only as the CBI text names them; no other protected attribute | Monthly; EOD | Family-funded study abroad, inheritance | Occupation, family links, funding source, purposes |
| SCN-ACCT-10 | Monthly flows at 80 to 99 percent of the tier limit in at least 2 months; more than one low-tier account per ID [CBI: IQD 1 million ceiling for occasional and digital walk-in transfers] | Low-tier and walk-in digital accounts | 3 months; MO | Genuine small users near the limit | Tier definition, flows, ID links |
| SCN-ACCT-11 | One ID with 2 or more names or birth dates: alert; ID flagged deceased before the transaction date: alert; the same document image behind 2 customers: alert [DP] | All; registry feed optional | At onboarding and nightly; EOD | Name spelling variants (Arabic, Kurdish, Latin), data-entry errors | Document copies, entity-resolution result, registry extract |
| SCN-ACCT-12 | Any transaction above USD 1,000 on an account with CDD incomplete for more than 30 days; delayed-verification limits breached [CBI: Law art. 10 third and Instructions 1/2023 art. 10 allow delay only with limits on number, type and amount] | Accounts with a CDD-status field | Daily; NRT; P2 candidate | Slow document supply by customers | CDD checklist, limits, last contact |
| SCN-ACCT-13 | Turnover above 3 times the sector median or above sector P99 [PCT]; signatory or operator with no ownership link and nationality or role recorded [DP] | By ISIC; companies and sole traders | Quarterly; WK | Fast-growing firms, franchise outlets | Sector comparison, owners, signatories, premises check |

AKR-TMS hooks:
- **Profile-versus-actual panel** on every alert: declared income, turnover, cash limit and purpose beside the observed values, with the
  declared field's age and source (file 20 L-28). A profile older than 12 months is shown as stale.
- **Robust baselines service** (median, MAD, percentiles by segment and currency, seasonal where needed) shared by all scenarios.
- **Segment-migration trigger** feeds the KYC refresh queue (file 26) as well as the alert queue.
- **Operator attribution** on edits (cbs_user_activity) so SCN-ACCT-07 and SCN-INS-03 can show who changed what and when.

### B11. Family PEP: PEPs and other high-risk customers

The CBI's PEP circular of 26 August 2026 (file 01 section 2c) extends EDD to PEPs, high-risk position holders and their relatives "up to the
second degree", requires daily screening of local and regional news, says EDD "continues after the PEP leaves office", and lists minimum
red flags (a to h) that banks may extend with scenarios. The 2024 MER found that banks identify relatives and associates "by the experience
and knowledge of financial institution employees" (file 01 section 2). This family implements the circular's red flags as scenarios and
adds the CBI catalogue's PEP and high-risk items (Art. 2 no. 13; Art. 3 nos. 22, 34, 38; Art. 13 nos. 3, 7, 10). It never names a PEP:
status comes from the PEP register (file 26, file 14) and from relationship data (family number, shareholding).

**Table 1: what each PEP scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-PEP-01 | Inflows or cash beyond official income (illicit enrichment, bribery proceeds). Typologies: 7. | Implements Art.2 no. 13. Extends Art.1 no. 17; Art.13 no. 6. Also: PEP circular red flags a, b | Inflows of a PEP, a relative or an associate exceed the declared official income by a multiple, or include cash or large non-salary credits | party (PEP status), customer expected profile, transaction |
| SCN-PEP-02 | Unjustified transactions among PEPs, relatives, associates and related entities. Typologies: 7. | Implements Art.2 no. 13. Extends Art.3 no. 27. Also: PEP circular red flag e | Transfers between a PEP and relatives (family number), associates or related companies without a recorded purpose | party_relationship (family to 2nd degree, associate, owner), transaction |
| SCN-PEP-03 | Wholesale conversion of IQD balances to USD by a senior official. Typologies: 1, 7. | Implements Art.3 no. 38 | Most of an IQD balance withdrawn or converted and USD deposited within days by a senior post holder | transaction, fx_deal, party (position) |
| SCN-PEP-04 | Companies and structures linked to PEPs (derived exposure). Typologies: 2, 7. | Extends Art.13 nos. 1, 3. Also: PEP circular red flag d; Instructions 1/2023 art. 7 (BO at 20 percent) | Companies in which a PEP or relative holds a share, signs or directs, that receive state payments or send funds to PEP-linked accounts | party_relationship (ownership share), transaction |
| SCN-PEP-05 | Undisclosed PEP. Typologies: 7. | Outside the Part A catalogue. Also: PEP circular item 7 (precautionary measures) | A screening hit says the customer, owner or signatory is a PEP while the KYC declaration says no | screening results, KYC declaration |
| SCN-PEP-06 | Sudden pattern or asset change, including after leaving office. Typologies: 7, 21. | Extends Art.13 no. 6. Also: PEP circular red flags b, h, item 8 (EDD continues after office) | Month-on-month or yearly growth of inflows or balances beyond a multiple; surge in the year before or after a change of office | transaction, balance_snapshot, PEP register (dates) |
| SCN-PEP-07 | High-risk customer types: tightened cash and pass-through profile (real estate, precious metals, car dealers, NPOs, exchange). Typologies: 3, 6. | Outside the Part A catalogue. Also: 2025 controls ch. 2 art. 2 (customer risk factors) | The cash, pass-through and third-party-payer scenarios run on these segments with halved floors and a composite score | customer (segment), transaction |
| SCN-PEP-08 | Adverse media or authority request triggers a look-back. Typologies: 7, 10. | Implements Art.3 nos. 22, 30; Art.13 nos. 7, 10. Also: PEP circular item 3 (daily news) | A new adverse-media item or authority request on a customer, owner, signatory or counterparty produces a 12-month exposure report and an alert above a value | adverse-media events, authority-request register, transaction |
| SCN-PEP-09 | Beneficial owner or controller from a high-risk country or area. Typologies: 10, 11. | Implements Art.3 no. 34; Art.13 no. 3 | BO, controller or signatory with nationality, residence or registration in a high-risk country, with flows to that place | party (nationality, residence), party_relationship, payment_message |

**Table 2: how each PEP scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-PEP-01 | Inflows above 120 percent of official income [KYC] (tighter than the general 150 percent); any cash deposit of at least USD 3,000; any non-salary credit of at least USD 10,000 [DP]; basis: PEP circular red flags a and b | PEP tiers (senior, other), relatives, associates; official income from payroll or the FCOI declaration where available | Monthly and rolling 12 months; EOD, NRT for cash | Family business income, inheritance, property sales, remittances | Official salary and allowances, source-of-wealth file, declaration extract, linked accounts |
| SCN-PEP-02 | Any transfer of at least USD 5,000 between a PEP and a related party without a documented purpose; at least 3 in 90 days [DP] | Relatives and associates from the PEP register | 90 days; EOD | Family support, family businesses | Relationship evidence (family number, shareholding), purposes, amounts |
| SCN-PEP-03 | At least 80 percent of an IQD balance out and USD in within 5 days, balance at least USD 20,000 [DP; basis Art.3 no. 38] | Senior government post holders | 5 days; NRT | Planned purchases abroad, travel | Balance path, FX rates, purpose, declared plans |
| SCN-PEP-04 | PEP or relative share of at least 10 percent (watch) and at least 20 percent (beneficial owner) [CBI: Instructions 1/2023 art. 7; MER: banks check CRD owners at 10 percent]; plus state payments received [DP] | Companies | Quarterly graph; MO | Minority holdings in listed firms | Ownership chain, state-payment records, flows to PEP accounts |
| SCN-PEP-05 | Any mismatch: case plus precautionary review within 5 working days [CBI: PEP circular item 7] | Customers, owners, signatories | On list or KYC change; NRT | Wrong match, outdated lists | Screening hit, evidence of position, declaration signed |
| SCN-PEP-06 | Monthly inflow growth above 200 percent, or 12-month growth above 300 percent; any surge within 12 months before or after leaving office [DP] | PEP register customers | Monthly; MO | New income source | Growth chart, tenure dates, assets, source of wealth |
| SCN-PEP-07 | Floors at 50 percent of the default; DNFBP cash and transfers of at least USD 10,000 a month with pass-through ratio at least 0.8 [DP] | Real-estate dealers, jewellers, car dealers, NPOs, exchange houses | Monthly; EOD | Normal trade of the sector | Segment baseline, counterparties, licence status |
| SCN-PEP-08 | Look-back 12 months; alert when exposure at least USD 10,000 [DP]; search of local and regional Arabic news daily [CBI: PEP circular item 3] | Customers, owners, signatories, counterparties | On event; NRT | Name-only matches, old stories | Media item, link strength, exposure table |
| SCN-PEP-09 | Tighter thresholds at 50 percent of default; any outward flow of at least USD 5,000 to the BO's high-risk country [DP] | Companies and trusts with such a BO | Per transaction; EOD | Dual nationals with no real link | BO documents, flows by country, risk table version |

AKR-TMS hooks:
- **PEP register with relatives and associates** (family number and shareholding links), effective-dated, with "status persists after office
  until a documented downgrade" (circular item 8) and the circular's seven KPIs as a ready report.
- **Derived PEP and derived-designation graph**: ownership percentages aggregated under the CBI, US (50 percent) and EU or UK (control) rules.
- **Daily Arabic and English news screening** with event-driven look-backs, so that SCN-PEP-08 is a feed, not a manual search.
- **Official-income baseline** from payroll data or declarations, shown beside every PEP alert.

### B12. Family PUB: public funds, payroll and government contracts

Corruption is the first-ranked threat in the NRA, with about USD 2.6 billion confiscated in 2018-2022 (B2.7). The CBI catalogue has no scenario
for it except through PEPs and cheque endorsement, yet the MER cases, the US Treasury action of 9 October 2025 and the CBI's own 2024 fraud guide
all describe flows that a bank can see: payroll manipulation, state cheques, government-contract sub-contracting. Salary localisation makes
public pay visible in bank data. This family is an extension to be agreed with the AML/CFT Office; its basis labels say so.

**Table 1: what each PUB scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-PUB-01 | Ghost-employee payroll cluster. Typologies: 7, 15. | Extends Art.13 nos. 2, 4. Also: CBI fraud guide 2024 annex 2; MER box 3.18; file 01 section 9 | Salary accounts of one employer or payroll batch sharing phone, address, device, onward beneficiary or cash-out terminal; accounts opened in bulk just before first pay | account, transaction (payroll batch, narrative), party_contact, card_transaction |
| SCN-PUB-02 | Salary diversion to a third party. Typologies: 7. | Extends Art.13 no. 2; Art.3 no. 25 | Salary credited and almost all moved or withdrawn by the same third party, month after month | transaction, account_party_role, card_transaction |
| SCN-PUB-03 | State cheques or transfers to private companies, then cash or abroad. Typologies: 7. | Extends Art.1 no. 21; Art.3 no. 14. Also: MER box 5.2; tax-deposit case 2022 [S] | Payments from state accounts or cheques issued by state departments to a company that then withdraws cash, issues cheques to others or sends most abroad within days; young company or many cheques to one payee | transaction (drawer type), cheque, counterparty (type, age) |
| SCN-PUB-04 | Unusual debits on government and public-body accounts. Typologies: 7. | Outside the Part A catalogue. Also: file 21 section 3c (segments of one) | Debits to beneficiaries outside the usual vendor pattern, spikes in cheque count, large cheques to young companies | account (government), transaction, cheque |
| SCN-PUB-05 | Sub-contracting chains from government contracts. Typologies: 2, 7. | Outside the Part A catalogue. Also: Treasury 9 Oct 2025; MER box 5.2 | A contractor paid from a government source passes most of it to several sub-contractors that pass it on to related or common parties within a few hops | counterparty graph, transaction (source flag) |
| SCN-PUB-06 | Pension and retiree-list manipulation. Typologies: 7. | Outside the Part A catalogue. Also: MER box 3.18 | Pension accounts with duplicate identifiers or flagged as deceased, or accounts that only receive credits and are cashed out by agents | account, party_identifier, payroll or pension batch flag, civil registry (optional) |
| SCN-PUB-07 | Illicit enrichment of public employees below PEP level. Typologies: 7, 21. | Extends Art.1 nos. 17, 14. Also: PEP circular red flag a (by analogy) | Non-PEP public employees whose inflows and cash far exceed their pay and allowances | payroll data, expected profile, transaction |

**Table 2: how each PUB scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-PUB-01 | At least 3 salary accounts of one payroll batch sharing a phone, device or onward beneficiary; at least 5 accounts cashed out at one terminal within 24 hours of credit [DP] | Public-salary accounts (federal tawtin, KRG MyAccount), by employer | Monthly payroll cycle; EOD | Households sharing a phone, rural ATM scarcity, one ATM per village | Cluster graph, payroll batch, ATM list and times, account opening dates |
| SCN-PUB-02 | At least 90 percent of the salary out within 24 hours to the same third party for at least 3 consecutive months; the third party on at least 3 accounts [DP] | Public-salary and pension accounts | 3 months; monthly | Family support, loan deductions, agents collecting for the elderly | Third-party identity and links, withdrawal records, authority to collect |
| SCN-PUB-03 | At least USD 100,000 received from a state source and at least 70 percent out within 7 days; company younger than 24 months or at least 5 cheques to one payee in 30 days [DP] | Companies and contractors | 7 and 30 days; NRT | Legitimate contractor paying suppliers and wages | Source document of the state payment, contract, outflow destinations, company registry |
| SCN-PUB-04 | Debit to a beneficiary not seen in 12 months of at least USD 250,000; cheque count above 3 times the account's 12-month weekly average [DP] | Government and public-body accounts; restricted to compliance roles | Weekly; NRT | Year-end budget spending, emergency procurement | Approval trail, vendor registry, cheque images, beneficiary profile |
| SCN-PUB-05 | At least 3 sub-contractors, at least 70 percent passed on within 30 days, at most 3 hops, total at least USD 250,000 [DP] | Contractors with government income | 30 days; graph, weekly | Normal sub-contracting in construction | Contract chain, graph, sub-contractor profiles, ownership links |
| SCN-PUB-06 | Duplicate ID or family number across pension accounts: alert; deceased flag before payment date: alert; credits only for at least 6 months with agent cash-outs [DP] | Pension and retiree batches | Monthly; MO | Name variants, family number errors | Duplicate records, registry extract, agent records |
| SCN-PUB-07 | Inflows above 5 times declared pay and allowances and at least USD 5,000 a month [DP] | Public-sector employees not on the PEP register | Monthly; EOD | Second jobs, family income | Payroll record, deposit sources, declarations |

AKR-TMS hooks:
- **Payroll and pension batch tags** read from the CBS (employer, batch ID, payment code) so that public-salary accounts become a segment
  with peer baselines, as file 01 section 9 recommends.
- **Government-source flag** on counterparties and on cheque drawers, maintained as reference data, so "state payment" is a query.
- **Contract-chain view**: a graph from a government payment through sub-contractors, with ownership and PEP flags, frozen into the alert.
- **Restricted visibility** for government-account scenarios (compliance head only) to avoid misuse; the Sharia board and branch managers
  see no PUB alerts (file 24 I-16, file 01 chapter 11).

### B13. Family CORR: correspondent, nested, exchange-house and PSP activity

The MER says Iraqi banks "do not act as correspondents for respondent banks" (file 01 section 2), so classic nested accounts are rare. The
nested risk sits in two places: the bank's own accounts with foreign correspondents, which the CBI wants monitored and reported monthly (2025
controls chapters 10 and 15), and in customers that are themselves intermediaries: exchange companies (rated high risk by the NRA, MER paragraph
109), salary-disbursement and e-payment offices, and payment service providers. Since the dollar auction ended, each bank answers to its own
correspondents (file 01 section 4), so the bank-level mirror in SCN-CORR-07 is the nearest thing to what FinCEN computed for Banque Misr UAE
(B2.2). Class A exchange companies are, according to the MER, "prohibited from opening bank accounts"; the scenarios apply to the exchange
and transfer companies that do hold accounts and to the accounts of their agents.

**Table 1: what each CORR scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-CORR-01 | Nested or pass-through use of intermediary accounts. Typologies: 3, 11. | Extends Art.2 no. 9. Also: 2025 controls ch. 10; Wolfsberg PSP guidance 2026; file 20 L-63 | For correspondent, PSP, e-payment and exchange accounts: matched inflow and outflow within days, and a large share of value with no underlying originator or beneficiary data ("opaque") | counterparty (type), payment_message_party, transaction |
| SCN-CORR-02 | Exchange and transfer company accounts: cash funding, few foreign counterparties (hawala settlement). Typologies: 3, 10. | Extends Art.9 nos. 3, 5; Art.3 no. 13. Also: MER paragraphs 109, 391; FATF 2013 and 2026 | Cash-funded accounts that pay most value to a few foreign counterparties, in UAE, Turkey or border corridors, settled by third-party cheques or round sums | account (customer type), transaction, counterparty (country) |
| SCN-CORR-03 | Salary-disbursement and e-payment offices: reconciliation gaps and cash-out spikes. Typologies: 3, 4, 10. | Outside the Part A catalogue. Also: MER boxes 4.4, 4.8; Wolfsberg PSP guidance 2026 (account activity reviews) | Volumes reported by the provider differ from the bank's statements; cash withdrawals take most of the funding; transfers to people not on the payroll list; growth in border governorates | account, PSP report files, transaction |
| SCN-CORR-04 | Correspondent account volume or counterparty deviation. Typologies: 11. | Implements 2025 controls ch. 10 and ch. 15 report (c). Outside the Part A catalogue | Monthly volume per foreign bank, currency and country against baseline; new countries; shifts toward high-risk corridors | account (nostro and vostro), payment_message, country_risk |
| SCN-CORR-05 | Payment-message integrity: blank or generic fields, cover payments, overtyping, routing diversion. Typologies: 11. | Outside the Part A catalogue. Also: 2025 controls ch. 5 art. 4; file 20 L-38 | Rates of blank, "." or generic ordering and beneficiary fields, cover share, manual repair, and rules that single out a named party, by operator, branch, customer and correspondent | payment_message (raw and parsed), operator |
| SCN-CORR-06 | Rejected, returned or held payments and their resubmission. Typologies: 11. | Outside the Part A catalogue. Also: file 20 L-39 | A payment rejected or held, then re-sent with the same or near-same beneficiary and amount through another route, user or correspondent | transaction_event, payment_message status (pacs.002, pacs.004) |
| SCN-CORR-07 | Typology exposure index per bank, branch and correspondent. Typologies: 2, 11. | Outside the Part A catalogue. Also: file 20 L-65; FinCEN notice on Banque Misr UAE | Monthly value and count of flows matching each published typology, as a share of total flows and of assets, with trend | all scenario outputs, bank aggregates |
| SCN-CORR-08 | PSP and wallet customers: pre-funding disguised as liquidity transfers, bundled flows, non-merchant outflows. Typologies: 3, 12. | Outside the Part A catalogue. Also: Wolfsberg PSP guidance 2026; FATF R.16 revision of 2025 | Transfers that behave like customer pre-funding, many-to-one and many-to-many bundles, settlement outflows to parties that are not merchants, new corridors | account (PSP), transaction, counterparty classification |

**Table 2: how each CORR scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-CORR-01 | Opaque share above 20 percent of value; at least 70 percent of value matched in and out within 72 hours [DP; basis MAS 2018 tolerance, file 20 L-63] | Correspondent, PSP, e-payment and exchange accounts | 72 hours and monthly; EOD | Treasury sweeps, settlement accounts | Matched flows, client-data availability, last attestation |
| SCN-CORR-02 | Cash share of funding above 50 percent; at least 70 percent of outward value to 3 or fewer foreign counterparties; third-party cheques or round sums above 30 percent of inflows [DP; basis FATF 2013 indicators] | Class B and C exchange companies and transfer companies, agents | Monthly; EOD | Genuine transfer business with few partners | Counterparty list, licence status, cash records, settlement documents |
| SCN-CORR-03 | Mismatch between provider report and bank statement above 5 percent; cash share of funding at least 70 percent; payments to non-payroll persons above 10 percent [DP; basis MER box 4.8] | Salary-disbursement offices, e-payment companies | Monthly; MO | Timing differences, reversals | Provider statements, payroll lists, cash-out records, branch region |
| SCN-CORR-04 | Monthly volume above 150 percent or below 50 percent of the 6-month median; any new corridor; share to high-risk countries up 50 percent [DP] | Each foreign bank and currency | Monthly; MO | Seasonal trade, new products | Volume series, corridor map, risk table version |
| SCN-CORR-05 | Rate of blank or generic fields above 2 percent per operator-month or 3 times baseline; any rule or routing that singles out a named party [DP; basis file 20 L-38] | Operators, branches, correspondents | Monthly; EOD; P2 inline enforcement | Truncated names, system defaults | Message samples, operator list, routing rule history |
| SCN-CORR-06 | Any resubmission within 7 days of a rejected or held payment with the same or similar beneficiary or amount through another route, user or correspondent [DP] | All outgoing payments | 7 days; NRT; P2 four-eyes release | Corrected data entry | Reject and hold register, original and new messages, user |
| SCN-CORR-07 | Index above twice the peer median or up 50 percent month on month; computed per typology (Iran-linked, front-company, exchange-house, oil, hawala) [DP; basis FinCEN ratio method] | Bank, branch, correspondent | Monthly; MO | Mix change, new customers | Typology flows, ratio series, flow breakdown by customer |
| SCN-CORR-08 | Pre-funding-like transfers: inbound from one PSP customer immediately followed by outbound of the same value to a party that is not a merchant, at least 3 times in 30 days; non-merchant outflow share above 20 percent [DP] | PSP accounts | 30 days; EOD | Settlement design of the PSP | Flow classification, PSP questionnaire answers, corridors |

AKR-TMS hooks:
- **Intermediary register** (banks, exchange houses, PSPs, agents) with licence status and expiry, expected corridors and volumes, client-list
  status and last attestation; "opaque" flows reported to the MLRO (file 20 L-63, file 04 section 2r).
- **CBI standard reports as scenarios' siblings**: the monthly correspondent-account report (all transfers, currency, beneficiary bank,
  number and volume per foreign bank, changes) is generated by the same data (2025 controls chapter 15).
- **Typology exposure index** (SCN-CORR-07) as a board-level dashboard and a respondent view of what correspondents can compute (file 04
  W-22, file 20 L-65).
- **PSP report ingestion** (file or API) so bundled payments can be unbundled and reconciled; without it the flow is labelled opaque.

### B14. Family TF: terrorist financing

The CBI catalogue has one TF-specific general scenario (Art. 3 no. 32, charities) and the circular of 26 August 2026 adds a TF group: reliable
negative information or authority requests; requests to transact for suspected persons; "using personal accounts to receive donations for
'charitable or humanitarian' purposes without a licence or institutional framework"; intermediaries hiding the beneficiary; refusing source-of-
funds information then cancelling (B2.10, B2.13). The evidence says three things for design. First, ISIL money in Iraq is small and
local: about USD 500,000 a year from extortion, salaries of USD 50-70 per fighter and USD 35 per family (UN, B2.10). Second, it moves by
hawala, exchange offices, couriers and, in the MER's cases, salary-disbursement offices. Third, other groups mostly self-finance and leave
no financial signal (MER box 4.1). A bank scenario can therefore find micro-patterns around conflict-affected districts and abused
NPO accounts, and must say openly that it cannot find self-financed acts. The flagged-district table comes from the bank's own risk assessment
and the UN's reports (Anbar and its western districts, Diyala, Salah al-Din, the Hamrin area, the Syrian border; S/2026/44 paragraph 63), and
is reviewed at least quarterly.

**Table 1: what each TF scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-TF-01 | Small recurring transfers to or from conflict-affected districts (salaries and family payments). Typologies: 10. | Extends Art.2 no. 2; Art.3 no. 45. Also: circular group 7; UN S/2025/482 paragraph 102; Wolfsberg TF statement 2024 | Many senders to one beneficiary, or one sender to many, in flagged districts, in the USD 30-100 band, repeated monthly | counterparty (district), transaction, payment_message |
| SCN-TF-02 | Donations collected in personal or unregistered accounts. Typologies: 10, 13, 14. | Extends Art.3 no. 32. Also: circular group 7; FATF NPO typology 2014 | A personal or non-registered account receives donation-type payments from many payers and forwards them to individuals, abroad or in cash | transaction, narrative, account type, NGO registry (optional) |
| SCN-TF-03 | NPO accounts: foreign funding out of line, fictitious salaries, outflows to individuals and conflict areas. Typologies: 10, 13. | Implements Art.3 no. 32. Extends Art.13 no. 4. Also: MER boxes 3.13, 4.11, 4.12; FATF 2014 | Foreign inflows that do not match the registered activity, payments to board members or non-payroll individuals, cash withdrawals, transfers to unregistered or listed bodies | customer (NPO), transaction, programme and budget data (optional) |
| SCN-TF-04 | Exchange-office payout chains and safe-drop patterns. Typologies: 3, 10. | Outside the Part A catalogue. Also: MER boxes 4.3, 4.4, 4.10; UN S/2025/482 paragraph 101 | Several payers fund one exchange-office account in a day, followed by many payouts to individuals in flagged areas within two days | account (exchange company), transaction, counterparty (district) |
| SCN-TF-05 | Digital hawala and virtual-asset exposure. Typologies: 3, 10. | Extends Art.3 no. 46. Also: UN S/2026/651 paragraphs 109-114; FATF 2026 (VA and gold settlement); CBI ban on cards and wallets for crypto | Payments to or from virtual-asset providers, peer-to-peer crypto narratives, gold-backed settlement narratives, from accounts that are not licensed to deal | transaction, counterparty, narrative |
| SCN-TF-06 | Network exposure to listed persons and shadow designations. Typologies: 10, 11. | Extends Art.2 no. 11; Art.3 nos. 22, 27. Also: CBUAE "shadow designated persons"; OFAC 50 percent rule (file 05 section 1e) | Customers or counterparties sharing a strong identifier or ownership with listed persons (UN, IQTFS, OFAC), within two hops, or entities owned in the aggregate by listed persons above the regime's threshold | party_identifier, party_relationship, watchlists, graph |
| SCN-TF-07 | Authority request or negative-information trigger. Typologies: 10. | Outside the Part A catalogue. Also: circular group 7; file 20 L-46 | A request or notice from an authority, or reliable negative information, creates a priority case with a look-back over linked accounts and counterparties | authority-request register, adverse-media events, graph |

**Table 2: how each TF scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-TF-01 | At least 5 senders to one beneficiary in the USD 30-100 band in 3 consecutive months, or one sender paying at least 5 beneficiaries in flagged districts monthly [DP; basis UN S/2025/482: USD 50-70 and USD 35]; district table versioned and reviewed quarterly | Individuals; payroll and standing charitable orders excluded by profile | 3 months; monthly | Families supporting relatives in the same districts, remittance to displaced persons | Sender and beneficiary list, district, narratives, relationships |
| SCN-TF-02 | At least 10 distinct payers in 30 days with donation-type narratives and at least 70 percent forwarded within 14 days [DP]; seasonal band by Hijri calendar [PCT] | Personal accounts, non-registered groups | 30 days; EOD | Ramadan and Eid giving, funerals, family fundraising | Narratives, registry result, onward flows, seasonal chart |
| SCN-TF-03 | Foreign inflow of at least USD 20,000 in 90 days not matching the registered programme; payments to board members or non-payroll individuals above 10 percent of outflows; cash above 30 percent of outflows; any transfer to an unregistered or listed body [DP; basis MER box 4.12, FATF 2014]; annual audit status checked against the IQD 75 million rule [MER] | NPOs and religious institutions; registered status from the NGO registry | 90 days; EOD | Humanitarian programmes with local cash needs | NPO registration, programme budget, donors, board payments, audit status |
| SCN-TF-04 | At least 5 payers funding one exchange-office account within 24 hours and at least 5 payouts to individuals in flagged districts within 48 hours [DP] | Exchange and transfer companies | 48 hours; NRT | Remittance peaks before Eid | Payout list, districts, licence status, cash records |
| SCN-TF-05 | Any virtual-asset provider counterparty or crypto narrative on an unlicensed customer [CBI ban and the MER on no licensed VASPs]; at least 3 unrelated payers with crypto narratives in 30 days [DP] | Accounts, cards, wallets | Event and 30 days; NRT | Mislabelled merchants | Counterparty, narrative, wallet identifiers if present |
| SCN-TF-06 | Any shared strong identifier (phone, address, IBAN, device); aggregated ownership by listed persons at least 50 percent (US) or above 50 percent or control (EU, UK) [PEER: OFAC 2014 guidance, EU and UK rules]; within 2 hops | All customers and counterparties | Continuous; NRT on each list change; P2 candidate | Common addresses (landlords, branches), common names | Identifier match, ownership chain, list entry and scope note |
| SCN-TF-07 | Priority 1 within hours; look-back 24 months over linked accounts and counterparties [DP] | Customers, owners, signatories, counterparties named | On event; NRT | Misidentified person | Request text and reference, graph, exposure table; restricted access |

**What is not detectable.** Self-financed attacks, cash donations out of salaries, and money that never touches a bank leave no signal in
transaction data (MER box 4.1, paragraph 386). AKR-TMS must not claim otherwise; the observation log (B21) and authority requests (SCN-TF-07)
are the only routes for such cases.

AKR-TMS hooks:
- **Flagged-district and border table** as governed reference data with an owner, a source (bank risk assessment, UN reports), a version
  and a quarterly review; every TF-01 alert shows the table version.
- **NPO profile**: registration number, registry status, foreign-funding flag, expected programme and budget, audit status, board and
  signatories as related parties (file 24 I-12), and an alias- and scope-aware screening of NPO counterparties (file 24 I-27).
- **Immediate-handling path** for list matches and authority requests: priority queue, MLRO notification, freeze workflow, report to the CFTF
  and the AML/CFT Office (file 01 section 6), outside the normal alert queue.
- **Calendar-aware baselines** so Ramadan, Eid, Muharram and Arbaeen giving does not create alert storms (file 24 section 4e).

### B15. Family PF: proliferation financing and sanctions evasion

The MER rates PF sanctions implementation (IO11) Low; the FATF action plan names PF sanctions evasion; US dollar access depends on how a bank's
flows look to its correspondents (B2.2, B2.11). CBI chapter 9 gives PF indicators for customers, activity, geography and trade documents
(file 01 section 2d) but the catalogue in Part A has no PF scenario except Art. 2 no. 11 (sanctioned parties and high-risk countries).
Library rules: (1) screening finds listed names; these scenarios find the behaviour around them: Iran nexus, front companies, oil of doubtful
origin, chains of exchange houses; (2) location and instruction evidence is used, not only names (Standard Chartered, Swedbank and Paxful
all turned on data the bank had collected and ignored, file 20 FM-14); (3) UN sanctions on Iran were restored on 27 September 2025 with 43
individuals and 78 entities on the restored 1737 list (file 04 section 4b), so Iran vigilance is a legal baseline, not an option; (4)
hard rules are P2 candidates, behavioural scores are not.

**Table 1: what each PF scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-PF-01 | Iran-nexus flows (direct or through third countries). Typologies: 11. | Extends Art.2 no. 11. Also: 2025 controls ch. 9; FinCEN FIN-2025-A002; MER IO11 | Any payment, instruction or document with an Iran nexus: counterparty, bank, vessel, address, phone prefix, goods origin, outside documented humanitarian or religious-tourism purposes | payment_message_party, trade_instrument, party_contact |
| SCN-PF-02 | Front-company profile score. Typologies: 2, 11. | Extends Art.3 nos. 13, 34. Also: 2025 controls ch. 9 (shared directors, owners, addresses; residential addresses); FinCEN FIN-2025-A002; file 20 L-66 | A weighted score from company age, shared address, director or phone, residential address, absent payroll, opaque ownership, free-zone registration, sector mismatch and many unrelated counterparties, with reason codes | customer, party_relationship, party_contact, transaction, registry data |
| SCN-PF-03 | Inflow sector versus outflow sector mismatch. Typologies: 2, 11. | Outside the Part A catalogue. Also: FinCEN FIN-2025-A002 | Most inflows from commodity or petroleum counterparties and most outflows to electronics, technology or chemical suppliers | counterparty (sector tags), transaction |
| SCN-PF-04 | Oil-trade payments of doubtful origin. Typologies: 11. | Outside the Part A catalogue. Also: 2025 controls ch. 9; FinCEN oil red flags; Treasury 3 Jul and 2 Sep 2025 | Oil, LPG or petrochemical payments or LCs with a listed vessel, a vessel with recent name or flag changes, a scrapped IMO number, a bill of lading without consignee, or origin claims that the seller cannot support | trade_instrument (vessel, IMO, ports, documents), maritime data (optional) |
| SCN-PF-05 | Dual-use goods, end-user and route anomalies. Typologies: 11. | Outside the Part A catalogue. Also: 2025 controls ch. 9 (dual-use, declared value below shipping cost, goods unfit for the destination) | Goods or HS codes on a dual-use list with a high-risk destination or intermediary, value below shipping cost, goods that do not fit the end-user's industry | trade_instrument (goods, HS, value), dual-use list |
| SCN-PF-06 | Chains of exchange houses and trading companies. Typologies: 3, 11. | Outside the Part A catalogue. Also: FinCEN FIN-2025-A002 | Funds passing through three or more exchange houses or trading companies in days, adding fees at each hop, with no commercial logic | counterparty graph, transaction |
| SCN-PF-07 | Designation look-back and derived-designation exposure. Typologies: 11. | Outside the Part A catalogue. Also: file 20 L-41 | On each new designation, a report of all flows with the party before and after the effective date, all connected accounts and owners, and the time from publication to freeze | watchlists (effective dates), transaction, graph |
| SCN-PF-08 | Location and instruction evidence. Typologies: 11. | Extends Art.2 no. 11; Art.7 no. 7. Also: file 20 FM-14, L-37 | Evidence of Iran or another restricted jurisdiction in phone prefixes, addresses, IP or GPS, instruction channel or device settings, combined with trade or payment activity | party_contact, channel logs, payment_message |
| SCN-PF-09 | Sudden governance or behaviour change at trading customers. Typologies: 2, 11. | Outside the Part A catalogue. Also: 2025 controls ch. 9 | Repeated changes of board members or signatories followed by a change in transfer behaviour | party_relationship (history), account_party_role, transaction |

**Table 2: how each PF scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-PF-01 | Listed party: any amount, hard rule, P2 candidate; other Iran nexus: at least USD 1,000 or at least 3 in 30 days [DP] | All customers; documented humanitarian and religious-tourism payments flagged, not suppressed | Event; NRT | Pilgrimage packages, students, family remittances, legitimate trade within exemptions | Nexus evidence, purpose and documents, list version and scope note |
| SCN-PF-02 | Score at least 60 of 100 [DP]; weights in file 20 L-66 order: company age under 24 months, shared address or director, residential address, no payroll, opaque ownership, free-zone registration, sector mismatch | Corporate and trader customers | Monthly; MO | Genuine start-ups sharing an incubator address | Score with reason codes, graph of shared attributes, registry extract |
| SCN-PF-03 | At least 60 percent of inflow value from commodity or petroleum counterparties and at least 60 percent of outflow to electronics, technology or chemical suppliers within 90 days [DP; basis FinCEN] | Traders; counterparty sector tags from enrichment | 90 days; MO | General trading companies | Counterparty sector table, flows, declared business |
| SCN-PF-04 | Any listed vessel or IMO inconsistency: alert and P2 candidate; otherwise at least 2 of: name or flag change, no consignee, ship-to-ship mention, unsupported origin claim [DP] | Oil, LPG and petrochemical payments and LCs | Per instrument; NRT | Legitimate shipping changes | Vessel history, documents, seller evidence, maritime data extract |
| SCN-PF-05 | Any dual-use match on a payment of at least USD 10,000: review; at least 2 signals: alert [DP] | Industrial and technical goods | Per instrument; EOD | Common dual-use items in ordinary trade | Goods list, end-user, route, shipping cost comparison |
| SCN-PF-06 | At least 3 hops through exchange houses or trading companies within 10 days; cumulative fees above 1 percent [DP] | Customers with exchange-house counterparties | 10 days; EOD | Normal use of exchange houses for small payments | Path graph, fees, counterparty classification |
| SCN-PF-07 | On every new designation: look-back to the first dealing; report before and after the date; time from publication to re-screen and freeze reported [file 20 L-41] | All | Event; NRT | None (every new designation) | Exposure report, connected accounts, freeze record |
| SCN-PF-08 | Any restricted-jurisdiction evidence on a customer with trade or payment activity of at least USD 1,000 in 30 days [DP]; geofence list tested daily with a user-style test [file 20 L-37] | Customers with phone, address or channel data | 30 days; NRT | Residents with Iranian phone numbers, pilgrims | Evidence type and source, activity, geofence test result |
| SCN-PF-09 | At least 2 board or signatory changes in 90 days and monthly transfer value above 3 times baseline [DP] | Companies | 90 days; EOD | Reorganisations, new management | Change log, new signatories, flows before and after |

AKR-TMS hooks:
- **Iran and PF vigilance pack** (file 04 U-08): restored UN list, OFAC lists, IRGC and IRISL entities, vessels and IMO numbers, with ownership
  and control tests; each advisory (FinCEN, OFAC, CBI) becomes a governed input that creates a mapping task (file 05 section 1g).
- **Dual-use keyword and HS list** and **vessel and port list** as reference data with owners and review dates; maritime data connector optional.
- **Front-company profile score** with visible reasons; a front-company alert is a review, never an automatic exit.
- **Designation look-back report** produced automatically for every list change (file 20 L-41).
- **Geofence and location-evidence profile** per customer: which evidence types the bank holds and how many customers have each.

### B16. Family INS: internal fraud, staff accounts and know-your-employee

The CBI makes insider risk explicit: article 14 lists nine staff indicators and the know-your-employee duties (KYE form tracked by compliance, two
consecutive weeks of leave a year for everyone, branch-manager rotation at most every five years); chapter 15 requires a monthly report of
all customer and employee account movements; the 2024 fraud and corruption guide lists in annex 2 bank-side schemes (fictitious employees,
altered payroll, bribes to ease procedures, customer data manipulation, forged cheques). TD Bank's staff were bribed with more than USD 57,000 in gift cards
(file 20 section 2.1); the MER cases include bank employees convicted as co-offenders (box 3.18). Access to INS alerts is restricted to the
compliance head, as Part A's design notes require. Staff data (HR) is grade C; the CBS user and audit tables are grade B.

**Table 1: what each INS scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-INS-01 | Staff account anomalies. Typologies: 18. | Implements Art.14 nos. 1, 7. Also: 2025 controls ch. 15 report (a) | Inflows, cash or counterparties of a staff member's accounts out of line with salary | staff, account, transaction, payroll data |
| SCN-INS-02 | Deposits for third parties by employees. Typologies: 18. | Implements Art.3 no. 44. Extends Art.14 no. 3 | The same employee depositing cash into unrelated customers' accounts, repeatedly | cash_event (depositor), cbs_user_activity (operator), account |
| SCN-INS-03 | Override, bypass and after-hours patterns. Typologies: 18. | Implements Art.14 nos. 2, 3, 5. Also: CBI fraud guide 2024 annex 2 | Overrides, limit exceptions, postings outside office hours or the same operator and authoriser pair on most high-risk transactions | cbs_user_activity (override flags), transaction (operator, authoriser) |
| SCN-INS-04 | Teller and customer concentration. Typologies: 18. | Implements Art.14 no. 4. Also: file 20 section 2.1 (TD Bank) | One teller handling most of a customer's large cash deposits, or repeated corrections of cash reports | cash_event (operator), transaction |
| SCN-INS-05 | Customer-data manipulation by staff. Typologies: 18. | Extends Art.14 nos. 3, 9. Also: CBI fraud guide 2024 annex 2 (electronic systems) | An operator changes a customer's address, phone, signatory or limits shortly before a large transaction, or edits many customers' data in a day | cbs_user_activity (edits), party_contact (history), transaction |
| SCN-INS-06 | Staff-linked parties. Typologies: 18. | Implements Art.14 no. 8. Also: file 24 IS-X3 | Staff relatives as borrowers, guarantors, suppliers or signatories, or customers sharing phone, address or bank account with an employee | staff (relatives, contact), party_contact, facility |
| SCN-INS-07 | Fictitious employees and payroll manipulation inside the bank. Typologies: 18. | Outside the Part A catalogue. Also: CBI fraud guide 2024 annex 2 | Payroll beneficiaries without a CBS user or HR record, one account paid for two employees, overtime without hours | HR payroll file, staff, account |
| SCN-INS-08 | Mandatory leave, rotation, neglect of duties and activity discontinuity. Typologies: 18. | Implements Art.14 no. 6. Also: Art.14 KYE duties (two weeks of leave, five-year rotation) | Staff with no two consecutive weeks of leave in 12 months, branch managers beyond five years at one branch, high-risk activity that stops only while the operator is on leave, and neglect of duties measured by overdue reviews, late alert handling and late monthly reports per officer | HR leave and rotation data, cbs_user_activity, case system clocks |
| SCN-INS-09 | Gifts, bribes and confidentiality. Typologies: 18. | Implements Art.14 no. 9. Extends Art.12 no. 12. Also: CBI fraud guide 2024 section 10 (gifts) | Gifts offered with transaction requests, or bulk exports or prints of customer data by one operator | gift register, observation log, system access logs |

**Table 2: how each INS scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-INS-01 | Inflows above 150 percent of salary and known allowances; cash deposits above 2 times salary; any transfer from a customer's account to a staff account [DP; payroll as the declared profile] | Staff and their relatives' accounts | Monthly; MO | Loans, family gifts, second jobs | Salary record, deposits, counterparties, disclosure forms |
| SCN-INS-02 | At least 3 deposits in 30 days by the same employee into unrelated accounts, or at least USD 5,000 in total [DP] | Teller and cash roles | 30 days; EOD | Deposits for relatives, cash-in-transit duties | Operator and depositor fields, relationships, slips |
| SCN-INS-03 | At least 5 overrides in 7 days by one operator; at least 3 postings outside office hours in 30 days; one operator-authoriser pair on at least 50 percent of high-risk transactions [DP] | Operators with override rights | 7 and 30 days; EOD | Month-end work, branch understaffing | Override log, reasons, customers affected, rota |
| SCN-INS-04 | One teller handling at least 50 percent of a customer's cash deposits of at least USD 5,000 in 90 days in a branch with at least 3 tellers; at least 3 corrections of cash reports for the same customer [DP] | Teller channels | 90 days; MO | Customer preference, small branches | Teller shares, correction log, customer profile |
| SCN-INS-05 | One edit followed by a transaction of at least USD 5,000 within 24 hours; edits to at least 5 customers' data in a day by one operator [DP] | CBS operators | 24 hours; NRT | Data clean-up projects | Edit log with before and after values, approval, transaction |
| SCN-INS-06 | Any shared attribute between staff and a borrower, guarantor, supplier or signatory [DP] | Staff data from HR | Continuous; MO | Common surnames and addresses | Shared attribute, approval chain, conflict declaration |
| SCN-INS-07 | Any payroll beneficiary without HR or CBS user record; one account for two employees [DP] | Bank payroll | Monthly; MO | Timing of joiners and leavers | Payroll file, HR file, account details |
| SCN-INS-08 | No 10 consecutive working days of leave in 12 months; branch manager tenure above 5 years [CBI: Art. 14 KYE duties]; high-risk activity down at least 50 percent during leave; overdue reviews or late alert handling above twice the bank median per officer [DP] | All staff; control staff included | Annual and event; MO | Operational cover gaps | Leave record, rotation record, activity chart |
| SCN-INS-09 | Any gift offered with a request; data exports above segment P99 per operator [DP] | Staff, branches | Event and monthly; MO | Courtesy gifts of low value under the bank's gift policy | Gift register entry, access log, customer request |

AKR-TMS hooks:
- **Employee-risk view** with strict access (compliance head only), built from CBS user and audit tables, the staff accounts flagged in the
  customer master and optional HR uploads; mandatory-leave and rotation tracking if HR data is provided (Part A design implications).
- **Staff-linked flag** on parties and accounts, so every customer scenario can carry "related to staff" as a risk input.
- **Branch and teller analytics** for cash-report corrections and repeated explanations (file 20 L-67).
- **Gift and conflict registers** in the case system, linked to the observation log and to the CBI fraud guide's reporting duties.

### B17. Family DORM: dormant accounts

The CBI's chapter 15 sets a hard control for dormant accounts (حسابات راكدة): "current accounts unused for 6 months are frozen" and may be reactivated only when the holder or legal agent
attends in person with ID and signs a request, with the reactivation "audited and signed by the branch manager AND the branch liaison
officer"; operations on such accounts go through the branch manager or deputy and "withdrawals by cheque only"; internal control audits
reactivations monthly; savings, term deposits, government and card-based accounts are outside the rule (file 01 section 2d). The catalogue adds
that large movements on dormant accounts (Art. 1 no. 6), large foreign transfers into them (Art. 2 no. 6) and dormant accounts becoming
active (Art. 3 no. 1) are suspicious. This is a ready-made family of hard rules plus two behavioural scenarios.

**Table 1: what each DORM scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-DORM-01 | Reactivation with a large movement. Typologies: 19. | Implements Art.3 no. 1; Art.1 no. 6 | The first transaction after a dormancy of six months or more is large for the account or segment | account (status history, last activity), transaction |
| SCN-DORM-02 | Reactivation without the required approvals. Typologies: 19. | Outside the Part A catalogue. Also: 2025 controls ch. 15 | A dormant account reactivated without in-person attendance, signed request, or both required signatures | account_hold (status change), approval records, cbs_user_activity |
| SCN-DORM-03 | Debits on a frozen dormant account through a non-permitted channel. Typologies: 19. | Outside the Part A catalogue. Also: 2025 controls ch. 15 | Any ATM, electronic or transfer debit on a frozen dormant account; withdrawals must be by cheque through the branch manager | account (frozen dormant flag), transaction (channel) |
| SCN-DORM-04 | Pass-through after reactivation. Typologies: 19. | Extends Art.2 no. 9; Art.3 no. 1 | An inflow and an outflow of most of it within days of reactivation | transaction, account |
| SCN-DORM-05 | Foreign inflow into a dormant account. Typologies: 19. | Implements Art.2 no. 6; Art.1 no. 6 | An incoming foreign transfer into an account dormant for six months or more | payment_message, account |
| SCN-DORM-06 | Mass or operator-concentrated reactivations. Typologies: 18, 19. | Extends Art.14 no. 2 | Many reactivations by one operator or branch, or of several related accounts, in a month | account (status history), cbs_user_activity |

**Table 2: how each DORM scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-DORM-01 | Dormancy of at least 180 days [CBI: 6-month freeze]; first movement at least segment P95 or at least USD 3,000 [DP] | Current accounts; savings, term, government and card accounts excluded as the CBI rule does | On first movement; NRT | Expatriates returning, seasonal workers, estates | Dormancy history, reactivation documents, amount against profile |
| SCN-DORM-02 | Any reactivation missing either approval or the in-person step [CBI: ch. 15]; hard rule | Reactivated accounts | On event and monthly audit; EOD | Approvals held in paper files not in the CBS | Approval trail, ID record, operator, audit result |
| SCN-DORM-03 | Any non-cheque debit on a frozen dormant account [CBI: ch. 15]; hard rule, P2 candidate | Frozen dormant accounts | Event; NRT | Standing orders created before freezing | Debit record, channel, account status history |
| SCN-DORM-04 | At least 80 percent of an inflow out within 3 days of reactivation [DP] | Reactivated accounts | 3 days; NRT | Family use of an old account | Ledger path, counterparties |
| SCN-DORM-05 | Foreign transfer of at least USD 1,000 [DP] | Dormant current accounts | Event; NRT | Remittances to old family accounts | Originator, purpose, account history |
| SCN-DORM-06 | At least 5 reactivations in a month by one operator or branch, or at least 3 of related accounts [DP] | Operators and branches | Monthly; MO | Campaigns to clean dormant accounts | Operator list, accounts, relationships |

AKR-TMS hooks:
- **Dormancy clock** per account from the last customer-initiated movement, using the CBI rule (six months, current accounts only) and the
  exclusions; the clock and its inputs are shown on every dormant-account alert.
- **Reactivation audit report** (monthly) listing every reactivation with its approvals, the report chapter 15 asks internal control to run.
- **Hard rules** (DORM-02 and DORM-03) are P2 candidates because each has a CBI page behind it and no tuning.

### B18. Family CHQ: cheques

Cheques are a documented Iraqi laundering instrument: the MER's STR categories include "cheque endorsement" (16 STRs in 2018-2019), the state-cheque case of box 5.2, and FinCEN's
description of fake cheques used to inflate balances before dollar purchases (B2.1). The CBI catalogue has two cash scenarios on cheques (Art. 1
nos. 20, 21), one general scenario (Art. 3 no. 14) and the fraud guide's cheque schemes (forged cheques, signatures that differ from the
specimen, insufficient balance). Library rule: a cheque is tracked as an instrument with drawer, payee, endorsers, clearing status and return
reason, not only as a ledger line (file 25 section 12.1: electronic cheques settle within two days in the C-ACH).

**Table 1: what each CHQ scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-CHQ-01 | Large cash purchase of bank cheques. Typologies: 3, 9. | Implements Art.1 no. 20 | Cash paid for manager's or bank cheques above a value, repeatedly or to the same payees | cash_event, cheque (bank-issued) |
| SCN-CHQ-02 | Large cheques endorsed by third parties. Typologies: 3, 7. | Implements Art.1 no. 21; Art.3 no. 14 | A large cheque endorsed to the depositor by an unrelated party, or several cheques from several endorsers, or a chain of endorsements | cheque (endorsers, chain), party_relationship |
| SCN-CHQ-03 | Cash deposit then cheque or transfer to third parties. Typologies: 3. | Implements Art.3 no. 3. Extends Art.1 no. 7 | Cash deposited, then most of it leaves by cheque or transfer to others within days, repeatedly | transaction, cheque |
| SCN-CHQ-04 | Fake-cheque funding and returns. Typologies: 1. | Outside the Part A catalogue. Also: FinCEN Al-Huda final rule; CBI fraud guide 2024 annex 2 (cheques) | Provisional credit from a cheque used for a dollar purchase, transfer or cash, then the cheque is returned; repeated returns | cheque (status, return reason), transaction |
| SCN-CHQ-05 | Cheque kiting and circular cheques. Typologies: 8. | Outside the Part A catalogue | Cheques exchanged among two or three accounts and deposited within days, inflating balances | cheque (drawer, payee, dates), account |
| SCN-CHQ-06 | Cheques just under the limit. Typologies: 3. | Extends Art.1 no. 12 (by analogy); Art.3 no. 15 | Several cheque deposits between 80 and 99 percent of the USD 10,000 anchor in a month | cheque, transaction |

**Table 2: how each CHQ scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-CHQ-01 | Cash purchase of at least USD 5,000 or at least 3 in 30 days, or the same payee twice [DP] | Individuals, SMEs | 30 days; NRT | Property purchases, vehicle purchases | Cash slip, payee, purpose, source of cash |
| SCN-CHQ-02 | Single endorsed cheque of at least USD 10,000 [CBI anchor]; at least 3 cheques from at least 3 endorsers in 30 days; a chain of 2 or more endorsements [DP] | Individuals, SMEs, contractors | 30 days; NRT | Contractors paid by cheques passed along, trade credit | Cheque images, endorser identities, relationships, underlying invoices |
| SCN-CHQ-03 | At least 80 percent of a cash deposit out by cheque or transfer to others within 48 hours, at least twice in 30 days [DP] | All | 48 hours and 30 days; NRT | Rent and wage payments | Deposit slip, cheque payees, relationships |
| SCN-CHQ-04 | Any use of uncleared funds above the cleared balance followed by a return; at least 3 returned cheques in 30 days [DP]; uncleared-credit use above USD 10,000 followed within 5 days by an outward foreign transfer is a P2 candidate | All; clearing days from the C-ACH rules (file 25 section 12.1) | 5 and 30 days; EOD and event on return | Genuine signature mismatches | Cheque image, drawer, return reason, funds trail |
| SCN-CHQ-05 | At least 2 loops among 2 to 3 accounts within 5 days in 30 days [DP] | Companies and individuals | 5 and 30 days; EOD | Supplier-customer cross payments | Cheque graph, dates, balances |
| SCN-CHQ-06 | At least 3 cheque deposits at 80 to 99 percent of USD 10,000 in 30 days [DP] | All | 30 days; EOD | Fixed-amount rents or instalments | Cheque list, drawers, purposes |

AKR-TMS hooks:
- **Cheque instrument record** (drawer, payee, endorsers, image reference, clearing and return status) feeding CHQ, PUB and TRADE scenarios.
- **Uncleared-funds guard**: the available-versus-cleared balance is a first-class field so SCN-CHQ-04 can compare what was spent with what cleared.
- **Endorsement graph** linking cheque chains to the party graph.

### B19. Family FX: currency exchange and dollarisation

Foreign exchange is where Iraq's dollar rationing meets ordinary customers. The CBI catalogue has six FX indicators (Art. 9), two general ones
(Art. 3 nos. 8 and 38) and the 2016 procedures set per-person caps. Library rule: IQD and USD are separate dimensions in every FX scenario,
and the exchange rate used (official CBI rate or a parallel-market premium) is recorded with each alert, because the premium is what makes the
abuse profitable (a parallel-market series is not part of the data set; the bank supplies it if it wants this view).

**Table 1: what each FX scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-FX-01 | Foreign-currency buying and selling above profile or caps; groups exchanging dinars. Typologies: 1. | Implements Art.9 nos. 1, 3, 6; Art.3 no. 8. Also: CBI FX procedures 2016 | Individual FX purchases above the per-person cap without documents, repeated purchases beyond the declared need, or many customers exchanging dinars into several currencies at one counter in a short period | fx_deal, transaction, customer expected profile, branch |
| SCN-FX-02 | Chains of FX transactions to hide the beneficiary. Typologies: 1, 3. | Implements Art.9 no. 5 | Successive conversions across currencies and accounts within days that end where they began or at a third party | fx_deal, transaction |
| SCN-FX-03 | Foreign-currency cash deposits with refused or unexplained source. Typologies: 4, 9. | Implements Art.9 nos. 3, 4, 2 | Large foreign-currency cash deposits with no source evidence recorded, or counterparties in drug-producing countries | cash_event, transaction, counterparty (country) |
| SCN-FX-04 | Foreign-currency deposits inconsistent with the declared income and source. Typologies: 21. | Implements Art.9 no. 6. Extends Art.1 no. 17 | Foreign-currency inflows far above the foreign-currency income declared in the KYC form | transaction, customer expected profile |

**Table 2: how each FX scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-FX-01 | More than USD 3,000 per person in 30 days without recorded documents [CBI-2016]; at least 5 different customers exchanging at one branch within 2 hours [DP] | Individuals; walk-ins reported separately; values configurable because the 2016 cap's current status is not verified | 30 days and 2 hours; NRT; P2 candidate for the cap | Hajj and Umrah, tuition, medical travel, exchange-rate rushes | Documents on file, purposes, other branches, linked customers |
| SCN-FX-02 | At least 3 conversion legs within 5 days returning to the original currency or to a third party [DP] | Individuals and SMEs | 5 days; EOD | Treasury operations of companies | Leg list, accounts, counterparties |
| SCN-FX-03 | Deposit of at least USD 5,000 in foreign cash with no source evidence; any counterparty in a country on the drug-producing list [DP] | Individuals and SMEs | Event; NRT | Sale of foreign-currency assets, expatriate savings | Source evidence, declaration, travel record |
| SCN-FX-04 | Foreign-currency inflows above 120 percent of declared foreign-currency income [KYC] | Customers with a declared foreign-currency income | Month and 90 days; EOD | Remittances from relatives | Declared source, inflow list, originator |

AKR-TMS hooks:
- **FX caps and eligibility rules** as configurable hard rules with effective dates and an "unverified status" label until the bank confirms
  what applies (the 2016 numbers are the only published ones read for this research).
- **Rate table** with the official and, optionally, a parallel-market series, stored with each alert for context.
- **Counter-level aggregator**: customers by branch, hour and currency pair, so "several customers exchanging dinars" is one query.

### B20. Family OTH: lockers, securities, insurance and term deposits

The CBI lists four safe-deposit locker indicators (Art. 10), four securities indicators (Art. 11) and general scenarios on walk-in fixed
deposits (Art. 3 no. 10), securities trading (no. 18) and single-premium insurance (no. 47). These products are small in Iraq (securities 0.5 percent
and insurance 0.5 percent of financial-sector assets, MER Table 1.1) but the CBI names them, so each needs an implementation or an honest
"not monitorable" statement.

**Table 1: what each OTH scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-OTH-01 | Safe-deposit locker anomalies. Typologies: 4, 17. | Implements Art.10 nos. 1, 2, 3, 4 | Frequent visits, several lockers per customer or group, visits by people far from the branch, visits near large cash movements | locker, locker_access, cash_event |
| SCN-OTH-02 | Securities: cash-funded purchase, quick sale and withdrawal; trading without purpose. Typologies: 17. | Implements Art.11 nos. 1, 2, 3, 4; Art.3 no. 18 | Cash used to buy long-term securities, sold and withdrawn within weeks; accounts funded steadily with few trades then emptied; repeated trades without a purpose; transfers between accounts without reason; amounts out of line with the customer's profile (the knowledge check in Art.11 no. 1 is observational, see SCN-OBS-01) | securities (trades), transaction, account |
| SCN-OTH-03 | Walk-in fixed deposits without an account. Typologies: 17. | Implements Art.3 no. 10 | Any term deposit opened for a walk-in customer without an account relationship | product (term deposit), customer (occasional) |
| SCN-OTH-04 | Single-premium insurance and early surrender. Typologies: 17. | Implements Art.3 no. 47 | A large single-premium policy partly withdrawn or surrendered soon after, out of line with activity | insurance or bancassurance data (optional), transaction |

**Table 2: how each OTH scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-OTH-01 | At least 6 visits in a month; at least 3 lockers per customer or related group; visit within 24 hours before or after a cash movement of at least USD 5,000 [DP] | Locker customers | Month; MO | Valuables storage, collectors | Access log, locker list, cash movements, residence |
| SCN-OTH-02 | Buy and sell within 30 days and at least 90 percent withdrawn; at least 20 trades in 30 days without net position change; daily deposits or withdrawals within 90 percent of the USD 10,000 broker cap [MER] | Securities customers | 30 days; EOD | Active traders | Trade list, funding sources, withdrawals |
| SCN-OTH-03 | Any such deposit of at least USD 3,000 [DP]; hard rule, P2 candidate | Walk-in term deposits | Event; NRT | Staff error in customer type | Deposit record, ID, source of funds |
| SCN-OTH-04 | Surrender or withdrawal of at least 30 percent within 12 months of a premium of at least USD 10,000 [DP]; grade C because insurance data is outside the CBS | Insurance products sold through the bank | Event; MO | Genuine cash needs | Policy file, premium source, surrender request |

AKR-TMS hooks:
- **Locker access feed** where the bank's locker module records visits, joined to cash events on the same customer.
- **Securities feed** where the bank offers brokerage; broker caps (USD 10,000 a day, MER) as configurable limits.
- **"Not covered" labels** in the CBI coverage report for products the bank does not offer or cannot supply data for.

### B21. Family OBS: branch observations and behavioural indicators (hybrid alerts)

The CBI's circular of 26 August 2026 says behavioural indicators "depend on human observation and many cannot be detected by AML systems unless front-line
staff document them", and orders banks to build automated scenarios for "transaction restructuring, splitting (structuring) and repeated
patterns" so that "automated monitoring and behavioural monitoring reinforce each other" (file 01 section 2c). Article 12 of the 2025
controls lists 15 customer-behaviour indicators, and several general scenarios (Art. 3 nos. 5, 6, 9, 11, 25) are also observational. A read-only replica
cannot see them; the branch observation log (file 31, file 28) can, and OBS scenarios combine log entries with transaction patterns.
Library rule: an observation is a first-class event on the customer timeline, entered in seconds, confidential to the MLRO channel (chapter 11
of the 2025 controls: branch liaison officers, موظفو الارتباط, report to the MLRO only, file 01 section 2d), and never visible to branch management or to the customer.

**Table 1: what each OBS scenario detects**

| ID | Scenario and typology | CBI link | Logic | Canonical data |
|---|---|---|---|---|
| SCN-OBS-01 | Avoiding due diligence. Typologies: 20. | Implements Art.3 no. 6; Art.12 nos. 6, 7, 9, 14. Also: circular group 1 | Refusal or avoidance of CDD information or documents, inconsistent or unverifiable answers, substitute documents, evasive source of funds, followed by activity | observation log, customer (CDD status), transaction |
| SCN-OBS-02 | Questions about limits, then restructuring. Typologies: 20. | Implements Art.12 no. 4. Extends Art.1 nos. 8, 12. Also: circular group 1 | An enquiry about reporting limits logged by staff, then splitting or reducing within days | observation log, transaction, SCN-CASH-02 and -13 outputs |
| SCN-OBS-03 | Third-party direction and companions (money-mule herder). Typologies: 12, 16, 20. | Implements Art.3 no. 25; Art.12 no. 11. Also: circular group 4 | The same companion, translator or ID holder logged with several customers' transactions, or a customer reading instructions from notes | observation log (companion ID), party_identifier |
| SCN-OBS-04 | Bribery, pressure or threats toward staff. Typologies: 18, 20. | Implements Art.3 no. 11; Art.12 nos. 2, 5, 8, 12. Also: circular group 5 | Offers of benefits, invoked influence, pressure to skip checks or emotional pressure, logged by the employee | observation log (confidential channel) |
| SCN-OBS-05 | Physical indicators on cash. Typologies: 9, 20. | Implements Art.3 no. 12. Also: circular group 3 | Chemical smell, mould, dyed notes, non-bank packaging, small disordered notes, logged with the deposit | observation log, cash_event |
| SCN-OBS-06 | Terrorist-financing behaviours and authority notices. Typologies: 10, 20. | Extends Art.3 no. 32. Also: circular group 7 | Requests to transact for suspected persons, donations to personal accounts, refusal of source-of-funds information then cancelling | observation log, authority-request register |
| SCN-OBS-07 | Post-transaction behaviour: cancellations, reversals, edits, receipt refusals. Typologies: 20. | Extends Art.1 no. 8. Also: circular group 6 | Cancelling or reversing soon after execution, asking to change details after completion, refusing receipts, repeated rewriting of the justification | transaction_event (cancelled, reversed, amended), observation log |
| SCN-OBS-08 | Indicator accumulator and SAR escalation checklist. Typologies: 20. | Outside the Part A catalogue. Also: circular section 8 (seven escalation criteria) | A customer accumulates at least two different indicator types (behavioural, operational, financial) in a chain of linked transactions, or the same indicators recur, or explanations contradict, or the customer links to a high-risk party | observation log, all scenario outputs, party links |

**Table 2: how each OBS scenario runs**

| ID | Start parameters (basis) | Segments and exclusions | Window and mode | False-positive drivers | Analyst evidence |
|---|---|---|---|---|---|
| SCN-OBS-01 | A logged indicator plus a transaction of at least USD 1,000 in 30 days, or two logged indicators in 30 days [DP; circular section 8 criterion 1] | All customers | 30 days; NRT | Customers confused by forms, elderly customers | Log entries with staff notes, CDD status, transactions |
| SCN-OBS-02 | A limit enquiry followed within 7 days by two or more deposits or withdrawals each below the limit [CBI: circular group 1 and Art.1 no. 12] | Cash and transfer customers | 7 days; NRT | Customers asking out of curiosity | Log entry, deposit list, tellers |
| SCN-OBS-03 | The same companion ID on at least 3 different customers in 30 days [DP; basis circular group 4 "the same person accompanies several customers at different times"] | All; relatives flagged by family number shown, not suppressed | 30 days; EOD | Drivers, family helpers, translators | Companion ID, customers, transactions, CCTV request |
| SCN-OBS-04 | Any entry; routed to the MLRO only; no branch management visibility [CBI: 2025 controls ch. 11] | All | Event; NRT | Misinterpreted courtesy | Staff note, customer history, gift register |
| SCN-OBS-05 | Any physical indicator plus a deposit of at least USD 3,000 [DP] | Cash deposits | Event; NRT | Market stall cash, charity boxes | Log entry, denomination breakdown |
| SCN-OBS-06 | Any entry in the TF group; priority 1 [CBI circular group 7] | All | Event; NRT | Misunderstood requests | Entry, request text, graph |
| SCN-OBS-07 | At least 2 cancellations or reversals in 30 days, or any post-completion change to beneficiary or purpose details [DP; the circular asks for automation of these patterns] | Teller and digital channels | 30 days; NRT | Genuine errors | Event log, reasons, operator |
| SCN-OBS-08 | At least 2 distinct indicator types in 30 days, or the same indicator on 2 occasions in 90 days, or any one with a high-risk link [CBI: circular section 8 criteria 1, 6, 3] | All customers | 30 and 90 days; NRT | Accumulated minor signals | Indicator timeline with sources, checklist outcome, SAR decision record |

AKR-TMS hooks:
- **Branch observation log** (file 31): a fast form with the circular's eight groups as tick-lists, a free-text note, optional companion ID,
  and a written acknowledgment with the tipping-off reminder (2025 controls chapter 2 article 3, file 01 section 2d).
- **Hybrid alert**: an observation plus a transaction pattern on the same customer timeline, with the log entry shown to the analyst and
  hidden from the branch.
- **SAR escalation checklist** built into each case from circular section 8, so that "more than one indicator", contradictory explanations
  and recurrence are tick-boxes with evidence.
- **Training and competency records** for front-line staff, as the circular requires for inspection (file 01 section 2c).

### B22. Mapping the CBI catalogue to the library

**How the mapping is built.** The mapping below is not typed twice: it is generated from the "CBI link" column of the scenario tables in B4 to B21 (the
word "Implements" names the library scenario that is the primary automated implementation of a CBI item; "Extends" names scenarios that
cover a related pattern or reinforce it). The same generator runs in the product, so the CBI coverage report an assessor sees (item to
scenario to parameters to alerts to outcomes, Part A design implications) cannot drift from the scenario register. Grades are those of B1:
**A** operable from replica fields; **B** needs an optional CBS module or message data; **C** needs data outside the CBS; **D** observational.

**Result.** The CBI's 86 numbered scenarios (articles 1 to 3) and its 87 product and behaviour indicators (articles 4 to 14) make 173 items.
Every item except one (Art. 3 no. 9, constant travel to drug-producing countries, covered only by a card-use proxy) has at least one library
scenario that implements it. The library has 161 scenarios; 99 implement at least one CBI item and 62 are extensions beyond the CBI floor
(all of the CORR, PF and PUB families, the ISL bundles, and others).

| Grade | Articles 1-3 (86 items) | Articles 4-14 (87 items) | All 173 items | Meaning |
|---|---|---|---|---|
| A | 35 (41 percent) | 12 (14 percent) | 47 (27 percent) | Runs from ledger, account and KYC fields |
| B | 33 (38 percent) | 47 (54 percent) | 80 (46 percent) | Needs payment messages, trade, card, teller, locker, securities or facility data; degraded without it |
| C | 11 (13 percent) | 15 (17 percent) | 26 (15 percent) | Needs data outside the CBS: documents, customs, open sources, registers, logs, HR |
| D | 7 (8 percent) | 13 (15 percent) | 20 (12 percent) | Observational: only a person at the counter can record it |

<!-- MAP-START (generated; regenerate after any change to the CBI-link column) -->

**Article 1: cash operations (21)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.1 no.1 | Large cash deposits made unusually by a natural or legal person, inconsistent with their business or job ... | SCN-CASH-01 | - | A | profile completeness drives accuracy; falls back to the segment percentile |
| Art.1 no.2 | Repeated small cash deposits meant not to attract staff attention, whose total over a period does not fit the ... | SCN-CASH-02 | - | A | - |
| Art.1 no.3 | Use of multiple accounts to deposit cash totalling a very large amount in a very short time. | SCN-CASH-03 | - | A | linked depositors need the depositor-ID field |
| Art.1 no.4 | Repeated cash deposits at several branches of the same bank in a short period, by the account holder or through ... | SCN-CASH-03 | SCN-CASH-02 | A | - |
| Art.1 no.5 | Large deposits and withdrawals through ATMs to avoid contact with staff, especially if inconsistent with the ... | SCN-CASH-04 | - | B | needs ATM and cash-machine records from the switch or CBS channel codes |
| Art.1 no.6 | Large cash deposits and withdrawals on dormant or inactive accounts, relatively large withdrawals, or accounts ... | SCN-DORM-01, SCN-DORM-05 | - | A | - |
| Art.1 no.7 | Repeated withdrawal of funds shortly after deposit without clear justification. | SCN-CASH-05 | SCN-CASH-10, SCN-CHQ-03 | A | - |
| Art.1 no.8 | Customer takes back part of the cash to be deposited on learning that CDD is required. | SCN-CASH-13 | SCN-OBS-02, SCN-OBS-07 | D | happens before posting; visible only in a teller event log or the observation log |
| Art.1 no.9 | Customer submits financial statements clearly different from similar companies in the same sector. | SCN-LOAN-08 | - | C | needs submitted financial statements and sector ratios outside the ledger |
| Art.1 no.10 | Repeated or large outgoing transfers funded in cash, inconsistent with activity and size. | SCN-CASH-06 | - | A | - |
| Art.1 no.11 | Companies with relatively large activity submitting financial statements not audited by a licensed accountant. | SCN-LOAN-08 | - | C | needs the audited-statement flag from the credit or KYC file |
| Art.1 no.12 | Depositing or withdrawing cash in several stages so each amount is under the bank's threshold but the total ... | SCN-CASH-02, SCN-CASH-13 | SCN-CHQ-06, SCN-OBS-02 | A | - |
| Art.1 no.13 | Customer suddenly repays a large debt with no clear, reasonable explanation of the source. | SCN-LOAN-01 | SCN-LOAN-02 | B | needs the lending module (repayment source) |
| Art.1 no.14 | Customer exceeds their average withdrawal and deposit movements. | SCN-ACCT-02, SCN-CASH-01, SCN-CASH-09 | SCN-ACCT-01, SCN-ACCT-03, SCN-CASH-11, SCN-CASH-12, SCN-PUB-07 | A | - |
| Art.1 no.15 | Customer exceeds their average outgoing and incoming transfers. | SCN-ACCT-01, SCN-ACCT-02 | - | A | - |
| Art.1 no.16 | Cash withdrawal above the limit declared in the KYC form. | SCN-CASH-11 | - | B | needs the KYC cash-withdrawal limit populated |
| Art.1 no.17 | Monthly income above the expected income declared in the KYC form. | SCN-ACCT-01 | SCN-CASH-01, SCN-FX-04, SCN-PEP-01, SCN-PUB-07 | B | needs declared income populated |
| Art.1 no.18 | Customer unexpectedly transfers the value of credit facilities obtained to outside Iraq. | SCN-LOAN-03 | - | B | needs facility disbursement joined to payment messages |
| Art.1 no.19 | Repeated transfers to the same beneficiary within a set period. | SCN-XFER-09 | - | A | - |
| Art.1 no.20 | Large cash purchases of bank cheques. | SCN-CHQ-01 | - | B | needs cheque issuance records |
| Art.1 no.21 | Receipt of large cheques endorsed by third parties to the customer's account. | SCN-CHQ-02 | SCN-PUB-03 | B | needs endorsement data from clearing or images |

**Article 2: transfers (16)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.2 no.1 | Large transfers abroad, or incoming transfers from abroad, with instructions to pay out in cash. | SCN-CASH-06, SCN-CASH-10 | - | B | the payout instruction is in the message text |
| Art.2 no.2 | Repeated incoming transfers from different parties with no clear relationship to the customer (or outgoing from ... | SCN-XFER-01 | SCN-TF-01 | A | - |
| Art.2 no.3 | Equal or near-equal transfers to several persons in different countries, or to one beneficiary across several ... | SCN-XFER-02 | - | B | beneficiary country needs message data |
| Art.2 no.4 | Large local transfers followed by transfers abroad in different currencies. | SCN-XFER-04 | - | A | - |
| Art.2 no.5 | Deposits moved abroad directly, in one or several payments. | SCN-XFER-04 | SCN-XFER-05 | A | - |
| Art.2 no.6 | Large transfers from abroad into dormant accounts. | SCN-DORM-05 | - | A | - |
| Art.2 no.7 | Repeated or large outgoing transfers funded in cash, inconsistent with activity and size. | SCN-CASH-06 | - | A | - |
| Art.2 no.8 | Repeated transfers whose total over a period does not fit the customer's activity. | SCN-ACCT-01 | SCN-XFER-05, SCN-XFER-09 | A | - |
| Art.2 no.9 | Customer uses the account as a pass-through to move funds between parties or accounts. | SCN-XFER-03 | SCN-CORR-01, SCN-DORM-04 | A | - |
| Art.2 no.10 | Outgoing or incoming transfers in near-identical value ranges. | SCN-XFER-02 | - | A | - |
| Art.2 no.11 | Transfers to or from high-risk countries or regions, or parties on local (asset-freeze) or international ... | SCN-XFER-06 | SCN-PF-01, SCN-PF-08, SCN-TF-06 | B | counterparty country and listed parties need message data and screening feeds |
| Art.2 no.12 | Incoming transfers sent on in the same period or shortly after. | SCN-XFER-03 | SCN-CASH-10 | A | - |
| Art.2 no.13 | Transfers to or from PEPs and high-risk persons. | SCN-PEP-01, SCN-PEP-02 | - | B | needs the PEP and high-risk register |
| Art.2 no.14 | Identical personal details for several unrelated persons (address, phone, date of birth, passport no.). | SCN-XFER-07, SCN-XFER-08 | SCN-ACCT-11 | A | - |
| Art.2 no.15 | Different documents presented on each transfer compared with the bank's records. | SCN-XFER-07 | SCN-ACCT-11 | B | needs document type and number captured on each transfer |
| Art.2 no.16 | Refusal to present the passport of sender and recipient, which name screening against local and international ... | SCN-XFER-07, SCN-XFER-12 | - | D | refusal is observed; the replica sees only the empty ID fields (SCN-XFER-07) |

**Article 3: general scenarios (49)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.3 no.1 | Dormant account becomes active without reasonable cause. | SCN-DORM-01 | SCN-DORM-04 | A | - |
| Art.3 no.2 | Exchanging large quantities of small notes for large notes without clear reason. | SCN-CASH-07 | - | B | needs denominations from the teller module |
| Art.3 no.3 | Unusually high cash deposits withdrawn by cheque and moved to other accounts repeatedly without convincing ... | SCN-CHQ-03 | SCN-CASH-05, SCN-CASH-06, SCN-CASH-14 | B | needs cheque records |
| Art.3 no.4 | Not using letters of credit for foreign trade and insisting on cash transfers. | SCN-TRADE-08 | - | B | needs the trade module |
| Art.3 no.5 | No logical reason for the customer's choice of branch. | SCN-CASH-03 | - | A | - |
| Art.3 no.6 | Customer refuses to give information or documents required for CDD. | SCN-ACCT-12, SCN-OBS-01 | - | D | refusal is observed; the replica sees an incomplete CDD status |
| Art.3 no.7 | Customer refuses credit facilities (extra benefits) to avoid giving information that could reveal the real ... | SCN-LOAN-07 | - | D | needs a credit-offer or decision log |
| Art.3 no.8 | Several customers come to exchange dinars into various foreign currencies with no sign of need. | SCN-FX-01 | - | B | needs counter-level FX deals |
| Art.3 no.9 | Customers who travel constantly to countries known for drug production or trafficking or illicit trade. | - | SCN-DIGI-14 | C | no travel data in the CBS; proxy only from card use abroad |
| Art.3 no.10 | Walk-in customers making fixed deposits without opening accounts. | SCN-OTH-03 | - | A | - |
| Art.3 no.11 | Suspicious behaviour: visible nervousness, sometimes threatening the employee to stop a suspicion report or ID ... | SCN-OBS-04 | - | D | observation only |
| Art.3 no.12 | Deposits containing counterfeit or heavily used, dirty, stored banknotes. | SCN-CASH-07, SCN-OBS-05 | - | D | physical; counterfeit logs help (SCN-CASH-07) |
| Art.3 no.13 | Customers holding several accounts their work does not need, especially with transactions involving unknown ... | SCN-ACCT-04 | SCN-CASH-03, SCN-CORR-02, SCN-ISL-01, SCN-PF-02, SCN-TRADE-11, SCN-XFER-13 | A | - |
| Art.3 no.14 | Large cheques endorsed by a third party to the customer with no clear relationship, or not consistent with the ... | SCN-CHQ-02 | SCN-PUB-03 | B | needs cheque endorsement data |
| Art.3 no.15 | Multiple transfers from abroad into one account, repeatedly just **below the USD 10,000** reporting threshold. | SCN-XFER-05 | SCN-CHQ-06 | A | - |
| Art.3 no.16 | Transfers from banking-secrecy jurisdictions. | SCN-XFER-06 | - | B | needs counterparty country |
| Art.3 no.17 | Persons or companies attracting large sums to invest in FX or tradeable instruments when deal size does not fit ... | SCN-DIGI-15 | SCN-ISL-05 | B | needs payment narratives |
| Art.3 no.18 | Repeated buying or selling of securities without clear purpose. | SCN-OTH-02 | - | B | needs the securities feed |
| Art.3 no.19 | Sudden early repayment of long-term loans without declaring the source; facilities secured by jewellery or real ... | SCN-LOAN-01, SCN-LOAN-06 | - | B | needs facility data |
| Art.3 no.20 | Loans secured by assets owned by others with no clear relationship. | SCN-LOAN-04 | - | B | needs collateral links |
| Art.3 no.21 | Several accounts tied to the same beneficiary or agent with the same data, addresses and job details, aimed at ... | SCN-DIGI-02, SCN-DIGI-08, SCN-XFER-08 | - | A | - |
| Art.3 no.22 | Open-source reports linking a person or entity to a past crime, especially human trafficking, migrant smuggling ... | SCN-PEP-08 | SCN-TF-06 | C | external open-source data |
| Art.3 no.23 | Attempts to open accounts with documents that look forged. | SCN-ACCT-11 | - | C | document-verification results; refused attempts never reach the CBS |
| Art.3 no.24 | Young customer with unclear account purpose, source of funds or transaction values. | SCN-ACCT-09 | - | A | - |
| Art.3 no.25 | A third party translates or directs the account holder during withdrawals or deposits, especially at peak ... | SCN-ACCT-06, SCN-OBS-03 | SCN-PUB-02 | D | observation only |
| Art.3 no.26 | Lifestyle not justified by business; profits or deposits far above peers in the same profession. | SCN-ACCT-08 | SCN-ACCT-01, SCN-CASH-01, SCN-CASH-09 | A | - |
| Art.3 no.27 | Dealings with persons or entities about whom the bank previously filed STRs. | SCN-XFER-10 | SCN-PEP-02, SCN-TF-06 | C | needs the bank's STR and internal-report register |
| Art.3 no.28 | Transactions by women, especially young, inconsistent with their activity, repeatedly sending or receiving ... | SCN-XFER-11 | - | B | needs the remittance-channel flag |
| Art.3 no.29 | Transactions of some legal persons (e.g. beauty salons) inconsistent with their activity, or foreigners ... | SCN-ACCT-13 | SCN-CASH-09 | A | - |
| Art.3 no.30 | Negative open-source information about sexual exploitation, marriage of minors or similar. | SCN-PEP-08 | - | C | external open-source data |
| Art.3 no.31 | Cash deposits to or from doctors, nurses, medical and hospital staff disproportionate to their work. | SCN-CASH-09 | SCN-CASH-01 | A | - |
| Art.3 no.32 | Unjustified dealings between charities (or their staff) and parties unrelated to humanitarian work. | SCN-TF-03 | SCN-ISL-07, SCN-OBS-06, SCN-TF-02 | B | needs the NPO flag and registry |
| Art.3 no.33 | Several accounts with several signatories who have no clear relationship. | SCN-ACCT-04, SCN-XFER-08 | - | A | - |
| Art.3 no.34 | Beneficial owner belongs to countries or regions known for crime, or high-risk countries. | SCN-PEP-09, SCN-XFER-06 | SCN-PF-02, SCN-TRADE-11, SCN-TRADE-12 | B | beneficial-owner data is mostly declared |
| Art.3 no.35 | Customer changes the place of payment named in the LC or invoice. | SCN-TRADE-03 | SCN-TRADE-04 | B | needs LC amendment history |
| Art.3 no.36 | Suspected under- or over-invoicing. | SCN-TRADE-01, SCN-TRADE-07 | SCN-ISL-02, SCN-TRADE-02, SCN-TRADE-12 | C | needs customs and price data |
| Art.3 no.37 | Several credit cards issued to the same person in a short time. | SCN-DIGI-08 | - | B | needs card issuance records |
| Art.3 no.38 | Customer holding a senior government post withdraws all IQD balances and deposits them in USD. | SCN-PEP-03 | - | B | needs the position or PEP flag |
| Art.3 no.39 | ATM withdrawals in a high-risk area far from the customer's address (card may not be in the customer's hands; ... | SCN-DIGI-09 | SCN-CASH-12 | C | needs terminal location from the card processor |
| Art.3 no.40 | Suspected illegal FX trading: card funded with IQD cash, then USD cash withdrawals from ATMs abroad. | SCN-DIGI-07 | - | B | needs card switch data |
| Art.3 no.41 | Repeated card purchases of large USD amounts abroad. | SCN-DIGI-07 | - | B | needs card switch data |
| Art.3 no.42 | Cash placed in a term deposit out of proportion to the customer's young age and unclear activity. | SCN-ACCT-09 | - | A | - |
| Art.3 no.43 | Purpose of a transfer from a customer to an account at another local bank cannot be verified, nor the ... | SCN-XFER-09 | - | B | needs the purpose field |
| Art.3 no.44 | Repeated deposits for third parties by bank employees with no stated relationship. | SCN-CASH-08, SCN-INS-02 | - | B | needs operator and depositor fields |
| Art.3 no.45 | Housewife of unknown activity repeatedly receiving small cash transfers from several people in several ... | SCN-XFER-01, SCN-XFER-11 | SCN-CASH-14, SCN-DIGI-01, SCN-TF-01 | A | - |
| Art.3 no.46 | Suspected use of credit cards for virtual-currency speculation, with deposits onto the card from unknown ... | SCN-DIGI-10 | SCN-TF-05 | B | needs MCC and merchant data |
| Art.3 no.47 | Single-premium insurance policy for a large amount, partly withdrawn soon after, inconsistent with activity. | SCN-OTH-04 | - | C | insurance data is outside the CBS |
| Art.3 no.48 | Social-media worker receiving fragmented, continuous cash transfers from abroad below the threshold. | SCN-XFER-01, SCN-XFER-05 | SCN-DIGI-01 | A | - |
| Art.3 no.49 | Large cash deposit backed by recent deferred-dowry (divorce) documents, followed by fragmented transfers to ... | SCN-CASH-14 | - | C | needs the supporting document and its data |

**Article 4: documentary letters of credit (9)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.4 no.1 | import/export inconsistent in type or value with the customer's business | SCN-TRADE-01, SCN-TRADE-09 | - | B | trade module |
| Art.4 no.2 | large gap between goods value in the LC and real value | SCN-TRADE-01, SCN-TRADE-07 | SCN-TRADE-10 | C | needs customs or price data |
| Art.4 no.3 | request to change the LC beneficiary before payment without clear reason | SCN-TRADE-04 | - | B | trade module |
| Art.4 no.4 | many LCs out of line with activity | SCN-TRADE-04, SCN-TRADE-13 | - | B | trade module |
| Art.4 no.5 | LCs against financial guarantees out of line with activity size and history | SCN-TRADE-05 | SCN-TRADE-13 | B | trade module |
| Art.4 no.6 | unusual payment terms or payment to foreign parties unrelated to the LC | SCN-TRADE-03 | - | B | trade module and messages |
| Art.4 no.7 | LC beneficiary or shipping company owned by the customer | SCN-TRADE-05 | SCN-ISL-01 | B | trade module and registry |
| Art.4 no.8 | amounts in submitted LC documents differ from originals | SCN-TRADE-04 | SCN-TRADE-10 | C | document comparison is manual or OCR-assisted |
| Art.4 no.9 | place of payment changed to an account in a country other than the beneficiary's | SCN-TRADE-03 | - | B | trade module and messages |

**Article 5: letters of guarantee (4)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.5 no.1 | many LGs out of line with activity | SCN-TRADE-06, SCN-TRADE-13 | - | B | trade module |
| Art.5 no.2 | LGs against financial guarantees out of line with activity and history | SCN-TRADE-06 | SCN-TRADE-13 | B | trade module |
| Art.5 no.3 | beneficiary calls the LG soon after issue without clear reason | SCN-TRADE-06 | - | B | trade module |
| Art.5 no.4 | LGs issued without underlying project contracts | SCN-TRADE-06 | - | B | contract-on-file flag |

**Article 6: credit facilities (10)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.6 no.1 | loans secured by others' assets with no clear link | SCN-LOAN-04 | - | B | collateral links |
| Art.6 no.2 | facilities against guarantees from a bank abroad without clear reason | SCN-LOAN-04 | - | B | collateral links |
| Art.6 no.3 | borrower asks to move the loan quickly to banks abroad | SCN-LOAN-03 | - | B | facility and messages |
| Art.6 no.4 | unexpected early repayment of distressed debt by the customer or third parties | SCN-LOAN-02 | SCN-ISL-02, SCN-LOAN-01, SCN-LOAN-06 | B | facility and payer |
| Art.6 no.5 | buying CDs from time to time and later pledging them for facilities | SCN-LOAN-05 | - | B | deposit, certificate and facility links |
| Art.6 no.6 | facilities against blocked deposits of a company or subsidiaries abroad, especially in drug-producing countries | SCN-LOAN-05 | - | B | collateral country |
| Art.6 no.7 | facilities secured by cash deposits | SCN-LOAN-05 | SCN-TRADE-06 | B | margin deposits |
| Art.6 no.8 | sudden repayment of a large debt with no clear source | SCN-LOAN-01 | SCN-ISL-02 | B | facility and payer |
| Art.6 no.9 | facility refused because of doubts about the guarantees | SCN-LOAN-07 | - | D | credit decision log |
| Art.6 no.10 | unaudited financial statements | SCN-LOAN-08 | - | C | audited-statement flag |

**Article 7: digital banking (10)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.7 no.1 | many small digital inflows then large digital transfers abroad | SCN-DIGI-01 | SCN-DIGI-12 | B | channel records |
| Art.7 no.2 | regular large digital payments from countries known for drugs, arms or human trafficking | SCN-XFER-06 | SCN-DIGI-03 | B | counterparty country |
| Art.7 no.3 | repeated online transfers between the customer's own accounts for no reason | SCN-DIGI-13 | - | A | - |
| Art.7 no.4 | regular large digital deposits or receipts from other high-risk countries | SCN-XFER-06 | - | B | counterparty country |
| Art.7 no.5 | online account opening with forged or misleading documents to get preferential services | SCN-ACCT-11 | SCN-DIGI-11 | C | onboarding system results |
| Art.7 no.6 | digital channels used for repeated transfers to different people without reason | SCN-DIGI-12 | SCN-XFER-01 | B | channel records |
| Art.7 no.7 | sending/receiving transfers while logged in from high-risk countries | SCN-DIGI-03 | SCN-PF-08 | C | IP and geolocation logs |
| Art.7 no.8 | changing **IP addresses** or using technical means to hide tracking | SCN-DIGI-02, SCN-DIGI-03 | SCN-DIGI-11 | C | IP and device logs |
| Art.7 no.9 | funds moved from several prepaid accounts to one person or entity | SCN-DIGI-04 | - | B | wallet and prepaid records |
| Art.7 no.10 | services used in many locations, regions and currencies | SCN-DIGI-14 | SCN-DIGI-09 | C | geolocation logs |

**Article 8: payment cards (5)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.8 no.1 | repeatedly using the full card limit then paying it all off | SCN-DIGI-06 | - | B | card records |
| Art.8 no.2 | repeatedly hitting the daily cash-withdrawal maximum | SCN-DIGI-06 | - | B | card records |
| Art.8 no.3 | account funded constantly with few or no transactions, then withdrawn | SCN-DIGI-01, SCN-DIGI-05 | - | B | card and balance records |
| Art.8 no.4 | unjustified transfers between the customer's accounts or with authorised persons | SCN-DIGI-13 | - | A | - |
| Art.8 no.5 | ATM cash deposit in one currency followed by ATM withdrawals in another | SCN-DIGI-07 | - | B | card switch data |

**Article 9: foreign exchange (6)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.9 no.1 | repeated FX buying or selling whose total does not fit the activity | SCN-FX-01 | SCN-TRADE-02 | A | - |
| Art.9 no.2 | counterparty located in drug-producing countries | SCN-FX-03 | SCN-XFER-06 | B | counterparty country |
| Art.9 no.3 | repeated, unexpected, relatively large domestic or foreign cash transfers in foreign currency | SCN-FX-01, SCN-FX-03 | SCN-CORR-02, SCN-TRADE-02 | A | - |
| Art.9 no.4 | refusal to explain the source of foreign currency deposited | SCN-FX-03 | - | D | refusal is observed; a missing recorded source is visible (SCN-FX-03) |
| Art.9 no.5 | chains of FX transactions to hide the real beneficiary | SCN-FX-02 | SCN-CORR-02 | B | FX deal records |
| Art.9 no.6 | FX deposits inconsistent with the source of funds and income declared in the KYC form | SCN-ACCT-01, SCN-FX-01, SCN-FX-04 | SCN-TRADE-02 | B | declared income populated |

**Article 10: safe-deposit lockers (4)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.10 no.1 | unusually frequent visits | SCN-OTH-01 | - | B | locker module |
| Art.10 no.2 | non-residents of the area keeping lockers without reason, especially if available near them | SCN-OTH-01 | - | B | locker module |
| Art.10 no.3 | heavy locker use suggesting cash storage | SCN-OTH-01 | - | B | locker module |
| Art.10 no.4 | renting many lockers without reason | SCN-OTH-01 | - | B | locker module |

**Article 11: securities (4)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.11 no.1 | huge amounts without basic knowledge of securities risk | SCN-OTH-02 | - | D | knowledge of risk is observed; value out of profile is monitored (SCN-OTH-02) |
| Art.11 no.2 | cash deposited to buy "long-term" securities, sold and withdrawn shortly after | SCN-OTH-02 | SCN-ISL-05 | B | securities feed |
| Art.11 no.3 | account funded constantly with few transactions, then withdrawn | SCN-DIGI-05, SCN-OTH-02 | SCN-ISL-05 | B | securities feed |
| Art.11 no.4 | unjustified transfers between accounts | SCN-OTH-02 | - | B | securities feed |

**Article 12: customer behaviour (15)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.12 no.1 | avoids staff (constant ATM use, evades officers) | SCN-CASH-04 | - | B | channel-use proxies |
| Art.12 no.2 | anxiety or confusion during the transaction | SCN-OBS-04 | - | D | observation only |
| Art.12 no.3 | many accounts without reason | SCN-ACCT-04 | - | A | - |
| Art.12 no.4 | persistent questions about AML rules and records, to learn how to avoid them | SCN-OBS-02 | - | D | observation only |
| Art.12 no.5 | asks for excessive secrecy | SCN-OBS-04 | - | D | observation only |
| Art.12 no.6 | cancels when staff ask for missing information | SCN-OBS-01 | - | D | observation only |
| Art.12 no.7 | reluctance once told details must be reported | SCN-OBS-01 | - | D | observation only |
| Art.12 no.8 | abnormal behaviour (ignores high interest on a large balance, ignores basic facts of the deal) | SCN-OBS-04 | - | D | observation only |
| Art.12 no.9 | refuses ID documents | SCN-ACCT-12, SCN-OBS-01 | - | B | CDD status; the refusal itself is observed |
| Art.12 no.10 | student requesting large, unusual transfers or FX | SCN-ACCT-09 | SCN-ISL-02 | A | - |
| Art.12 no.11 | customer controlled by another person, unaware of what they are doing, or an elderly person accompanied by an ... | SCN-OBS-03 | SCN-ACCT-06 | D | observation only |
| Art.12 no.12 | gifts or bribes to staff, pressure not to verify ID | SCN-OBS-04 | SCN-INS-09 | D | observation only |
| Art.12 no.13 | phone numbers disconnected or not working | SCN-ACCT-07 | SCN-XFER-08 | C | messaging-gateway status |
| Art.12 no.14 | refuses to disclose business details or documents | SCN-ACCT-12, SCN-OBS-01 | - | D | the refusal is observed; incomplete CDD shows as a status |
| Art.12 no.15 | permanent address outside the branch's area | SCN-CASH-03 | SCN-ACCT-07 | A | - |

**Article 13: other indicators (11)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.13 no.1 | several unrelated signatories on one account, especially foreign nationals | SCN-ACCT-04 | SCN-ACCT-13, SCN-PEP-04 | A | - |
| Art.13 no.2 | using accounts belonging to others | SCN-ACCT-06 | SCN-CASH-08, SCN-PUB-01, SCN-PUB-02 | B | operator and presenter fields |
| Art.13 no.3 | beneficial owner from an area known for crime | SCN-PEP-09 | SCN-PEP-04 | B | BO data is mostly declared |
| Art.13 no.4 | accounts used to receive and pay out large sums unrelated to the holder's activity | SCN-XFER-13 | SCN-ISL-07, SCN-PUB-01, SCN-TF-03, SCN-XFER-03 | A | - |
| Art.13 no.5 | accounts that receive many cash deposits or transfers and are then closed or go dormant | SCN-ACCT-05 | - | A | - |
| Art.13 no.6 | sudden luxury out of line with economic status | SCN-ACCT-08 | SCN-PEP-01, SCN-PEP-06 | A | - |
| Art.13 no.7 | criminal record of the suspect, BO or a party | SCN-PEP-08 | - | C | adverse-media or authority data |
| Art.13 no.8 | purchase of high-value real estate, vehicles, jewellery | SCN-ACCT-08 | SCN-ISL-04 | A | - |
| Art.13 no.9 | proven document forgery | SCN-ACCT-11 | - | C | document-verification results |
| Art.13 no.10 | a party under investigation abroad | SCN-PEP-08 | - | C | authority or foreign data |
| Art.13 no.11 | fictitious contracts | SCN-TRADE-06 | SCN-ISL-01, SCN-ISL-08, SCN-TRADE-10 | B | trade and facility documents |

**Article 14: employee behaviour and KYE (9 indicators)**

| CBI item | What it says (shortened) | Implemented by | Also covered by | Grade | Replica limit or note |
|---|---|---|---|---|---|
| Art.14 no.1 | sudden rise in lifestyle and spending vs salary | SCN-INS-01 | - | B | staff flag and payroll data |
| Art.14 no.2 | loosely bypassing controls | SCN-INS-03 | SCN-DORM-06 | B | CBS user and override tables |
| Art.14 no.3 | helping execute transactions where the beneficiary or counterparty is not fully known | SCN-INS-03 | SCN-INS-02, SCN-INS-05 | B | CBS user tables |
| Art.14 no.4 | exaggerating a customer's credibility and means in reports to management | SCN-INS-04 | - | D | observation only |
| Art.14 no.5 | staying after hours without reason | SCN-INS-03 | - | B | login times |
| Art.14 no.6 | clear neglect of duties | SCN-INS-08 | - | C | case-system clocks |
| Art.14 no.7 | assets out of line with salary | SCN-INS-01 | - | C | assets outside the bank |
| Art.14 no.8 | dealing with persons known for lack of integrity | SCN-INS-06 | SCN-ISL-08 | C | HR or integrity data |
| Art.14 no.9 | breaching confidentiality, leaking documents | SCN-INS-09 | SCN-INS-05 | C | system access logs |
<!-- MAP-END -->

**Overlaps inside the CBI catalogue.** Several numbered items describe the same pattern, so a coverage report that counts 173 items overstates
the number of distinct patterns. Examples: Art. 1 no. 10 and Art. 2 no. 7 are identical; Art. 1 no. 6 and Art. 2 no. 6 both cover movements
on dormant accounts; Art. 1 nos. 14 and 15 overlap Art. 2 no. 8; Art. 1 no. 18 repeats Art. 6 no. 3; sudden or early repayment appears in Art.
1 no. 13, Art. 3 no. 19 and Art. 6 nos. 4 and 8; rapid onward movement in Art. 1 no. 7, Art. 2 no. 12 and Art. 3 no. 3; third-party endorsed
cheques in Art. 1 no. 21 and Art. 3 no. 14; unaudited statements in Art. 1 no. 11 and Art. 6 no. 10; amounts just under the threshold in
Art. 1 no. 12, Art. 3 nos. 15 and 48; shared personal details in Art. 2 no. 14, Art. 3 no. 21 and Art. 12 no. 13; changed place of payment
in Art. 3 no. 35 and Art. 4 nos. 3 and 9; over- and under-invoicing in Art. 3 no. 36 and Art. 4 no. 2; loans secured by others' assets in Art.
3 no. 20 and Art. 6 no. 1; card-based illicit FX in Art. 3 nos. 40, 41 and Art. 8 no. 5; many small unrelated senders in Art. 2 no. 2 and Art.
3 no. 45; young or student customers in Art. 3 nos. 24, 42 and Art. 12 no. 10; many accounts or signatories in Art. 3 nos. 13, 33, Art. 12 no.
3 and Art. 13 no. 1. The library implements each pattern once and maps every numbered item to it, which is why 99 scenarios implement 172
items. The coverage report keeps the CBI numbering so an examiner can trace every item.

**What cannot be monitored from a read-only replica, and why.** Forty-six of the 173 items (26 grade C, 20 grade D) need something the replica
does not hold. The bank must say so openly in its coverage report and show the route it uses instead.

| Group | CBI items | Why the replica cannot see it | Route in AKR-TMS |
|---|---|---|---|
| Observed at the counter (16 items, grade D) | Art.1 no.8; Art.2 no.16; Art.3 nos. 6, 11, 25; Art.9 no.4; Art.11 no.1; Art.12 nos. 2, 4, 5, 6, 7, 8, 11, 12, 14 | A person sees nervousness, coaching, bribery, reluctance, a refusal or lack of knowledge; the database sees only aftermath (a cancelled entry, an empty ID field, an incomplete CDD status) | Branch observation log (SCN-OBS-01 to -04, -08) and aftermath scenarios (SCN-CASH-13, SCN-XFER-07, SCN-ACCT-12, SCN-FX-03) |
| Physical cash (1 item, D) | Art.3 no.12 | Smell, mould, dye, packaging | Observation log plus counterfeit-detection results (SCN-CASH-07, SCN-OBS-05) |
| Credit decisions (2 items, D) | Art.3 no.7; Art.6 no.9 | A refused or declined facility is a decision, often held outside the CBS | Credit-decision log (SCN-LOAN-07) |
| Staff conduct (1 item, D) | Art.14 no.4 | Exaggerated credibility in reports to management is in documents and reviews | Case-system and audit samples (SCN-INS-04 proxy) |
| Documents and financial statements (7 items, C) | Art.1 nos. 9, 11; Art.3 nos. 23, 49; Art.6 no.10; Art.7 no.5; Art.13 no.9 | Submitted statements, forged documents, dowry papers and onboarding evidence sit in document stores and eKYC systems; refused account openings never reach the CBS | Document-verification and KYC document feeds, OCR extraction with human review (SCN-LOAN-08, SCN-ACCT-11, SCN-CASH-14) |
| Customs, price and document comparison (3 items, C) | Art.3 no.36; Art.4 nos. 2, 8 | Values must be compared with customs and market prices and original documents | Optional customs and price connectors (SCN-TRADE-01, -07) |
| Open-source, register and authority data (5 items, C) | Art.3 nos. 22, 27, 30; Art.13 nos. 7, 10 | Adverse media, the bank's own STR register, criminal records and foreign investigations are not CBS fields | Adverse-media feed, STR and authority-request registers (SCN-PEP-08, SCN-XFER-10, SCN-TF-07) |
| Channel and processor logs (5 items, C) | Art.3 no.39; Art.7 nos. 7, 8, 10; Art.12 no.13 | IP, device, geolocation, ATM terminal location and message delivery status live in channel, card-processor and messaging systems | Optional read-only links to those systems (SCN-DIGI-02, -03, -09, -14, SCN-ACCT-07) |
| Travel and insurance (2 items, C) | Art.3 nos. 9, 47 | No travel record in the CBS; insurance sits outside the bank's ledger | Proxy from card use abroad for no. 9 (SCN-DIGI-14); insurance feed where the bank sells policies (SCN-OTH-04) |
| Staff data (4 items, C) | Art.14 nos. 6, 7, 8, 9 | HR, disclosures and system access logs are outside the CBS | HR upload and access-log feed (SCN-INS-06 to -09) |

**Grade B items need a named ask.** Eighty items depend on an optional module or on message data. For each bank the data-readiness view lists which
module is missing and which scenarios it blocks, so the bank can ask its CBS vendor or payment gateway for exactly those fields (file 25
section 13.6).

AKR-TMS hooks:
- **CBI baseline pack register**: 173 items, each with its implementing scenarios, parameter set and version, data-readiness grade, alert count,
  case count and STR count for the last 12 months, producing the coverage matrix an assessor under standard D2 will ask for (B25).
- **Not-covered declarations** with owner, reason, interim route and review date; a CBI item with no operating scenario and no declaration is an
  open finding on the compliance dashboard (file 20 L-02).
- **Generator parity test**: the coverage report and the scenario register are produced by one query; a release that breaks the link fails its tests.

### B23. Coverage against the bank's own risk assessment

The CBI requires each bank to build "a detailed risk matrix" over four elements (customer, product, delivery channel, geography) and to "load the
AML/CFT Office's scenarios into the AML system" (2025 controls chapter 2 article 2, file 01 section 2d). The FATF, MAS and CBUAE texts say the same
thing from the other side: scenarios follow the risk assessment, so coverage gaps can be shown (file 05 section 5c, point 1; MAS 2018
paragraph 2.7; CBUAE "typology assessment"). The CBI catalogue is a national list; the bank's own coverage is a second question.

**Method (a monthly report, and a review at every risk-assessment update).**

1. Import the bank's risk statements (one row per risk: customer type, product, channel, geography, typology, inherent rating, control
   strength, residual rating) from the enterprise risk assessment. Each statement is tagged with the typologies of B2.
2. Map each statement to library scenarios through the typology tags, and show for each mapping: the scenario's data-readiness grade at this
   bank (A to D, or blocked), whether it ran in the last 90 days, alerts, cases and STRs over 12 months, and the date it was last tuned.
3. Report four lists: **risks with no scenario**; **risks whose scenarios are blocked or degraded** by missing data; **risks whose scenarios
   have produced no alerts although eligible volume exists** (dead-rule test, file 20 L-20); and **bank-added scenarios with no risk
   statement** (candidates to retire; CBI minimum scenarios are never retired, file 21 section 2b).
4. Feed the result back into the risk assessment (CBUAE: MIS "should feed back into an LFI's financial crimes risk assessment") and into the
   board report.

**Illustration: the NRA's threat and vulnerability list against the library.** The ratings are those of the 2022 NRA as reported in the 2024
MER (file 02). The scenario lists are what this library offers; the "residual gap" column says what no scenario can fix with replica data.

| NRA risk | NRA rating | Library scenarios | Typical grade | Residual gap |
|---|---|---|---|---|
| Corruption and embezzlement of public funds | High, ranked first | SCN-PUB-01 to -07; SCN-PEP-01 to -04; SCN-CHQ-02; SCN-INS-07; SCN-TRADE-01 | A or B; C for civil registry and declarations | No access to budget and contract data; PEP data quality; ghost-employee detection only where payroll batches are tagged |
| Drug trafficking (crystal, Captagon) | High, ranked first | SCN-CASH-02, -03, -08; SCN-XFER-01; SCN-ACCT-08; SCN-FX-03; SCN-OBS-03 | A or B | No Iraq typology study to calibrate on; no ML convictions to learn from |
| Smuggling of oil and derivatives | Moderate to high | SCN-PF-04 (Iranian oil sold as Iraqi); SCN-PUB-03; SCN-CASH-12; SCN-TRADE-09 | B or C | Tanker-truck smuggling leaves little bank trace; cash and sector patterns are indirect |
| Tax and customs evasion | Moderate | SCN-TRADE-01, -07, -09; SCN-ACCT-01, -13; SCN-XFER-13 | B or C | No customs or tax data without connectors |
| Trafficking in antiquities | Moderate | Only generic: SCN-CASH-01, -02, -03; SCN-ACCT-08 | A | No dealer segment or typology; stated as weakly covered |
| Forgery (documents, invoices, medical) | Moderate | SCN-ACCT-11; SCN-TRADE-01, -03, -07, -10 | B or C | Forged paper is seen only after document checks |
| Fraud (social media, unlicensed schemes) | Moderate | SCN-DIGI-01, -02, -11, -12, -13, -15; SCN-ACCT-09; SCN-XFER-01 | B or C | Channel logs optional; authorities report no cybercrime complaints (MER paragraph 59) |
| Human trafficking | Ranked 11th; credible sources: major threat in Kurdistan | SCN-ACCT-09; SCN-XFER-11; SCN-CASH-14; SCN-OBS-03 | A or B | Weak indicators; underreported |
| Terrorist financing (ISIL remnants, other groups) | High | SCN-TF-01 to -07; SCN-CORR-02, -03; SCN-DIGI-09; SCN-OBS-06 | B or C | Self-financed acts leave no signal (MER box 4.1) |
| Proliferation financing (Iran) | IO11 rated Low | SCN-PF-01 to -09; SCN-TRADE-02, -03, -12; SCN-CORR-05 to -07 | B or C | Vessel, customs and maritime data are optional |
| Real-estate sector vulnerability | High | SCN-ACCT-08; SCN-PEP-07; SCN-LOAN-06; SCN-ISL-04 | A or B | Registry data absent; the USD 380,000 rule is a hard check only on the bank's own deposits |
| Exchange offices | High | SCN-CORR-02, -03, -08; SCN-TF-04; SCN-PF-06 | B | Cannot see unlicensed counterparties' books |
| Banks (large cash deposits and transfers abroad) | Moderate to high | SCN-CASH-01 to -06, -10, -11; SCN-XFER-01 to -07 | A | None for the core pattern |
| E-payment companies | Moderate | SCN-DIGI-01 to -15; SCN-CORR-03, -08 | B or C | Provider ledgers are outside the bank |
| Legal persons: LLCs and foreign-company branches | Medium ML; moderate-high TF | SCN-TRADE-11; SCN-PF-02; SCN-XFER-08, -13; SCN-ACCT-04, -13 | B | Beneficial-owner data is mostly declared |
| NPOs (16 misused for TF) | Moderate | SCN-TF-02, -03; SCN-ISL-07 | B or C | Registry and programme data optional |
| Hawala and informal transfers | Named in the FATF action plan | SCN-XFER-01, -03, -05, -13; SCN-CASH-02, -08; SCN-CORR-02; SCN-CHQ-02 | A or B | Settlement offsets and couriers are invisible; no agreed standard indicators (FATF 2026, paragraph 146) |

**The four CBI risk elements against the families.** Customer risk (PEPs, NPOs, DNFBPs, exchange houses, non-residents, legal persons) is
carried by PEP, TF, CORR and ACCT; product risk (transfers, correspondent, stored value, new products) by XFER, CORR, DIGI, TRADE and LOAN;
delivery-channel risk (non-face-to-face, digital, agents) by DIGI and OBS; geography (border governorates, FATF lists, Kurdistan, sanctioned
countries) by XFER-06, CASH-12, TF-01, PF-01 and the region segment in every family.

**Four coverage measures a bank reports to its board and to the assessor.**

| Measure | Definition | Why |
|---|---|---|
| CBI item coverage | Share of the 173 CBI items with an operating scenario, by grade | The legal floor; shows Part A is met and what is not monitorable |
| Risk-statement coverage | Share of risk statements with at least one operating scenario | Shows the bank's own risk picture is monitored |
| Data-ready coverage | Share of mapped scenarios operable (not blocked or degraded) at this bank | Shows whether the data feed supports the scenarios on paper |
| Outcome coverage | Share of mapped scenarios with at least one alert in 90 days and at least one case in 12 months | Detects dead rules and unproductive typologies |

AKR-TMS hooks:
- **Risk-statement importer** and a **typology tag set** shared by the risk assessment, the scenario register and the STR typology field
  (STR statistics by typology, file 01 section 5).
- **Four lists** (no scenario; blocked or degraded; dead; no risk statement) as a monthly compliance report and a risk-assessment input.
- **Region and sector segments** (border governorate, Kurdistan Region, exchange, oil, NPO) so a risk statement about a place or sector
  maps to a segment with its own thresholds.

### B24. Tuning and testing methodology

Part A gives a bank 86 scenarios and almost no numbers: the CBI text says "large", "repeated", "unusual", "inconsistent with activity". Every threshold is
therefore a bank decision, and the MER found that no interviewed bank had built a scenario of its own (file 01 section 2). The methods below
are the common core of what foreign supervisors write about threshold setting and testing, adapted to a bank that reads its CBS through a
replica. Design proposals are marked [DP]; sources are quoted where they exist.

#### B24.1 Principles

1. **Calibrate to risk, never to capacity.** TD Bank removed nine scenarios and made every change reduce alerts; HSBC UK had thresholds that
   could never be reached; US Bancorp capped alerts (file 20 FM-05, FM-06). NYDFS Part 504 is concerned with changes made "for purposes of
   minimizing generated alerts" (file 29 section 9f). Rule: no parameter limits alert counts; every change carries a reason code (risk-driven or
   capacity-driven), the capacity-driven ones need below-the-line evidence and an approver outside operations (file 20 L-16 to L-19).
2. **Thresholds follow segments, and the basis of every value is written down.** HKMA: "appropriate customer segmentation reduces noise and
   improves the quality of alerts" and banks should document the analysis behind each threshold (file 29 section 5b). CBUAE: thresholds
   "tailored to the institution's risk profile and the specific product or service and customer type"; "maintain documentation that
   articulates ... current detection scenarios and their underlying assumptions, parameters, and thresholds" (CBUAE outreach, 2021).
3. **Test before and after a change.** MAS (September 2018, paragraph 2.11): "Before effecting recalibrations, especially material recalibrations,
   FIs should assess whether to back-test the changes using sample data" and verify afterwards that the updates "do not inadvertently
   compromise the organisation's conduct of TM"; all outcomes "properly justified and documented" and "senior management ... apprised".
4. **Measure quality, not only volume.** MAS case study C: semi-annual reviews "focused on rules that generated zero alerts, but overlooked
   ... rules that led to 200-400% increases in the volume of alerts", and used only alert counts "while neglecting ... quality-related
   metrics (e.g. alerts that led to information requests to front office and/or STRs filed, as well as feedback from the bank's financial
   intelligence unit)". The CBUAE asks for alerts, false positives, investigations and STRs per rule.
5. **No "no SAR left behind".** The Wolfsberg Group names re-detecting every historical SAR as a failure mode, says "simply lowering thresholds by
   10% is not meaningful false-negative testing" and that "aiming for 100% recall ... is likely to lead to an ineffective system"
   (file 04 section 2a). The bank records an accepted miss rate in its risk appetite.
6. **Independent validation.** FFIEC: the programming methodology and effectiveness "should be independently validated"; CBUAE: validators
   "not responsible for the development or use of the model and ... do not have a stake in whether a model is determined to be valid"
   (file 05 sections 1d, 4a).

#### B24.2 Initial thresholds: with history, and without

A bank that connects AKR-TMS usually has years of ledger history in the replica, so the first calibration can use it. "Without history"
applies to a digital bank, a new product, a new segment, a scenario that needs data the CBS never held, or a bank whose CBS keeps too little history.

| Situation | How the first threshold is set | Basis tag | Gate to move on |
|---|---|---|---|
| A regulatory number exists (USD 10,000 reporting and linked occasional transactions, IQD 1 million simplified due diligence, six-month dormancy, USD 380,000 real estate, IQD 10 million qard hasan, 2016 FX caps) | Use the number as written; structuring bands are set below it (70 to 99 percent) | [CBI], [MER], [CBI-2016] | None for hard rules; band checked by below-the-line sample |
| The KYC profile is populated | A ratio to the declared value (for example 150 percent of declared income, 100 percent of declared cash limit); PEP and EDD customers get a tighter ratio | [KYC] | KYC completeness above 80 percent in the segment, otherwise fall back to the next row |
| CBS history exists (at least 12 months and at least 200 customers in the segment) | Segment percentile (P99 for single events, P95 for ratios, robust z-score above 4 for own-history spikes), recomputed monthly, seasonal where needed; then above- and below-the-line tests | [PCT] | Replay and 30-day shadow pass; below-the-line sample within tolerance |
| Short or no history | Conservative anchors from the table of the scenario family; segments below 200 customers merged upward; peer priors from synthetic Iraq-shaped data (file 21 section 16e) used only as a plausibility check; review at 90 days | [DP] | 90 days or segment reaches 200 customers |
| A new typology with no bank cases | Start from the published red-flag text (FinCEN, FATF, UN, CBI circular), run in shadow, collect analyst feedback | [DP] | Three monthly cycles |

Two cautions. First, KYC fields are often overwritten in a CBS, so a replay of last year's alerts uses today's declared income: a look-ahead bias that
the back-test report must state. Second, heavy tails: cash and trade amounts are dominated by a few large customers, so mean and standard
deviation either miss or flood; use percentiles or median and MAD, separately for IQD and USD (file 29 section 9c table).

#### B24.3 Segmentation

Three levels (file 21 section 3b): static business segment (salaried government staff, pensioners, private employees, small traders, wholesale
and import traders, contractors, NPOs, exchange and transfer companies, PSPs, government bodies, students), behavioural cluster inside it
(cash-heavy retailer versus electronic trader; pass-through versus accumulator), and risk overlay (customer risk rating; "high-risk customers
get more stringent thresholds", HKMA). Rules: minimum segment size 200 customers, merge upward below it; large corporates and government bodies
are segments of one judged on their own history; clusters rebuilt quarterly in shadow and approved by the bank; a segment change is a material
change with an alert-volume impact analysis; check fairness of alert rates across governorates and customer groups (file 21 section 3d, file 29 section 9i).
MAS case study A is the warning: thresholds for premium segments set "at least double (and in one case, five times) the AUM that Bank A
expected these customers to have" made large-value detection ineffective.

#### B24.4 Above-the-line and below-the-line testing

Definitions. MAS (footnote 2): "ATL and BTL testing involves increasing and decreasing the pre-determined thresholds of TM rules in order to
identify the best possible configuration, taking into account factors such as the efficiency of existing rules, the alert-to-case ratios
obtained, and the extent of true hits and false positives/negatives generated by the tests performed". HKMA's wording: "adjusting current
thresholds and parameters to arrive at an optimal configuration". AKR-TMS uses the working definitions of file 29 section 9c: above-the-line
asks what would be lost if the threshold were raised and measures the productivity of alerts nearest the line; below-the-line samples activity
that did not fire and has experienced analysts review it blind, together with cases that reached the bank by other routes.

**Procedure for each scenario and segment (monthly sampling job, full report every six months for scenarios above the volume gate).**

| Step | What is done | Default (all [DP]) |
|---|---|---|
| 1 Rank | Rank fired alerts by distance above the threshold; rank unfired events by their share of the threshold | Share of threshold: near band 70 to 99 percent; far band 40 to 70 percent |
| 2 Sample above | Review alerts in the lowest quintile above the line and measure case and STR yield | 100 alerts |
| 3 Sample below | Blind review of events in the near band, the far band and a random draw from the rest; analysts do not see the rule | 100 per stratum |
| 4 Other-route cases | Take every case that arrived by another route in the period (branch referrals, authority requests, RFIs, STRs not raised by the system, FIU feedback) and test whether the rule would have fired; if not, record why | All |
| 5 Compare | Productive rate above and below; unique catches; workload at each candidate threshold; sensitivity curve (move one parameter at a time to find the knee, not one "optimal" point) | Curve over plus or minus 50 percent of the threshold |
| 6 Decide | Compare with the tolerance in the risk appetite statement; record the accepted miss rate; a below-the-line productive rate above tolerance sets "threshold review required" | Tolerance 2 percent for general scenarios, 0.5 percent for TF, PF and sanctions-evasion scenarios, approved by the board committee |

Sample sizes. If a below-the-line sample of n reviewed items contains no productive case, the 95 percent upper bound on the productive rate is
about 3 over n (rule of three, file 29 section 9c): n = 100 gives 3 percent, n = 300 gives 1 percent, n = 600 gives 0.5 percent. So a 0.5 percent
tolerance needs about 600 reviewed items per stratum in a quarter; small scenarios use pooled quarters. Seeded known-positive cases (synthetic
transactions that must alert) are added to each sample to check the reviewers as well as the rule (file 28 M-50).

#### B24.5 Back-testing on the bank's own history

1. **Replay** the draft scenario over 12 to 24 months of replica history using as-of data (the canonical model is bi-temporal, file 25 section 13.1) so
   the rule sees what the bank knew on each day; seasonal scenarios need a window that contains Ramadan, Eid and Arbaeen at least once.
2. **Compare** with outcomes in a label ladder (file 21 section 4d): STRs filed, authority and FIU requests, internal reports, cases closed with
   findings, accounts closed for AML reasons, branch referrals. STRs alone are too few: banks filed 451 in 2024, about 6 per bank (file 01 section 5).
3. **Report** alerts by segment, branch, governorate and month; overlap with other scenarios; precision and recall on the known set with
   confidence intervals and lag-aware windows; estimated analyst hours; the dead-rule and reach tests (what share of customers could ever
   trigger it; file 20 L-20); a unit-test pack with positive and negative cases, including the MAS pass-through case and the TD Zelle cap case.
4. **Do not tune to re-detect every old STR.** The promotion criteria are priority-risk coverage, precision, stability and explainability
   agreed in advance (Wolfsberg part II, file 04 section 2b), not 100 percent recall.

#### B24.6 Champion and challenger

A new parameter set or logic version (the challenger) runs in shadow beside the live version (the champion) on live data, produces no alerts, and is
compared on the same customers and on analyst outcomes for at least one full monthly cycle (two for seasonal scenarios). Wolfsberg part II calls the
classic parallel run "an artefact of a less advanced testing environment"; AKR-TMS therefore promotes against criteria set beforehand, records
criteria, results, approver, date and rollback point, and keeps the champion available for roll-back (file 29 section 9e). A permanent
challenger runs for scenarios with a high alert share or a high unique-catch count.

#### B24.7 The scenario card (template)

A scenario card is generated from the scenario register, test results and monitoring data, never typed by hand (file 29 section 9g; the "independent third
party could replicate it" test of SS1/23). Fields:

| Block | Field | Content |
|---|---|---|
| Identity | ID, name (Arabic and English), version, status, owner, approver | Status: draft, testing, shadow, live, retired |
| Purpose | Typology, CBI items implemented (article and number), risk statements covered, regulatory basis | Links to B2 typologies and the bank's risk assessment |
| Logic | Declarative definition (hash), plain-language logic, aggregation level (customer, linked parties), counting rules (posted events, reversals netted), settled-time delay | Aggregate, event or profile type (file 32 section 7.3) |
| Data | Inputs with canonical fields, source mapping version, null rate, last validation date, readiness grade and phase | A to D; "not operating as intended" if an input fails |
| Parameters | Per segment, currency and risk tier: value, basis tag, rationale, approval reference, effective date | [CBI], [KYC], [PCT], [DP] and so on |
| Segments and exclusions | Segments served, exclusions with owner, reason and expiry | Exclusions are governed objects (file 20 L-02) |
| Expected behaviour | Alert volume range per month, reach (share of customers who can trigger it), expected false-positive drivers | From the replay |
| Tests | Unit tests, replay result, ATL/BTL result and date, seeded-case result, challenger comparison | Pass or fail with dates |
| Performance | Alerts, cases, STRs, precision, unique catches, time to close, analyst hours, overlap | 3, 12 months, trend |
| Change history | Each change: reason code (risk or capacity), before and after, impact analysis, requester, second approver, post-change review at 30 and 90 days | Immutable |
| Review | Last tuned, next review date, trigger events, limitations, fairness result, phase-2 candidate flag | Recalibration clock (file 20 L-24) |

Example, abridged, for SCN-CASH-02 (structuring): purpose "cash placement below limits"; implements Art.1 nos. 2, 12; aggregate logic over 7 and 30 days
by customer and linked parties; data: cash transactions, branch, depositor ID (capture rate 71 percent at this bank, grade A degraded to B for
depositor aggregation); parameters: at least 3 deposits in 7 days, total at least USD 10,000 [CBI], band 70 to 99 percent [DP], segment overrides
for cash retailers; expected 40 to 120 alerts a month; replay found 3 of 4 known structuring cases; below-the-line sample of 300 had no productive
case (1 percent bound); next review in 6 months. (Figures are illustrative of the format, not results.)

#### B24.8 Review cadence

| Activity | Cadence | Trigger for an earlier review | Anchor |
|---|---|---|---|
| Threshold health check: alert volume, productivity, unique catches, flat-line and silence alarms | Monthly; quarterly report to the oversight committee | Volume up 200 percent or down 50 percent week on week for a scenario | MAS case study C; file 20 L-29 |
| Below-the-line sample | Monthly job; full report every 6 months for high-volume scenarios | Productive rate above tolerance | MAS semi-annual practice; file 20 L-19 |
| Full scenario review and coverage review against the CBI pack and typology inventory | Every 12 months | New product or channel, CBS upgrade, new typology circular, STR or authority post-mortem, audit finding | HKMA 12 to 24 months; SAMA yearly test; file 20 L-24 |
| Data integrity pack: reconciliation of transaction codes between CBS and AKR-TMS, null rates, lineage | Continuous; formal report every 12 to 18 months | CBS change, new code | CBUAE 12 to 18 months; file 20 L-04 to L-09 |
| Segmentation rebuild | Quarterly, in shadow | Cluster instability, fairness drift | File 21 section 3d |
| Independent validation of rules (configuration, replay, functional re-implementation) | Every 12 months, delta validation per release | Material change | FFIEC; CBUAE; file 29 section 9c |
| Post-mortem on high-suspicion STRs and production orders | Per event, sampled | Any STR with high suspicion, any false negative found by other routes | MAS paragraph 2.31, case study L |
| Reference-data review (country table, flagged districts, dual-use and vessel lists, sector-goods map, crypto counterparties) | Quarterly; effective-dated | Designations, FATF plenary, UN or OFAC changes | File 04 U-08; file 20 L-25 |

#### B24.9 Go-live gate for any scenario

A scenario goes live only when: its card is complete; unit tests pass (a failing test blocks activation, file 20 L-22); replay shows a non-zero
reach and an alert volume inside the stated range; below-the-line sample is within tolerance; the data inputs are mapped and above their minimum
completeness; the maker-checker approval is recorded, with a flag if the main effect is to cut alerts; and the first 30 days run in shadow.
CBI-minimum scenarios are never retired. For them the bank sets its own numbers (the CBI gives few), documents the basis, and needs a second approver
and below-the-line evidence to loosen a threshold below its previous value; whether the Office must be told of such changes is not stated in
the 2025 controls and should be confirmed with it (Unverified). Only the bank's own extension scenarios can be retired, on productivity data and
unique-catch counts (file 21 section 2b).

AKR-TMS hooks:
- **Scenario studio** (B26) runs steps B24.4 to B24.9 as workflow, with the sampling plans, sensitivity curves and replay reports stored against
  each change request.
- **Threshold-tuning workbench** with the HKMA methods (standard deviation, percentile, robust, clustering, sensitivity, ATL/BTL), shown with
  their limits for heavy-tailed Iraqi data (file 29 GV-12, GV-13).
- **Sampling-plan generator** with rare-event bounds and other-route cases; **seeded-case harness** for rules and reviewers.
- **Calendar service** (Hijri, pay days, Hajj and Umrah, Arbaeen, harvest, school fees) used by baselines, replays and explanations.
- **Recalibration clock** per scenario (amber at the interval, red at twice the interval) and a monthly board line for scenarios overdue (file 20 L-24).

### B25. What the CBI assessor and foreign supervisors expect

**The CBI assessor (standard D2).** Under the CBI's Banking Reform Initiative every continuing private bank had to submit by 30 September 2026
"an assessment from an independent external operational auditor approved by the CBI that includes the test results together with a detailed
remediation plan" for AML/CFT and sanctions (standard D2), with gaps fixed within the following period (file 18 section 12.2; the CBI's own words
as translated by the lead; the list of approved firms was not yet published). On monitoring, D2 says the system "must support dynamic rule creation, alert
generation and investigation workflow, with periodic calibration and quality assurance controls" (D2.2(b)(5) as quoted in file 28). The assessor
tests "rule logic, alert generation, communication and reporting paths, the review and improvement mechanism", "configuration files, alert
thresholds, user access permissions, audit trail integrity", "integration across KYC, monitoring and sanctions modules", and whether the data used for
detection and reporting "can be traced"; D2 also expects high-quality AML data kept for 7 years (file 18 section 12.2). The table turns each test
into the artefact AKR-TMS produces.

| Assessor test (D2) | What a bank can show from AKR-TMS | Artefact and where it is defined |
|---|---|---|
| Rule logic | Every scenario as a versioned declarative document with plain-language text in Arabic and English, and an independent re-implementation run on the same replica data with a diff of alerts ("simulated versus deployed") | Scenario register, scenario card (B24.7), functional-test harness (file 29 section 5b hooks) |
| Alert generation | A known-answer suite: positive and negative cases per CBI item, run on every release and daily in production; replay on bank history; reach and dead-rule tables | Seeded-case harness; replay report (B24.5); file 20 L-20 to L-22 |
| Alert thresholds | The parameter register: value, basis tag, rationale, approver, effective date, reason code for every change, with before and after impact | Parameter register; change log (file 29 section 9f); file 20 L-18 |
| Dynamic rule creation | Scenario studio: a compliance officer drafts a bank-specific scenario, back-tests it on history, shadows it and deploys it without a software release | B26 |
| Periodic calibration | Recalibration clock per scenario, below-the-line reports, sensitivity curves, challenger comparisons, quarterly KPI pack | B24.4 to B24.8 |
| Quality assurance | Sampling and rubric for closed alerts, reviewer-level failure rates, four-eyes enforcement, seeded cases | File 28; file 20 L-53, L-54 |
| User access permissions | Role matrix with branch scope, restricted families (INS, PUB, TF-07) visible only to the compliance head, access reviews | File 30; chapter 11 of the 2025 controls (file 01 section 2d) |
| Audit-trail integrity | Append-only, hash-chained logs of parameter changes, alert decisions and access; edit history on every record; object-locked evidence storage | File 30; file 32 (MinIO object lock) |
| Data traceability | Lineage from CBS column to scenario input to alert, reconciliation certificates, coverage ledger, quarantine queue | File 25 sections 8 and 13; file 20 L-04 to L-09 |
| Integration across KYC, monitoring and screening | One customer key; events that cross modules (SCN-PEP-05, SCN-TF-06, SCN-OBS-08); screening hits visible in cases | File 26; file 14; file 28 |
| 7-year data | Retention set to 7 years by default, configurable, with backup in a second location | File 18 section 12.2; file 30 |

**Foreign supervisors and standard-setters.** Correspondent banks examine Iraqi banks through these lenses (file 05 section 5a: "the correspondent is
the second supervisor").

| Source | What it says about scenarios, thresholds and testing | Where AKR-TMS meets it |
|---|---|---|
| FFIEC BSA/AML Manual (as quoted in file 05 section 1d; the FFIEC site blocks automated reading) | Banks must understand their filtering criteria; "Management should periodically review and test the filtering criteria and thresholds established to ensure that they are still effective"; programming methodology and effectiveness "should be independently validated" | Plain-language rule text from configuration; threshold review tasks with below-the-line results; validation pack |
| CBUAE guidance on transaction monitoring and screening, 13 September 2021 | Typology assessment drives scenarios; risk-based segmentation; above- and below-the-line testing for larger volumes; multi-factor scenarios; documentation of assumptions, parameters, thresholds; back-testing; pre-implementation testing on historical data for material changes; independent validation; suppression "should not be applied to higher-risk customer or transaction types"; data integrity testing "at minimum every 12 to 18 months" | B2 typology inventory; B23 coverage; B24; suppression engine with a hard exclusion for high-risk segments (file 05 section 4e) |
| MAS, Guidance for Effective AML/CFT Transaction Monitoring Controls, September 2018 | Thresholds and parameters calibrated per segment and "appropriately back-tested for proper operation and effectiveness" (paragraph 2.5); scenarios grouped into typologies such as smurfing, pass-through payments, circular flows (2.7); back-test before material recalibration (2.11); case studies on excessive thresholds, weak pass-through logic, reviews that ignored volume spikes, absent below-the-line testing; post-mortems after high-suspicion STRs (2.31); senior-management decisions on back-test results documented | B24 whole; SCN-XFER-03; the post-mortem job; board pack |
| HKMA, thematic review of transaction monitoring, April 2024 (file 29 section 5b) | Critical data elements and lineage; segmentation; threshold methods (standard deviation, percentile, clustering, above/below-the-line, sensitivity); functional testing by re-implementing scenarios from their specifications; reviews every 12 to 24 months plus trigger events; quarterly KPIs | CDE registry; threshold workbench; functional-test harness; review calendar |
| Wolfsberg Statement on Effective Monitoring for Suspicious Activity, parts I and II (file 04 sections 2a, 2b) | Hybrid of rules, supervised and unsupervised models; priority-risk coverage; precision and recall; false-negative assessment from other routes; below-the-line and champion-challenger testing; no mandatory parallel run; typology-to-feature explainability | B24.4 to B24.6; typology tags; reason codes |
| NYDFS Part 504 (file 05 section 1f) | Transaction monitoring "defined, managed, controlled, reported, and audited"; pre- and post-implementation testing; end-to-end review; scrutiny of changes that only cut alerts | Maker-checker change control with the capacity-driven flag |
| BCBS 2020 paragraphs 28-31 (file 04 section 1b) | Parameters "must reflect and take into account the bank's own risk situation"; vendor defaults may be used only on that condition | Basis tags on every parameter; [DP] values labelled and time-limited |
| SAMA AML/CTF guide (file 05 section 4b) | Monitoring tools tested "periodically (Once a year a maximum)" | Annual full review and effectiveness test |

**A one-hour session with an assessor: ten things to show.**

1. The CBI baseline pack register (173 items, B22) with each item's scenarios, parameters, grade, alerts and outcomes.
2. The list of items not monitorable from the replica and the route used instead (B22).
3. The parameter register with basis tags and approvals, and the change log with capacity-driven changes highlighted.
4. One scenario card in full, and a re-run of one scenario on a past month with the same result.
5. A below-the-line report and a sensitivity curve for one high-volume scenario, with the accepted miss rate.
6. The coverage ledger and the last reconciliation certificate (what share of transactions the scenarios see).
7. A seeded-case run: a transaction injected for each of ten CBI items produces the expected alert.
8. A dead-rule and flat-line report, and the silence alarm history.
9. The role matrix and an access-review record; the audit-trail integrity check.
10. A change from request to deployment: draft, back-test, shadow, approval, live, 30-day review.

AKR-TMS hooks:
- **Assessor mode**: a read-only role for the CBI-approved firm with exports of the scenario register, parameters, change log, test results,
  coverage ledger and alert and STR statistics, and a record of what was shown to whom (file 18 section 12.2 hooks).
- **D2 test-script map**: the table above kept as data, each line linked to the live artefact, so the evidence for a re-assessment after remediation
  is a query, not a project.
- **Remediation generator**: from an assessor finding to a task list in the studio (new scenario, changed parameter, data fix) with evidence of closure.

### B26. Concrete design: catalogue, versions, scenario studio and performance metrics

This section proposes how the library of B4 to B21 lives inside the product. It follows the stack decisions of file 32 section 7 (a scenario is a versioned
JSON document with a strict schema; aggregates compile to SQL over the canonical partitions from a closed operator set; event conditions use CEL;
profile scenarios use feature tables; no free SQL in any document) and the governance of file 29 sections 9a to 9g.

#### B26.1 Catalogue structure

```text
Pack  (a named, versioned set of scenarios with a purpose and an owner)
  Family  (CASH, XFER, TRADE, ... as in B1)
    Scenario  (id SCN-<FAMILY>-NN, typology, CBI items, data requirements)
      Version  (logic definition, immutable once approved)
        Parameter set  (values per segment x currency x risk tier, basis tag, rationale, effective dates)
          Run  (data-as-of, version, parameter set, counts)
            Alert  (frozen evidence, reason codes, reproducible)
Reference data  (country risk, flagged districts, dual-use list, vessel list, sector-goods map, crypto counterparties, calendar)
Registers       (exclusions, exemptions, not-covered declarations, suppression ledger)
Test suites     (unit cases, seeded cases, replay results, ATL/BTL reports)
Scenario card   (generated view of all of the above, B24.7)
```

| Pack | Content | Who may change it |
|---|---|---|
| CBI baseline pack | The 173 CBI items mapped to their implementing scenarios with default parameter sets and "not-covered" declarations (B22); items are never removed | AKR-TMS ships it; the bank sets its own parameter values within the card rules; the CBI numbering is kept |
| AKR library pack | All 161 scenarios of B4 to B21 (99 implement CBI items, 62 extend them) | AKR-TMS releases; a bank enables, disables (extensions only) and tunes |
| Optional extension packs | ISL (file 24, 57 IS scenarios in 8 bundles), PUB, PF-Iran, TF district pack | Switched on per bank; agreed with the AML/CFT Office where the bank treats them as its own scenario set |
| Bank extension pack | Scenarios the bank authors in the studio, IDs 901 to 999 in each family (for example SCN-CASH-901) | The bank only; the card is shared with the Office "in coordination" as Part A's legal note requires |

A bank-tuned parameter set is not a new scenario: tuning keeps the scenario ID and creates a new parameter-set version. A bank may add segments and
exclusions (with owner, reason and expiry) but may not delete a CBI-baseline scenario.

#### B26.2 Versioning

| Object | Version rule | Change class (file 29 section 9f) |
|---|---|---|
| Scenario logic | major.minor.patch: major = new operator, window or aggregation level; minor = new optional parameter, exclusion type or evidence field; patch = text, reason-code wording or evidence template | Major: material. Minor: minor or material by band. Patch: minor |
| Parameter set | Integer version per scenario and bank, with effective-from and effective-to, approver, basis tag and reason code | Inside the approved band: minor; outside, or a loosening: material; sanctions or regulator-ordered: emergency, reviewed within 5 working days |
| Reference data | Effective-dated rows, never overwritten; table version shown on each alert | Minor, except lists that drive hard rules |
| Segment definition | Version with impact analysis of alert volumes by segment | Material |
| AKR library pack | Release tag (for example AKR-LIB 1.2); validated once per release, then delta-validated per bank (Wolfsberg part II, file 29 section 9c) | Release train; the bank pins a version and defers within the supported window; every logic or data-feed change is notified in advance |
| CBI pack version | An identifier of the CBI instruments in force plus the mapping revision, for example "CBI-2025.01 + CBI-2026.08 circulars, mapping r3": the controls of 8 January 2025, the circulars of 26 August 2026, and the library mapping | Every CBI or Office issuance opens a regulatory-input task, a new draft mapping and a release note listing new, changed and removed items |

Every alert records scenario version, parameter-set version, reference-data versions, CBI pack version and data-as-of, so an assessor can reproduce it.
The CBI coverage report states the CBI pack version it was produced under. When the Office issues new scenarios, the catalogue importer loads them as drafts with
their data requirements (file 32 section 7.7), the studio guides mapping to library scenarios or authoring of a new one, and the bank records its adoption date.
A new library release runs in shadow against the pinned version for one cycle, with a difference report before the bank accepts it. Rollback is within one
change window; every scenario has a kill switch that falls back to the previous version, not to "off".

#### B26.3 Scenario studio workflow

| Stage | Who | What happens | Evidence stored | Gate |
|---|---|---|---|---|
| 1 Trigger | Anyone in compliance; automatic from typology circulars, post-mortems, risk-assessment gaps, authority requests | A trigger is logged with its source and a proposed typology | Trigger register | Owner assigned |
| 2 Author | Compliance analyst or MLRO delegate | Draft the card; build logic in a visual editor that writes the JSON document; pick data fields from the readiness view; set parameters and segments; the plain-language text in Arabic and English is generated | Draft version, card | Card complete; inputs mapped |
| 3 Unit-test | Author, with the studio's checks | At least 3 positive and 3 negative cases; automatic checks for nulls, currency, reversals netted, late data; a failing test blocks the next step (file 20 L-22) | Test suite results | All tests pass |
| 4 Replay on history | Author; independent reviewer for CBI-baseline changes | Run on 12 to 24 months of replica data as-of; report alerts by segment, branch and month, reach, overlap, dead-rule test, estimated analyst hours, look-ahead flag | Replay report | Volume in the stated range; non-zero reach |
| 5 Above- and below-the-line | Experienced analysts, blind to the rule | Sampling plan generated (B24.4); reviews recorded; sensitivity curve; other-route cases tested | Sampling plan, labels, curve | Productive rate within tolerance; accepted miss rate recorded |
| 6 Shadow | System | Runs on live data for 30 days (two cycles if seasonal), creates no alerts; challenger compared with champion | Shadow comparison | Success criteria met |
| 7 Approve | MLRO plus a second approver outside operations; independent validator for CBI-baseline logic changes | Maker-checker with impact analysis; flag if the main effect is to cut alerts; decision on the phase-2 candidate flag; for a bank-added scenario, the card is exported for the Office | Approval record, rationale | Requester differs from approver |
| 8 Deploy | System, scheduled | Version and parameter set activate at the effective time; coverage matrix and branch notes update; emergency path available | Activation log | Post-deployment smoke test |
| 9 Monitor | MLRO, analysts' leads | Health dashboard, monthly below-the-line job, silence and flat-line alarms, fairness, post-change reviews at 30 and 90 days | Monitoring series, review notes | Red status opens an incident |
| 10 Tune or retire | Author and approver | New parameter-set version with reason code; retirement only for the bank's extension scenarios, on productivity and unique-catch data; a temporary switch-off needs an end date | Change request, retirement record | Same gates as 7 |

Roles: author (compliance analyst), builder (AKR-TMS implementer for complex logic, only on request), independent validator (internal audit, risk
or an outside firm), approver (MLRO and a second approver), operator (alert team lead), assessor (read-only). The studio is Arabic-first with English (file 31).

#### B26.4 A scenario document (illustrative)

Following file 32 section 7.3, with the closed operator set; names and values are illustrative of the format.

```json
{
  "id": "SCN-CASH-02", "version": "1.0.0", "status": "shadow",
  "name": {"ar": "إيداعات نقدية متكررة دون حد الإبلاغ", "en": "Structuring: repeated cash deposits just under the limit"},
  "typology": "cash_placement_structuring",
  "cbi_refs": [{"article": 1, "no": 2, "rel": "implements"}, {"article": 1, "no": 12, "rel": "implements"},
               {"article": 1, "no": 4, "rel": "extends"}],
  "owner": "mlro", "phase2_candidate": false,
  "data_requirements": ["transaction.cash_flag", "transaction.amount_usd", "transaction.booking_date",
                        "branch.id", "cash_event.depositor_id?", "party_relationship.family_number?"],
  "aggregation": {"level": "customer_and_linked_parties", "net_reversals": true, "settle_delay_s": 120},
  "parameters": {
    "min_deposits": {"type": "int", "default": 3, "min": 2, "max": 10, "basis": "DP"},
    "window_days": {"type": "int", "default": 7, "allowed": [7, 14, 30], "basis": "DP"},
    "total_usd": {"type": "decimal", "default": 10000, "min": 3000, "basis": "CBI",
                  "overrides": [{"segment": "cash_retailer", "value": 15000, "approval": "CR-2026-114"}]},
    "just_below_band": {"type": "range", "default": [0.70, 0.999], "basis": "DP"}
  },
  "logic": {"kind": "aggregate",
            "where": "transaction.cash_flag = true AND transaction.dr_cr = 'C'",
            "measure": [{"op": "count", "as": "n"}, {"op": "sum", "field": "amount_usd", "as": "total"}],
            "having": "n >= :min_deposits AND total >= :total_usd"},
  "outputs": {"severity": [{"when": "limit_enquiry_logged", "level": "high"}],
              "reason_codes": ["CASH02_SPLIT_BELOW_LIMIT", "CASH02_MULTI_BRANCH"],
              "template": "cash_structuring_v1"},
  "tests": {"unit": 8, "replay": "2026-09-30", "btl": "2026-09-30", "seeded": 4}
}
```

#### B26.5 Scenario performance metrics

Per scenario, by segment, branch and month, shown to the right audience (file 21 section 15d): analysts see their queue metrics; the MLRO sees all; the board sees the
summary; the assessor sees the register and the series.

| Group | Metric | Definition and use |
|---|---|---|
| Volume | Alerts, alerts per 1,000 customers, share of all alerts, week-on-week change | Surge (200 percent) and silence alarms; flat-line detector (same count for N days) |
| Reach | Eligible customers, share that could ever trigger, share that did | Dead-rule and unreachable-rule test (file 20 L-20) |
| Productivity | Alert to case, case to STR, alert to STR; STRs per 1,000 alerts; productive rate by distance from the threshold | The quality measure MAS case C asked for; lag-aware windows |
| Uniqueness | Unique catches (cases found only by this scenario), overlap with other scenarios | A low-volume scenario with unique catches earns its place (file 21 section 2b) |
| Outcome | Share of STRs from this scenario graded good, moderate, acceptable or poor by the FIU; authority use where known | The FIU grades STR quality (file 01 section 5); feeds tuning |
| Misses | Below-the-line productive rate; other-route cases not caught; seeded-case detection rate; accepted miss rate | False-negative assessment (Wolfsberg) |
| Timeliness | Data latency (CBS commit to alert), alert to decision, ageing | Two clocks (file 20 L-31) |
| Quality | QA failure rate on alerts from this scenario, override rate, re-open rate | File 28 QA programme |
| Stability | Input drift (PSI, 0.10 watch and 0.25 action as a credit-scoring convention, file 29 section 9d), parameter changes, days since tuned | Recalibration clock |
| Fairness | Alert and STR rates by governorate, branch and customer group | File 29 section 9i |
| Cost | Analyst minutes per alert, share of analyst hours | Capacity planning, never used to set thresholds |
| Coverage | CBI items implemented, risk statements covered, readiness grade, share of customers with the required data | B22 and B23 |

A scenario is **red** if it is silent with eligible volume, volume jumps 200 percent without a known cause, a below-the-line sample is above tolerance, an
input falls below its minimum completeness, or recalibration is more than twice overdue; **amber** at the first signal of each (for example 100 percent
volume change or half the overdue interval). Red opens an incident with an owner and a due date and appears in the board pack (file 20 L-56).

AKR-TMS hooks:
- **Scenario register** as the single source for the studio, the engine, the card, the CBI coverage report and the assessor pack.
- **Catalogue importer** for CBI and Office issuances; **release notes and shadow comparison** for each library release.
- **Metrics service** computing B26.5 from alerts, cases, STR outcomes, sampling results and data-quality counters; one dashboard per audience.
- **Kill switch and rollback** per scenario to the previous version; **bank extension namespace** with the Office-sharing export.

### B27. What it means for AKR-TMS

1. **The CBI floor is implementable almost completely, but not from the ledger alone.** All 173 CBI items (86 numbered scenarios, 87 product and behaviour
   indicators) map to a library scenario except one (constant travel to drug-producing countries, a card-use proxy only). Only 47 (27 percent) run
   from ledger, account and KYC fields; 80 need an optional module or message data, 26 need data outside the CBS and 20 are observational (B22). A
   vendor that says "we cover all 86" without a data-readiness grade is overclaiming; AKR-TMS shows the grade, the missing field and the route.
2. **The library goes well beyond the floor, where Iraqi evidence points.** 161 scenarios, 99 implementing CBI items and 62 extending them, aimed
   at the typologies that foreign authorities have actually described about Iraqi banks: dollar-access abuse, front companies, Iran-linked oil and
   exchange-house chains, militia-linked government contracts, public payroll and cheques, hawala settlement through bank accounts, and mules (B2).
   These extensions are the product's differentiator and also its supervisory risk: they must be agreed "in coordination with" the AML/CFT Office
   (Part A legal note) and labelled with their evidence grade.
3. **Public funds, payroll and contracts need their own family.** The NRA ranks corruption first and the CBI catalogue has no scenario for it;
   salary localisation makes public pay visible for the first time. PUB and the PEP family together give a bank something to run against its biggest
   national threat, with restricted visibility (B11, B12).
4. **Thresholds are the real work, and the replica helps.** The CBI text gives few numbers. A bank with CBS history can calibrate on day one from
   its own segments; one without uses regulatory anchors, KYC ratios and conservative anchors, with every [DP] value time-limited (B24.2). The method
   is the foreign-supervisor consensus: segment, document the basis, back-test before changing, test above and below the line, never tune to capacity.
5. **Below-the-line testing is the control that failed most often** (TD Bank, US Bancorp, HSBC UK, MAS case study F). AKR-TMS makes it a monthly job with
   a sampling plan, a tolerance, an accepted miss rate and other-route cases (B24.4), which also answers the Wolfsberg warning against "simply
   lowering thresholds by 10%".
6. **Observation is data.** The CBI's August 2026 circular says behaviour cannot be detected unless staff record it; 16 CBI items are only
   observable at the counter. The branch observation log and hybrid alerts (OBS family) are therefore part of the monitoring system, not a separate
   case-management feature (B21).
7. **Phase 2 has a concrete first list.** The hard rules with a CBI page behind them (dormant reactivation and frozen-account debits, delayed-
   verification limits, 2016 FX caps if confirmed, Islamic prohibitions and qard hasan caps), sanctioned-corridor and listed-party payments, missing
   mandatory transfer data, post-issue changes to LC beneficiaries, walk-in linked transactions and cash-payout instructions are the candidates for
   hold-until-cleared (the CBI's "stop until the MLRO verifies" item). Phase 1 cannot do it and should say so.
8. **The assessor (standard D2) will test exactly what B26 builds.** "Dynamic rule creation", "periodic calibration and quality assurance", thresholds,
   permissions, audit trail and data traceability map one-to-one to the scenario studio, the parameter register, the QA programme, the audit log and the
   lineage (B25). The ten-exhibit session is the product demo for an assessor.
9. **A bank-level mirror is a commercial asset.** FinCEN's finding on Banque Misr UAE rests on a ratio computable from the bank's own ledger.
   SCN-CORR-07 gives a bank and its correspondents the same view per typology, branch and correspondent (B13).
10. **Say what cannot be seen.** Self-financed terrorist acts, hawala offsets, cash couriers' journeys, physical goods and other banks' accounts
    are outside the replica (B3, B14). Overstating detection of these would be the quickest way to lose credibility with an assessor or with the FIU.
11. **Protected attributes stay out.** Several CBI indicators are worded around women or young customers. The library uses profile mismatch, age band
    and declared occupation only where the CBI text names them, and reports alert rates by governorate and customer group (B3, B10).
12. **Calendars and regions matter.** Hijri and religious seasons, pay days, harvest and school fees change baselines; Kurdistan, border governorates and
    exchange, oil and NPO sectors are segments (B4, B14, B23).
13. **The evidence is uneven.** Dollar access, front companies, Iran oil, militia contracts, TF in the UN and MER cases and the CBI's own lists are
    well sourced; real estate, Captagon laundering, zakat abuse and Kurdistan flows rest on thin public evidence and carry the [N] grade until
    validated on the first bank's cases (B2). The FIU gives STR quality grades but no outcome feedback; a feedback register is the way to learn.
14. **Market position.** In Trans Iraq Bank's report a vendor added 81 and then 96 scenarios beyond the CBI's 25 (file 18). AKR-TMS offers the in-house
    path the MER said banks lack: author, test on history, approve, deploy and monitor without a vendor project (B26).

**Gaps and items not verified (2026-10-07).**

- The status of the 2016 CBI FX procedures (USD 3,000 per person, six months of account age, seven-day funding, USD 150,000 and ten remittances a day) is not
  verified; the read-out is from a scan and the Arabic reading is mine. The numbers are configurable and labelled.
- Whether the AML/CFT Office must be told of threshold changes to CBI-minimum scenarios is not stated in the 2025 controls (Unverified).
- The October 2022 tax-deposit theft rests on Wikipedia citing The National and Iraq Business News; no primary Iraqi or UN document was read. The
  May 2026 arrest of a deputy oil minister and the USD 107 million seizure appear only on the same Wikipedia page and are not used.
- FinCEN's "approximately $70 billion USD in profit, from 2019 through 2020" in the Al-Huda notice is quoted as printed and was not checked against a
  source; it is not used in any parameter.
- No Iraq-specific study of Captagon laundering, of zakat abuse or of real-estate laundering was found; no Integrity Commission annual report was
  reached (nazaha.iq returns a gateway page; the Wayback listing was offline at times). Ghost-employee evidence comes from the MER cases, the CBI fraud guide
  and the NRA language, not from an Integrity Commission count.
- MENAFATF's 2022 NPO typology draft was only partly downloaded (the file was truncated at 1 MiB); the 2014 FATF NPO typology was used instead.
- The CBI's Banking Reform assessment guide was read through the lead's translation (file 18 section 12.2); the assessor's own test script was not seen.
- FFIEC text is quoted as reproduced in file 05 section 1d (the FFIEC site blocks automated access).
- The district list for TF-01 and DIGI-09 is derived from UN and MER text; the bank's own risk assessment must replace it.

### B28. Derived requirements (S-01 to S-60)

Phase 1 = read-only CBS link, post-event; phase 2 = optional pre-release API or richer data. "B" references are sections of this file.

| ID | Requirement | Source |
|---|---|---|
| S-01 | Ship the CBI baseline pack as a versioned register of all 173 CBI items (86 numbered scenarios and 87 product and behaviour indicators), each mapped to its implementing scenarios, default parameter set and data-readiness grade; items are never removed | Part A; B22; CBI 2025 controls ch. 2 art. 2 |
| S-02 | Generate the CBI coverage report (item to scenario to parameters to alerts to cases to STRs) from the scenario register, stamped with the CBI pack version, as an assessor-ready export | B22; B25; CBI standard D2 |
| S-03 | Data-readiness view per scenario and per bank: grade A to D, the missing field named, the vendor ask written out, and "not-covered" declarations with owner, reason, interim route and review date | B3; B22; file 25 section 13.6; file 20 L-02 |
| S-04 | Ship the AKR library (161 scenarios in 18 families) as an extension pack with typology tags, a basis tag on every parameter, an evidence grade, and a phase-2 candidate flag; extension scenarios are presented as the bank's own set only after agreement with the AML/CFT Office | B1; B26.1; Part A legal note |
| S-05 | Bank extension namespace (IDs 901 to 999 in each family), an export of the scenario card for the Office, bank-tuned values stored as parameter-set versions, and no deletion of a CBI-baseline scenario | B26.1 |
| S-06 | Catalogue importer for CBI and Office issuances: new items load as drafts with data requirements, a mapping workflow, release notes and the bank's adoption date | B26.2; file 32 section 7.7 |
| S-07 | One scenario register as the single source for the studio, the engine, the scenario card, the coverage report and the assessor pack | B26.5; B22 |
| S-08 | Aggregate every aggregable scenario by customer and linked parties (family number, phone, address, depositor, companion) through a shared linked-party service | B4; Art.1 no. 4; file 20 L-26 |
| S-09 | Scenario counting rules: posted events only, reversals netted, evaluation at the settled time, and alerts amended with an explanation when late data change a decision | B3; file 25 sections 6 and 7.4 |
| S-10 | A missing KYC value falls back to the segment percentile and marks the customer "degraded"; unknown is never treated as low | B3; B10; file 20 L-10, L-44 |
| S-11 | Resolved counterparty entity (first-seen date, country, bank, class) on every transaction so "no relationship" and "new counterparty" are queries | B5; file 25 section 13.2 |
| S-12 | Robust baseline service (percentiles and median-MAD by segment, currency and season) with a minimum segment size of 200 and merge-upward | B24.3; file 21 section 3 |
| S-13 | Calendar service with Hijri dates, Ramadan, Eid, Muharram, Arbaeen, Hajj and Umrah, pay days, harvest and school-fee cycles, used by baselines, replays and explanations | B4; B14; file 24 section 4e |
| S-14 | Effective-dated, versioned country and area risk table and a flagged-district table for TF with owner, source and quarterly review; the table version appears on every alert | B5; B14; CBI ch. 8 |
| S-15 | Governed reference registers (dual-use goods and HS list, vessel and IMO list, sector-goods map, peer price tables, crypto counterparties, government-source flag, DNFBP counterparty classes) with owner, version and review date | B6; B8; B12; B15 |
| S-16 | Cheque instrument record (drawer, payee, endorsers, clearing status, return reason) and an available-versus-cleared balance field | B18; file 25 section 12.1 |
| S-17 | Payroll and pension batch tags and a government-source flag so public-salary accounts form peer segments | B12; file 01 section 9 |
| S-18 | Facility event stream (disbursement, repayment with payer identity, early settlement, collateral and guarantor change) and a collateral and guarantor graph | B9 |
| S-19 | Optional connectors (customs, vessel and maritime, company registry, civil registry, NGO registry, adverse media, HR) each shown in the readiness view as upgrading named scenarios from C to B | B3; B6; B22 |
| S-20 | Optional read-only links to channel databases and the card processor, labelled grade C, with retention limits and a purpose statement for device and location data | B8; file 04 section 2q; file 30 |
| S-21 | Teller event capture where the CBS holds it (cancelled, reduced and amended entries, depositor ID, denominations, counterfeit flag, operator ID) and a depositor-ID capture rate per branch on the dashboard | B4; B21 |
| S-22 | Branch observation log with the circular's eight groups, companion ID, written acknowledgment with the tipping-off reminder, a confidential channel to the MLRO and no visibility for branch management | B21; CBI circular 26 Aug 2026; 2025 controls ch. 2 art. 3, ch. 11 |
| S-23 | Hybrid alerts combining an observation and a transaction pattern, an indicator accumulator, and the circular's seven SAR-escalation criteria as a checklist inside every case | B21; circular section 8 |
| S-24 | Automated scenarios for restructuring, splitting and repeated patterns as the circular orders (SCN-CASH-02, SCN-CASH-13, SCN-OBS-07) | B4; B21; circular instruction 5 |
| S-25 | Hard-rule pack with a CBI page behind each rule (dormant reactivation approvals, frozen dormant debits, delayed-verification limits, Islamic prohibitions, qard hasan caps, 2016 FX caps) as configurable rules with effective dates and a "status unverified" label where applicable | B17; B7; B19; B9 |
| S-26 | Phase-2 candidate flag on every scenario, with the first list (hard rules, sanctioned corridors and listed parties, missing mandatory data, post-issue LC changes, walk-in linked transactions, cash-payout instructions) and four-eyes release with a decision log for each hold | B3; file 25 section 15; file 29 GV-53 |
| S-27 | Cash feature library and scenarios SCN-CASH-01 to -14, including limit-enquiry severity uplift and linked-party aggregation | B4; file 21 section 16g |
| S-28 | Pass-through matching engine (cumulative, tolerance plus or minus 10 percent, 72 hours) with unit tests that include the MAS pass-through case and the TD Zelle cap case | B5; B24.5; MAS 2018 case B; file 20 section 2.1 |
| S-29 | Public-funds family SCN-PUB-01 to -07 with restricted visibility, government-account segments of one and contract-chain graphs | B12; MER box 3.18, 5.2; Treasury 9 Oct 2025 |
| S-30 | Front-company profile score with visible reasons; an alert opens a review, never an automatic exit | B15; FinCEN FIN-2025-A002; file 20 L-66 |
| S-31 | Typology exposure index per bank, branch and correspondent, with trend and a respondent view of what correspondents can compute | B13; file 20 L-65; FinCEN notice on Banque Misr UAE |
| S-32 | Iran and PF vigilance pack: restored UN list, OFAC and IRGC lists, vessel and IMO checks, ownership aggregation by each regime's rule, and an automatic designation look-back report on every list change | B15; B14; file 04 U-08; file 20 L-41 |
| S-33 | NPO profile and scenarios: registry status, foreign-funding flag, expected programme and budget, audit status against the IQD 75 million rule, board and signatories as related parties | B14; MER boxes 4.11, 4.12; FATF 2014 |
| S-34 | Mule and wallet scenario set with a device and phone graph, wallet and card features, and an onboarding-channel field for "follow the data" reporting | B8; FATF/Interpol/Egmont 2023; file 04 section 2q |
| S-35 | Employee-risk view restricted to the compliance head: staff-linked flag, mandatory-leave and rotation tracking, neglect-of-duty clocks, gift and conflict registers | B16; CBI Art. 14; CBI fraud guide 2024 |
| S-36 | Dormancy clock per account on the CBI rule (six months, current accounts only) and a monthly reactivation audit report | B17; 2025 controls ch. 15 |
| S-37 | Optional Islamic extension pack bundling file 24's 57 scenarios into 8 library scenarios with basis labels, deal-view data and Arabic narratives in contract terms | B7; file 24 section 8 |
| S-38 | Protected-attribute rule: gender, religion, ethnicity and nationality never drive a threshold; age band and occupation only where the CBI text names them; alert and STR rates reported by governorate, branch and customer group | B3; B10; file 29 section 9i |
| S-39 | Parameter register with value, basis tag, rationale, approver, effective dates and reason code (risk or capacity); no parameter limits alert counts; capacity-driven changes need below-the-line evidence and an approver outside operations | B24.1; file 20 L-16 to L-19 |
| S-40 | Bootstrap ladder for first thresholds (regulatory anchors, KYC ratios, history percentiles, conservative anchors with peer priors, shadow period), with every [DP] value time-limited and flagged | B24.2 |
| S-41 | Replay engine on as-of history using the bi-temporal model, reporting alerts by segment, branch and month, reach, overlap, dead rules, estimated analyst hours and a look-ahead-bias flag | B24.5; file 25 section 13.1; HKMA functional testing |
| S-42 | Above- and below-the-line sampling-plan generator with strata, rare-event bounds, other-route cases, seeded positives, a tolerance per scenario class and a recorded accepted miss rate | B24.4; Wolfsberg; MAS footnote 2 |
| S-43 | Threshold-tuning workbench with standard-deviation, percentile, robust, clustering, sensitivity and ATL/BTL analyses, their limits for heavy-tailed data shown, stored against each change request | B24; file 29 GV-12 |
| S-44 | Champion and challenger shadow runs of at least one cycle against pre-agreed promotion criteria, permanent challenger for high-share scenarios, rollback point recorded | B24.6; file 29 section 9e |
| S-45 | Scenario card generated from the register and test results (identity, purpose, logic, data, parameters, segments, expected behaviour, tests, performance, change history, review) meeting the "independent third party could replicate" test | B24.7; file 29 GV-30 |
| S-46 | Review calendar and recalibration clock per scenario (amber at the interval, red at twice), with triggers (volume surge, new product, CBS change, new typology circular, audit finding) and a post-mortem job after high-suspicion STRs | B24.8; MAS paragraph 2.31; file 20 L-24 |
| S-47 | Go-live gate: complete card, passing unit tests, replay inside the stated range, below-the-line sample within tolerance, inputs mapped above minimum completeness, maker-checker approval, 30-day shadow | B24.9; file 20 L-22, L-23 |
| S-48 | Independent functional-test harness that re-implements each scenario from its specification and diffs alerts "simulated versus deployed" | B25; HKMA 2024; file 29 section 5b hooks |
| S-49 | Known-answer seeded suite with at least one positive and one negative case per CBI item, run on every release and daily in production, with silence and flat-line alarms | B25; file 20 L-22, L-29 |
| S-50 | Scenario studio workflow (trigger, author, unit-test, replay, above and below the line, shadow, approve, deploy, monitor, tune or retire) with roles, evidence per stage and no software release needed | B26.3; CBI standard D2 |
| S-51 | Declarative scenario documents (strict JSON Schema, aggregate, event and profile logic from a closed operator set, no free SQL, bilingual text) | B26.4; file 32 section 7.3 |
| S-52 | Versioning: logic major.minor.patch, parameter-set versions, effective-dated reference data, library release tags and a CBI pack version, all recorded on every alert; shadow comparison for library upgrades; per-scenario kill switch that falls back to the previous version | B26.2; file 29 section 9f |
| S-53 | Scenario performance metrics (volume, reach, productivity, uniqueness, outcome, misses, timeliness, quality, stability, fairness, cost, coverage) with the red and amber rules and an FIU-grade feedback loop | B26.5; file 01 section 5; file 21 section 15 |
| S-54 | Four coverage measures (CBI item, risk statement, data-ready, outcome) and four lists for the risk assessment (no scenario, blocked or degraded, dead, no risk statement) | B23 |
| S-55 | Risk-statement importer and a typology tag set shared by the risk assessment, the scenario register and the STR typology field | B23; CBI 2025 controls ch. 2 art. 2 |
| S-56 | Assessor mode (read-only role with exports and a record of what was shown), the D2 test-script map kept as data, and a remediation generator from finding to task | B25; file 18 section 12.2 |
| S-57 | A one-hour assessor session pack of ten exhibits generated from the live system | B25 |
| S-58 | Replay history, scenario results and parameter history retained at least 7 years by default, configurable | B25; file 18 section 12.2; file 20 L-13 |
| S-59 | Evidence label on every library scenario ([P], [S] or [N]) and validation of [N] typologies (real estate, Captagon, zakat, Kurdistan) on the first bank's own cases before they are described as proven | B2; B27 |
| S-60 | Office engagement pack: scenario cards and coverage for extension scenarios, the open questions on CBI-minimum threshold changes and on the 2016 FX caps, and a feedback register for FIU STR grades | B27; Part A legal note; file 28 |

**Count:** 60 requirements (S-01 to S-60). Of the 161 library scenarios, 99 implement CBI items and 62 extend them; 173 CBI items are mapped.

### B29. Sources

Official and primary (read in this session unless stated)
- [CBI-2025] CBI, supervisory controls for AML, CFT and counter-proliferation, 8 January 2025 (Part A source; chapters 2, 5, 8 to 12, 14, 15 via file 01 section 2d): https://cbi.iq/static/uploads/up/file-173666439114818.pdf
- [CBI-CIRC-BEH] CBI, behavioural indicators for transactions suspected of ML or TF, 26 August 2026 (scanned, read from images): https://cbi.iq/static/uploads/up/file-178963291760044.pdf
- [CBI-CIRC-PEP] CBI, due diligence towards PEPs, 26 August 2026 (scanned): https://cbi.iq/static/uploads/up/file-178963295727758.pdf
- [CBI-FX-2016] CBI, procedures to verify the source of funds of foreign-currency buy and sell applicants, letter of 31 July 2016 (scanned; pages 1 to 8 read from images; Arabic reading mine): https://cbi.iq/static/uploads/up/file-151505168588140.pdf
- [CBI-FRAUD-2024] CBI, guidance on combating fraud and corruption in banks operating in Iraq, 2024, annex 2 and sections 9 and 10 (scanned; read from images): https://cbi.iq/static/uploads/up/file-170530434494172.pdf
- [MER] MENAFATF, Mutual Evaluation Report of Iraq, May 2024, chapters 1, 3, 4, 5 (paragraph and box numbers as cited): https://www.menafatf.org/sites/default/files/Newsletter/IRAQ%20MER_En_0.pdf (read from the local text of the English PDF)
- [FR-2024-14415] FinCEN, final rule imposing a special measure against Al-Huda Bank, 89 FR 55051, 3 July 2024: https://www.govinfo.gov/content/pkg/FR-2024-07-03/html/2024-14415.htm
- [FINCEN-2025-A002] FinCEN Advisory FIN-2025-A002, 6 June 2025 (red flags for Iranian oil smuggling, shadow banking and weapons procurement): https://www.fincen.gov/system/files/FinCEN-Advisory-Illicit-Oil-Smuggling-508.pdf
- [TSY-SB0188] US Treasury press release of 3 July 2025 (Iranian oil networks, including a network blending Iranian and Iraqi oil): https://home.treasury.gov/news/press-releases/sb0188
- [TSY-SB0233] US Treasury press release of 2 September 2025 ("Treasury Intensifies Pressure on Iranian Oil Smuggling and Sanctions Evasion Schemes in Iraq"): https://home.treasury.gov/news/press-releases/sb0233
- [TSY-SB0277] US Treasury press release of 9 October 2025 ("Treasury Takes Aim at Iran-Backed Militia Groups Threatening the Safety of Americans"): https://home.treasury.gov/news/press-releases/sb0277
- [TSY-SB0458] US Treasury press release of 17 April 2026 ("Economic Fury Targets Iran-Backed Iraqi Militia Commanders"): https://home.treasury.gov/news/press-releases/sb0458
- [UN-S2026-651] UN Security Council, thirty-eighth report of the Analytical Support and Sanctions Monitoring Team (ISIL, Al-Qaida), S/2026/651, 10 August 2026: https://docs.un.org/S/2026/651 (PDF read through https://documents.un.org/api/symbol/access?s=S/2026/651&l=en&t=pdf)
- [UN-S2026-44] Thirty-seventh report, S/2026/44, February 2026: https://docs.un.org/S/2026/44
- [UN-S2025-482] Thirty-sixth report, S/2025/482, 29 July 2025: https://docs.un.org/S/2025/482 ; index of reports: https://main.un.org/securitycouncil/en/sanctions/1267/monitoring-team/reports
- [FATF-2013] FATF, The Role of Hawala and Other Similar Service Providers in ML/TF, October 2013, section 3.6.4 (read through the Wayback Machine): https://www.fatf-gafi.org/content/dam/fatf-gafi/reports/Role-of-hawala-and-similar-in-ml-tf.pdf
- [FATF-2026-UB] FATF, Investigating Professional Money Laundering, Underground Banking, and the Use of Hawala and Other Similar Service Providers, September 2026 (read through the Wayback Machine): https://www.fatf-gafi.org/content/dam/fatf-gafi/reports/pml-underground-banking-hawala-fatf-report-2026.pdf.coredownload.pdf
- [FATF-EGMONT-2020] FATF and Egmont Group, Trade-Based Money Laundering: Trends and Developments, December 2020: https://www.fatf-gafi.org/content/dam/fatf-gafi/reports/Trade-Based-Money-Laundering-Trends-and-Developments.pdf
- [FATF-TBML-2021] FATF and Egmont Group, Trade-Based Money Laundering: Risk Indicators, March 2021: https://www.fatf-gafi.org/content/dam/fatf-gafi/reports/Trade-Based-Money-Laundering-Risk-Indicators.pdf
- [FATF-CEF-2023] FATF, Interpol and Egmont Group, Illicit Financial Flows from Cyber-Enabled Fraud, annex A (risk indicators), November 2023: https://www.fatf-gafi.org/content/dam/fatf-gafi/reports/Risk-Indicators_Illicit-financial-flows-cyber-enabled-fraud.pdf
- [FATF-NPO-2014] FATF, Risk of Terrorist Abuse in Non-Profit Organisations, June 2014: https://www.fatf-gafi.org/content/dam/fatf-gafi/reports/Risk-of-terrorist-abuse-in-non-profit-organisations.pdf.coredownload.pdf
- [MENAFATF-2014] MENAFATF, Biennial Typologies Report 2014 (trends and indicators update; forgery and counterfeiting of financial instruments and LCs): https://menafatf.org/sites/default/files/Final_Biennial_Typologies_report_EN.pdf (read through the Wayback Machine)
- [MAS-2018] Monetary Authority of Singapore, Guidance for Effective AML/CFT Transaction Monitoring Controls, September 2018 (read through the Wayback Machine): https://www.mas.gov.sg/-/media/MAS/Regulations-and-Financial-Stability/Regulatory-and-Supervisory-Framework/Anti_Money-Laundering_Countering-the-Financing-of-Terrorism/Guidance-for-Effective-AML-CFT-Transaction-Monitoring-Controls.pdf
- [CBUAE-2021] Central Bank of the UAE, outreach presentation on the guidance for LFIs on transaction monitoring and sanctions screening, 14 September 2021: https://www.centralbank.ae/media/rdzlhqse/outreach-presentation-cbuae-amlcft-guidance-for-lfis-on-transaction-monitoring-and-sanctions-screening-14-sept-2021.pdf
- [FFIEC] FFIEC BSA/AML Examination Manual, suspicious activity reporting overview (quotations as reproduced in file 05 section 1d; the site blocks automated reading): https://bsaaml.ffiec.gov/manual/AssessingComplianceWithBSARegulatoryRequirements/04

Secondary (used with a grade [S] and flagged where relevant)
- Wikipedia, Corruption in Iraq (tax-deposit theft of October 2022, citing The National and Iraq Business News): https://en.wikipedia.org/wiki/Corruption_in_Iraq
- Wikipedia, Captagon (background only, not Iraq-specific): https://en.wikipedia.org/wiki/Captagon

Earlier research files used for cross-reference (not repeated here)
- File 01 (Iraqi framework: CBI controls 2025, circulars of 26 August 2026, MER summary, STR statistics, salary localisation); file 02 (risk landscape); file 04 (Basel, Wolfsberg parts I and II, trade principles, TF statement, UN lists, PSP guidance); file 05 (FinCEN advisory, OFAC, NYDFS, FFIEC, CBUAE, SAMA); file 18 (Iraqi market, Trans Iraq Bank scenario counts, CBI reform standard D2); file 20 (enforcement failures and requirements L-01 to L-68); file 21 (technology approaches, segmentation, label ladder); file 24 (Islamic scenario pack IS-xx); file 25 (canonical model, data readiness, status lifecycle); file 28 (D2.2 quotation, QA calibration); file 29 (governance, rule card, ATL/BTL, change classes); file 30 (security and access); file 31 (analyst UX, observation log); file 32 (scenario DSL and engine).

Not reached or not read
- Integrity Commission (nazaha.iq) annual reports and ghost-employee counts; the Office's guidance manuals on suspicious-transaction indicators for NPOs and for human trafficking; MENAFATF's 2022 NPO typology (file truncated); FinCEN alert FIN-2026-Alert-002 on IRGC front companies (described in file 20 section 9.1 only); the CBI's approved-firm list for the D2 assessment; any CBI document that states the current status of the 2016 FX caps.
