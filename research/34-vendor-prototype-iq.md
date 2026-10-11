# 34 — Vendors: Prototype for Information Technology (Prototype Tech, Baghdad) and the "prototype member check" question

Status: done (2026-10-11); checked and corrected the same day against research/_membercheck/90-verification.md

A profile of one Iraqi company, written to answer the owner's question about "something in the lines of prototype member check". Sister file: 35 (MemberCheck and NameScan, the Australian screening brands that the owner's phrase may point to). This file adds no requirement IDs.
Cross-references: file 14 (screening engines, and the twelve needs of an Iraqi bank in its section 1; file 35 rates eight of them), file 15 (risk data providers), file 18 (MENA and Iraq market, vendors and integrators), file 22 (Arabic name matching), file 23 (goAML reporting), file 35 (MemberCheck and NameScan).

How to read this file. Source keys in square brackets, such as [PIQ-SITE], are defined once in section 13. Factual sentences and table rows end with one or more keys. Three marks matter:
- "The company says" or "the site claims" means Prototype's own words. Nobody outside the company has checked them.
- [INFERENCE] means our own reading of the evidence. It is not a stated fact.
- [N12] and similar keys point to a working note in research/_membercheck/. They are used for "not found" results, and the note names what was searched.

Terms used in this file (written for a reader who is not a compliance specialist):
- **AML, CFT, KYC:** anti-money-laundering and counter-financing-of-terrorism rules, and the "know your customer" checks a bank runs on its customers.
- **Sanctions list:** an official list of people, firms or countries that the law forbids dealing with, such as the United Nations list or the list of the US Treasury office OFAC. **PEP:** politically exposed person, a senior public official (or a relative or close associate of one) who carries a higher corruption risk. **Watchlist:** a loose word for any such list.
- **Screening:** comparing a customer's name with those lists. A **hit** is a possible match. A **false positive** is a hit that turns out to be a different person. An **alias** (also written AKA, "also known as") is another name or spelling for the same person.
- **Transliteration, romanization:** writing a name from one alphabet in another, for example Arabic in Latin letters. One Arabic name can come out in several Latin spellings, which is why screening Arabic names is hard.
- **goAML:** the filing system through which a bank sends suspicious-transaction reports to the country's financial intelligence unit (**FIU**); see file 23.
- **Reseller and white label:** a reseller sells the maker's product under the maker's name. A white-label partner sells it under its own name, so the buyer may never see the maker's name. These are the Australian vendor's own definitions (file 35, section 5).
- **API:** a way for one computer system to call another automatically. **CBS:** a bank's core banking system, its main ledger. **On-premises:** running on the bank's own servers.
- **FATCA:** a US tax law under which foreign banks report accounts held by US taxpayers to the US tax authority (the IRS). **PCI DSS, ISO 27001, SWIFT CSP, SOC 2:** security and audit standards (card data, information-security management, the Swift customer security programme, and an audit report on a service provider). A **QSA** is a firm or person that the card industry approves to assess against PCI DSS.

## Contents

1. How the name was resolved
2. Who they are
3. What they sell
4. How they say it works
5. Clients claimed and independent evidence
6. Evidence quality
7. What it appears to do right
8. What it does wrong or leaves unproven
9. Overlap with AKR-TMS
10. What it means for AKR-TMS
11. How to settle the open points
12. Open questions and things not found
13. Sources

## Findings in brief

1. **Who, what, how, and how sure.** Prototype for Information Technology (brand "Prototype Tech", site title "Prototype IQ") says it is an Iraqi fintech and cybersecurity company in Baghdad; its web domain dates from 2019 and LinkedIn reports 2 to 10 employees. [PIQ-SITE] [RDAP] [LI-PIQ] It says it sells sanctions-screening, anti-money-laundering, regulatory-reporting, identity, fraud, SWIFT and security-consulting products, and its home page lists 31 client entries (28 distinct banks, one of them listed twice, and 2 insurers). [PIQ-SITE] It describes how it works only in one-line marketing text and brochure drawings: AI name translation and alias generation, OFAC as the only named list, and a partner list that includes Namescan. [PIQ-SITE] [PIQ-BROCHURE] [N11] Confidence: likely (medium to high) that this is the company the owner meant, on its name, the Iraqi origin it claims, the sanctions-screening products it claims and the Namescan partner entry; the words "member check" and any person's name are not matched on its own pages. Low on what its products actually do. [PIQ-SITE] [N08] [N12]
2. **What is not proven.** That any Iraqi bank uses a Prototype AML or sanctions system; that its screening comes from NameScan, MemberCheck or its own engine; that anything it sells is called "member check"; who founded or runs it, and whether the Mohammed of one MemberCheck review is connected to it (the review ties a Mohammed to MemberCheck distribution; no source ties him to Prototype or to Iraq); and its Iraqi legal registration. The words "Member Check" appear nowhere next to Prototype. [N12] [N10] [N08] [GA-ARCH]
3. **How the name was resolved.** Of ten candidates, Prototype is the only Iraq-linked company with that name, it sells the right kind of product, the Arabic title of its Facebook page means roughly "the first model", and its partner list names Namescan, a sister brand of MemberCheck. Likely (medium to high): the company. Possible to likely (medium; an inference): "member check" means MemberCheck or NameScan screening that it resells or builds on, because one MemberCheck review shows an IT-services firm led by a Mohammed distributing its products, and no source names Prototype as that firm. One candidate, not confirmed (low to medium): the Mohammed is that reviewer; no source ties him to Prototype or to Iraq. (Section 1.) [N07] [N08] [PIQ-SITE] [FB-PIQ] [GA-ARCH] [INFERENCE]
4. **Independent evidence is absent.** In 15 searches and the documents read, no bank, auditor, regulator, news item or event names Prototype as a supplier. None of the four banks whose annual reports name their AML suppliers is on its list, and about ten of its 28 banks (some by name match only) also appear on Pio-Tech's references page. (Section 5.) [N12] [F18]
5. **The counter overstates and the badges lack papers.** "31+ Banking Clients" counts list entries (one bank twice, two insurers). Only seven entries carry a product label. "PCI DSS Certified QSA", "IRS-approved" and "100% Compliance Rate" come with no document, and the site shows both "5+" and "15+" years. (Section 6.) [PIQ-SITE] [N11]
6. **The engine question is open, and NameScan has a visible route.** Namescan is a listed partner and some card wording matches, but no page says Prototype uses it. NameScan's partnership page offers referral, distributor and integrator routes and names eight partners, none Iraqi and none Prototype. If Prototype's screening were NameScan-based, customer names would leave the bank's servers for a hosted service with a daily refresh and no Iraqi list shown [INFERENCE]. (Section 4.) [PIQ-SITE] [NS-PARTNERSHIP] [F35]
7. **Arabic and the Iraqi name chain are framed, not shown.** The company leads with name translation, alias generation and passport-name romanization, but publishes no method, accuracy or languages. The only specifics are drawings, and nothing mentions mother's name, tribe or birth year. (Sections 4.1 and 8.) [PIQ-SITE] [PIQ-BROCHURE] [N11]
8. **Closest overlap with AKR-TMS is screening and goAML, and every Prototype product is grade C (section 6).** AKR-TMS is designed to differ by running on the bank's servers, reading the database read-only, loading the Iraqi list, matching the Iraqi name chain and checking goAML files. It is exposed to "AI" wording, bundling with services the bank already buys, existing bank relationships and eKYC, which is outside its plan. (Sections 9 and 10.) [MODULES] [N11] [F35]
9. **The fastest way to settle it is from inside a client bank.** Ask for the name, logo and web address on the screening login page, the supplier's name, how often lists update and whether the system runs on a bank server. Also search the Iraqi company register (not yet done), and decide whether to ask NameScan if Prototype is a customer or distributor. (Section 11.) [N10]
10. **Everything here comes from public pages, registers, one real-browser read and four web searches by the lead.** Nothing was tested. No demo was requested, no account made and no one contacted. One page was fetched by the writer (NameScan's partnership page). The searches (section 1.3a) looked for the MemberCheck reviewer and found nothing that ties him to Prototype. Company claims are marked as claims, and no requirement IDs were added. (Section 13.) [PIQ-SITE] [NS-PARTNERSHIP] [LEAD-SEARCH]

---

## 1. How the name was resolved

### 1.1 The owner's three clues

The owner described the system in three loose pieces and said twice that the memory was uncertain (the owner's words; typing slips corrected). [OWNER]

1. **The name:** "something in the lines of prototype member check".
2. **The maker:** "its by some guy named mohammed".
3. **The origin:** "its a local iraqi system (i think im not really sure)".

Twelve working notes were written on 2026-10-11 (research/_membercheck/, notes 01 to 12). Notes 01 to 05 profile MemberCheck and NameScan and feed file 35. Notes 06 to 12 try to work out what the owner meant. They ask: is MemberCheck used in Iraq [N06]; what else could the words be [N07]; is there a company called Prototype in Iraq [N08]; is there any Iraqi-built screening system [N09]; who are Prototype's people [N10]; what does Prototype sell [N11]; and does anyone outside the company confirm it [N12]. The lead also opened Prototype's website in a real browser on 2026-10-11. The site fills in its content after the page loads, so the downloaded code alone does not show it. [PIQ-SITE] [N08]

### 1.2 Candidates considered

"Fits which clue" uses the numbers above. The verdict is ours.

| Candidate | Fits which clue | Evidence | Verdict |
|---|---|---|---|
| **Prototype for Information Technology** ("Prototype Tech", "Prototype IQ"), Baghdad | Clue 1 in part (the word "prototype"); clue 3 (it says it is Iraqi and based in Baghdad). Clue 2 is not shown on its pages: no person is named there | Its own site and brochure and its LinkedIn page. Nobody outside the company confirms its work [PIQ-SITE] [PIQ-BROCHURE] [LI-PIQ] [N08] [N10] [N12] | Most likely the company. "Member check" and any person's name are not shown on its pages. No source ties the reviewer of section 1.3a to Prototype [N08] [N10] [GA-ARCH] |
| **MemberCheck** (MEMBER CHECK PTY LIMITED, Chatswood, Australia) | Clue 1 in part (the words "member check"). Not clue 3 (Australian company). Clue 2 only in one respect: a GetApp review (2024-10-28) by a "Mohammed", "CEO and Founder" of an IT-services firm, says his firm distributes and develops products provided by MemberCheck; the review gives no country [GA-ARCH] [F35] | The vendor's pages. Its Iraq page claims "Screening coverage for Iraq" but names no Iraqi client, partner or list. No vendor page links it to Prototype. The review is the only link to a Mohammed, and nothing ties him to Prototype or to Iraq (section 1.3a) [MC-HOME] [MC-IRAQ] [N06] [F35] [GA-ARCH] | A real product with the right words. No Iraq or Prototype link of its own. One review ties a Mohammed to its distribution. Possible engine behind a local product [INFERENCE] |
| **NameScan** (sister brand of MemberCheck, same group and address) | Not the words "member check". It is the screening brand that Prototype's site names as a partner | Prototype's partner block. NameScan's own pages name no Iraqi customer or partner and no Prototype, although it runs a partner programme (section 4.4) [PIQ-SITE] [NS-HOME] [NS-PARTNERSHIP] [N10] [N12] [F35] | The only bridge between Prototype and the MemberCheck family. No source shows Prototype uses it [N10] [N12] |
| **NameCheck** (Cleversoft Group) | Clue 1, if "member check" was a slip for "name check" [INFERENCE] | A marketplace listing. Country and people not shown [SWONE-NC] [N07] | Low [N07] |
| **World-Check** (LSEG) | Clue 1 only in the "-Check" ending | Named in the Central Bank of Iraq's AML controls of 2025; sold in Iraq through Iraqna IBTS in 2020 [F14] [F18] [N07] | Low. It is a global data product that the Central Bank already names, not a system "by some guy" [F14] [F18] |
| **Pio-Tech** (Amman, Jordan) | Clue 1, if "prototype" is a slip for "Pio-Tech" [INFERENCE]. Clue 3 only in part: it works across Iraq but is Jordanian | Research file 18 lists 29 banks operating in Iraq (27 Iraqi banks plus the Ziraat and VakifBank branches) as its references, plus AML, goAML and an Arabic name translator [F18] [N07] [N09] | Low. No source ties the owner's words to it [N07] [F18] |
| **Capital Banking Solutions** (CapitalCompliance) | Not by name. It is an AML product sold to Iraqi banks | A 2021 press item (UR Islamic Bank) and research file 18 [FINTECH] [F18] [N07] [N09] | Low. Foreign, and no name match [F18] [N09] |
| **A GitHub practice project** for sanctions screening, with "Prototype" in its name, owned by an account whose display name starts with "Mohammed" and whose location is Saudi Arabia | Clue 2 and the word "prototype". Not clue 3. Not "member check" | Repository page and account page [GH-SSP] [N07] | Low. It looks like a practice project [INFERENCE] and has no Iraq link [N07] |
| **Other "membercheck" code projects** (26 on GitHub: Telegram membership checkers, club tools, one billing tool for the MemberCheck API) | Clue 1, letter for letter | GitHub search [GH-API] [N07] | Very low. None is AML screening [N07] |
| **A small local system with no web trace** | All three clues, if the owner's description is literal | Eight Arabic and English searches, plus GitHub, npm and GitLab, found no Iraqi-built screening system [N07] [N09] | Cannot be ruled out. No evidence of one [N07] [N09] |

### 1.3 Why Prototype

Four reasons, in order of weight.

1. **It is the only Iraq-linked company found with that name.** Note 08 searched for companies called Prototype in Iraq, or selling into Iraq from Jordan, the UAE, Lebanon or Turkey, and found this one only. Other firms called Prototype exist in the United States and have no Iraq link in the results. [N08]
2. **It sells the right kind of thing, by its own account.** The site calls the company "an Iraqi fintech and cybersecurity company", lists sanctions screening and anti-money-laundering (AML) systems among its products, and shows 31 client entries that are mostly Iraqi banks. [PIQ-SITE]
3. **Its partner list is the only bridge to "member check" [INFERENCE, medium to low on its own].** The site's partner block names Namescan, described as "Global sanctions and PEP screening". NameScan says it belongs to the same Australian group as MemberCheck. No source shows that Prototype uses NameScan. [PIQ-SITE] [F35]
4. **The Arabic title of its Facebook page sits close to the word "prototype".** The company's Facebook page title reads "النموذج الاول لتكنلوجيا المعلومات / Prototype Technology". The first words mean roughly "the first model", and "prototype" is a natural English rendering of them [INFERENCE]. [FB-PIQ] Note 07 reported no company called Prototype, but its Arabic search used the English sound of the word spelled in Arabic letters ("بروتوتايب"). The Arabic title of the company's Facebook page is not spelled that way, which may explain the miss [INFERENCE]. [N07] [FB-PIQ]

Note 08 (written before the lead's browser read) put the confidence that the owner meant this company at "medium", and low for the full description. [N08]

### 1.3a The Mohammed clue: one review

The owner's second clue is "its by some guy named mohammed". One page in the evidence ties a Mohammed to MemberCheck: a review signed "mohammed habeeb" in the saved copy of MemberCheck's GetApp page, which file 35 cites. Nothing ties him to Prototype or to Iraq. The lead confirmed the review in that saved copy on 2026-10-11. [GA-ARCH] [F35]

- **What the review says.** It was syndicated from Capterra (review id `Capterra___6559489`), written on 2024-10-28 and marked verified. The signer's job title is "CEO and Founder" and his industry is "Information Technology and Services". Under drawbacks it says: "It is preferable to add control over the number of searches at the organization or sub-organization level." Its comment reads: "Our experience was very valuable as it increased financial revenues through distributing and developing products provided by Member check Company." [GA-ARCH]
- **What it does not say.** It gives no country and no company name. [GA-ARCH]
- **Reading [INFERENCE].** A request for limits "at the organization or sub-organization level" reads like an account that runs sub-organisations for its own clients, which fits a reseller or white-label partner (file 35, section 5). [F35]
- **Follow-up searches.** On 2026-10-11 the lead ran four web searches: the reviewer's name with Prototype's English name and Iraq; the name with CEO, Founder, Prototype, Baghdad and fintech terms (the search tool reported that it dropped the name, so this one is weak); the name with MemberCheck and distributor; and the Arabic form of the name with the Arabic title of Prototype's Facebook page. They found no page that ties the reviewer to Prototype, to Iraq or to MemberCheck. The name is common, so a search cannot settle it. Nobody was contacted. [LEAD-SEARCH]
- **In plain words.** The review ties a Mohammed to MemberCheck distribution. No source ties him to Prototype or to Iraq. Whether he runs Prototype is not known. [GA-ARCH] [LEAD-SEARCH]

### 1.4 What stays unproven

- **"Member check" is not a Prototype product name.** The words appear nowhere next to Prototype: not in the site code, not in the service and product cards the lead read in the browser, not in the readable brochure text, and not in the app-store and web searches. [PIQ-SITE] [PIQ-BROCHURE] [N08] [N10]
- **No Mohammed is tied to Prototype.** The site names no founder, owner or staff member. The LinkedIn and Facebook pages name nobody. A venture-fund co-founder with a similar first name appears in a press profile that never mentions Prototype. One review ties a Mohammed to MemberCheck distribution (section 1.3a), but no source ties him to Prototype or to Iraq, and whether he runs Prototype is not known. [PIQ-SITE] [LI-PIQ] [N08] [N10] [GA-ARCH]
- **The link to NameScan or MemberCheck is a listing, not a finding.** No page says Prototype uses NameScan's data or software, and NameScan's partnership page does not name Prototype. [NS-PARTNERSHIP] [N10] [N11] [N12] [F35]
- **No outside source confirms any Prototype system at any bank.** See section 5. [N12]
- **The Iraqi company register was not searched,** so the legal identity is open. See section 2. [N10]

### 1.5 Three confidence levels

| Level | Statement | Why |
|---|---|---|
| **Likely** (medium to high) | The company the owner heard of is Prototype for Information Technology | Basis: the name, the Iraqi origin it claims, the sanctions-screening products it claims, and the Namescan partner entry. Not matched on the company's own pages: the words "member check" and any person's name [PIQ-SITE] [N08] [N10] |
| **Possible to likely** (medium; an inference) | "Member check" means MemberCheck or NameScan screening that Prototype resells or builds on | Basis: the Namescan partner entry; the vendor's reseller model ("the customer knows they are buying MemberCheck, and the platform says so") and white-label model ("the customer may never encounter our name"); NameScan's partner programme; and a review showing that an IT-services firm led by a Mohammed distributes MemberCheck products. Missing: any source that names Prototype as that firm [MC-PARTNER] [NS-PARTNERSHIP] [GA-ARCH] [F35] [PIQ-SITE] [INFERENCE] |
| **One candidate, not confirmed** (low to medium) | The Mohammed is the reviewer of section 1.3a | No source ties him to Prototype or to Iraq, and whether he runs Prototype is not known [GA-ARCH] [LEAD-SEARCH] |

---

## 2. Who they are

Short answer: a firm that reports 2 to 10 employees, whose web domain dates from 2019, and that presents itself as an Iraqi financial-technology and cybersecurity company with a Baghdad address and a California address. No person is named on the company's pages. No Iraqi registration was found. Everything below about the company comes from its own pages or from pages the company filled in itself, unless a row says otherwise. [PIQ-SITE] [LI-PIQ] [RDAP] [N10]

### 2.1 Names in use

| Name | Where it appears |
|---|---|
| "Prototype IQ" | The browser-tab title: "Prototype IQ - Elite Software Development & Cybersecurity Solutions" [PIQ-SITE] |
| "Prototype Tech" | The brand shown on the page and in the footer: "© 2025 Prototype Tech. All rights reserved. Financial Technology, Banking Security & Compliance Specialists." [PIQ-SITE] |
| "Prototype for Information Technology" | The opening words of the About text, the name of the LinkedIn page, and the company name in capitals in the brochure [PIQ-SITE] [LI-PIQ] [N11] |
| "PROTTYPE Tech" | How a logo on page 20 of the brochure reads, apparently a spelling slip (read from a page image by a research agent) [PIQ-BROCHURE] [N11] |
| "النموذج الاول لتكنلوجيا المعلومات / Prototype Technology" | The Facebook page title; the Arabic part means roughly "the first model of information technology" [FB-PIQ] [N10] |
| "PROTOTYPE FOR INFORMATION TECHNOLOGY LLC" | A Michigan company with the same English name. Nothing shows it is the same business [OC-PIQ] [N10] |

### 2.2 Where they say they are

- **On the site:** "Location ( USA , IRAQ )", then "IRAQ Baghdad ,Karada / California , USA, 15442 Ventura Blvd., Ste 201-1772". [PIQ-SITE]
- **On LinkedIn:** "Baghdad , Al YArmouk , Near Modn , Baghdad 10075, IQ" and "15442 Ventura Blvd., Ste 201-1772 California , USA". So the Baghdad place differs between the site (Karada) and LinkedIn (Al Yarmouk). [LI-PIQ] [N10]
- **The California entry is unexplained.** Nothing shows whether it is an office or a mailbox. A web search for the street address found no page that carries it. The suffix "Ste 201-1772" has the shape of a mailbox number [INFERENCE]. [N10]

### 2.3 Contacts

The site gives "info@prototype-iq.com" and the telephone number "+9647761114477", with links to a Facebook page, a LinkedIn company page and a WhatsApp chat. A booking-page link (calendly.com/prototype-iq-com) sits in the site code and was not opened. We used none of these: no one at the company was contacted. [PIQ-SITE] [PIQ-JS] [N10]

### 2.4 Age

| Evidence | What it shows |
|---|---|
| The web domain prototype-iq.com was registered on 2019-07-24; the first security certificate for it starts about 20 minutes later | The name was set up on that day [RDAP] [CRT] [INFERENCE] |
| LinkedIn says "Founded 2019" | Entered by the company; no register confirms it [LI-PIQ] [N10] |
| The first archived copy of the domain that loaded properly is dated 2022-02-07, with further copies in 2024 | The domain carried a page by early 2022 [WAYBACK-ROOT] |
| The first archived copy of the current site address (site.prototype-iq.com) is dated 2025-12-23 | Archive coverage is patchy, so the present site may be older [WAYBACK-ALL] [INFERENCE] |
| The brochure's file was created on 2025-11-02; the footer says "© 2025" | The present marketing material dates from late 2025 [PIQ-BROCHURE] [PIQ-SITE] |
| The site says "5+ Years Experience" beside the client list and shows "15+" beside the same words elsewhere | Two figures on one site. "5+" fits a 2019 start (about seven years by 2026). "15+" does not, unless it counts staff careers before the company, which the page does not say [PIQ-SITE] [N10] [INFERENCE] |

No source gives the date the company itself was formed. [N10]

### 2.5 Size

LinkedIn shows "Company size 2-10 employees" and "263 followers". Both are entered by the company, and followers are not staff. [LI-PIQ] [N10] The site claims "31+ Banking Clients" and more than twenty product and service lines. A staff band of 2 to 10 looks small for that, but the band may be out of date [INFERENCE]. [PIQ-SITE] [N10]

### 2.6 People

**No person is named on the company's pages.** The site names no founder, owner or staff member; a search of its code found no "founder" and no "CEO". The logged-out LinkedIn page lists no employee names. The Facebook About page returned only its title. No form of "Mohammed" (English or Arabic spelling) appears in the site code. The only Mohammed tied to MemberCheck anywhere in the evidence is the signer of one review (section 1.3a); no source ties him to Prototype or to Iraq, and whether he runs Prototype is not known. [PIQ-SITE] [PIQ-JS] [LI-PIQ] [FB-PIQ] [N10] [GA-ARCH]

### 2.7 Legal identity

- **Iraq: not found.** No Iraqi registration number, Arabic legal name or registration date was found. The Iraqi company register was not searched, and no online register was reached. [N10]
- **United States: a name match only.** One register entry shares the English name: a Michigan limited liability company, number 900085947, with a registered address in Okemos, Michigan (an aggregator's copy of the state record; the official record was not opened). No page links it to the Iraqi company. [OC-PIQ] [N10]
- **Domain owner: not shown.** The domain record lists the registrar (One.com A/S) and an abuse contact, and no registrant. [RDAP] [N10]
- **Central Bank of Iraq: nothing found.** No Central Bank approval, licence or notice naming Prototype was found, and no Central Bank document was read for this. [N10]

### 2.8 Social pages

LinkedIn carries the long name, the 2019 year, the size band and the two addresses above. [LI-PIQ] The Facebook page carries the Arabic and English title; a plain web request to it returned an error and the About page returned only its title. [FB-PIQ] [N10] The site's downloaded code held a template placeholder (facebook.com/yourpage) in place of a real Facebook link, but the lead's browser read on 2026-10-11 shows real Facebook, LinkedIn and WhatsApp links, so the real links must be filled in after the page loads [INFERENCE]. The browser read wins for what the site showed on 2026-10-11. [PIQ-SITE] [PIQ-JS] [N10]

### 2.9 What the site's build tells us

- **The content is filled in after the page loads.** The downloaded code holds the page title, the "Trusted by Leading Banks" heading, the sentence "We proudly serve major financial institutions across Iraq and the Middle East, delivering mission-critical technology solutions." and the partner list. The client names, product text and contact details are not in it and appear only in a browser. The code points to a third-party hosted backend service, which is probably where they load from [INFERENCE]. [PIQ-JS] [N08] [N10]
- **The home page carries a preview-build marker** (the text data-artifact-kind="preview_commit_sha"). That suggests a preview deployment rather than a finished production site [INFERENCE]. [N08]
- **The "Learn More" button on a product card opened nothing** (no page, no dialog) when the lead clicked it on 2026-10-11. [PIQ-SITE]
- **The services page and the home page disagree.** A fetch of the services page listed six consulting and managed-service lines (Compliance and Audit Excellence, Core Banking Systems, SWIFT Integration Services, Advanced Cybersecurity, FinTech Development, Managed Security Provider) and no AML, KYC or screening line, while the home page shows the AML and screening cards. That fetch returns a model-written summary of a page that draws itself in the browser, so it may be incomplete. [PIQ-SERVICES] [N08]
- **Other domains.** The bare domain redirects to the site address. prototype.iq and protoype.tech do not resolve. prototypeiq.com redirects to a for-sale page, and no link to the company is shown. [PIQ-ROOT] [N08]

---

## 3. What they sell

The home page shows a long list of product and service cards, and the brochure adds pages on the same themes. Every row below is the company's own wording. Nobody has checked it, and no card comes with a demo, a screenshot of live software, a price or a description of how it is installed (product grade C; see section 6). The "Learn More" button on a card opened nothing when the lead clicked it. [PIQ-SITE] [PIQ-BROCHURE] [N11]

### 3.1 AML and screening

| Product | What the company says it does |
|---|---|
| Sanctions Screening | "AI-based screening of sanctions lists is vital to protecting your business from risks." Bullets: Accurate Name Translation; Addressing the "Name Not in Passport" Challenge; Reduce risk and increase compliance. The brochure heading is "AI-POWERED SANCTIONS SCREENING", with "Global & Local Watchlist Translation" and "Passport Name Romanization & Verification". [PIQ-SITE] [PIQ-BROCHURE] |
| AliasGuard | A "smart sanctions and compliance screening system." Bullets: Ultra-accurate translation; AI AKA Generation; Instant translation interface; Reduce false alarms. The brochure says "Instant Translation & Sanctions Screening" and "AI-Powered Alias Matching for Global Compliance". [PIQ-SITE] [PIQ-BROCHURE] |
| International Sanctions Lists | "Protect your financial business from crime and reduce risk by scanning your customers against the world's best checklists." Bullets: Real-time sanctions screening; Global watchlist monitoring; Automated compliance alerts; Risk assessment reporting; PEP database integration; AML compliance support. [PIQ-SITE] |
| Local Sanction List | "Comprehensive local sanctions screening and compliance management tailored for regional regulatory requirements." Bullets: Local watchlist monitoring; Regional compliance checks; Custom sanction rules; Automated screening; Risk categorization; Compliance reporting. [PIQ-SITE] |
| AML AI Systems (possibly the same area as "AGIES AI for AML/CFT" in the About text; no page links them) | "Artificial Intelligence-powered Anti-Money Laundering detection and compliance systems." Bullets: AI-Based Detection; Transaction Monitoring; Risk Scoring; Regulatory Reporting. The brochure's AML page is headed "AI AML & Fraud Monitoring with OFAC Analyzer" and lists "Smart Pattern Detection", "AI-driven Anomaly Insights" and "Real-time OFAC Compliance". [PIQ-SITE] [N11] |
| Customer risk assessment (brochure only) | "AI-POWERED CUSTOMER RISK ASSESSMENT", with "Intelligent Analytical Risk Scoring" and "Automated Due Diligence & Monitoring". The picture is a risk meter with a high and a low end. [PIQ-BROCHURE] [N11] |

### 3.2 Regulatory reporting

| Product | What the company says it does |
|---|---|
| Go AML Reporting System | An "Advanced Anti-Money Laundering reporting system for regulatory compliance and suspicious activity monitoring." Bullets: Suspicious activity detection; Regulatory reporting automation; Transaction monitoring; Risk scoring algorithms; Compliance dashboards; Audit trail management. The card does not say which authority's portal it feeds or whether it writes goAML files. [PIQ-SITE] [N11] |
| FATCA Reporting System | "An advanced, IRS-approved platform designed for seamless and fully compliant FATCA reporting." The brochure calls the company the "FIRST IRS-APPROVED COMPANY with a dedicated FATCA Reporting System". [PIQ-SITE] [PIQ-BROCHURE] [N11] |
| Credit Bureau (ICI - CBS) and ICI CBS System | "Comprehensive credit reporting and bureau integration services for financial institutions." Bullets include credit history analysis, real-time credit scoring and bureau data synchronization. The second card is described as "for Debit Data System". The page does not say which bureau ICI or CBS means. [PIQ-SITE] [N11] |
| IFRS 9 Compliance Solutions | A card title for the accounting rule on expected credit losses; the brochure has one page on it. No description was recorded. [PIQ-SITE] [N11] |

### 3.3 Identity and fraud

| Product | What the company says it does |
|---|---|
| Digital ID and EKYC | An "Advanced Digital Identity & Electronic Know Your Customer (eKYC) solution designed for banks and financial institutions." Bullets: Biometric Verification; Face Matching; Liveness Detection; OCR Document Scanning; AML Screening Integration; Real-Time Identity Validation; Mobile Onboarding; Fraud Detection; API Integration; Regulatory Compliance. [PIQ-SITE] [N11] |
| Fraud Management and Prevention Systems | "Comprehensive fraud detection and prevention systems supporting VISA, MasterCard, and other payment networks." Bullets: Real-time fraud detection; Payment network integration; Machine learning algorithms; Risk rule engine; Transaction blocking; Fraud analytics dashboard. [PIQ-SITE] [N11] |
| Deepfake Detection and Verification | A card title. No description was recorded. [PIQ-SITE] |

### 3.4 Payments and SWIFT

| Product | What the company says it does |
|---|---|
| Swift ISO Message Interface (ISO 20022) | A card title. The page does not say how it connects to a bank's systems. [PIQ-SITE] [N11] |
| 3D Secure Management Systems | A card title for the card-payment authentication step. No description was recorded. [PIQ-SITE] |
| SMS Gateway and SMS Creators | A card title; the brochure has one page on it. [PIQ-SITE] [N11] |
| SWIFT Integration Services (services page) | "Complete SWIFT messaging and Customer Security Program implementation." Bullets: CSP Implementation; Message Processing; Security Controls; Alliance Access. [PIQ-SERVICES] [N11] |

### 3.5 Security consulting and certification

| Product | What the company says it does |
|---|---|
| Consulting cards: PCI DSS Compliance, ISO 27001 Certification, SWIFT CSP Compliance, CSP Cloud Security, SOC 2 Compliance, GDPR Compliance, Vulnerability Assessment | Card titles. The "Certifications & Compliance" block shows "PCI DSS Compliance Consulting", an ISO 27001 "Implementation Partner" and a SWIFT CSP "CSP Assessment Provider". The headline figures include "100% PCI DSS Certified", "PCI DSS Certified QSA", "Expert ISO 27001", "ISO 27001 Lead Implementer" and "24/7 SWIFT CSP Ready". The ISO card offers "ISO 27001 certification support", which is help toward a certificate, not a certificate. [PIQ-SITE] [N11] |
| Cybersecurity; Managed Security Provider | A card title and a services-page line. [PIQ-SITE] [PIQ-SERVICES] [N08] |
| IT audit and risk management (brochure only) | "PROTECT YOUR DIGITAL FRONTIER", with the sub-heading "IT AUDIT & RISK MANAGEMENT". The page carries the printed label "AI-generated content". [PIQ-BROCHURE] [N11] |
| SSL certificates | The partner block lists DigiCert with the text "Official SSL Certificate Reseller #1407277". Read plainly, the company says it resells DigiCert certificates. A search for that number found nothing. [PIQ-SITE] [N10] |

### 3.6 Software and infrastructure

| Product | What the company says it does |
|---|---|
| Software Development; Cloud Solutions | Card titles. [PIQ-SITE] |
| Core Banking Data Migration | Brochure: "SEAMLESS DATA MIGRATION between Core Banking Systems", with "Zero Data Loss" and "Minimal Downtime". It shows no interface, bank or method. [PIQ-SITE] [PIQ-BROCHURE] [N11] |
| Core Banking Systems (services page) | "Mission-critical banking infrastructure and real-time transaction processing." Bullets: Core Banking Implementation; Real-time Processing; Multi-currency Support; Regulatory Reporting. [PIQ-SERVICES] [N11] |
| Online and Onboarding Loan Management System | A card title. No description was recorded. [PIQ-SITE] |
| ProtoAudit and ProtoBot | Named in the About text as part of the portfolio and nowhere else. A web search for either name found no page about it. [PIQ-SITE] [N11] |

### 3.7 Reading the range

- **The range is wide for the staff band.** The list runs from sanctions screening to SSL certificates and an SMS gateway, from a company that reports 2 to 10 employees. [PIQ-SITE] [LI-PIQ]
- **The company looks services-led [INFERENCE].** The services page carries six consulting and managed-service lines with price labels only ("Enterprise", "Custom", "Quote", "Monthly", "Project"), while the product cards carry no price at all. No figure appears anywhere. [PIQ-SERVICES] [N11]
- **Partners are named, but not their role.** The site lists DigiCert, Namescan, SumSub, Auth0, TeleSign and Online AML as "Global Technology Partners" and does not say which supplies which product. [PIQ-SITE] [N11]

---

## 4. How they say it works

Nothing here was seen working. It is the company's description, read from card text and brochure pictures. The brochure screens are drawn illustrations, not captured software. [PIQ-SITE] [PIQ-BROCHURE] [N11]

### 4.1 Name translation and romanization (Sanctions Screening, AliasGuard)

- **What is claimed.** The cards promise "Accurate Name Translation", a fix for the "Name Not in Passport" challenge, "Ultra-accurate translation", "AI AKA Generation" and an "Instant translation interface". The brochure adds "Global & Local Watchlist Translation" and "Passport Name Romanization & Verification". [PIQ-SITE] [PIQ-BROCHURE]
- **Plain reading [INFERENCE].** The system seems to turn a name into several spellings, for example Arabic into Latin letters, and to screen all of them, so that a passport spelling still finds a list entry written another way. "AI AKA Generation" reads as automatic creation of alias spellings. The company does not say this in these words, and "Name Not in Passport" is not explained anywhere. [PIQ-SITE]
- **What the pictures show.** One example name appears in about six Latin spellings around an "AI" chip, captioned "AI-Powered Alias Matching for Global Compliance" (brochure page 15). Another page shows a name in Arabic script on an ID-style card, an arrow into an AI block, and a screen listing "AKA (Also Know As)" spellings with a red alert mark (page 17). [PIQ-BROCHURE] [N11]
- **What is not said.** The method, the accuracy, the false-alarm rate, the languages and scripts, and whether the pictures show a real screen. The card text names no language. Nothing in the text read mentions the Iraqi name chain (given name, father, grandfather, tribe, mother's name, birth year) that file 14 lists as a need and file 22 covers in depth. [N11] [F14] [F22]

### 4.2 International and local lists

- The International card promises "Real-time sanctions screening", "Global watchlist monitoring" and "PEP database integration" against "the world's best checklists". The Local card promises "Local watchlist monitoring" and "Custom sanction rules" for "regional regulatory requirements". [PIQ-SITE]
- **Only one list is named: OFAC**, on brochure page 7 ("Real-time OFAC Compliance" and an "OFAC list" graphic). No page read names the UN, EU or UK lists or the Iraqi national list, and the Local card names no regional list. [PIQ-BROCHURE] [N11]
- Against the first two needs in file 14, section 1 (the lists the Central Bank names, refreshed at least every 12 hours), the site says nothing that can be checked. [F14] [PIQ-SITE]

### 4.3 AML AI, "AGIES", goAML reporting and customer risk

- **Possibly one area, three names [INFERENCE].** "AGIES AI for AML/CFT" (About text), "AML AI Systems" (card) and "AI AML & Fraud Monitoring with OFAC Analyzer" (brochure page 7). No page links them. The card lists "Transaction Monitoring" and "Regulatory Reporting". The brochure page lists pattern detection and OFAC checks, and mentions neither transaction monitoring nor reporting. [PIQ-SITE] [PIQ-BROCHURE] [N11]
- **The labels that link a bank to an AML or sanctions product** are "AML Solutions" (Misk Islamic Bank, Regional Commercial Bank - RTB) and "Sanctions list system" (IDB). The insurers Al Batek Insurance Company and Asia Gate Insurance also carry "AML Solutions". The labels do not say which module, or since when. [PIQ-SITE] [N11]
- **Go AML Reporting System.** The card does not say which authority's portal it feeds, which report types it builds or whether it writes goAML files. The name echoes goAML [INFERENCE]. The goAML platform was launched in Iraq in 2022, and nothing links that to Prototype. [PIQ-SITE] [N11] [F18]
- **Customer risk assessment** appears only as a brochure page with a risk meter. No model, data or bank is described. [PIQ-BROCHURE] [N11]

### 4.4 The engine question: its own, or a partner's

- **Points to a partner feed [INFERENCE].** The partner block lists Namescan ("Global sanctions and PEP screening": Sanctions Screening, PEP Lists, Watchlist Management) and Online AML ("Anti-money laundering solutions": AML Screening, Transaction Monitoring, Compliance Reporting). Prototype's cards use close wording ("PEP database integration", "Global watchlist monitoring", "Transaction Monitoring"), and "integration" usually means an outside data feed. [PIQ-SITE] [N11]
- **Points nowhere.** No page says Prototype uses these partners' data or matching, and the brochure names no vendor. NameScan's pages name no Iraqi customer and no Prototype, and its partnership page names no Iraqi partner (see the next point). "Online AML" was not identified: two searches found no company of that name. Searches for AliasGuard and AGIES found no page, so these may be internal names. [N11] [N12] [N10]
- **NameScan does run a partner programme, so a route exists.** Note 11 relied on a search summary of this page, and note 12 read only NameScan's home and pricing pages, which show none. The writer fetched the partnership page on 2026-10-11 (the fetch returns a model-written summary). It names three arrangements: referrals ("Promote NameScan's KYC services and recieve a fee for every successful conversion"), distributors ("MarkUp NameScan's discounted subscription fee and establish direct connections with your client") and application integrators ("Integrate with our API to streamline your clients onboarding process"), with the spellings as the summary gives them. It names eight partners, located in Sweden, the UK, Georgia, South Africa, Hong Kong, Estonia, Latin America and France. It names no Iraqi or Middle Eastern partner and does not mention Prototype. File 35 had not checked this page. [NS-PARTNERSHIP] [N11] [N12] [F35]
- **Verdict: not settled.** The data could be Prototype's own, a partner's, or a mix. A partner feed for at least the lists looks plausible [INFERENCE].

### 4.5 What a NameScan-based service would mean

- For NameScan's pay-per-search pricing, its two data tiers and its cloud hosting, see file 35 (sections 3.7, 4.4 and 7.1). In short: prepaid search credits, drawn down one per completed search, used through a web dashboard or an API. [F35] [NS-PRICING] [NS-HOME]
- If Prototype's service were built on NameScan, it would be a front end to a hosted service run from Australia. NameScan says it serves businesses in any jurisdiction, so a bank could buy the same searches directly. What a local firm could add is Arabic handling, support and the bank relationship [INFERENCE]. [NS-PRICING] [F35]
- **What that would imply for the bank [INFERENCE].** Each customer name checked would leave the bank's own servers and go to a vendor-run cloud service whose data location is not stated. The stated refresh for NameScan is "daily", against the Central Bank's 12 hours, and its 2024 coverage table lists Iraq but shows no separate Iraqi list. Whether names may leave the bank is a decision for the bank's compliance and legal staff; no source read settles it. The bank might also never see NameScan's name on the contract, which is how white label works. [F35] [N12] [MC-PARTNER]
- AKR-TMS is designed to run on the bank's servers because file 30 concluded that Central Bank data-classification rules block a vendor-hosted multi-tenant model in the first release. [F30]

### 4.6 Connection to the bank, hosting and price

The About text promises "secure banking integrations designed specifically for financial institutions and payment providers." No interface description, API document, core-banking adapter, deployment model (on the bank's servers, hosted or cloud) or price is published. The services page shows price labels only. [PIQ-SITE] [PIQ-SERVICES] [N11]

---

## 5. Clients claimed and independent evidence

**What the site says.** Under "Trusted by Leading Banks" it writes: "We proudly serve major financial institutions across Iraq and the Middle East, delivering mission-critical technology solutions." A counter reads "31+ Banking Clients". The list below has 31 entries: 28 distinct banks (Cihan Bank appears twice) and 2 insurers. So the counter matches the number of list entries, not the number of banks [INFERENCE]. Only seven entries carry a product label: IDB ("Sanctions list system"), Misk Islamic Bank, Al Batek Insurance Company, Asia Gate Insurance and Regional Commercial Bank - RTB ("AML Solutions"), and the second Cihan entry and Iraqi Middle East Investment Bank ("Compliance Solutions"). The company also sells SSL certificates, security consulting and other non-AML services, so a place on the list does not mean an AML or screening system. For rows 10 to 31 of the table below, the pairing of each name with its web address was matched by order, not read from each link. [PIQ-SITE]

**Client grades** (the scale of note 12, applied to the claim "this bank uses a Prototype system"): A = an audited report or the bank's own statement; B = a statement by the company or a partner; C = a staff profile, job advert or social post; D = a hint only. [N12]

| # | Client as listed (address on the site; for rows 10 to 31 the name-to-address pairing was matched by order, not read from each link) | Text on Prototype's site | Grade | What other sources record |
|---|---|---|---|---|
| 1 | IS Bank (isbank.iq) | "Leading commercial bank in Iraq" | B | Not in file 18; no bank source read [PIQ-SITE] [F18] [N12] |
| 2 | Ziraat Bank (ziraatbank.iq) | "Turkish bank operations in Iraq" | B | File 18: foreign branch, group systems not found; Pio-Tech reference [PIQ-SITE] [F18] |
| 3 | Vakif Bank (vakifbank.com.tr) | "International banking services" | B | As row 2; the address shown is a Turkish (.com.tr) site [PIQ-SITE] [F18] |
| 4 | Rasheed Bank (rasheedbank.gov.iq) | "Government banking institution" | B | File 18: core vendor not named, no AML supplier found [PIQ-SITE] [F18] |
| 5 | Agricultural Bank (agbank.gov.iq) | "Agricultural financing specialist" | B | File 18: no core or AML supplier found [PIQ-SITE] [F18] |
| 6 | IDB, International Dev. bank (idb.iq) | "Sanctions list system" (label) | B | File 18: core Path iMAL, no AML system found. No source for the label; no reachable bank website [PIQ-SITE] [F18] [N12] |
| 7 | Nahreen Islamic Bank (nib.gov.iq) | "Islamic financial services" | B | Not in file 18; no source read [PIQ-SITE] [F18] [N12] |
| 8 | Cihan Bank (cihanbank.com.iq) | "Private commercial bank" | B | File 18: core Path iMAL since 2009; Pio-Tech reference. The site read for it (cihanbank.com, not the address Prototype links to) shows only a placeholder page [PIQ-SITE] [F18] [CIHAN-LANDER] [N12] |
| 9 | Qurtas Bank (alqurtas.iq) | "Modern banking solutions" | B | File 18: core BML Istisharat ICBS, no AML supplier found [PIQ-SITE] [F18] |
| 10 | Asia Bank (aiib.iq) | "Investment and commercial banking" | B | Read as Asia Iraq Islamic Bank; its home page has no AML text; Pio-Tech reference in file 18 [PIQ-SITE] [F18] [AIIB-HOME] [N12] |
| 11 | Mashriq Bank (amb.iq) | "Regional banking services" | B | Not in file 18; no source read [PIQ-SITE] [F18] [N12] |
| 12 | International Islamic Bank of Iraq (imtb.iq) | "Commercial banking institution" | B | File 18: core ICS BANKS Islamic; goAML from Pio-Tech, live April 2024. Prototype also sells a "Go AML Reporting System", so two suppliers claim one product class [PIQ-SITE] [F18] [N12] |
| 13 | Rafidian Bank (rafidain-bank.gov.iq) | none shown | B | File 18: no AML supplier found; K2 Integrity partnership, August 2025. The bank said on 2025-04-11 that its AML and goAML systems were active and named no supplier [PIQ-SITE] [F18] [N09] |
| 14 | Kurdistan Islamic Bank (kib.iq) | none shown | B | Azentio's page and file 18 record Azentio AMLOCK at Kurdistan International Islamic Bank, probably the same bank (name match not confirmed). That supplier is not Prototype [PIQ-SITE] [AZENTIO-KIIB] [F18] [N12] |
| 15 | Al Baraka Turk Bank -Iraq Branch (albaraka.com.iq) | none shown | B | Not in file 18; no source read [PIQ-SITE] [F18] [N12] |
| 16 | Erbil Bank (ebif.iq) | none shown | B | File 18: core BML Istisharat ICBS, no AML supplier found [PIQ-SITE] [F18] |
| 17 | Economy bank (ebi.iq) | none shown | B | Not in file 18; no source read [PIQ-SITE] [F18] [N12] |
| 18 | Mosul Bank (almosulbank.iq) | none shown | B | Filed 2024 final accounts with the Iraq Securities Commission; the notice names no AML system (A for the filing only). Pio-Tech reference in file 18 [PIQ-SITE] [ISC-MOSUL] [F18] [N12] |
| 19 | Arabia Islamic Bank (ab.iq) | none shown | B | File 18 (probably as "Al-Arabiya Islamic Bank"): core ICS BANKS Islamic; Pio-Tech reference [PIQ-SITE] [F18] [N12] |
| 20 | Misk Islamic Bank (miskbank.iq) | "AML Solutions" (label) | B | Not in file 18; no source and no reachable website [PIQ-SITE] [F18] [N12] |
| 21 | Warka Bank for Investment Finance (warkabank.iq) | none shown | B | File 18: no core or AML supplier found [PIQ-SITE] [F18] |
| 22 | Sumer Commercial Bank (sumerbank.iq) | none shown | B | File 18: Capital Banking Solutions core and CapitalCompliance with goAML and CRS (vendor release, August 2022). Home page has no AML text [PIQ-SITE] [F18] [SUMER-HOME] [N12] |
| 23 | National Islamic Bank (nib.iq) | none shown | B | Not in file 18; no source read [PIQ-SITE] [F18] [N12] |
| 24 | Rajih Islamic Bank (rib.iq) | none shown | B | File 18: core Path iMAL; Pio-Tech reference [PIQ-SITE] [F18] |
| 25 | First Iraq Bank (fib.iq) | none shown | B | File 18 lists "First Iraqi Bank" (match likely, not confirmed): core ICS BANKS Digital; Pio-Tech credit-bureau product [PIQ-SITE] [F18] [N12] |
| 26 | Trust Int. Islamic Bank (itbank.iq) | none shown | B | Not in file 18; no source read [PIQ-SITE] [F18] [N12] |
| 27 | Al Batek Insurance Company (albatek.iq) | "AML Solutions" (label) | B | An insurer, outside file 18's bank map; no source read [PIQ-SITE] [F18] |
| 28 | CIHAN BANK, second entry | "Compliance Solutions" (label) | B | Same bank as row 8. File 18 records no AML supplier; a reference list names no product, so it can neither confirm nor deny the label [PIQ-SITE] [F18] [N12] |
| 29 | Asia Gate Insurance (asiagateco.com) | "AML Solutions" (label) | B | An insurer, outside file 18's bank map; no source read [PIQ-SITE] [F18] |
| 30 | Iraqi Middle East Investment Bank (imeib.iq) | "Compliance Solutions" (label) | B | File 18: Capital Banking Solutions core since 2006, no AML system found. Filed 2024 final accounts, no AML system named (A for the filing only). The site read for it (imeib.com, not the address Prototype links to) shows only a placeholder page [PIQ-SITE] [F18] [ISC-IMEIB] [IMEIB-LANDER] [N12] |
| 31 | Regional Commercial Bank - RTB (rtb.iq) | "AML Solutions" (label) | B | The bank's home page ("RTB \| Home") says "Aligned with the best local and international compliance practices" and names no system (A for that). A press item calls the Erbil lender "Region Trade Bank"; file 18 lists it with a Pio-Tech reference. The name is not resolved [PIQ-SITE] [RTB-HOME] [RUDAW-RTB] [F18] [N12] |

**What the cross-check with file 18 shows**
- **None of the four banks whose annual reports name their AML suppliers is on Prototype's list.** Those banks are Bank of Baghdad, National Bank of Iraq, Ameen Al-Iraq Islamic Bank and Trans Iraq Bank. [F18] [PIQ-SITE]
- **Three listed banks have a different AML-side supplier in a vendor release.** They are Sumer (Capital Banking Solutions), International Islamic Bank of Iraq (Pio-Tech's goAML) and, if the name match holds, Kurdistan International Islamic Bank (Azentio). A bank can use more than one supplier, so the independent record neither confirms nor contradicts Prototype's list. [F18] [AZENTIO-KIIB] [N12]
- **Reference lists in this market overlap.** About ten of the 28 banks (some by name match only) also appear on Pio-Tech's references page, which names banks without naming products. [F18] [N12]
- **The labelled entries are the weakest to check.** For IDB and Misk Islamic Bank no source of any kind was found. RTB's name does not match across sources. The Cihan and IMEIB sites that were read show placeholder pages. [N12]

**Independent confirmation: none found.** In 15 web searches (English and Arabic) and in the documents read (bank home pages, six Iraq Securities Commission PDFs, vendor pages), no bank, auditor, regulator, news item or event page names Prototype as a supplier. No signing, training, exhibition, Central Bank event or award involving Prototype and a bank was found. No Iraqi bank document read contains "Member Check", "MemberCheck" or "NameScan". File 18 holds no record of Prototype. This is not proof of absence: the search index covers few Iraqi bank documents, and five of the six regulator PDFs could not be read as text. [N12] [N06] [F18]

---

## 6. Evidence quality

**Product grades** (the scale of note 11): A = confirmed by a third party or shown working; B = described in detail by the company, with a method; C = a one-line claim, bullets, labels or an illustration only. Every product claim is grade C, with no demo, documentation or outside confirmation. None reaches B or A. These product grades are separate from the client grades of section 5. [N11] The table lists the claims that are unsupported or inconsistent and what was checked.

| Claim or sign | Why it is weak or inconsistent | What was checked |
|---|---|---|
| "31+ Banking Clients"; "Trusted by Leading Banks" | The list has 31 entries: 28 distinct banks (one repeated) and 2 insurers. No bank confirms any entry | The list in a real browser; 15 searches, bank sites and regulator notices (section 5) [PIQ-SITE] [N12] |
| "5+ Years Experience" and "15+" on the same site | Two figures. The domain dates from 2019, which fits 5+ and not 15+ for the company | Site code and browser read; domain record; LinkedIn [PIQ-SITE] [N10] [RDAP] [LI-PIQ] |
| "100% PCI DSS Certified"; "PCI DSS Certified QSA" | No certificate, number or date is shown. A QSA is listed publicly by the card industry; that list was not opened | Site and brochure text; a web search for the company name with "PCI QSA" found nothing [PIQ-SITE] [N11] |
| "IRS-approved"; "FIRST IRS-APPROVED COMPANY" | No approval letter, number or date. The same seal sits on a FATCA page and on a security-assessment page, which suggests a marketing badge, not an approval [INFERENCE] | Brochure pages 2 and 20 [PIQ-BROCHURE] [N11] |
| "100% Compliance Rate"; "99.9% Uptime SLA"; "24/7 Support" | No sample, period, method or contract is shown | Site text [PIQ-SITE] [N11] |
| ISO 27001 "Lead Implementer" and "certification support" | Help toward a certificate, not a certificate | Site text [PIQ-SITE] [N11] |
| DigiCert "Official SSL Certificate Reseller #1407277" | A search for the number matched it to neither DigiCert nor Prototype; DigiCert's partner directory was not reached | One web search for the number; the partner directory was not reached [N10] |
| Six "Global Technology Partners" | No partner page names Prototype. NameScan's partnership page does not; the SumSub partner list was not checked; "Online AML" was not identified | NameScan's home, pricing and partnership pages; two searches for "Online AML" [NS-HOME] [NS-PRICING] [NS-PARTNERSHIP] [N10] [N12] |
| Every product (grade C) | No demo, no screenshot of live software, no documentation, no price, no deployment model. Brochure screens are drawn illustrations, and one page carries the printed label "AI-generated content" | Home page, services page and the readable brochure pages [PIQ-SITE] [PIQ-BROCHURE] [N11] |
| AGIES, ProtoAudit, ProtoBot, AliasGuard | Named in the About text or on a card. ProtoAudit and ProtoBot have no description. Web searches found no page for any of the four names | Searches for each name with "Prototype" and "Iraq" [N11] |
| "Learn More" buttons | Opened nothing | Clicked by the lead on 2026-10-11 [PIQ-SITE] |
| Services page against home page | The services page lists no AML, KYC or screening line, while the home page shows the cards | A fetch of each page (a model-written summary of a page drawn in the browser) [PIQ-SERVICES] [N08] |
| Staff of 2 to 10 and 263 followers, against 28 or so cards and 31 client entries | The scale of the claims is large for the size [INFERENCE] | LinkedIn [LI-PIQ] [N10] |
| Addresses | Karada (site) and Al Yarmouk (LinkedIn) in Baghdad; a California suite number shaped like a mailbox number [INFERENCE]; no page found that carries the street address | LinkedIn; a web search [LI-PIQ] [PIQ-SITE] [N10] |
| Finish | A preview-build marker in the page code; brochure headings that note 11 read as misspelled (for example "Scamless", "Vullerrablity"); a logo read as "PROTTYPE Tech". The lead's quote of one such heading (customer risk, page 14) is spelled correctly, so the typo readings are from page images and not confirmed | Page source; brochure page images [N08] [N11] [PIQ-BROCHURE] |

**Where our own notes disagreed, and what this file uses**
- Note 07 says no company called Prototype was found; note 08 found one. This file uses note 08 and the browser read (section 1.3). [N07] [N08]
- Note 10 says the brochure was encrypted and unreadable; note 11 and the lead read it. This file uses note 11 and the lead's reading. [N10] [N11] [PIQ-BROCHURE]
- Notes 08 and 11 treat the client list, the address and the product names as unverified, because they came from fetch summaries. The lead's browser read of 2026-10-11 shows them, so this file treats them as what the site showed on 2026-10-11 (still the company's own claims). [N08] [N11] [PIQ-SITE]
- Note 11 (search summary) said NameScan has a partnership page; note 12 read only the home and pricing pages and found no partner programme. The writer's fetch of 2026-10-11 confirms the page exists (section 4.4). This file uses the fetch. [N11] [N12] [NS-PARTNERSHIP]

---

## 7. What it appears to do right

Only points that the evidence supports. None of them shows that a product works.

- **It puts the Arabic name problem up front.** The cards and brochure lead with "Name Not in Passport", alias generation and passport-name romanization. File 14 found that none of the eight engines it reviewed documents a mother's-name feature, a chain-position-aware comparison of Iraqi names or Kurdish letter folding, so a seller that opens with the Arabic name problem speaks to what Iraqi compliance staff meet [INFERENCE]. Naming the problem is not solving it. [PIQ-SITE] [PIQ-BROCHURE] [F14]
- **It splits international and local lists into two products.** That mirrors the Central Bank's need for both kinds of list (need 1 in file 14, section 1). Whether the Local card loads the Iraqi national list is not shown. [PIQ-SITE] [F14]
- **It presents itself as local.** The site says "Built in Iraq", gives a Baghdad address and claims "a deeper understanding of local regulatory and operational environments". These are the company's words. The outside facts are a web domain held since 2019 and due to expire on 2027-07-24, and a LinkedIn page. [PIQ-SITE] [RDAP] [LI-PIQ]
- **Its breadth may open doors at banks [INFERENCE].** Certification consulting, SSL certificates, SWIFT and core-banking work and FATCA reporting are routine bank purchases. A firm that sells them has reasons to be in the building, and an AML or screening sale can follow. Only seven of the 31 client entries carry a product label, which fits this pattern. [PIQ-SITE]
- **It lists six names as "Global Technology Partners".** They are DigiCert, Namescan, SumSub, Auth0, TeleSign and Online AML. None of them confirms it (NameScan's partnership page does not name Prototype; Online AML was not identified). Listing the names openly is more open than the white-label model, in which the buyer may never see the maker's name [INFERENCE]. It does not say which partner supplies what. [PIQ-SITE] [NS-PARTNERSHIP] [N10] [MC-PARTNER]

---

## 8. What it does wrong or leaves unproven

Each point rests on a named check. The words "unproven" and "not shown" mean that nothing was found, not that the opposite is true.

- **No outside confirmation of any client.** No bank, auditor, regulator, news item or event page names Prototype as a supplier (section 5). [N12]
- **Credentials shown as badges without papers.** "PCI DSS Certified QSA", "IRS-approved" and "100% Compliance Rate" come with no document, number or date, and one IRS seal sits on two unrelated pages (section 6). [PIQ-SITE] [PIQ-BROCHURE] [N11]
- **No technical disclosure.** No method, list names beyond OFAC, refresh time, accuracy, false-alarm rate, response time, interface, deployment model or price is published. A bank cannot test a claim that has no definition. [N11]
- **Arabic and Iraqi name matching unproven.** The only specifics are drawings. Nothing mentions the Iraqi name chain, mother's name or birth year, so need 3 of file 14 (search in Arabic and English; native matching of the Iraqi name shape) stays unknown. [PIQ-BROCHURE] [N11] [F14]
- **Lists and freshness unproven.** Against needs 1 and 2 of file 14 (the Central Bank's named lists, refreshed at least every 12 hours), the site names only OFAC and gives no refresh time. [N11] [F14]
- **Engine and data source unclear.** It may be Prototype's own, NameScan's or Online AML's. If a partner's hosted service sits behind it, customer names leave the bank (section 4.5) [INFERENCE]. [N11] [N12]
- **Transaction monitoring not shown.** The AML card lists it; the brochure's AML page does not. No scenarios, thresholds, alert screens or tuning are described, and no bank is named for it. [PIQ-SITE] [PIQ-BROCHURE] [N11]
- **Who is behind it is not public.** No person is named on its pages, no Iraqi registration was found, and LinkedIn reports 2 to 10 staff. No source ties the MemberCheck reviewer of section 1.3a to it. A bank checking a new supplier would find little to check [INFERENCE]. [N10] [LI-PIQ] [GA-ARCH]
- **Uneven finish.** A preview-build marker, buttons that open nothing, a services page that disagrees with the home page, two Baghdad addresses and an unexplained California one (section 6). [PIQ-SITE] [N08] [N10]
- **Capacity.** Delivering and supporting 28 banks and a product list of this size with 2 to 10 staff is unproven [INFERENCE]. [LI-PIQ] [PIQ-SITE]

---

## 9. Overlap with AKR-TMS

AKR-TMS is designed to read each bank's core banking database through a read-only connection, to run on the bank's own servers, and to do transaction monitoring, customer risk rating, name screening with Arabic and Iraqi name-chain matching, case work and goAML reporting. It is a plan: in the status table of 2026-10-11, phase 00 of 32 is done on the build machine and phases 01 to 31 are at "Spec ready". An optional screening API for payment gateways is planned for a later phase. The module names below are those of docs/MODULES.md. "Grade" in the third column is the product grade of section 6. [MODULES] [STATUS] [F35]

| AKR-TMS module | Prototype's nearest product | Evidence grade for Prototype's product | Where AKR-TMS differs or is exposed |
|---|---|---|---|
| M9 Screening and Name Matching | Sanctions Screening; AliasGuard; International Sanctions Lists; Local Sanction List | C | Designed to differ: it is to load the national list from the Office plus the UN, OFAC, EU and UK lists, to match the Iraqi name chain, to run daily known-answer tests and to run on the bank's servers. Exposed: the "translation and aliases" pitch is easy to grasp and can be sold with services the bank already buys [INFERENCE] [PIQ-SITE] [N11] [MODULES] |
| M10 Customer Risk Rating and KYC | Customer risk assessment (one brochure page); Digital ID and EKYC | C | Designed to differ: it is to rate risk from the bank's own data. Exposed: AKR-TMS is not planned to offer onboarding identity checks (face match, liveness, document scanning), so a bank buying those may get screening in the same bundle [PIQ-BROCHURE] [PIQ-SITE] [N11] [F35] |
| M11 Transaction Monitoring and Scenario Studio | AML AI Systems (the About text's "AGIES AI for AML/CFT" may be the same product); Fraud Management and Prevention Systems | C | Designed to differ: a scenario studio with replays, a go-live gate and fingerprinted alerts. Prototype describes no scenarios, thresholds or tuning, and its brochure AML page omits transaction monitoring. Exposed: "AI" wording may appeal to buyers [INFERENCE] [PIQ-SITE] [PIQ-BROCHURE] [N11] [MODULES] |
| M12 Alerts, Cases and Investigation | Bullets "Automated compliance alerts", "Compliance dashboards", "Audit trail management" | C | No case-management product or screen is named. AKR-TMS is designed to cover case work in full [PIQ-SITE] [N11] [MODULES] |
| M13 Reporting to the AML/CFT Office | Go AML Reporting System | C | Designed to differ: AKR-TMS is to check goAML files against the Office's schema and Iraqi business rules (mother's name, city, state) and give an estimated grade. Prototype's card does not say what it builds. Exposed: Pio-Tech and Capital Banking Solutions already sell goAML at Iraqi banks, and two of those banks are on Prototype's list (International Islamic Bank of Iraq and Sumer) [PIQ-SITE] [MODULES] [F18] [N12] |
| M19 Assessment API (phase 2) | International Sanctions Lists ("Real-time sanctions screening"); the eKYC bullet "API Integration" | C | Designed to differ: a signed, replayable decision endpoint with a local list index and a fail policy per payment route, never reading the bank database. Prototype describes no interface, response time or failure behaviour. Exposed: a vendor that says "real-time" could reach a payment gateway first [INFERENCE] [PIQ-SITE] [N11] [MODULES] |
| M5 Sources, Discovery and Ingestion | "Secure banking integrations"; Core Banking Data Migration; Core Banking Systems | C | Designed to differ: a read-only connection to the bank's database. Prototype describes no connection method or hosting. Exposed: core-banking and migration work, if real, would give it knowledge of the banks' data; no project is named [INFERENCE] [PIQ-SITE] [PIQ-BROCHURE] [N11] [MODULES] |
| M14 Operations Tasks and Freezes; M18 Islamic Banking Pack | None found | not applicable | No freeze or Islamic-banking feature is named on the cards read, although many listed clients are Islamic banks by name [PIQ-SITE] [MODULES] |

**Reading the table.** The closest overlaps are M9 (screening) and M13 (goAML), where Prototype's cards name the same job but give no method. M11 and M12 are where Prototype shows least. Digital ID and EKYC sits outside AKR-TMS's scope. Every Prototype product is grade C, so the table compares claims with a design, not products with products. [PIQ-SITE] [N11] [F35]

---

## 10. What it means for AKR-TMS

**Competitive reading**
- **On the evidence, Prototype is a set of claims, not yet a proven rival.** Every product is grade C, no bank confirms a deployment, and the company reports 2 to 10 staff. [N11] [N12] [LI-PIQ]
- **Treat it as a name that bank staff may know.** It lists 28 distinct banks and sells a wide range of bank services. If the owner's "prototype member check" is this company, the name has reached the owner by some route, which suggests it circulates [INFERENCE]. [PIQ-SITE]
- **Where AKR-TMS is designed to have the edge on the public evidence:** it is designed to run on the bank's servers, to read the bank database read-only, to load the Iraqi national list, to match the Iraqi name chain and to check goAML files, and a pre-release payment check is planned. Prototype shows none of these in public. "Not shown" does not mean "absent". [MODULES] [PIQ-SITE] [N11]
- **Where not to assume AKR-TMS wins:** speed of start, price, breadth (certification, SSL, SWIFT and eKYC services), existing bank relationships and "AI" wording. A hosted tool may simply tick the bank's "screening" box [INFERENCE]. [F35]
- **If the screening is built on NameScan,** it would inherit the gaps file 35 records for that family unless Prototype adds its own: vendor-hosted, a daily refresh against the Central Bank's 12 hours, and no Iraqi list shown. The AKR-TMS answer would be that, by design, names stay on the bank's servers [INFERENCE]. [F35] [N12]
- **A partner role is possible but premature.** File 18 treats local integrators as possible channels. For Prototype, who runs it and what it has delivered are unknown, so there is nothing yet to weigh [INFERENCE]. [F18] [N10]
- **Copy and avoid.** Copy the plain statement of the name problem. Avoid badges without papers, claims without definitions, and a headline count that counts list entries and not customers. [PIQ-SITE] [N11]

**Practical next steps**
1. Ask the owner for the smallest useful detail: a screenshot or photo of where the name was seen, with the web address and logo visible, and who said "Mohammed". [N07]
2. Run the checks in section 11 through someone inside a bank that is on the list, asking only for what that person may properly say or show. No contact with Prototype is needed.
3. Search the Iraqi company register for Prototype for Information Technology (not yet done) to get the legal name, number, date and owners. [N10]
4. Prepare a short AKR-TMS demonstration for the "translation and aliases" pitch: one fictional customer name in several Latin spellings, with mother's name and birth year, matched through the Iraqi name chain. No sanctioned person's name is needed [INFERENCE]. [F22]
5. Do not repeat Prototype's client list or badges as fact in AKR-TMS material. If cited, write "the company says".
6. Decide whether to ask NameScan whether Prototype is a customer, distributor or integrator. That is the quickest way to test the "possible" level. This file made no contact. [NS-PARTNERSHIP]
7. When file 18 is next revised, add Prototype to it as "claimed, unconfirmed". File 35 is being corrected now and will carry the NameScan partnership page. No other file was changed here.

---

## 11. How to settle the open points

The fastest route is from inside a bank on the client list. None of these steps needs contact with Prototype. Ask only for what a bank's own staff may properly say or show.

**Things to ask or look at inside a client bank**
1. The name, logo and web address on the screening system's login page and screens. This tells "Prototype's own product" from "a NameScan or MemberCheck screen" at once.
2. The supplier's name, as the bank's own staff use it.
3. How often the lists update.
4. Whether the screening system runs on a bank server.
5. Who the supplier's contact is.
6. What the product is called on screen, in Arabic and in English. This settles whether anything is called "member check".

**Only where the bank is entitled and willing to share them**
- For each listed bank, its published annual report, including the paragraph in which the external auditor names AML systems and suppliers (the pattern in file 18, section 8). Several reports could not be read as text, and none was found for Mosul, IMEIB, KIIB, Sumer, Asia Iraq Islamic, Trust International, National Islamic or Economy Bank. [F18] [N12]
- Nobody should be asked to pass on a supplier's confidential documents, such as a contract, an invoice, a manual, training slides or a data-flow diagram.

**Public registers and NameScan**
- From public registers: the Iraqi company register entry for Prototype (number, owners, date); the card industry's list of QSA firms, to test "PCI DSS Certified QSA"; DigiCert's partner directory, to test the reseller number; the official Michigan record, to see whether the LLC with the same name is related. [N10] [N11]
- From NameScan, at the owner's decision: whether Prototype is a customer, distributor or integrator. [NS-PARTNERSHIP]

---

## 12. Open questions and things not found

**Open questions**
1. **Who founded, owns or runs Prototype, and whether the Mohammed of one MemberCheck review (section 1.3a) is connected to it.** No person is named on the company's pages. The review ties a Mohammed to MemberCheck distribution; no source ties him to Prototype or to Iraq, and whether he runs Prototype is not known. Name searches through the search tool were weak because it dropped first names from queries; LinkedIn staff, job adverts and Facebook posts were not read. [N08] [N10] [N11] [N12] [GA-ARCH]
2. **Legal identity.** The Iraqi company register was not searched. The Arabic legal name, the registration date and any Central Bank licence are not found. Whether the Michigan LLC is related (the official record was not opened) and whether the California address is an office are open. [N10]
3. **Whether any bank uses a Prototype AML or sanctions system.** Not found in 15 searches and the documents read. Several annual reports were scanned images and could not be searched, so this is not a full census. [N12]
4. **Whether "member check" is a Prototype product, or the owner's name for NameScan or MemberCheck screening.** No page puts the words next to Prototype. [N08] [N10]
5. **Where the screening data and matching come from.** "Online AML" was not identified; the SumSub partner list was not checked; DigiCert's partner directory was not reached. [N10] [N11]
6. **How Arabic names are processed, which lists are used, how often they refresh, and how accurate the matching is.** Not published. [N11]
7. **Deployment model, interface and price.** Not published. [N11]
8. **Brochure pages not viewed as images:** 1, 5, 10, 16, 18, 21, 23 and 24. Pages 16, 18, 21 and 23 repeat the text of pages 15, 14, 2 and 4; pages 1, 5, 10 and 24 have no text layer. [N11]
9. **Name matches for four listed banks are unconfirmed:** Kurdistan Islamic Bank (against Kurdistan International Islamic Bank), Regional Commercial Bank - RTB (against Region Trade Bank), First Iraq Bank (against First Iraqi Bank) and Arabia Islamic Bank (against Al-Arabiya Islamic Bank). [N12] [F18]
10. **Differences inside the company's own data:** "5+" against "15+" years; a 2019 domain against a first archived copy of the present site dated 2025-12-23; Karada against Al Yarmouk. [N10] [PIQ-SITE]
11. **A pattern to test, not a finding about Prototype.** File 18 names a local firm (United Engineers Group) as the supplier of a foreign screening product at one bank. No search confirmed it. Local resale of foreign products is therefore a pattern to ask about. [N09] [F18]
12. **The full list of hostnames under prototype-iq.com.** The certificate-log query failed (HTTP 502 on every attempt). [N10]
13. **Who is the reviewer "mohammed habeeb", and where is his firm?** The review gives no country and no company name, and no source ties him to Prototype. Four web searches by the lead on 2026-10-11 found no page that ties him to Prototype, to Iraq or to MemberCheck. The name is common, so a search cannot settle it. Nobody was contacted. [GA-ARCH] [LEAD-SEARCH]

**Searched and not found**
- A locally built Iraqi AML, sanctions-screening, blacklist or KYC system with a matching name, or with a maker called Mohammed: eight Arabic and English searches, and GitHub, npm and GitLab searches. [N07] [N09]
- A "Member Check" or "name check" service from an Iraqi publisher: the App Store Iraq storefront (41 results), the readable part of Google Play, and domain checks. [N08] [N09]
- MemberCheck in Iraq: no Iraqi user, seller or partner in English or Arabic searches or on the vendor's pages; file 35 covers it. [N06] [F35]
- Prototype in startup directories and ecosystems (KAPITA, Takween, Iraq Tech Ventures, Five One Labs, LinkedIn, Crunchbase): not listed in the pages that could be read; The Station's page could not be read. [N08]
- A Central Bank of Iraq notice, licence or event naming Prototype: none found; no Central Bank document was read for this. [N10] [N12]

---

## 13. Sources

All pages were read on 2026-10-11 by the research agents or the lead unless a date is given. [PIQ-SITE] was read in a real browser by the lead. The other pages were read through fetch tools, curl or archive queries, as the working notes record. "(not opened)" means a note saw the page only in a search result or did not open it. Company pages are the company's own words (marketing, not audited). "Secondary" marks third-party pages. Negative searches are cited through the working notes, because a search result is not an address.

**Marks and keys that are not addresses**
- [INFERENCE] Not a source. It marks our own reading of the evidence, not a stated fact.
- [OWNER] The owner's own words, as passed to the research team in the task brief of 2026-10-11 (no address).
- [LEAD-SEARCH] Four web searches by the lead on 2026-10-11 for the MemberCheck reviewer of section 1.3a (no address; search results are not addresses): the name with Prototype's English name and Iraq; the name with CEO, Founder, Prototype, Baghdad and fintech terms (the search tool reported that it dropped the name, so this one is weak); the name with MemberCheck and distributor; and the Arabic form of the name with the Arabic title of Prototype's Facebook page. They found no page that ties the reviewer to Prototype, to Iraq or to MemberCheck. Nobody was contacted.

**Prototype for Information Technology: its own pages**
- [PIQ-SITE] Home page, read in a real browser by the lead on 2026-10-11 (the page is drawn by JavaScript; the bare domain redirects to it): https://site.prototype-iq.com/
- [PIQ-SERVICES] Services page, read by fetch tool in notes 08 and 11 (a model-written summary of a page drawn in the browser): https://site.prototype-iq.com/services
- [PIQ-JS] The site's JavaScript bundle, downloaded and searched in notes 08 and 10: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- [PIQ-ROOT] The bare domain, which redirects to the site (note 08). Also checked there: https://prototype.iq and https://protoype.tech (neither resolves) and https://prototypeiq.com (redirects to a for-sale page): https://prototype-iq.com
- [PIQ-BROCHURE] Brochure linked from the site: 24 pages, mostly images, file created 2025-11-02. Note 11 read its text layer and viewed sixteen pages as images; the lead also read it: https://static.webapp-portal.com/resources/prototypeiq/lp/721526/0.pdf

**Company records, archives and social pages (secondary)**
- [RDAP] Domain record for prototype-iq.com: registered 2019-07-24, expires 2027-07-24, registrar One.com A/S, no registrant shown: https://rdap.verisign.com/com/v1/domain/prototype-iq.com
- [CRT] Certificate log, exact-name query (first certificate 2019-07-24): https://crt.sh/?q=prototype-iq.com&output=json
- [WAYBACK-ROOT] Internet Archive index for the bare domain: http://web.archive.org/cdx/search/cdx?url=prototype-iq.com&output=json&fl=timestamp,original,statuscode,mimetype&limit=60
- [WAYBACK-ALL] Internet Archive index for all names under the domain: http://web.archive.org/cdx/search/cdx?url=*.prototype-iq.com&output=json&fl=timestamp,original,statuscode&limit=60
- [OC-PIQ] OpenCorporates search for the English name (an aggregator's copy of the Michigan record; the official record was not opened): https://opencorporates.com/companies?q=prototype+for+information+technology
- [LI-PIQ] LinkedIn company page, logged-out view, entered by the company: https://www.linkedin.com/company/prototype-for-information-technology/
- [FB-PIQ] Facebook page and its About page; each fetch returned only the page title, and the About page's title gave the Arabic name: https://www.facebook.com/prototypeiq and https://www.facebook.com/prototypeiq/about

**MemberCheck and NameScan, the vendors' own pages (file 35 covers them in full)**
- [NS-HOME] NameScan home page: https://namescan.io/
- [NS-PRICING] NameScan pricing (read with curl in note 12): https://namescan.io/pricing
- [NS-PARTNERSHIP] NameScan partnership page, fetched by the writer (see the fetch record below): https://namescan.io/partnership
- [MC-HOME] MemberCheck home page: https://membercheck.com/
- [MC-IRAQ] MemberCheck Iraq page, "AML/CTF Compliance in Iraq": https://membercheck.com/aml-cft-legislation-in-iraq/
- [MC-PARTNER] MemberCheck partnership page (reseller, white-label and licensing models): https://membercheck.com/partnership/

**Banks, regulators, vendors, press and code sites**
- [AZENTIO-KIIB] Azentio, vendor announcement of AMLOCK at Kurdistan International Islamic Bank: https://www.azentio.com/insights/news-and-announcements/azentio-partners-with-kurdistan-international-islamic-bank-to-strengthen-anti-money-laundering-compliance-with-azentio-amlock
- [ISC-MOSUL] Iraq Securities Commission notice, Mosul Bank 2024 final accounts: https://isc.gov.iq/en/news/2544
- [ISC-IMEIB] Iraq Securities Commission notice, Iraqi Middle East Investment Bank 2024 final accounts: https://isc.gov.iq/en/news/2552
- [RTB-HOME] Home page of RTB (read with curl in note 12): https://rtb.iq/
- [AIIB-HOME] Home page at the address Prototype lists for "Asia Bank", read by note 12 as Asia Iraq Islamic Bank: https://aiib.iq/
- [SUMER-HOME] Home page of Sumer Commercial Bank: https://www.sumerbank.iq/
- [CIHAN-LANDER] Placeholder page at cihanbank.com: https://www.cihanbank.com/lander
- [IMEIB-LANDER] Placeholder page at imeib.com: https://www.imeib.com/lander
- [RUDAW-RTB] Rudaw press item calling the Erbil lender "Region Trade Bank" (secondary; as cited in note 12): https://rudaw.net/english/categories/kurdistan/1078152
- [FINTECH] Fintech Global, 25 November 2021, UR Islamic Bank and Capital Banking Solutions (secondary): https://fintech.global/?p=66393
- [SWONE-NC] SoftwareOne marketplace listing for NameCheck, Cleversoft Group (secondary; the product code at the end of the address is given in note 07): https://platform.softwareone.com/product/namecheck/
- [GH-SSP] GitHub sanctions-screening practice repository with "Prototype" in its name (secondary): https://github.com/Memeos135/vibeCodingPractice_SanctionScreeningPrototype
- [GH-API] GitHub repository search for "membercheck", 26 results, none about AML (secondary): https://api.github.com/search/repositories?q=membercheck
- [GA-ARCH] GetApp, archive copy of the MemberCheck listing captured 12 June 2026, as cited in file 35 (secondary). The review of section 1.3a was found by the independent check in the agents' saved copy of this page and confirmed there by the lead on 2026-10-11: https://web.archive.org/web/2026/https://www.getapp.com/finance-accounting-software/a/membercheck/

**Project files (local)**
- [F14] Vendors: screening engines; lines 1 to 80 read (findings in brief and the yardstick in section 1): /home/haithem/Projects/AKR-TMS/research/14-vendor-screening-engines.md
- [F18] Vendors: MENA and Iraq market; lines 185 to 238 (vendors, integrators) and 458 to 540 (market map, competitor table) read: /home/haithem/Projects/AKR-TMS/research/18-vendor-mena-iraq-market.md
- [F22] Arabic name matching; a cross-reference only, not opened: /home/haithem/Projects/AKR-TMS/research/22-arabic-name-matching.md
- [F30] Security and privacy; its data-classification conclusion is quoted as given in the project brief and in file 35, and the file itself was not opened: /home/haithem/Projects/AKR-TMS/research/30-security-privacy.md
- [F35] MemberCheck and NameScan; read: the findings in brief, sections 3.7, 4.4, 5, 10 and 11, and its sources: /home/haithem/Projects/AKR-TMS/research/35-vendor-membercheck-namescan.md
- [MODULES] Module list; the module headings and the first lines of M9, M10, M11, M13 and M19 read: /home/haithem/Projects/AKR-TMS/docs/MODULES.md
- [STATUS] Plan status table (phase 00 "Done on the build machine"; phases 01 to 31 "Spec ready"); the table rows were checked for this line on 2026-10-11: /home/haithem/Projects/AKR-TMS/docs/plan/status.md
- [N06] Working notes 06, MemberCheck in Iraq and the Middle East: /home/haithem/Projects/AKR-TMS/research/_membercheck/06-iraq-mena.md
- [N07] Working notes 07, name resolution and candidates: /home/haithem/Projects/AKR-TMS/research/_membercheck/07-name-resolution.md
- [N08] Working notes 08, is Prototype a company in Iraq: /home/haithem/Projects/AKR-TMS/research/_membercheck/08-prototype-company.md
- [N09] Working notes 09, locally built Iraqi systems: /home/haithem/Projects/AKR-TMS/research/_membercheck/09-iraqi-local-systems.md
- [N10] Working notes 10, Prototype company and people: /home/haithem/Projects/AKR-TMS/research/_membercheck/10-prototype-people.md
- [N11] Working notes 11, Prototype products: /home/haithem/Projects/AKR-TMS/research/_membercheck/11-prototype-products.md
- [N12] Working notes 12, independent evidence at Iraqi banks: /home/haithem/Projects/AKR-TMS/research/_membercheck/12-prototype-third-party.md

**Fetches made by the writer on 2026-10-11 (1 of 5 allowed; each returns a model-written summary of the page, so quotes come from that summary)**
1. NS-PARTNERSHIP, to settle whether NameScan has a partner programme (note 11 relied on a search summary; note 12 read only the home and pricing pages): it does. It names three arrangements (referrals, distributors, application integrators) and eight partners in Sweden, the UK, Georgia, South Africa, Hong Kong, Estonia, Latin America and France. It names no Iraqi or Middle Eastern partner and does not mention Prototype.
