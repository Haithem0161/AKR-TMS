# 24. Islamic banking: AML/CFT risks, product typologies and monitoring design

Status: done (2026-10-06)

ID prefix for derived requirements: I (I-01, I-02, ...)

Contents:

1. Why Islamic banking needs its own treatment (scope, size, standards bodies)
2. What the standard-setters say (IMF 2016; IFSB-AMF 2019; FATF, AAOIFI and IFSB position)
3. Product typologies: murabaha and commodity murabaha, tawarruq, ijara, mudaraba and musharaka accounts, qard hasan, sukuk, takaful, salam/istisna'
4. Zakat, sadaqa, waqf: charity flows and terrorist-financing risk
5. Islamic windows and the Sharia board (governance, who can see what)
6. Data model differences in Islamic core banking (contracts, commodity trades, profit distribution)
7. Iraqi Islamic banks: number, market share, Law No. 43 of 2015, CBI regulations, US dollar bans and why
8. Monitoring scenarios specific to Islamic products
9. What it means for AKR-TMS
10. Derived requirements (I-xx)
11. Sources



---

## 1. Why Islamic banking needs its own treatment

### 1a. Size

| Measure | Figure | Source |
|---|---|---|
| Global Islamic financial assets, end-2025 | **USD 4.4 trillion**, up 13.4% on 2024; about USD 1.7 trillion added since 2020 | IFSB Stability Report 2026 as reported by Daily Trust (7 Jul 2026) and Islamic Economics Project (8 Jun 2026); IFSB primary page not fetched |
| Share of Islamic banking | nearly 70% of Islamic financial assets; banking assets +9.7% in 2025 | same |
| Sukuk outstanding | passed **USD 1 trillion** (+21.8%); Islamic funds +25.6%; takaful +13.9% | same |
| Where | 75% of assets in the GCC and East Asia-Pacific; growth increasingly in emerging and frontier markets | same |
| Earlier readings | USD 1.4 trillion and nearly 400 institutions (IMF, 2014 data); USD 2.19 trillion, banking 72%, 60+ countries, 12 of systemic importance (IFSB-AMF, 2018 data) | IMF WP/16/42; IFSB-AMF 2019 |
| Iraq | 34 of 78 licensed banks are Islamic (31 private, 1 state, 2 foreign branches); 8.1% of banking assets and 3.7% of deposits at end-2021 | CBI register; Fitch 8 Jul 2022 (section 7a) |

### 1b. Who sets what

| Body | Role | Binding in Iraq? |
|---|---|---|
| FATF and MENAFATF | AML/CFT standards; no tailoring for Islamic finance ("The FATF standards are implemented without any form of tailoring", IMF 2016) | Yes, through Law 39/2015 and the CBI controls (file 01) |
| AAOIFI | Accounting, auditing, governance, ethics and Sharia standards (26 accounting, 5 auditing, 2 ethics, 7 governance standards in the older set, with GS 10-12 added later) | **Yes**: Law 43 of 2015 applies AAOIFI standards where the law is silent |
| IFSB | Prudential standards and guiding principles (IFSB-3 governance, IFSB-10 Sharia governance (2009, from memory), IFSB-17 core principles, IFSB-19 sukuk disclosure, IFSB-31 on supervising Sharia governance adopted July 2025) | Persuasive; the CBI is an IFSB-aligned supervisor |
| IsDB and its research arm IRTI | Research, Islamic social finance report, waqf core principles (2018, with Bank Indonesia and the Indonesian Waqf Board) | No |
| CIBAFI | Industry association, 140 institutions from 37 countries (Shafaq News, 20 Feb 2024) | No |
| OIC International Islamic Fiqh Academy | Sharia rulings, for example resolution 179 (April 2009) against organised and reverse tawarruq | Reflected in CBI 2018 controls |
| Sharia supervisory board of each bank | Binding rulings on products and contracts (Law 43 art. 7) | **Yes** |

### 1c. Why this deserves a file of its own for Iraq

1. **Islamic banks are 44% of licensed banks and 61% of the banks on the CBI's USD-restricted list** (section 7d). A plan for Iraqi banks that ignores them leaves out nearly half the licensed banks.
2. The **CBI 2018 controls ban organised tawarruq** (a line the OIC Fiqh Academy also draws, and stricter than AAOIFI Standard 30 as a secondary summary describes it; the AAOIFI text was not read) **and foreign-currency murabaha**; those rules are checkable and no AML tool read for this research encodes them.
3. The **CBI AML scenario catalogue has no Islamic-product scenario** (file 27). A product that adds a sound extension fills a gap examiners can see.
4. **No published typologies exist**; the IMF asked FATF and the Islamic standard-setters to study them in 2016 and the IFSB-AMF paper in 2019 found only supervisor opinion. The honest message for leaders: the evidence says "no proven difference", and also "no one has looked at transaction data".
5. The **Sharia board is a second control layer** that touches the same contracts the AML system monitors.
6. The segment is **many small banks** with thin capital (IQD 250bn) facing a CBI push to IQD 400bn and mergers.

### 1d. Bottom line for the plan

"Islamic" is not the risk factor. The evidence in Iraq points to **ownership transparency, dollar-window behaviour, import-trade documents and young, thinly capitalised banks**. Islamic contracts add a small number of extra places where a round trip can hide: the supplier who is really the customer, the commodity trade that never happened, the profit allocation, the charity flow. The product should therefore read contracts and parties differently, not rate Islamic customers higher.

---

## 2. What the standard-setters say about ML/TF in Islamic finance

Short version: the two main primary sources (IMF 2016; IFSB and Arab Monetary Fund 2019/2020) both conclude there is
**no proven difference in ML/TF risk** between Islamic and conventional banking, but both say the question has
**never been tested with typologies or data**, and both name the same three places where Islamic finance could
differ: the partner-style customer relationship, the asset-based complexity of some contracts (tawarruq, SPVs), and the
volume of zakat and charity flows. No typology report or standard on AML/CFT for Islamic finance was found in the AAOIFI
standards list (governance standards GS 1-12 cover Sharia governance, ratings, internal Sharia audit and sukuk, none AML), in the IFSB standards
(IFSB-17 principle 33 only repeats Basel Core Principle 29; IFSB-31 on Sharia governance supervision was adopted in July 2025) or in the two
papers below. The FATF website blocks automated access, so its publication list was not checked directly (gap). For AKR-TMS
this means: Islamic-specific detection logic has to be designed from the contract mechanics, not copied from a
published typology. The product must treat that as a design risk, and say so openly to bank leadership.

### 2a. IMF Working Paper WP/16/42 (February 2016)

"Islamic Finance and Anti-Money Laundering and Combating the Financing of Terrorism (AML/CFT)", Nadim Kyriakos-Saad,
Manuel Vasquez, Chady El Khoury, Arz El Murr, IMF Legal Department, 12 pages. (IMF blocks automated download; text
obtained through the IMF full-text path and read in full.)

| Point | What the paper says (verbatim where quoted) | Para |
|---|---|---|
| Overall | "There is no evidence that the ML/TF risks in Islamic finance are any different from those posed by conventional finance. Rather, the choice of conventional or Islamic financial institutions to launder the proceeds of crimes or finance terrorism would appear to be dictated by convenience and opportunity." | 5, 9 |
| But untested | "No ML/TF typologies or indicators related to Islamic finance have been published by AML/CFT agencies or FSRBs, and there is no common understanding of the typologies and techniques used for ML/TF in Islamic finance." | 10 |
| Standard-setters | AAOIFI and IFSB "have not yet developed comprehensive standards or best practices related to AML/CFT in the Islamic finance context" (IFSB-17 CPIFR principle 33 only repeats Basel Core Principle 29). | 5 |
| Supervisory capacity | "The limited capacity of AML/CFT supervisors combined with a weak understanding of the specific features of Islamic finance in jurisdictions generally identified as facing higher ML/TF risks represents a vulnerability that could be exploited". | 11 |
| Vulnerability 1: partner relationship | An Islamic institution has a "partner" relationship with clients. The partnership "may distort the institution/client relationship from an AML/CFT perspective" and cause a "potential conflict of interest" over suspicious transaction reporting. Joint liability: "if a mudarabah investment is found to have involved illicit activities or the funding of terrorism, both partners (and not only the client) could be considered to have perpetrated the offense and be criminally liable." | 11 |
| Vulnerability 2: complexity | Islamic products are asset-based ("economic intermediation, as opposed to financial intermediation"), which "introduces a layer of complexity". Tawarruq: "Since tawarruq can provide quick access to funds, which can be factored against the purchase of an asset and repaid relatively quickly, it has the potential of being misused in frequent or large recycling of criminal proceeds." SPVs, often offshore, add opacity of origin, destination and valuation of the underlying assets. | 11, fn 21 |
| Vulnerability 3: zakat and sadaqat | "Zakat and Sadaqat funds are collected on behalf of and from customers, often in large volumes... The institutions have often the discretion to designate as beneficiaries natural and legal persons, including non-profit organizations (NPOs)". Without sound ML/TF risk management this raises "reputational, operational, compliance and concentration risks". | 11 |
| FATF concept gap: "customer" | The standards do not define "customer"; it is "not clear whether it applies to the types of partner-based relationship... that are a key feature of Islamic finance". CDD should explicitly apply "to the other party in the contractual relationship, including the partner". | 13 |
| FATF concept gap: "financial institution" | The glossary defines a financial institution as acting for or on behalf of a customer; Islamic institutions act "for or on behalf of partners". | 13 |
| Commodity murabaha definition | "a customer in need of liquidity or financing arranges for an Islamic bank to buy metals for that amount on his behalf. The bank then sells the metals to the customer at a mark-up that is payable over a period of time... In turn, the client immediately sells the metals on the spot market and obtains the needed liquidity." | fn 21 |
| Size (2014) | Nearly 400 Islamic banking institutions, assets above USD 1.4 trillion; about 70 percent stand-alone, 30 percent windows of conventional banks; banking is about 80 percent of Islamic finance assets. | 4, fn 6 |
| Recommendations | (1) standard-setters (FATF with AAOIFI, IFSB, FSRBs) study ML/TF methods, trends and typologies in Islamic finance; (2) Islamic institutions build experience and tailor preventive measures to their products; (3) assessors check whether FATF standards need adapting. | 15 |

### 2b. Joint IFSB-AMF Working Paper "ML/FT Risks in Islamic Banking" (December 2019, published 2020)

Authors Md Salim Al Mamun and Abideen Adeyemi Adewale (IFSB), Ghassan Abu Mwis and Nouran Youssef (Arab Monetary Fund
- AMF); 62 pages; the authors state it does not represent the views of the IFSB or AMF. Method: a **survey of 24
regulatory and supervisory authorities (RSAs)**, closed and open questions, so the evidence is **supervisors' opinions,
not transaction data**. Read in full on 2026-10-06 (file: backup.amf.org.ae PDF).

Headline conclusions (abstract and section 5):
- "The paper does not find any significant difference in the ML/FT risks between conventional and Islamic banking."
- The concern that zakah, waqf and sadaqah platforms could be used to mobilise, store and disburse funds for ML/FT "is
  unfounded based on the survey responses."
- Most RSAs see "no merit in introducing specific regulations": 15 of 24 (63%) no merit; 7 (29%) merit; 2 (8%) merit only
  for Islamic banks and windows. 23 of 24 (96%) have no Islamic-specific AML/CFT laws or regulations; 23 of 24 apply the
  same supervisory approach to both.
- Global context: Islamic financial services industry USD 2.19 trillion (IFSB Stability Report 2019); Islamic banking about
  72% of it, present in 60+ countries, 12 of systemic importance (15% market share test). Sukuk outstanding USD 399.9
  billion at end-2017.

Where the same paper still finds specific points of attention (these are the parts that matter for product design):

| Topic | Finding (numbers are RSAs out of those answering) | Design meaning |
|---|---|---|
| KYC scope | Islamic banks must know "the end-to-end parties involved in the Islamic contract": "Banks should also conduct CDD and screening of suppliers/developers/manufacturers" (murabaha, istisna'). 88% (15/17) of RSAs say knowing the customer is very important/important; 82% (14/17) say knowing the sources and uses of funds is at least as important as for conventional banks. | The party model must hold supplier, developer, commodity broker and agent, not only the customer. Screening must cover them. |
| Sharia process as a control | Because the bank must own the asset before selling it (murabaha, istisna'), ex-ante and ex-post Sharia review of contracts "will not only enhance Sharia compliance but also increase the disclosure requirement" (IFSB-3 principles 2.1 and 3.1). | Sharia-review records are an evidence source. A contract without the purchase leg is both a Sharia breach and a red flag. |
| Complex transactions | 8 of 24 (33%) rate their management of complex-transaction risk only "moderate" (60% at least substantial). "An example is made of a commodity murabahah transaction and how it can be misused to recycle proceeds from crime. While this remains a theoretical conjecture..." | Commodity-murabaha and tawarruq monitoring is a gap area; build it. |
| Correspondent services | 11 of 19 RSAs rate correspondent services "high" or "highest" risk for both Islamic and conventional banks. CIBAFI 2018: closure of correspondent relationships is the fifth-biggest risk faced by Islamic banks. | Matches Iraq's dollar-access problem (section 7). |
| Awqaf (waqf) accounts | 17 of 20 RSAs allow Islamic banks to open waqf accounts. 6 of 12 see no specific risk; 4 see specific risk (religious NPOs, diversion of waqf funds, bank failing to know the beneficiaries); 2 want transparency rules. Paper's recommendation: EDD for awqaf accounts, "approval of the NPO by the respective competent authority, thorough customer profiling, and ongoing monitoring and transaction scrutiny", plus "a profile of anticipated account activity" and checks that transfers "are in accordance with the spending plans of the charity". | A waqf customer type with beneficiary and mutawalli (trustee) records, spending-plan profile, and spending-versus-plan monitoring. |
| Zakat and sadaqah | 17 of 20 RSAs see no ML/FT risk in banks collecting or distributing zakat. But: in 7 jurisdictions banks pass zakat to a government fund; in 2 they pass it to approved NGOs/NPOs; in 4 of 16 they distribute on customers' instructions; in 8 of 16 (50%) at the bank's own discretion. "The role of Islamic banks in transmitting zakah collections to non-profit organisations may expose them to FT risks owing to their association with the NPOs, especially where the NPOs are not regulated." Large charity distributions through a bank "may indicate that enhanced CDD is warranted". | A charity-disbursement register: beneficiary due diligence, screening and approval before the bank pays out. |
| Profit-sharing investment accounts | Freeze and confiscation effects differ: for a restricted PSIA (RPSIA) the loss falls on the account holder unless the bank was negligent in CDD; for an unrestricted PSIA (UPSIA) the funds are pooled, so a freeze "will invariably affect the bank's assets to the extent of the client's investment". | Freeze workflow must know the account type; pooled-fund freezes need Finance and Sharia-board involvement. |
| STR behaviour | Islamic banks filed fewer STRs than conventional banks where comparable (mostly for non-ML/FT offences); Bangladesh Islamic banks 426 STRs by 2015 and about 670,000 cash transaction reports; Pakistan Islamic banks 172 STRs by 2018. No RSA reported reluctance to file because the customer is a "partner". Quality no different (same compliance teams serve both). | The partner-conflict risk is theoretical in the survey, but AKR-TMS should still measure STR rates for Islamic versus conventional portfolios as an effectiveness check. |
| Alternative remittance (hawala) | More than 40% of RSAs rate informal remittance as high risk for FT, with no difference between conventional and Islamic banking. Paper recommends EDD and intensified monitoring of accounts linked to alternative remittance transfers, "in Islamic banks or in conventional banks that have Islamic facilities or windows". | Hawala-pattern scenarios apply equally (FATF action plan item 2, file 01). |
| De-risking | 19 of 22 RSAs say improved CDD and stricter ongoing due diligence manage the risks; "rather than following the de-risking approach, the Islamic banking sector should focus on implementing a risk-based approach". Senior-management approval before accepting the customer and disbursing funds. | Approval-before-disbursement gate for high-risk financing. |
| Sanctions pressure | 21 RSAs reported no listing, lawsuit or correspondent pressure linked to Islamic products; two institutions in two countries were sanctioned for links to a sanctioned country; one correspondent relationship was terminated after a shareholder pressured the bank into breaching its own controls. | Shareholder-pressure indicator (Iraq-relevant, file 01 section 4). |
| Supervisory frequency | Of the RSAs: annual monitoring 7 (29%), semi-annual 4 (17%), monthly 2 (8%), other 9 (38%). One RSA reports on-site exams at least every two years. | Reporting calendar defaults. |

Caveats the plan should state plainly: (1) the survey is self-reported by supervisors, several of them from countries with weak
AML/CFT effectiveness; (2) it covers no transaction data; (3) the IFSB itself noted in its 2016 Stability Report "the lack of
empirical data to validate" claims that Sharia provisions make Islamic banking inherently safer; (4) the authors still list
tawarruq-style recycling as "theoretical conjecture" that deserves "supervisory interest".

### 2c. Documented enforcement narratives that involve Islamic institutions (what exists in this research set)

None of these is a case of laundering through an Islamic contract; they concern **Islamic banks as counterparties**, judged by their owners, links and controls like any other bank.

| Case | What the record says | Source |
|---|---|---|
| HSBC USA and Al Rajhi Bank (US Senate subcommittee report 17 July 2012; OCC 2012) | After 9/11 evidence emerged that Al Rajhi Bank and some owners had links to financing organisations associated with terrorism; HSBC reversed a decision to cut ties and, "after Al Rajhi Bank threatened to pull all of its business", resumed a US banknote programme worth "nearly USD 1 billion" until 2010 "despite ongoing troubling information"; HSBC USA also supplied dollars to Islami Bank Bangladesh | File 20 section 2.4 (item 8), grade A |
| Habib Bank Limited, New York (NYDFS, 7 Sep 2017) | The branch "facilitated billions of dollars in transactions with a Saudi private bank, the Al Rajhi Bank, with reported links to al Qaeda, without adequate anti-money laundering controls" | File 20 section 3.4, grade B (press release) |
| CBI USD restrictions (2019-2024; section 7d) | 17 Islamic banks on the restricted list and 2 blacklisted, for dollar-window and trade-document concerns rather than Sharia contracts | CBI lists, section 7 |
| UN-listed charities and exchange houses (section 4c) | Charity and hawala channels, including an Iraqi exchange house | UN consolidated list |
| UAE central bank, 25 June 2025 | Islamic window barred from new customers for six months for Sharia non-compliance | CBUAE release, section 5a |

Reading: the only enforcement pattern with an Islamic label in the documents read is **correspondent exposure to an owner or group**, which is why file 24 stresses ownership screening (7d, hook list) and the correspondent-readiness view. A targeted search for transaction-level cases (Gulf, Pakistan, Malaysia, Turkey, Sudan) is still open (section 9, gaps).

### AKR-TMS hooks (section 2)

- Treat Islamic and conventional business as **one rule set plus an Islamic extension pack**, not a separate regime: 96% of
  supervisors apply the same rules, so the base library (file 27) applies to Islamic banks unchanged.
- The extension pack covers the three IMF/IFSB areas: party model for contract counterparties (supplier, broker, agent,
  developer), commodity-trade integrity (section 3), and charity flows (section 4).
- Report Islamic versus conventional **alert rates, STR rates and typology coverage** per bank that has both, as an
  effectiveness check that the literature says nobody has done yet. This gives banks and the CBI a data point no standard-setter has.
- The **Sharia board and Sharia review records** are a data source for monitoring (contract approvals, non-compliance income
  sent to charity), not only a governance feature (section 5).

---

## 3. Product typologies: how each Islamic contract moves money and where it can be abused

Method and honesty note. No published ML/TF typology exists for Islamic products (IMF 2016 para 10, section 2a). This section therefore combines
(S) statements in sources read in full (IMF, IFSB-AMF, CBI 2018 controls, Law 43, MER, UN list), (R) rules the CBI wrote that imply a known abuse, and (A) analysis by this
research from the contract mechanics, to be tested with bank data. Each row says which kind of evidence supports it. "Customer" below means the
bank's financing customer; the key difference from a conventional loan is that in sales- and lease-based contracts **the bank's money usually goes to a third party
(supplier, seller, contractor, broker), not to the customer**, so the party that receives value is often outside the customer's account (IFSB-AMF section 2.2:
"an Islamic bank must ensure that it purchases the asset from a third party such as a supplier, developer or manufacturer before it is sold to a customer"; the
paper adds that "banks should also conduct CDD and screening of suppliers/developers/manufacturers").

| Product | How it works (money and asset flow) | Abuse patterns | Evidence | Data AKR-TMS needs |
|---|---|---|---|---|
| **Murabaha to the purchase orderer** (cost-plus sale, price paid later) | Customer names a good; the bank buys it from a third-party supplier, takes legal and constructive possession, resells at cost plus an agreed margin payable in instalments (IFSB-AMF glossary). Bank pays the supplier; customer pays the bank. | (1) Supplier is the customer or a related party: a disguised cash loan; (2) inflated invoice, difference returned in cash; (3) **import murabaha used to buy dollars against paper deals**; (4) ghost supplier (new company, shared address or phone, no customs record); (5) early settlement from an unrelated third party or another country; (6) fictitious goods. | S/R: CBI 2018 items 6-2-1 to 6-2-3 void a murabaha whose seller is the customer or his agent and require any earlier customer-seller link to be cancelled "really and not formally". S: MER and NRA name "falsified import invoices" and "import licences in names of people not active in trade" (file 02). A: dollar-window use. | Contract (customer, supplier, goods, quantity, unit price, cost, margin, deferred price, schedule); the supplier payment (account, bank, country, date); possession evidence (invoice, customs declaration, title or registration); Sharia approval reference. |
| **Commodity murabaha and tawarruq** (liquidity product) | Bank buys a commodity (metals) from a broker on credit for the customer, sells it to the customer at a mark-up payable later; the customer sells it for cash to a third party. Net effect: cash today, a larger debt later. | Recycling of criminal proceeds through "frequent or large" quick cycles (IMF); fictitious commodity trades; circular chains among related brokers; same-day legs showing the contracts are linked; round amounts. | S: IMF fn 21 and para 11 ("has the potential of being misused in frequent or large recycling of criminal proceeds"); IFSB-AMF 4.1.2 (33% of regulators rate their control of complex transactions only "moderate", the example is commodity murabaha, "theoretical conjecture"). R: **CBI 2018 prohibits organised tawarruq** (section 7c). | Commodity, broker, seller and buyer for each leg, quantities, prices, timestamps of each leg, delivery records, account credited with sale proceeds. |
| **Bay' al-'inah** (sale and buy-back) | Bank sells a good to the customer on credit and buys it back for less cash. | A loan with interest in a sham sale: cash today, a larger debt later, no real goods. | R: CBI 2018 prohibits it as a "sham Islamic contract" (section 7c). | Goods or SKU identity on both legs; bank and customer as seller and buyer in opposite directions within a window. |
| **Ijara and ijara muntahia bittamleek** (lease and lease-to-own) | Bank buys an asset (vehicle, equipment, building) and leases it; in lease-to-own ownership passes at the end. The bank owns the asset during the lease. | Sale-and-leaseback round trip (customer sells an asset to the bank at an inflated price, gets cash, leases it back); asset bought from a related supplier; rentals paid by third parties or in cash; early buy-out from unknown funds; double financing of one asset; property flips. | R: CBI 2018 (pp. 5-6, 12) allows sale-then-lease-back in both directions with accounting rules, and for real estate bought for investment the bank may not resell until **two years after the project's break-even point** except in default (item 16). Law 43 art. 6 bans financing real-estate brokerage. A: the rest. | Asset register (VIN, serial, title or Tabu registration, valuation, owner history), lessor/lessee, rent schedule, who pays the rent, buy-out events, insurer (takaful). |
| **Mudaraba and profit-sharing investment accounts (PSIA)** | Depositor (investment account holder) supplies capital; the bank manages it as mudarib; profit is shared by agreed ratio, loss falls on the capital provider unless the bank is negligent. **Restricted** accounts go to a named investment; **unrestricted** accounts are pooled with the bank's funds. | Parking proceeds in an account that looks like savings; third-party funding of the account; profit allocations that favour one account (a way to create "clean" income or to siphon); deposits just before the profit run and withdrawals just after; fixed "profit" payments that make it a loan in disguise. | S: IMF para 11 (joint criminal liability of partners). S: IFSB-AMF 4.4 (a freeze or confiscation of an unrestricted account "will invariably affect the bank's assets to the extent of the client's investment"; for restricted accounts the holder bears it unless the bank was negligent in CDD). R: CBI 2018 mudaraba rules: profit only as a ratio, never a fixed sum; capital must be cash or valued goods and may not be a debt owed by the mudarib. A: profit-allocation abuse. | Account type, pool, weightage and days-weighted balance, profit rate declared against the account's credited profit, deposit and withdrawal dates relative to the profit run, source of funds. |
| **Musharaka and diminishing musharaka** (partnership financing) | Bank and customer co-own a venture or an asset; the bank sells its share gradually. | Capital contributed from unknown sources; partner companies with hidden beneficial owners; buy-out accelerated by third-party money; related-party partnerships where the bank's money returns to the customer as "profit". | S: IMF para 13 ("partner" is not covered by the FATF concept of "customer"). A: patterns. | Partner records and beneficial owners, capital contribution source, share schedule, buy-out payments and their payers. |
| **Wakala deposits and investment agency** | Customer gives the bank money to invest as agent for a fee plus an incentive above a stated return (Law 43 art. 5 item 11). | Large third-party or government wakala deposits used as pass-through; incentive structures that disguise returns; investments through SPVs. | S: Law 43 art. 5. A: patterns. | Depositor, source, agency terms, investment target, fees paid. |
| **Salam and istisna' (and their parallel contracts)** | Prepayment for goods to be delivered later (salam) or for something to be made or built (istisna'); the bank signs a parallel contract with the real supplier or contractor. | Prepayments with no delivery; progress payments to related contractors; invoice or progress-certificate manipulation; circular parallel contracts; public-contract context. | S: MER para 695 lists companies with public-sector contracts among those most exposed (file 02). A: patterns. | Contract and parallel contract parties, milestones, certificates, payments against each milestone, delivery evidence. |
| **Qard hasan** (benevolent loan) | Interest-free cash loan, principal only. | Structuring below the cap, loans to relatives of insiders, recipients abroad, repayments in cash by unrelated third parties, charity labels used to justify flows. | R: CBI 2018 limits (section 7c): IQD 10m per loan, recipient inside Iraq, six months between loans, 10% of net profit in total, no insiders. | Loan, recipient class, recipient residence, repayment source, repeat history, staff or board link. |
| **Sukuk and Islamic funds** | Certificates of ownership in assets held by an issuer or SPV. | Layering and integration through securities, anonymous holders, SPV opacity, offshore issuers. | S: IFSB-AMF 2.3.2 (regulators should collect "the originator of the sukuk, location of origin, issuer, location of issuance, and sukuk holders"; IFSB-19 sets disclosure rules). Sukuk outstanding passed USD 1 trillion in 2025 (IFSB 2026 via press). Iraq: the CBI auctions Islamic deposit certificates (for example ICD148, 14 days, cbi.iq). | Issuer, originator, holders (including nominees), secondary trades, redemptions. |
| **Takaful and other Islamic insurance** | Mutual risk pooling with a takaful operator. | Early surrender, overfunding, premium paid by third parties. | S: Law 43 art. 5 item 8 lets banks set up self-insurance and reciprocal insurance funds. Insurance is 0.5% of Iraq's financial assets (file 02), so low priority. | Policy, premium payer, surrender or refund events. |
| **Islamic FX and currency dealing (sarf)** | Spot currency exchange under strict hand-over rules. | Obtaining dollars through "Islamic" financing; same-day FX turnaround; forward-like trades. | R: CBI 2018 bans foreign-currency murabaha and sets sarf conditions (section 7c); Law 43 art. 5 item 10 lets banks finance FX dealing in real-time markets. | Contract currency versus settlement currency, FX leg timing. |
| **Islamic trade finance** (murabaha LC, kafala guarantees, wakala LC) | LC and guarantee lifecycle with an Islamic contract on top. | Same as conventional TBML, plus the murabaha supplier problem. | S: CBI scenario catalogue Arts. 4-5 cover LCs and guarantees in conventional terms (file 27); no Islamic-product scenario exists in the CBI catalogue as transcribed in file 27. | Trade documents plus the murabaha contract record. |

### AKR-TMS hooks (section 3)

- **Party model with roles**: customer, supplier, seller, agent (wakil), commodity broker, partner, lessee, contractor, developer, with the CBI's "third party, not the customer or his agent" test as a first-class relationship check
  (shared owner, address, phone, email, signatory, bank account, device).
- **Deal view next to account view**: a financing deal page linking contract, parallel contracts, asset or commodity, supplier payment, schedule and Sharia approval, with the supplier payment shown as a transaction on the supplier, not hidden inside the customer's account.
- **Product taxonomy mapping** from each bank's product codes to canonical Islamic classes with a risk class for each (import murabaha is trade finance risk; qard hasan is small consumer credit; PSIA is term-deposit risk).
- **Evidence status per deal**: possession evidence present or missing, with ageing (a murabaha with no invoice or title after N days is an alert and a Sharia breach).

---

## 4. Zakat, sadaqa and waqf: charity flows and terrorist-financing risk

### 4a. What the flows are

- **Zakat**: an obligatory annual levy on wealth above a minimum (nisab) held for a lunar year (hawl); 2.5% on gold, silver, currencies and trade goods; 10%, 5% or 7.5% on
  agricultural produce depending on irrigation (AAOIFI Shari'ah Standard 35, quoted in the IFSB-AMF paper fn 44 and 55). **Sadaqa**: voluntary giving. **Waqf**: an endowment, the property cannot be sold or transferred and its yield goes to a purpose
  (AAOIFI Shari'ah Standard 33: charitable, family, joint and self-dedicated types).
- **Flows an Islamic bank sees** (IFSB-AMF 2.7; IMF para 11): (1) the bank's own zakat; (2) zakat or sadaqa collected from customers, often as a deduction from deposit accounts or through
  branches (3 of 20 regulators say banks collect from deposit accounts, 1 from non-account holders); (3) **income the Sharia board classes as non-compliant**, which "must be deducted and
  transferred to charitable activities"; (4) in Iraq, **late-payment surcharges on deferred sales, which go to a charity fund** and not to income (CBI 2018, p. 26); (5) customers' donations to NPOs by transfer; (6) waqf accounts;
  (7) the bank's own distribution to beneficiaries, which the Sharia board or management directs.
- **Who decides where it goes** (IFSB-AMF figs 2.11-2.12): of 16 regulators, 8 (50%) say banks distribute at their own discretion, 4 follow customers' instructions; 6 of 15 say banks distribute from their own profit; 7 say zakat is passed to a
  government fund; 2 say it goes to approved NGOs or NPOs. The IMF says the institutions "have often the discretion to designate as beneficiaries natural and legal persons, including
  non-profit organizations".

### 4b. What the sources conclude, and the counter-evidence

- IFSB-AMF: 17 of 20 regulators see no ML/FT risk in banks' zakat and sadaqa roles; the concern that these platforms are used "to mobilise, store and disburse funds for ML/FT is unfounded based on the survey responses". But the same paper finds that passing zakat
  to NPOs "may expose them to FT risks owing to their association with the NPOs, especially where the NPOs are not regulated", that distributing on a scale that needs a bank "may indicate that enhanced CDD is
  warranted", and that banks need "sufficient information not only on the NPOs, but also on the beneficiaries".
- IMF (2016): the large volume of zakat and sadaqat with discretion over beneficiaries, "in the absence of sound ML/TF risk management", increases "reputational, operational, compliance and concentration risks".
- Context: a 2002 estimate quoted in the IFSB-AMF paper (fn 14, from a UN-commissioned report) claimed Al-Qaida received USD 300-500 million through zakat; the paper reports it as a claim, not a finding.

### 4c. Verified examples of charity and exchange abuse (UN consolidated list, parsed 2026-10-06; entities only)

| UN ref | Entity | Listed | What the list says (abridged) | Pattern |
|---|---|---|---|---|
| QDe.005 | Al Rashid Trust | 2001-10-06 | "Involved in the financing of Al-Qaida and the Taliban"; headquarters in Pakistan; operations in Afghanistan, Kosovo, Chechnya; 10 aliases including "Al Amin Welfare Trust" and "Al Madina Trust"; banned in Pakistan since 2001; reportedly defunct. | Charity with many names and offices; alias proliferation. |
| QDe.091 | Global Relief Foundation | 2002-10-22 | Chicago-based; "Other Foreign Locations: Afghanistan, Bangladesh, Eritrea, Ethiopia, India, **Iraq**, West Bank and Gaza, Somalia and Syria". | Diaspora charity with a branch footprint in conflict zones including Iraq. |
| QDe.093 | Benevolence International Foundation | 2002-11-21 | reportedly defunct. | Charity listing. |
| QDe.104-106, 109, 116 | Al-Haramain Foundation branches (Pakistan, Kenya, Tanzania, Comoros) and Al-Haramain & Al Masjed Al-Aqsa Charity Foundation (Bosnia and Herzegovina) | 2004 | branches of one foundation designated country by country. | Branch-by-branch networks. |
| QDe.070 | Revival of Islamic Heritage Society | 2002-01-11 | "Only the Pakistan and Afghanistan offices of this entity are designated". | A listing that applies to part of an organisation: screening must read the scope note. |
| QDe.147 | Hilal Ahmar Society Indonesia | 2015-03-13 | "Ostensibly humanitarian wing of Jemaah Islamiyah"; "recruiting, funding and facilitating travel of foreign terrorist fighters to Syria"; "Not affiliated with" the Red Cross and Red Crescent movement. | Humanitarian branding that borrows a legitimate name; flows towards Syria. |
| QDe.157 | Al-Kawthar Money Exchange | 2018-03-06 | Exchange house in **Al-Qaim, Iraq** (licence number 202 issued 17 May 2000, since withdrawn), alias "Al-Kawthar Hawala"; "Facilitated financial transactions on behalf of companies associated with" ISIL. | Licensed Iraqi exchange house as the channel; licence withdrawn after the fact. |
| QDe.153 | Hanifa Money Exchange Office (Albu Kamal, Syria) | 2017-07-20 | "Used exclusively for ISIL-related transactions". | Border-town exchange house. |

Iraq-specific (MER 2024, file 02): **16 NPOs misused for TF in 2018-2022** (suspicious foreign funding, activity in former ISIL areas); NPO TF risk rated moderate; pre-2017 ISIL funding included "fake charities"; since 2017 extortion, theft, drug smuggling and foreign funding.
FATF R.8 requires a risk-based, focused approach to NPOs and warns against blanket de-risking (file 03); the IFSB-AMF paper also recommends a risk-based approach "rather than... de-risking".

### 4d. Waqf in Iraq and in the bank

See 7g: 3,555 endowments (2,902 general, 553 private, 100 joint), government-supervised, small annual payouts (USD 100-500 per beneficiary), no abuse found in 2018-2022. The IFSB-AMF paper's EDD recommendation for waqf accounts (competent-authority approval, profile of expected activity, spending-plan check) fits the Iraqi structure well, because the
spending plan is defined by the endowment deed and supervised by the Endowments Department and the court. Iraq's instructions (Instructions No. 1 of 2023 art. 7/First/C, file 01) already require the settlor, trustee, beneficiaries and controllers of waqf to be identified and screened.

### 4e. Indicators for charity flows (analysis, to be tested; none is in the CBI catalogue)

1. Donation collection through a **personal account** or one that is not a registered NPO (check against the NGO Department registry, ngoao.gov.iq).
2. Many small donations from unrelated donors followed by one large transfer, especially abroad or to a border area.
3. NPO account where inflows equal outflows within days (pass-through), or outflows to individuals rather than to registered beneficiaries.
4. Payment narrative "zakat", "sadaqa" or "waqf" on transfers to high-risk destinations or to listed or unregistered bodies.
5. Waqf account flows that do not match the deed's spending plan or the Endowments Office approval.
6. **Seasonality**: zakat and sadaqa peak in Ramadan and around Eid; Iraqi inflows spike around Muharram and Arbaeen (pilgrimage and religious tourism with Iran, file 02). Baselines must be built on the Hijri calendar, or every Ramadan will be an alert storm.
7. The bank's own **charity fund**: disbursements concentrated on one beneficiary, or paid to a party linked to staff, board or Sharia board members.

### AKR-TMS hooks (section 4)

- **Customer types**: NPO, waqf (with trustee, endower, beneficiaries as related parties), religious institution; fields for registration number and registry status, foreign funding flag, expected spending plan.
- **Charity-disbursement register** for the bank's own zakat, non-compliant income and late-payment surcharges: beneficiary, approval by the Sharia board, screening result, amount; alert on concentration or on a beneficiary with a screening hit.
- **Hijri-calendar baselines** and a religious-season calendar (Ramadan, Eid, Hajj, Muharram, Arbaeen) in the behaviour model; seasonal peaks are not anomalies until they exceed the seasonal norm.
- **Screening scope**: NPO officers, trustees, beneficiaries and the NPO's counterparties against UN, IQTFS and OFAC; read the UN scope note (for example only some offices designated).
- **Scenario pack C1-C8** (section 8).

---

## 5. Islamic windows and the role of the Sharia board

### 5a. Islamic windows

- **Definition** (IFSB glossary as printed in the IFSB-AMF paper, p. iv): "That part of a conventional financial institution (which may be a branch or a dedicated unit of that institution) that provides both fund management
  (investment accounts) and financing and investment that are Sharia-compliant, that is, with **separate funds**. It could also provide takaful or retakaful services."
- **Size**: IMF (2016, data to 2014): about 70% of Islamic banking institutions are stand-alone and 30% are windows of conventional banks. The IFSB Stability Report 2026 (press summary of 8 June 2026) lists risks of the "hybrid banking model" among its themes.
- **Iraq**: the CBI register classifies banks as conventional or Islamic and lists two foreign Islamic branches; Law 43 speaks of "Islamic Bank or branch" (art. 10). **No Iraqi regulation of Islamic windows in conventional banks was found** in the documents read (gap). The product should support windows anyway, because conventional Iraqi banks can be expected to open them and
  because the Gulf banks that run Iraqi branches (for example the two Islamic branches) run both books.
- **AML points for windows**: one customer can sit in both books, so the customer view and the risk rating must be consolidated (BCBS para 29, file 04 B-03); funds must stay separate, so a transfer between the conventional and
  the Islamic book is both a Sharia issue and a layering signal; the compliance team is usually shared ("most banking groups use the same compliance team for both", IFSB-AMF 4.5), so alert handlers need Islamic-product knowledge.
- **Supervisory precedent for windows**: on 25 June 2025 the UAE central bank **suspended an unnamed bank from onboarding new customers in its Islamic window for six months and fined it AED 3,502,214** after Sharia supervisory inspections found non-compliance with the instructions on
  Sharia requirements for Islamic windows (CBUAE press release, Arabic, one page). It is a Sharia-compliance action, not AML, but it shows regulators can freeze onboarding in one product line: the AML system must be able to honour a per-product-line onboarding block.

### 5b. The Sharia board and where it touches AML

| Source | What it sets | Relevance |
|---|---|---|
| Law 43 of 2015 arts. 7-10 (section 7b) | Five-member board, CBI-approved, independent of directors, employees and shareholders, three-year term, binding decisions; internal Sharia audit department; the board "express[es] an opinion on the business, activities and contracts". | A binding control that approves the contract forms AML rules read. |
| CBI 2018 controls, p. 61 item 3 | "The Sharia supervisory board bears the responsibility of identifying the types of sales that involve formality (sham) in creating contracts and whose purpose is to provide credit and make profit." | The board is the first line against sham contracts; its decisions should be data. |
| IFSB-3 principles 2.1 and 3.1 (cited in IFSB-AMF) | Investment account holders have a right to monitor their investments; banks need a mechanism to monitor Sharia compliance in all products and activities; ex-ante and ex-post review of contracts. | Disclosure that helps CDD. |
| IFSB-10 (2009, from memory) and **IFSB-31 (adopted July 2025)**, "Guiding Principles for Effective Supervision of Sharia Governance" | Supervisory expectations for Sharia governance in banks, takaful and capital-market institutions (IFSB press release, July 2025, via search summary). | Direction of travel: supervisors will inspect Sharia governance more. |
| AAOIFI governance standards (GS 6 governance principles; GS 10 Sharia compliance and fiduciary ratings; GS 11 internal Sharia audit; GS 12 sukuk governance, issued later) and Law 43 art. 13 binding AAOIFI standards on Iraqi Islamic banks | Internal Sharia audit and rating. | Findings from internal Sharia audit are risk signals. |
| IFSB glossary | "Sharia non-compliance risk: an operational risk resulting from non-compliance of the institution with the rules and principles of Sharia in its products and services." Non-compliant income must be deducted and given to charity (IFSB-AMF 2.2). | Non-compliance events are an operational-risk register the AML system can use. |

**Three layers of Sharia control and their AML value.**
1. *Ex ante*: product approval and contract templates fix what a "normal" murabaha, ijara or mudaraba looks like. A contract that uses a product code without a current approval, or that departs from the template (for example a fixed profit on a mudaraba), is an alert.
2. *During*: rulings on individual deals. These are evidence for why a structure was used and are kept in the evidence vault with the deal.
3. *Ex post*: Sharia audit and the non-compliance register (for example a murabaha without possession, a late fee booked as income instead of to the charity fund). Findings concentrated on one branch, officer or product are the same signals insider-risk analysts use.

**What the Sharia board must not see.** Suspicious activity reports and open AML cases are need-to-know and tipping-off-protected (Law 39 art. 12/Fourth, file 01). The board should get aggregate product dashboards and the list of sham-sale alerts, not customer-level SAR material.

**Independence and fit.** The board's independence rule (not a director, employee or shareholder) is checkable: screen members for PEP status, sanctions, adverse media, shareholding and cross-membership of several banks' boards.

### AKR-TMS hooks (section 5)

- **Book flag** on every account, contract and transaction (conventional, Islamic window, Islamic bank), a consolidated customer view across books, and an **inter-book transfer** alert.
- **Per-product-line onboarding block** state, driven by a bank's or the CBI's decision.
- **Sharia governance register**: board members (CBI approval date, term end, independence checks), product approvals (approved, restricted, prohibited, expired), Sharia audit findings, non-compliance events and remediation. Deals reference the approval; rules check it.
- **Non-compliance event** as its own case type routed to the Sharia board and compliance, separate from SAR workflow.
- **Role model**: Sharia board read-only on aggregated product dashboards; no access to SAR or investigation files.

---

## 6. Data model differences in Islamic core banking

Why this matters for a read-only mapping layer: an Islamic CBS stores **deals, assets and pools**, not just accounts and loans, and the money that matters often moves **outside the customer's account**.
Vendor evidence (Temenos page fetched 2026-10-06): its Islamic solution lists "Islamic Financing" (contracts across retail, commercial and corporate), "Islamic Profit Distribution" ("automate profit calculation and distribution with transparent investment pooling"),
"Shari'ah Compliance" ("configurable parameters"), a "Portfolio Manager" with trading restrictions and approval workflows, treasury including sukuk, trade finance (letters of credit, guarantees, collections, reimbursements) and microfinance. File 17 section 3 covers Path Solutions iMAL and other CBS vendors in detail; file 18 section 5 covers which Iraqi Islamic banks run what (both being written by other agents; cross-reference when complete).
Several Iraqi Islamic banks may instead book Islamic financing in a conventional loan engine with a product code, so the mapping must work both ways.

| Concept | Conventional CBS | Islamic CBS (typical) | Monitoring consequence | Canonical field or entity |
|---|---|---|---|---|
| Unit of business | Loan or deposit account | **Deal**: contract with type, parties, asset or commodity, parallel contracts, schedule, Sharia approval | Alerts attach to the deal and its parties, not only to a customer account | `financing_deal`, `deal_party(role)`, `deal_asset` |
| Parties | Borrower, guarantor | Customer, **supplier/seller, agent (wakil), broker, partner, lessee, developer, contractor**; waqf: endower, trustee, beneficiaries | Screen and graph all roles; supplier-equals-customer tests | `party`, `party_relation`, `deal_party.role` |
| Where the money goes | Loan proceeds credit the customer's account | **Bank pays the supplier or seller directly**; customer's account may show only instalments | Transaction monitoring must read disbursement and supplier-payment tables and GL entries, not only customer accounts | `disbursement`, `counterparty_payment` linked to `financing_deal` |
| Asset records | Collateral register | **Asset or commodity register**: description, quantity, unit cost, invoice, possession date (legal and constructive), title or registration, VIN or serial, valuation; inventory "available for deferred sale" on the balance sheet (CBI 2018 p. 25) | Possession evidence, price outliers, double financing, flips | `asset`, `asset_evidence`, `asset_valuation` |
| Commodity trades | None | **Legs** with broker, seller, buyer, timestamps, quantities, delivery notes | Linkage between legs, circular chains | `commodity_trade_leg` |
| Pricing | Interest rate | Mark-up fixed at signing (cost plus margin), **deferred price**, **deferred profit account**, rent, profit-sharing ratio; early-settlement rebate; **late-payment surcharge to the charity fund** (CBI 2018 pp. 25-26) | Margin outliers; rebates; charity-fund inflows | `deal_price`, `deferred_profit`, `rebate_event`, `charity_fund_entry` |
| Deposits | Interest-bearing | **Current/qard or wadi'a** (no return, principal guaranteed), **mudaraba investment accounts** (restricted or unrestricted), **wakala deposits** | Different risk and expected-activity logic per type | `account.type`, `investment_account` |
| Profit distribution | Interest accrual | **Pool run**: profit allocated to investment accounts by weightage (balance x days x weight) after the bank's mudarib share and reserve transfers (profit equalisation, investment risk; general Islamic accounting practice, not verified in this session) | Compare each account's credited profit with the pool rate and weightage; outliers show allocation abuse | `pool`, `profit_run`, `profit_allocation` |
| Charity and zakat | Not a core concept | **Charity fund, zakat payable, non-compliant income** and their disbursements | Disbursement register and screening | `charity_fund_entry`, `disbursement_approval` |
| Governance data | None | **Sharia approval reference**, fatwa, Sharia audit finding, non-compliance event | Rules check approval; findings feed risk | `sharia_approval`, `sharia_finding` |
| Product taxonomy | Loan, overdraft, term deposit | Murabaha, ijara, mudaraba, musharaka, wakala, salam, istisna', qard hasan, sukuk, takaful; AAOIFI reporting classes | Map to canonical risk classes | `product_class_map` |
| Currency | Same currency in and out | Contract currency can differ from settlement currency (CBI 2018 bans FX murabaha: dollars financed, dinars repaid) | Flag mismatch | `deal.contract_ccy`, `deal.settlement_ccy` |
| Calendars and language | Gregorian, English or Arabic | **Hijri dates** in Sharia documents, Arabic contract text | Hijri-aware baselines; Arabic text fields for contract description matching | `date_hijri` support, `text_ar` |

Data-quality traps to test per bank: Islamic and conventional products in the same tables; contract type held only in a free-text description; parallel contracts without a link key; supplier names typed free-hand (no supplier master); asset fields blank for old deals.

### AKR-TMS hooks (section 6)

- Mapping templates per CBS (Temenos Islamic, Path iMAL, FLEXCUBE Islamic, Finacle, local builds) that land in the canonical entities above; **mapping coverage report** that shows which Islamic fields a bank can and cannot supply.
- A **supplier master and entity-resolution pass** on free-text supplier names (Arabic variants, spelling, commercial registration numbers).
- **Profit-distribution reconstruction** from the CBS pool and allocation tables, or from daily balances if the bank only stores the result.

---

## 7. Iraqi Islamic banks: size, law, CBI rules, dollar restrictions

Sources read for this section (all 2026-10-06 unless stated): English translation of Islamic Banks Law No. 43 of 2015 (cbi.iq);
CBI "Controls on Islamic financing instruments" of 1 August 2018 (68 pages, scanned Arabic, read from page images);
the CBI's own register of licensed banks (cbi.iq pages 23, 24, 114); the CBI lists of USD-restricted banks (published
22 February 2026, PDF dated 26 February 2026); CBI statement of 18 July 2026; CBI "Strategic Achievements 2020-2025"
(Governor's Office, 18 May 2026, 63 slides); Fitch (8 July 2022); MER 2024. Treasury and press reports are marked as such.

### 7a. How many Islamic banks, and how big

The CBI's own register (crawled 2026-10-06) lists 79 entries in the banks section, including banks in liquidation, guardianship or custody. Islamic banks
are 34 of the 78 licensed entries (the 79th, a Kurdistan state bank, has not started work):

| Group (CBI register) | Count | Islamic | Notes |
|---|---|---|---|
| State banks | 7 operating | 1: Al-Nahrain Islamic Bank (licensed 17 Aug 2015, capital IQD 150bn) | the "one state-owned Islamic bank" in the Fitch count |
| Private local conventional banks | 24 | 0 | 3 in custody, guardianship or liquidation; 11 flagged "banned from dealing in dollars" |
| Private local Islamic banks | 31 | 31 | 5 in custody, guardianship or liquidation; 17 flagged "banned from dealing in dollars"; typical capital IQD 250bn, largest IQD 510bn |
| Foreign bank branches | 16 | 2: Abu Dhabi Islamic Bank, Al Baraka Turk Islamic Bank (USD 80m capital each) | 2 Iranian branches under guardianship and banned |
| **Islamic total** | **34 of 78 (44%)** | | |

Trend: the first mutual evaluation (World Bank-led, MENAFATF, adopted 28 November 2012) counted **46 banks, 9 of them Islamic**, and wrote "Islamic banks need to be brought under the
purview of CBI regulations and supervision through adoption of an Islamic banking law". Fitch counted **30 of 76 banks** Islamic at end-2021 (1 state, 29 private); CBI
said "over 30 Islamic banks" in February 2024 (Shafaq, 20 Feb 2024). **22 of the 31 private Islamic banks (71%) were licensed in 2016
or later**, that is in the first years after Law No. 43 of 2015 (4 November 2015); only 7 date from 2001-2008 and 2 from 1992-1993
(two of the six private banks with assets above USD 1 billion in the CBI deck are Islamic: Iraqi Islamic Bank, licensed 1993, and Cihan Bank for Investment and Islamic Finance).

Market share (Fitch, 8 July 2022, data for end-2021): Islamic banks' assets **IQD 12.9 trillion (USD 8.8bn)**, **8.1% of banking
assets** (5.3% in 2017) and **3.7% of deposits** (3.0% in 2017); asset growth 18.2% against 15.1% for conventional banks; 81% of
adults unbanked in 2021; Fitch lists "weak operating environment", "limited regulatory framework enforcement" and "money
laundering concerns" among the risks. No later share figure was found (gap). For scale, the CBI deck gives sector assets of **IQD 186.7
trillion at 30 June 2025**: state banks 141.9tn (76%), private banks 40.5tn (22%), foreign branches 4.2tn (2%); private banks hold 54%
of equity but only 13% of deposits. Islamic banks sit inside that 22% private slice, so their present share of assets is likely
a single-digit to low-teens percentage (inference, not a published number).

Consequence for the market: the Islamic segment is **many small banks** (31 private banks with about IQD 250bn capital each) rather
than a few large ones. The CBI reform plan (7f) sets IQD 400bn minimum capital and pushes mergers, so the customer count is likely to fall
while each remaining bank grows.

### 7b. Islamic Banks Law No. 43 of 2015 (President's Decision No. 46, 4 November 2015)

Read in the CBI's English translation (10 pages). Article numbering in the translation is imperfect (the general-provisions article
prints as "7" but is the 13th in sequence); wording below is quoted from it.

| Topic | Provision | AML/KYC relevance |
|---|---|---|
| Nature | An Islamic bank is set up under the Companies Law 21/1997, Public Companies Law 22/1997 and Banking Law 94/2004; its memorandum must commit it to banking that "does not deal with given or taken interest" and to "the form of banking transactions that do not conflict with the provisions of Islamic Shari'a" (art. 1). | Contract form is part of the licence conditions. |
| Capital | Paid-in capital "not less than 250 billion dinars", 100bn on establishment and the rest in three annual instalments (art. 4). | Explains the many IQD 250bn banks; CBI now wants 400bn. |
| Permitted business | Financing and investment; deposits; real estate and leasing; investment portfolios and funds; "mutual muqharada" and "specified muqardha" instruments; self-insurance and reciprocal insurance funds; financing "for the purposes of dealing in foreign currencies in the real-time foreign exchange markets"; deposit investment under wakala "for a fixed fee only or a fixed fee plus a profit... if it exceeds a certain limit mentioned in the contract in advance" (art. 5, items 1-12). | Product list the data model must hold: wakala deposits, muqarada bonds, takaful-like funds, FX financing. |
| Prohibitions | Interest; investing or financing "any commodity or project not permitted by Islamic law"; "financing brokerage in real estate projects"; fixed assets above 30% of net own funds (art. 6). | Real-estate brokerage financing is banned, and real estate is a high-risk sector in the NRA (file 02). |
| Sharia Supervisory Board | Five members, at least three jurists and at least two with banking, legal or financial expertise; appointed by the founders (later the general body) **with CBI approval**; members may not be directors, employees or shareholders; three-year renewable term; "The decisions of the Board shall be binding on the Islamic Bank"; removal only by a two-thirds board vote with general-body approval (arts. 7-9). Duties: monitor operations, review the Internal Sharia Audit reports, report to shareholders, "Express an opinion on the business, activities and contracts" (art. 8). | The board is a control with binding force and a CBI approval step. |
| Internal Sharia Audit | "In each Islamic Bank or branch, an independent department, called the Internal Sharia Audit Department, is established" (art. 10). | A second review function whose findings the AML system can reference. |
| Standards | Where the law is silent, Islamic banks are subject to the CBI Law 56/2004, Banking Law 94/2004, the **Anti-Money Laundering Law No. 93 of 2004** "in force", Companies Law, "and the International Accounting Standards, Shariah and Accounting Standards issued by [AAOIFI]". | AAOIFI Sharia standards are binding by law. The law cites the 2004 AML order, replaced by Law 39 of 2015 on 16 Nov 2015 (file 01), so read the reference as the current AML law. |
| Tax | Exemption from "tax and fees resulting from the contracts of transactions relating to the purchase and sale of real estate, land and cars in the transactions of the sale of Murabaha, Ijarah ended in ownership and Musharaka". | Real estate and vehicle murabaha involve a double transfer (supplier to bank to customer) at no transfer tax; a data-model and scenario hook (section 8). |

### 7c. CBI Controls on Islamic financing instruments (circular of 1 August 2018; 68 pages)

Cover letter from the CBI Banking Supervision Department (Islamic Banks Supervision section, instructions and controls unit), dated 1 August 2018 (file 01 lists
the document as 2018-08-01; the signature block of the acting Governor bears 2 July 2018), tells all licensed Islamic banks to work to the attached controls "guided by the standards of AAOIFI". Chapters:
1 ijara and ijara muntahia bittamleek; 2 murabaha (murabaha to the purchase orderer, deferred sale); 3 mudaraba; 4 salam and parallel salam;
5 istisna' and parallel istisna'; 6 musharaka and diminishing musharaka (including shirkat al-wujuh, al-a'mal, shirkat al-mudaraba);
7 qard hasan; 8 **prohibited sales**; 9 **restricted sales**. This is the Iraqi "rulebook" a rule engine can check contract data against.
The rules that matter most for AML and for transaction-monitoring design (my reading of the Arabic; page numbers are the
document's own):

| Rule (document page) | What it says | Monitoring consequence |
|---|---|---|
| Prohibited sales: bay' al-'inah (p. 61) | Defined as "sales based on sham Islamic contracts ("عقود إسلامية صورية") whose purpose is to obtain a loan or liquidity containing interest". 'Inah: the bank sells a good to the customer at a deferred price then buys the same good back at a lower cash price (can also run between two banks or a bank and another institution). Reasons: it is a sale intended for loan with profit, and the two contracts are linked, "so the operation becomes sham". | Detect a sale and repurchase of the same asset or SKU between bank and customer, or sell-back within a short window, at a lower cash price. |
| Prohibited sales: organised tawarruq (p. 61) | "The bank buys a commodity or benefit for the customer, payment being deferred, and then the customer appoints the bank as agent to sell the commodity". If the sale is to the party from which it was bought it is the prohibited 'inah; if to an unrelated party, genuine tawarruq requires that the contracts are not linked. "If there is no commodity the operation involves usury and deception, in addition to unsound bookkeeping; and if the buyer of the last commodity is an external party, the operation is subject to the AML rules and legislation." The Sharia board must identify the sale types that are sham and meant to provide credit. | **Organised tawarruq is prohibited for Iraqi Islamic banks**, a line consistent with the OIC Fiqh Academy and, per a secondary summary, stricter than AAOIFI Standard 30 (text not read) (resolution 179, 19th session, Sharjah, 26-30 April 2009: organised and reverse tawarruq "is considered a deception in order to get the additional quick cash"). A tawarruq-like flow at an Iraqi Islamic bank is therefore both a compliance breach and an ML/TF red flag. |
| Restricted sales (p. 62) | Real tawarruq is allowed only in line with AAOIFI Shari'ah Standard No. 30. | Needs the commodity, the sequence of ownership and the final buyer on record. |
| Murabaha to the purchase orderer: no round trip (pp. 18-19, items 6-2) | Any earlier contractual link between the customer and the original seller "must be cancelled... and it is required that this cancellation be real and not formal"; the customer-seller contract may not be transferred to the bank; "The bank must ensure that the person who sells the goods to it is a third party other than the customer or his agent, and in any case where this is proven, the contract is considered void." The bank may refuse the seller the customer names. | Core ML check: **supplier equals customer, agent, or a related party** (shared owner, address, phone, account, signatory) means a round trip. |
| Murabaha: seriousness margin and promise (p. 18) | A "hamish jiddiyya" (seriousness margin) may be taken; a non-binding promise returns it in full; a binding one lets the bank take actual loss only. | A deposit trail that must reconcile with the deal. |
| Murabaha: no foreign-currency murabaha (p. 22, item 7) | Citing art. 5 item 10 of Law 43: "It is not permitted to conduct murabaha in foreign currencies (as if the bank lent the customer dollars with repayment in dinars)", nor "between Islamic banks or between Islamic banks and financial institutions". | Alert on a dollar-denominated financing settled in dinars (or the reverse): the pattern of obtaining dollars through "Islamic" finance. |
| Currency dealing conditions (pp. 22-25) | Exchange of currencies needs hand-over before the parties separate, equal like-for-like value, no option period, no hoarding intent, no forward trading; constructive possession by bank entry is recognised with a "tolerated period" during which the beneficiary may not use the currency until actual delivery is possible. | Same-day FX turnaround, forward-like patterns, and use of funds before the tolerated period ends. |
| Deferred sale accounting (pp. 25-27) | Deferred sale receivables, **deferred profits account**, provisions for doubtful debts; a customer who pays early may get a rebate; extra amounts collected for late payment go to a **charity fund (sunduq al-khayrat)**, not to income. | Early settlement of a long murabaha, unusual rebates, and charity-fund inflows are monitoring features (section 8). |
| Qard hasan (pp. 58-60) | Cash only, never in kind. Beneficiary classes the bank may define: students, patients, people hit by disasters, charitable projects owners, "essential social needs", university graduates for a start-up project. **Limits: total qard hasan to customers at most 10% of net profit after the investment account holders' share; one loan at most IQD 10 million and the recipient inside Iraq; no new loan to a customer with an unpaid qard hasan at this or another bank; a new loan only six months after full repayment of the previous one; no qard hasan to board members or senior staff**; bank-to-bank qard hasan at most 0.5% of the lender's capital and sound reserves. Current accounts treated as qard are exempt from the 10% and 0.5% caps. | Hard-coded rule pack: qard hasan above IQD 10m, recipient abroad, repeat within six months, insider recipients, aggregate over the cap. |
| Disclosure | AAOIFI FAS 1 for presentation; AAOIFI standards for each contract. | Reconcile the CBS contract tables with the financial statements categories. |

### 7d. USD restrictions: the official CBI lists and what they show

**Where we stand (CBI statement of 22 February 2026 and lists published with it; PDF dated 26 February 2026).** The CBI said it is
leading "a precise and strict reform process" of licensed banks and that listed banks "cannot participate in foreign currency windows
or deal in US dollars, whether directly or indirectly" until reform is complete; international audit firms may still audit them. Two
lists exist: **28 "USD Restricted Banks"** (with CBI codes BIQxxx/2) and **4 "Blacklisted Banks"**, plus a separate list of restricted companies (4 pages, not analysed).

| Restriction date | Banks (Islamic by name marked I) |
|---|---|
| 30 Oct 2019 | Iraq Noor Islamic (I) |
| 24 Dec 2020 | Babylon Bank (in liquidation) |
| 3 Nov 2022 | Al Qabidh Islamic (I); Asia Al Iraq Islamic (I); Al Anssari Islamic (I); Iraqi Middle East Investment Bank |
| 20 Jul 2023 (the 14 of the US Treasury and New York Fed action) | Al Mustashar Islamic (I); Al Qurtas Islamic (I); Al Taif Islamic (I, now under guardianship); Elaf Islamic (I); International Islamic (I); Rajih Islamic (I); Trust International Islamic (I); Al Mal Islamic (I); World Islamic (I); Zain Iraq Islamic (I, under guardianship); Erbil Bank; Iraq Trans Bank; Mosul Bank; Sumer Commercial Bank |
| 29 Jan 2024 (the "8 banks" of February 2024 press) | Kurdistan International Islamic (I); Al Janoob Islamic (I); Arabia Islamic (I); Investment Bank of Iraq; Union Bank of Iraq; Ashur International Bank; Al Huda Bank; Hammurabi Commercial Bank |
| Blacklisted (4) | Al Ataa Islamic Bank (formerly Al-Bilad), 16 May 2018 (I); Islamic Cooperation Bank for Investment, 8 Jan 2016 (I, in liquidation); Bank Melli Iran, 8 Jan 2016; Parsian Bank, 8 Jan 2016 (both Iranian branches under guardianship) |

Counts: **17 of the 28 restricted banks are Islamic by name (61%)**, 11 conventional; with the two blacklisted Islamic banks, 19 Islamic banks are
on one of the lists. The five banks restricted in **February 2025** (Al-Mashreq Al-Arabi Islamic, United Bank for Investment, Al-Sanam
Islamic, Misk Islamic, Amin Iraq Islamic; file 01 section 4) **do not appear** on the 22 February 2026 list and carry no "banned" flag in the
CBI register; whether they were cleared was not verified. The CBI web register also differs slightly from the PDF (it shows no flag
for Al Anssari and Trust International, which the PDF lists). The register shows 8 banks in custody, guardianship or liquidation
across both groups, 5 of them Islamic.

**Developments after the lists.**
- CBI statement 18 July 2026 (Arabic, cbi.iq news 3263): after the Governor's US visit and meetings with the US Treasury, a joint
  understanding "to return restricted Iraqi banks to external correspondent banking channels not linked to the US dollar", once they meet
  approved compliance and governance standards and complete the first phase of the CBI's banking reform and re-licensing programme;
  banks completing reform and re-licensing "will regain their eligibility to carry out US dollar transactions" later. The Governor said
  **seven banks** are currently qualified for the non-dollar correspondent channel. The CBI repeated that it will act against any bank
  below standard, "including restricting or suspending its access to international financial channels or revoking its licence".
- Press (The National, 22 Jul 2026; secondary, fetched summary only): the US Treasury's Office of Terrorism and Financial Intelligence
  said "no banned banks have regained access to foreign currency transactions", and described a two-phase path whose second phase
  needs a **third-party audit of the compliance function**, a qualified investor and international fit-and-proper standards. The same
  report says a restricted Islamic bank is linked to the current Prime Minister (reported as its former chairman); not verified by us and included
  only because it shows that bank owners can themselves be politically exposed persons.

**What authorities said about the reasons** (secondary sources: WSJ via Iraqi Economists Network, 26 Jul 2023; Reuters/AGBI Feb 2024; FinCEN final rule on Al-Huda, 3 Jul 2024, file 01):
money laundering and fraudulent transactions; "fraudulent import transactions" and trade-based laundering; dollars siphoned to Iran-aligned
groups; owners with links to those groups. The 14 banks of July 2023 held "only 1.29% of Iraq's total banking assets" (WSJ) but the CBI said they
were **8% of external transfers**: a size-to-dollar-flow mismatch that a monitoring system can compute for any bank.

### 7e. Why Islamic banks dominate the lists: what the evidence supports

1. **Mostly arithmetic.** Islamic banks are 31 of the 55 private local banks (56%). Of the private banks, **17 of 31 Islamic (55%) and 11 of 24 conventional (46%)
   are USD-restricted**. The Islamic share of the lists (61%) is close to the Islamic share of the private banks (56%). The restriction attaches to the
   **private, young, small-capital, dollar-window-dependent bank**, not to the Islamic contract (11 conventional private banks are on the same list).
2. **Vintage.** 15 of the 17 restricted Islamic banks were licensed in 2016-2018, the licensing wave after Law 43/2015, with IQD 250bn
   capital and short track records (the other two date from 2001 and 2005).
3. **Controls were set for a licence, not for dollar volume.** The 14 banks of July 2023 together held 1.29% of assets but were 8% of external
   transfers, so a small-capital bank could still move large amounts of dollars through the CBI window. The pattern is size-to-flow mismatch plus weak
   beneficial-owner transparency (the MER notes the CRD had no BO verification before 2023), not Sharia contract structure.
4. **Product mechanics can still help.** The CBI controls prohibit 'inah, organised tawarruq and currency murabaha. That the
   regulator wrote these prohibitions down in 2018 suggests it had seen Islamic contract forms used to obtain liquidity or foreign currency
   (inference: no CBI statement links them to the listed banks). An import-financing murabaha creates a documentary trail
   (invoice, shipping documents, supplier) that fits the "fraudulent import transactions" the Treasury cited.
5. **What is not supported:** no source found says the listed Islamic banks used Sharia contracts to launder; the IFSB-AMF survey (section 2b) found no
   difference in risk. The honest framing for bank leadership is: "Islamic" is not the risk factor; ownership, dollar-window behaviour, trade documents and
   governance are, and Islamic contracts add a few extra places to hide a round trip.

### 7f. CBI banking reform 2026-2028 (context for Islamic banks)

From the CBI deck of 18 May 2026: reform project run with Oliver Wyman and international audit firms (talks from Nov 2024; plan announced
Apr 2025; standards guide Aug 2025; banks signed a legal undertaking Sep 2025; digital-bank licensing rules Oct 2025; foreign-branch standards
Nov 2025; minimum capital, liquidity and ownership submissions Dec 2025; first-phase assessment Jun 2026). **25 detailed standards, 16 of them to be audited
by approved international firms**, in four groups: ownership and governance (ownership structure, **due diligence on owners**, board, fit and proper);
business-model sustainability (including a **comprehensive banking system**, digital channels, data infrastructure, credit registry); financial metrics
(capital, liquidity, stress tests); **risk and compliance (related parties, AML/CFT and sanctions, reporting transparency, internal controls)**. Private banks
are sorted into: reached minimum capital of **IQD 400bn**; institutional partner or serious talks; chose to continue; chose merger into new entities; in liquidation;
undecided. Implication: each Islamic bank will be audited on its AML/CFT and sanctions compliance, so AKR-TMS evidence packs (coverage, tuning records,
model documentation) are a purchase driver, and merging banks create **data-migration and multi-entity** needs.

### 7g. Waqf (endowments) in Iraq, a feature of the Iraqi customer base

From MER 2024 paras 685, 692, 699, 716-718: Iraq has no trusts; the only legal arrangements are **endowments** (Sunni, Shiite, Christian, Yazidi and Sabian-Mandaean).
**3,555 endowments at end-2022: 2,902 general (charitable), 553 private (family) and 100 joint.** Trustees must be of good conduct and are approved by a
scientific council; the trustee deposits endowment funds with the Endowments Department (a government agency); beneficiaries get annual
payments of about **USD 100-500**; trustees may not collect donations; each beneficiary of a private or joint endowment has a bank account supervised
by a Personal Status Court judge; the trustee can withdraw only with the Endowments Office's approval; endowment offices screen trustees and beneficiaries against
sanctions lists. The MER found endowments "unattractive for ML/TF purposes" and no abuse proven in 2018-2022. A waqf customer at a bank is therefore usually a
**government endowment office or a court-supervised trustee**, a low-volume, rule-bound flow, so a large or irregular flow through a waqf account is a strong anomaly.

### AKR-TMS hooks (section 7)

- **Contract rulebook as data**: a rule pack mapping the 2018 CBI controls to checks (supplier-equals-customer, 'inah pattern, organised-tawarruq pattern,
  FX murabaha, qard hasan caps, charity-fund inflows). Each rule carries its CBI document reference so a supervisor can see why it fired.
- **Bank-level exposure view**: for each client bank, ratio of external transfers to assets, dollar-window use, and a "size-to-flow mismatch"
  indicator; plus a **correspondent-readiness dashboard** that mirrors the CBI path (compliance-function audit, ownership due diligence, related parties, sanctions).
- **Ownership and related-party screening** for the bank's own shareholders, board, Sharia board members and senior staff (standards 1-4 and 22 of the CBI reform), with PEP
  flags, because owners of Iraqi banks can be PEPs.
- **Sharia board register**: members, CBI approval date, term end, independence check (not director, employee or shareholder), and board resolutions that
  AKR-TMS rules reference (for example an approved product list).
- **Multi-entity and merger support**: tenant model where two banks' data merge into a new legal entity with preserved history.
- **Evidence pack for the CBI-approved assessor and for correspondents**: coverage matrix of the CBI scenario catalogue (file 27) plus the Islamic extension (section 8).

---

## 8. Monitoring scenarios specific to Islamic products (IS-xx pack)

Design notes. (1) The CBI scenario catalogue as transcribed in file 27 has **no scenario naming murabaha, ijara, mudaraba, musharaka, qard hasan, zakat or waqf**; its LC, credit and general indicators
still apply and several map below. Banks may add scenarios "under the supervision of, and in coordination with, the AML/CFT Office" (file 27), so this pack is an extension to be agreed with the Office, not a replacement.
(2) "Phase 1" = needs only fields most CBS hold (contract type, parties, amounts, dates); "Phase 2" = needs asset, supplier-master, pool or graph data. (3) **Hard rules** come from CBI text and need no tuning; **statistical rules** need a baseline of 6 to 12 months of the bank's own deals.

| ID | Scenario | Logic | Data | Basis | Phase |
|---|---|---|---|---|---|
| **Murabaha** | | | | | |
| IS-M1 | Supplier is the customer, his agent or a related party | Supplier shares owner, signatory, address, phone, email, bank account or device with the customer | supplier master, party links | CBI 2018 item 6-2-3 (contract void); CBI catalogue Art. 4 no. 7 (LC beneficiary owned by customer) | 1 |
| IS-M2 | Round trip of supplier payment | Money paid to the supplier returns to the customer or a related party within N days | supplier payment, downstream transfers | A; MER NRA fake-invoice pattern (file 02) | 2 |
| IS-M3 | Price outlier | Unit cost or margin for the same SKU or commodity differs from peer deals, customs value or a benchmark by more than a tuned band | asset lines, benchmark prices, customs data | A; FATF/Wolfsberg trade principles (file 04 W-14) | 2 |
| IS-M4 | Import murabaha payment geography | Payment to a country not matching the goods' origin, to a third-country account, or in a currency unlike the contract | supplier payment, country, goods origin | A; Treasury "fraudulent import transactions" (section 7d) | 1 |
| IS-M5 | Supplier profile risk | Supplier younger than 12 months, shared address or phone with other suppliers or customers, no customs history, one supplier serving many unrelated customers, high-risk country | supplier master, registry extract, graph | A; MER on CRD data (7d) | 2 |
| IS-M6 | No possession evidence | Deal older than N days without invoice, delivery or title; same invoice number on several deals; payment released before evidence | evidence status, document numbers | CBI 2018 (bank must own before selling); CBI catalogue Art. 13 no. 11 (fictitious contracts) | 1 |
| IS-M7 | Foreign-currency murabaha | Contract currency differs from settlement currency (for example USD financing repaid in IQD) | contract ccy, settlement ccy | CBI 2018 p. 22 item 7 (prohibited) | 1 |
| IS-M8 | Early or third-party settlement | Early settlement, large partial payment or rebate funded by an unrelated person, cash or a foreign account | settlement payer, rebate events | CBI catalogue Art. 6 nos. 4 and 8; A | 1 |
| IS-M9 | Serial rescheduling | Repeated renewal or rescheduling to avoid default or recycle limits | schedule changes | A | 2 |
| IS-M10 | Real-estate or vehicle churn | Asset resold or refinanced within two years, or same asset financed twice, or value far from registry or dealer price | asset id, registry data, valuation | CBI 2018 item 16 (two-year rule); Law 43 tax exemption (7b) | 2 |
| IS-M11 | Profile mismatch | Goods or amount do not fit the customer's declared business, occupation or income (for example a student financing machinery) | KYC profile, deal goods | CBI catalogue Art. 12 no. 10 | 1 |
| **Tawarruq and 'inah** | | | | | |
| IS-T1 | 'Inah pattern | Bank sells an asset to the customer and buys the same asset or SKU back (or sells it to a bank-related party) within a window at a lower cash price | asset id on both legs | CBI 2018 (prohibited) | 1 |
| IS-T2 | Organised tawarruq | Agency clause making the bank the customer's seller, or sale proceeds credited the same day as the purchase | contract clauses, timestamps | CBI 2018 (prohibited); IIFA resolution 179 | 1 |
| IS-T3 | Circular or related commodity chain | Legs among related parties, same broker both sides, A to B to C to A | commodity legs, party links | IMF para 11; A | 2 |
| IS-T4 | Liquidity cycling | Repeated tawarruq or commodity murabaha cycles by one customer, round amounts, proceeds wired abroad within days | commodity legs, outgoing transfers | IMF para 11 ("frequent or large recycling") | 1 |
| **Ijara** | | | | | |
| IS-I1 | Inflated sale-and-leaseback | Sale price to the bank above valuation or market; seller related to a customer | valuation, price, links | CBI 2018 pp. 5-6; A | 2 |
| IS-I2 | Third-party buy-out | Early buy-out of a lease funded by a third party or cash | buy-out payer | A | 1 |
| IS-I3 | Rent paid by someone else | Rentals paid in cash, by a non-lessee or from abroad | rent payer | A | 1 |
| IS-I4 | Asset encumbrance | Leased asset registered to a related party or financed twice | asset id, registry | A | 2 |
| IS-I5 | Early real-estate sale | Property bought for investment sold before two years after break-even without default | break-even date, sale date | CBI 2018 item 16 | 1 |
| **Mudaraba and PSIA** | | | | | |
| IS-P1 | Profit allocation outlier | An account's credited profit differs from pool rate times weightage beyond tolerance | pool, weightage, credited profit | A (insider and ML risk) | 2 |
| IS-P2 | Profit harvesting | Deposit shortly before the profit run and withdrawal just after, repeatedly or for large amounts | deposit and withdrawal dates, run dates | A | 1 |
| IS-P3 | Third-party or cash funding of investment accounts | Funding source unrelated to the account holder | source of funds | A; IFSB-AMF 4.4 | 1 |
| IS-P4 | Fixed "profit" | Constant payout to a mudaraba account (a loan in disguise) | profit credits | CBI 2018 mudaraba rules (ratio only) | 1 |
| IS-P5 | Value transfer by early withdrawal | Early termination of investment accounts right after a related financing, or closing with forfeiture to move value | closure events, linked financing | A | 2 |
| **Musharaka** | | | | | |
| IS-S1 | Capital source mismatch | Capital contributed in cash, from abroad or from a third party, not matching declared source | contribution record | IMF para 13; A | 2 |
| IS-S2 | Accelerated buy-out | Diminishing musharaka bought out early by third-party money | buy-out payer | A | 2 |
| IS-S3 | Related partners | Bank-financed partner and counterparties share beneficial owners | BO data, graph | A | 2 |
| **Qard hasan** (hard rules) | | | | | |
| IS-Q1 | Over the per-loan cap | Single loan above IQD 10 million | loan amount | CBI 2018 p. 59 | 1 |
| IS-Q2 | Recipient abroad | Recipient not inside Iraq | residence | CBI 2018 p. 59 | 1 |
| IS-Q3 | Repeat loan | New loan within six months of full repayment, or while another qard hasan is unpaid at this or another bank | loan history | CBI 2018 pp. 58-59 | 1 |
| IS-Q4 | Insider recipient | Recipient is a board member, senior staff or their relative | staff and board register | CBI 2018 p. 59 | 1 |
| IS-Q5 | Aggregate cap | Total qard hasan above 10% of net profit after the investment account holders' share (bank-level) | bank profit data | CBI 2018 p. 59 | 1 |
| IS-Q6 | Class mismatch | Declared beneficiary class (student, patient) not matching account activity | KYC, activity | CBI 2018 p. 59; A | 2 |
| IS-Q7 | Third-party cash repayments | Repayments in cash by unrelated persons | payer | A | 1 |
| **Salam and istisna'** | | | | | |
| IS-Z1 | Prepayment without delivery | Salam delivery overdue with no evidence | delivery records | A | 2 |
| IS-Z2 | Milestone payments to related contractor | Payments exceeding progress certificates, or to a contractor linked to the customer | milestones, certificates, links | MER para 695; A | 2 |
| IS-Z3 | Price or variation outlier | Unit price or repeated variation orders far from peers | prices, change orders | A | 2 |
| IS-Z4 | Circular parallel contracts | Parallel-contract counterparties form a loop | party graph | A | 2 |
| **Charity, zakat, waqf** | | | | | |
| IS-C1 | Donation collection outside registered NPOs | Many donors paying into a personal or non-registered account | account type, registry status | A; FATF R.8 | 1 |
| IS-C2 | NPO pass-through | Inflows equal outflows within days; outflows to individuals | NPO account flows | CBI catalogue Art. 13 no. 4; A | 1 |
| IS-C3 | Transfers to NPOs abroad or unregistered | Destination is unregistered, listed or in a conflict area | beneficiary, country | UN list examples (4c); A | 1 |
| IS-C4 | Waqf flow versus plan | Waqf account activity outside the deed's spending plan or Endowments Office approval | spending plan, flows | IFSB-AMF section 2.4; MER 7g | 1 |
| IS-C5 | Charity-fund concentration | Bank charity-fund disbursements concentrated on one beneficiary or linked to staff, board or Sharia board | disbursement register | A | 1 |
| IS-C6 | Seasonal excess | Spike above the Hijri-calendar seasonal norm | baselines by season | A | 2 |
| IS-C7 | Narrative mismatch | "Zakat", "sadaqa" or "waqf" on transfers to high-risk destinations | narrative text, country | A | 1 |
| IS-C8 | Alias-aware charity screening | UN and IQTFS aliases and scope notes applied to NPO counterparties | watchlists | UN list (4c) | 1 |
| **Sukuk and funds** | | | | | |
| IS-K1 | Off-market related trades | Secondary trades between related accounts at off-market prices | trade data | IFSB-AMF 2.3.2; A | 2 |
| IS-K2 | Holder concentration or nominees | Few holders or nominee accounts in one issue | holder register | IFSB-19 disclosure idea; A | 2 |
| IS-K3 | Redemption to third parties | Redemption proceeds paid to someone other than the holder | payee | A | 1 |
| **Cross-cutting** | | | | | |
| IS-X1 | Sharia non-compliance density | Findings concentrated by officer, branch or product | Sharia findings | IFSB glossary; A | 2 |
| IS-X2 | No valid Sharia approval | Deal uses a product code with no current approval, or contract changed after approval | approval register, amendment log | Law 43 art. 8; A | 1 |
| IS-X3 | Supplier linked to staff | Supplier or contractor shares data with an employee (KYE) | HR link, supplier master | CBI catalogue Art. 14 | 2 |
| IS-X4 | Inter-book transfer | Movement between conventional and Islamic books | book flag | IFSB glossary ("separate funds") | 1 |
| IS-X5 | Bank-level size-to-flow mismatch | External transfers or dollar purchases large relative to assets versus peers | bank aggregates | 14 banks: 1.29% of assets, 8% of external transfers (7d) | 1 |
| IS-X6 | Dormant reactivation | Dormant Islamic accounts reactivated | account status | CBI dormant-account rule (file 01) | 1 |
| IS-X7 | Contract data completeness | Share of deals missing supplier, asset or evidence fields | data-quality counters | A | 1 |

### AKR-TMS hooks (section 8)

- Ship the IS pack as an **optional extension** to the CBI baseline, with each scenario showing its basis (CBI page, UN entry or "analysis") and a **data-readiness** tag; scenarios whose data the bank cannot supply are shown as "not covered", not silently skipped.
- Arabic alert narratives that use the contract's own terms (murabaha, ijara, mudaraba) so branch liaison officers and the Sharia audit team read them naturally.
- **Back-test before go-live** on the bank's own history, and agree the extension with the AML/CFT Office before treating it as the bank's own scenario set.

---

## 9. What it means for AKR-TMS

1. **One platform, an Islamic extension pack.** The base library, risk rating and case management apply unchanged (96% of supervisors apply identical rules). The extension adds: deal and party model, hard-rule pack from CBI 2018, charity and waqf module, Sharia governance register, Hijri-aware baselines, and the IS scenario pack.
2. **Read where the money goes.** In sales- and lease-based contracts value moves to suppliers and sellers. The read-only link must therefore bring in disbursement, supplier-payment and asset tables, not only customer accounts. Where a bank's CBS cannot supply them, the product should say which scenarios are blind.
3. **Encode the CBI's own Islamic rulebook.** The 2018 controls give checkable rules (supplier not the customer, no FX murabaha, no organised tawarruq, no 'inah, two-year real-estate rule, qard hasan caps). These are the cheapest, most defensible alerts because each has a CBI page reference.
4. **Give leaders the bank-level mirror.** Seventeen of 28 restricted banks are Islamic by name, 15 of them licensed 2016-2018. A "correspondent readiness" and "size-to-flow" view lets a bank see itself as US Treasury and its correspondents see it, and tracks the CBI's reform path (compliance-function audit, owner due diligence, related parties).
5. **Make charity flows visible and boring.** Registered-NPO checks, waqf profiles, a disbursement register and seasonal baselines will keep zakat flows from either hiding abuse or drowning analysts in Ramadan alerts.
6. **Treat the Sharia board as a data source and a stakeholder, not an AML owner.** Approvals and audit findings feed rules and risk; the board does not see SAR files.
7. **Produce the first Islamic-versus-conventional evidence.** Alert, STR and typology-coverage rates per bank that has both books fill a gap the IMF and IFSB papers say exists. That is a credibility asset with the CBI and with IFSB-type audiences, and a reason for the CBI to endorse the extension.
8. **Say what is not known.** The claim "Islamic banks are safer" and the claim "Islamic banks are riskier" are both unsupported by transaction data. The plan should show the product as the tool that finds out.

### Gaps and items not verified (2026-10-06)

- No 2024-2026 market-share figure for Iraqi Islamic banks (Fitch 2021 is the latest found); IFSB primary Stability Report 2026 page not fetched (figures via two secondary reports).
- The five banks restricted in February 2025 are absent from the 22 February 2026 list; whether they were cleared was not verified.
- The claim in The National (22 Jul 2026) that a restricted Islamic bank is linked to the current Prime Minister was seen only in a fetch summary and through search snippets; treat as unverified.
- No Iraqi regulation of Islamic windows in conventional banks was found; the CBI risk-management controls for Islamic banks (13 Jul 2026, 104 pages) were opened and found to be a prudential guide (governance, credit, market, liquidity, operational, funding and environmental and social risk, stress tests); only the contents and introduction were read.
- AAOIFI Sharia standards (for example SS 8 murabaha, 9 ijara, 12 sharikah, 13 mudaraba, 17 sukuk, 30 tawarruq, 33 waqf, 35 zakah) were not read directly; definitions come from the IFSB-AMF glossary and the CBI 2018 text. The FATF site blocks automated access.
- No documented enforcement case of laundering through an Islamic contract was found (section 2c lists the counterparty, sanctions and Sharia-compliance cases that do exist); the web-search budget for this session ran out (200 of 200) before this was pursued further, so a targeted search for such cases (for example in the Gulf, Pakistan, Malaysia, Turkey, Sudan) remains open.
- Several CBI documents are scans; Arabic readings of the 2018 controls are mine and should be checked by an Arabic-reading compliance officer before they are quoted in the plan.

---

## 10. Derived requirements (I-xx)

Phase 1 = read-only CBS link, post-event; phase 2 = optional pre-release screening and scoring API or richer data.

| ID | Requirement | Source |
|---|---|---|
| I-01 | Map each bank's Islamic product codes to a canonical taxonomy (murabaha, commodity murabaha and tawarruq, ijara and ijara muntahia bittamleek, mudaraba and PSIA, musharaka and diminishing musharaka, wakala, salam, istisna', qard hasan, sukuk, takaful, sarf) with a risk class and rule pack per class | IMF 2016 para 11; IFSB-AMF 2.3; CBI 2018 contents |
| I-02 | Deal-centric canonical model: deal, parties by role, asset or commodity, parallel contracts, schedule, evidence status, Sharia approval, plus disbursement and supplier-payment records linked to the deal | IFSB-AMF 2.2; CBI 2018 |
| I-03 | Extend CDD, screening and graph analysis to suppliers, sellers, brokers, agents, developers, contractors, partners and waqf parties, not only the customer | IFSB-AMF 2.2 ("end-to-end parties"); IMF para 13 |
| I-04 | Hard rule: seller or supplier must be a third party other than the customer or his agent; detect shared owner, address, phone, email, signatory, account or device | CBI 2018 item 6-2-3 |
| I-05 | Possession-evidence tracker (invoice, delivery, title, registration) with ageing; alert on missing, late or duplicated evidence | CBI 2018 murabaha; IFSB-AMF 2.2 |
| I-06 | Hard-rule pack for prohibited structures: bay' al-'inah, organised tawarruq, foreign-currency murabaha (contract versus settlement currency) | CBI 2018 pp. 22, 61; IIFA resolution 179 |
| I-07 | Real-estate rule: no sale before two years after break-even except in default; flip and double-financing detection | CBI 2018 item 16 |
| I-08 | Qard hasan rule pack: IQD 10m cap, recipient in Iraq, six-month gap, no unpaid prior loan, no insiders, 10% of net profit aggregate cap | CBI 2018 pp. 58-59 |
| I-09 | Mudaraba and PSIA integrity checks: profit as ratio only, capital rules, profit-allocation reconciliation to pool and weightage, profit-harvesting pattern | CBI 2018; IFSB-AMF 4.4; analysis |
| I-10 | Freeze workflow aware of account type (restricted, unrestricted, pooled funds), involving Finance and the Sharia board for pooled funds | IFSB-AMF 4.4 |
| I-11 | Price and margin outlier engine for murabaha, istisna' and ijara asset lines against peer deals, customs values and benchmarks | analysis; Wolfsberg trade principles (file 04 W-14) |
| I-12 | Charity module: NPO, waqf and religious-institution customer types with trustee and beneficiaries as related parties, registry status, foreign-funding flag and expected spending plan | IFSB-AMF 2.4; MER paras 685-718; FATF R.8 (file 03) |
| I-13 | Register of the bank's own charity disbursements (zakat, non-compliant income, late-payment surcharges) with Sharia-board approval, screening and concentration alerts | IFSB-AMF 2.7; CBI 2018 p. 26; IMF para 11 |
| I-14 | Hijri-calendar and religious-season baselines (Ramadan, Eid, Hajj, Muharram, Arbaeen); Hijri date support | analysis; file 02 (religious tourism) |
| I-15 | Sharia governance register (board members with CBI approval and term, product approvals, audit findings, non-compliance events); rules check approval status | Law 43 arts. 7-10; CBI 2018 |
| I-16 | Role model: Sharia board sees aggregated product dashboards only; no access to SAR or case files | Law 39/2015 art. 12 (file 01); F-16 |
| I-17 | Book flag (conventional, Islamic window, Islamic bank), consolidated customer view, inter-book transfer alert, per-product-line onboarding block | IFSB glossary; IFSB-AMF 4.5; CBUAE release 25 Jun 2025; B-03 |
| I-18 | Bank-level exposure analytics: external transfers and dollar purchases relative to assets and peers; correspondent-readiness dashboard mirroring the CBI reform path | Section 7d, 7f; CBI 18 Jul 2026 |
| I-19 | Ownership and related-party screening of the bank's shareholders, board, Sharia board and senior staff with PEP flags | CBI reform standards 1-4, 22 (May 2026 deck); MER |
| I-20 | IS scenario pack (section 8) as an optional extension with basis label, data-readiness tag, back-testing, and agreement with the AML/CFT Office | File 27; section 8 |
| I-21 | Evidence pack for the CBI-approved assessor and correspondents covering the Islamic extension: coverage matrix, tuning log, Sharia-rule documentation | CBI standards handbook 2025 (file 01); CBI reform |
| I-22 | Multi-entity and merger support: merge two banks' data into a new entity with preserved history and audit trail | CBI reform plan (IQD 400bn, mergers) |
| I-23 | Islamic-versus-conventional effectiveness metrics per bank with both books (alert rate, STR rate, typology coverage) | IFSB-AMF 4.5; IMF para 10 |
| I-24 | Mapping coverage report for Islamic fields; supplier master with entity resolution on Arabic variants and commercial registration numbers | Section 6; file 22 |
| I-25 | Arabic alert narratives using contract terms; Arabic text matching for contract and supplier descriptions | Sections 6, 8; file 22 |
| I-26 | Data-quality counters for Islamic contract fields (missing supplier, asset, evidence); a high rate is shown as a control failure | IS-X7 |
| I-27 | Alias- and scope-aware screening of NPOs, charities and exchange houses against UN, IQTFS and OFAC, reading UN scope notes (for example only some offices designated) | UN list (section 4c) |
| I-28 | Customer risk rating inputs for Islamic specifics (product class, supplier-related flags, NPO or waqf status, PSIA funding source) | File 26 (K-xx) |
| I-29 | Respondent self-view for Islamic banks seeking or keeping correspondents, including the metrics the CBI reform and US Treasury path use | IFSB-AMF 2.3.3 (CIBAFI: correspondent closures fifth-biggest risk); W-22 |
| I-30 | Label every Islamic-specific rule with its basis (CBI page, UN entry, analysis) and never present Islamic contracts as inherently higher risk | IMF 2016; IFSB-AMF 2019 |

---

## 11. Sources

Primary and official
- [IMF Working Paper WP/16/42, Islamic Finance and AML/CFT (Feb 2016): abstract page](https://www.imf.org/en/publications/wp/issues/2016/12/31/islamic-finance-and-anti-money-laundering-and-combating-the-financing-of-terrorism-aml-cft-43754)
- [IMF WP/16/42 full text (PDF)](https://www.imf.org/-/media/websites/imf/imported-full-text-pdf/external/pubs/ft/wp/2016/_wp1642.pdf)
- [Joint IFSB-AMF Working Paper, ML/FT Risks in Islamic Banking (Dec 2019)](https://backup.amf.org.ae/sites/default/files/publications/2022-02/risks-in-islamic-banking-laundering-and-financing-of-terrorism-joint-ifsb%E2%80%93amf-working-paper-on-money-mlft.pdf)
- [CBI, Islamic Banks Law No. 43 of 2015, English translation](https://cbi.iq/static/uploads/up/file-156759065639534.pdf)
- [CBI, Controls on Islamic financing instruments, Aug 2018 (68 pages, Arabic scan)](https://cbi.iq/static/uploads/up/file-153372094957997.pdf)
- [CBI, Supervisory guide and risk-management controls for Islamic banks, Jul 2026 (104 pages)](https://cbi.iq/static/uploads/up/file-178480220727035.pdf)
- [CBI, register of banks (index)](https://cbi.iq/page/93); [state banks](https://cbi.iq/page/23); [private local banks, conventional and Islamic](https://cbi.iq/page/24); [foreign bank branches](https://cbi.iq/page/114)
- [CBI statement of 22 Feb 2026 on restricted banks](https://cbi.iq/news/view/3135); [USD Restricted Banks and Blacklisted Banks list (PDF, 26 Feb 2026)](https://cbi.iq/static/uploads/up/file-177208376739661.pdf); [restricted companies list (PDF)](http://cbi.iq/static/uploads/up/file-177173877392091.pdf)
- [CBI statement of 18 Jul 2026 on the US Treasury understanding (Arabic)](https://cbi.iq/news/view/3263)
- [CBI, Strategic Achievements 2020-2025, Governor's Office, 18 May 2026 (63 slides)](https://cbi.iq/static/uploads/up/file-177935854666319.pdf)
- [MENAFATF Mutual Evaluation Report of Iraq, May 2024](https://menafatf.org/sites/default/files/MER_Iraq_English.pdf) and first-round MER of 28 Nov 2012 (local copy iraq-mer-en)
- [UN Security Council Consolidated List (XML), parsed 2026-10-06](https://scsanctions.un.org/resources/xml/en/consolidated.xml)
- [CBUAE press release, 25 Jun 2025, Islamic window sanction (Arabic)](https://www.centralbank.ae/media/yhqlqjhw/cbuae-suspends-the-onboarding-of-new-customers-in-the-islamic-window-of-a-bank-ar.pdf)
- [IFSB press release, July 2025, IFSB-31 adopted](https://www.ifsb.org/press-releases/ifsb-council-adopts-new-standard-and-approves-the-issuance-of-a-guidance-note/)
- [OIC International Islamic Fiqh Academy, resolution 179 (19/5) on tawarruq, Sharjah April 2009](https://iifa-aifi.org/en/32987.html)
- [AAOIFI standards pages (accounting, auditing, governance)](https://aaoifi.com/standard/accounting-standards/)
- [Temenos for Islamic Banking (vendor page)](https://www.temenos.com/products/islamic-banking/)

Secondary (marked as such in the text)
- [Fitch via Pakistan Observer, Iraq's Islamic banking sector assets likely to continue growing (8 Jul 2022)](https://pakobserver.net/iraqs-islamic-banking-sector-assets-likely-to-continue-growing-fitch/)
- [Shafaq News, CBI: over 30 Islamic banks (20 Feb 2024)](https://shafaq.com/en/Economy/CBI-Over-30-Islamic-banks-grow-quickly-in-Iraq-s-banking-sector)
- [Daily Trust, IFSB 2026 Stability Report (7 Jul 2026)](https://dailytrust.com/?p=1925665); [Islamic Economics Project, key highlights of the Islamic Finance Stability Report 2026 (8 Jun 2026)](https://islamiceconomicsproject.com/2026/06/08/key-highlights-of-the-islamic-finance-stability-report-2026/)
- [Iraqi Economists Network, US bans 14 Iraqi banks (WSJ summary, 26 Jul 2023)](https://iraqieconomists.net/en/2023/07/26/u-s-bans-14-iraqi-banks-in-crackdown-on-iran-dollar-trade-treasury-department-federal-reserve-bank-of-new-york-bar-the-lenders-from-conducting-dollar-transactions)
- [The National, US Treasury signals years-long plan to rebuild Iraq banking sector (22 Jul 2026)](https://www.thenationalnews.com/business/banking/2026/07/22/us-treasury-signals-years-long-plan-to-rebuild-iraq-banking-sector/)
- Cross-references inside this research set: file 01 (CBI controls, Instructions 1/2023, dollar restrictions), file 02 (Iraq risk landscape), file 03 (FATF), file 04 (Basel, Wolfsberg, UN list), file 17 section 3 and file 18 section 5 (Islamic CBS vendors and banks, being written), file 22 (Arabic names), file 27 (CBI scenarios).
