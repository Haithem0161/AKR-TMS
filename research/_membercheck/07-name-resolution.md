# MemberCheck notes 07 — Name resolution: what else could "prototype member check by a guy named Mohammed" be
Status: done (2026-10-11)

## Summary

- **Best candidate: MemberCheck** (Chatswood, New South Wales, Australia). Its logo reads "MemberCheck, a Nexiant brand", and it sells AML, CTF and KYC screening. Confidence: medium, because the name and the category match. Source: https://membercheck.com/
- **No locally built Iraqi system found.** Eight Arabic and English searches, and GitHub, npm and GitLab searches, found no local name-screening or blacklist system sold to Iraqi banks or exchange companies. See "Not found".
- **No "Prototype" company found.** No company named Prototype, Proto Type, ProType or Protype sells banking or compliance software in Iraq, Jordan, the UAE or Lebanon in the results. See "Not found".
- **Mohammed is not tied to MemberCheck.** No page reached names a Mohammed: not the vendor, CB Insights, LinkedIn, Zoftwarehub or namescan.io. One GitHub account, Memeos135, has a display name that begins with Mohammed and owns a sanctions-screening repository whose name ends in "SanctionScreeningPrototype". It is not named "member check" and has no Iraq link, so confidence is low. Source: https://github.com/Memeos135/vibeCodingPractice_SanctionScreeningPrototype
- **GitHub: 26 repositories match "membercheck"; none is about AML.** Three are Telegram membership checkers, one calls the MemberCheck API for billing, and the rest are club, gym, course or unclear. Source: https://api.github.com/search/repositories?q=membercheck
- **Sound-alike candidates, all low confidence:** NameCheck (Cleversoft Group) could be a mishearing of "name check" [INFERENCE]. Pio-Tech (Jordan) could be a sound-alike of "prototype" [INFERENCE]. World-Check (LSEG) is named by the Central Bank of Iraq. NameScan shares MemberCheck's Chatswood address in CB Insights and on its own footer. Sources: https://platform.softwareone.com/product/namecheck/PCP-5815-8271, https://www.pio-tech.com/reference, https://namescan.io/
- **Parent company unconfirmed.** Zoftwarehub says MemberCheck is a subsidiary of the Neurocom group, now part of Nexiant. The vendor's home page names Nexiant only in its logo text, and the Nexiant directory page returned HTTP 403. Source: https://zoftwarehub.com/ar-ae/products/membercheck/product-details
- **The Iraqi AML deals in the results are with foreign vendors.** Capital Banking Solutions is named in a UR Islamic Bank press item dated 2021-11-25, and Pio-Tech is named in research/18. Source: https://fintech.global/?p=66393
- **Q6:** the single most useful detail is a screenshot of where the owner saw the name, with the web address and logo visible. Source: https://membercheck.com/

## Findings

### Q1. Other products, companies or projects that fit the words "prototype", "member check" and "Mohammed"

- [PRIMARY] The vendor home page carries the logo alt text "MemberCheck, a Nexiant brand" (it appears twice). It names no founder, owner or legal entity, and it has no link to Nexiant or Neurocom. Source: https://membercheck.com/ (raw HTML downloaded and searched on 2026-10-11).
- [PRIMARY] The vendor says it can "screen customers and entities against global sanctions, PEP and watchlist data, with match scoring and an audit trail". Source: https://membercheck.com/
- [PRIMARY] The vendor's home page shows "1,000+ PEP and sanctions watchlists", "60M Scans performed" and "75% Fewer false positives". These are the vendor's own claims. Source: https://membercheck.com/
- [PRIMARY] The home page offers "Talk to our team" and "Book a demo" and does not call the product a prototype or beta. Source: https://membercheck.com/
- [PRIMARY] The home page has a "For our Australian clients" link to a Tranche 2 landing page. Its links also go to a LinkedIn company page, a Facebook page, a Crunchbase page and four app login hosts (app.membercheck.com, app.eu, app.id and app.z, each under membercheck.com). Source: https://membercheck.com/
- [SECONDARY] CB Insights gives MemberCheck the founding year 2007 and the address "7 Railway St Suite 213, Chatswood, New South Wales, 2067, Australia". It lists no founders or executives. It describes MemberCheck as providing "anti-money laundering (AML) and counter-terrorism financing (CTF) compliance solutions". Source: https://www.cbinsights.com/company/membercheck/
- [SECONDARY] The LinkedIn company page (logged-out view) gives the same Chatswood address and the year 2007. It quotes "over 13 years of experience in the industry" and, in paraphrase, describes an AML/CTF and KYC screening platform for medium to large organisations. It names no founder or executive. The page lists four employees without titles: Mei Chua, Robin Deane, Davood Sadeghi and Shaun Thomas. A post calls Harmen Sandhu "our AML & Risk Management Specialist" and Shaun Thomas "our Head of Corporate Development". None of the five is named Mohammed. Source: https://www.linkedin.com/company/membercheck
- [SECONDARY] Zoftwarehub's Arabic page says "MemberCheck هي شركة تابعة لمجموعة Neurocom" (MemberCheck is a subsidiary of the Neurocom group), that the Neurocom group "أصبحت الآن جزءًا من Nexiant" (is now part of Nexiant), that the company was founded in "2007/2008", and that its headquarters are in Chatswood, New South Wales. It names no founder, owner or CEO. This is one directory's claim. Source: https://zoftwarehub.com/ar-ae/products/membercheck/product-details
- [SECONDARY] A search summary says a Capterra listing presents Nexiant as an umbrella for AML and fraud brands, naming NameScan, MemberCheck, GPayments and FraudShield, with Sydney headquarters and a 2023 date. The Capterra page returned HTTP 403 when fetched, so this is unverified. Source: https://www.capterra.com.de/software/1107091/Nexiant
- [SECONDARY] A search summary says a TrustRadius comparison page places NameScan in the Neurocom group. The one TrustRadius page fetched returned HTTP 403, so this is unverified. Source: one of the trustradius.com NameScan comparison pages in the search results, for example https://www.trustradius.com/compare-products/fraudblock-vs-namescan (not opened)
- [PRIMARY] The namescan.io footer gives "Suite 213, 7 Railway St, Chatswood NSW 2067, Australia" and a Chatswood PO Box. The page names no parent company, founder or CEO. Source: https://namescan.io/
- [INFERENCE] MemberCheck and NameScan appear to share one office address, Suite 213, 7 Railway St, Chatswood. Reason: CB Insights and LinkedIn give that address for MemberCheck, and namescan.io gives it for NameScan. Sources: https://www.cbinsights.com/company/membercheck/, https://www.linkedin.com/company/membercheck and https://namescan.io/
- [SECONDARY] The founding years disagree: 2007 (CB Insights, LinkedIn), "2007/2008" (Zoftwarehub) and 2008 (a Capterra vendor profile, per a search summary, not opened). Sources: https://www.cbinsights.com/company/membercheck/ and https://zoftwarehub.com/ar-ae/products/membercheck/product-details
- [SECONDARY] A different US company called Nexiant appears in 2007 and 2011 news items on socaltech.com that name its chief executives. No link to the AML brand is shown. Sources: https://socaltech.com/nexiant_names_ceo/s-0011819.html and https://socaltech.com/nexiant_names_new_ceo/s-0033570.html (search results, not opened)

### Q2. Locally built Iraqi screening systems (Arabic and English searches)

No locally built Iraqi name-screening or blacklist system has been found. Eight Arabic and English searches found none, and the GitHub, npm and GitLab searches found none either (see "Not found"). The leads below are the nearest matches.

- [SECONDARY] UR Islamic Bank (Iraq) "will implement Capital Banking Solutions integrated compliance platform". The press item is dated 2021-11-25. Capital Banking Solutions is a foreign vendor (Lebanese-linked, per research/18 section 9), not a local builder. Source: https://fintech.global/?p=66393
- [SECONDARY] The same press item says the platform has "over 22 installations in Iraq". It does not say how many banks use it. Research/18 records a 2022 vendor release that claims "more than 25 Iraqi retail, commercial and islamic banks". Sources: https://fintech.global/?p=66393 and research/18 line 656 (CBSOL-SUMER).
- [SECONDARY] Iraq keeps a domestic freezing list, and an OpenSanctions entity page republishes part of it (search summary; the page was not opened). A local blacklist tool would need this list as its input. Source: https://opensanctions.org/entities/iq-aml-48efffcc448e58c1893d609f429ee34697d2c1ad
- [SECONDARY] A press statement, quoted in a search summary, says Al-Rafidain Bank's online banking went live once its AML and goAML systems were running. No software vendor is named in the results. Source: https://www.intellinews.com/iraq-s-al-rafidain-bank-launches-online-banking-service-with-anti-money-laundering-systems-376254/ (search result; not opened)
- [SECONDARY] A conference record lists an academic paper on compliance-monitoring applications against money laundering in Iraqi banks. A search summary names its authors as Nagham Hussein Alnama and Youssef Kadhim Kattan; this is not verified on the page. The full text could not be fetched (the portal returned only its site name), so it is unknown whether the paper describes a prototype system. Source: https://portal.arid.my/en/Publications/Details/21976

### Q3. Public code and profile sites: projects named member check or membercheck tied to AML, sanctions or KYC

- [SECONDARY] The GitHub repository search API returns 26 repositories for "membercheck", all on one page (checked 2026-10-11). None of the 26 owner logins is a Mohammed name; the check covered "moh", "muham" and "mhd". No README read contains the name Mohammed or the word "prototype". Source: https://api.github.com/search/repositories?q=membercheck
- [SECONDARY] One repository calls the MemberCheck API for a billing tool: ByteToHex/MemberCheck-API-Pre-Billing-Tool (Java, created 2023-10-30). Its own description says it "Automates pre-billing scan retrieval from MemberCheck API, improving efficiency and reducing manual work." The README never uses the words anti-money laundering, sanctions, KYC, compliance, screening or bank. Its Java app is named "AppAML". [INFERENCE] That name hints at an AML use of the scans, but the README does not say so. Source: https://github.com/ByteToHex/MemberCheck-API-Pre-Billing-Tool
- [SECONDARY] ulxng/membercheck-bot (Go, a Telegram bot) shows no AML text. Its only "aml" and "kyc" strings sit inside base64 image data in the README. Source: https://github.com/ulxng/membercheck-bot
- [SECONDARY] Ten repositories are membership, club, gym, course, game, staff-portal or test-bench tools, judged from their README or description. Examples: a Zurich trail-club membership app (https://github.com/guidosch/membercheck), a Norwegian university sports club's member check (https://github.com/flottorp/tf-membercheck) and a photography studio staff portal (https://github.com/itsukison/Membercheck).
- [SECONDARY] Three repositories by the account prime8489 are small Node.js servers named after member checks: rehan-membercheck2.0-api, MemberCheckerbyrehan and Rehan-membercheck-api (its package name is Rehan-multiplecheck-api). Each package.json describes the project as a "Telegram Multiple Channel Membership Check API", and each server.js has a GET /check-membership route. They check Telegram channel membership, not AML. All three were created in 2025. The repository names include the word "rehan", which is not Mohammed. Sources: https://github.com/prime8489/rehan-membercheck2.0-api, https://github.com/prime8489/MemberCheckerbyrehan, https://github.com/prime8489/Rehan-membercheck-api
- [SECONDARY] The other eleven repositories have no README, a very short README or a template README, so their purpose is not shown. Their README or description contains no AML or sanctions text. Source: https://api.github.com/search/repositories?q=membercheck (README reads through https://api.github.com/repos/{owner}/{repo}/readme)
- [SECONDARY] A GitHub repository search for "member check" with aml returns "Your search did not match any repositories". Source: https://github.com/search?q=%22member+check%22+aml&type=repositories
- [SECONDARY] Repository API searches for membercheck with aml, sanctions or kyc each return zero repositories. The same query limited to README text (in:readme) also returns zero, so no indexed README has both words. Source: https://api.github.com/search/repositories?q=membercheck+aml
- [SECONDARY] Searching GitHub for "member-check" or "member_check" returns 1,614 repositories. The first eight include an Azure AD group check, AWS IAM linting, several WordPress membership plugins and one project with no description (NotFlexCoder/member-checker). None is about AML or sanctions. Source: https://api.github.com/search/repositories?q=member-check
- [SECONDARY] A GitHub search for "prototype" with "sanctions" and "screening" returns five 2026 repositories that describe themselves as sanctions-screening prototypes or projects: Memeos135/vibeCodingPractice_SanctionScreeningPrototype (created 2026-07-13, no description), tatianapodobivskaia-del/tradescreen-ai-v2 (2026-03-27, described as an academic research prototype), subritii/names-to-networks (2026-09-23, described as a prototype risk-intelligence platform), Ark310/blns-sanctions-screening-rag (2026-09-25, offline OFAC-style name screening on a local GPU) and PrettyEvangelin/Compliance-Checker (2026-08-26, described as a small compliance screening prototype). None uses the name member check. The owner login Memeos135 has a display name that begins with "Mohammed", and a profile location of Jeddah, Saudi Arabia. Its repository has no description, was created and last updated on 2026-07-13, and its top-level files include app/, docs/ and a file named session-ses_0a4b.md. [INFERENCE] The name "vibeCodingPractice" suggests a practice project, not a product. The other four logins have no Mohammed display name. Sources: https://api.github.com/search/repositories?q=prototype+sanctions+screening, https://api.github.com/users/Memeos135 and https://github.com/Memeos135/vibeCodingPractice_SanctionScreeningPrototype
- [SECONDARY] A GitHub search for "prototype" with "aml" and "screening" returns one repository, datajav/AML-KYC-Compliance-AI-Agent (created 2026-04-21), described as a multi-agent AI system for the compliance audit lifecycle. It is not described as a prototype. Source: https://api.github.com/search/repositories?q=prototype+aml+screening
- [SECONDARY] A GitHub search for "procheck" with "aml" returns zero repositories. "protocheck" returns eleven repositories, none about AML (protobuf validators, clinical-trial protocol review, a JavaScript type checker and similar). Source: https://api.github.com/search/repositories?q=protocheck
- [SECONDARY] npm has no package matching membercheck (zero results), and PyPI has no project named membercheck (HTTP 404). Sources: https://registry.npmjs.org/-/v1/search?text=membercheck&size=20 and https://pypi.org/pypi/membercheck/json
- [SECONDARY] GitLab search for membercheck returns one public project, fancypantsinc/membercheck (created 2021-07-08, no description). Its setup.py names the package membercheck and describes it as "A plugin for publishing moon extractions, and keeping track of moon scan data." Its README says "Just dont use this please." It is not AML. Source: https://gitlab.com/fancypantsinc/membercheck
- [SECONDARY] A GitHub search for Arabic name matching with sanctions returns one repository, 0xfaisl/mirsad (created 2026-06-02, MIT license, TypeScript, no stars). Its description reads "Adversarial, explainable Arabic/Latin sanctions name-screening" and adds "Open data only." Its README does not mention Iraq, Baghdad, Erbil, Kurdistan or the word prototype. The owner's profile location is Saudi Arabia, and the display name is not Mohammed. The README uses the spellings Mohammed (3), Muhammad (2) and Mohamed (1). Their context was not copied, and they do not identify the author. A GitHub search for "iraq aml screening" returns zero repositories. Sources: https://github.com/0xfaisl/mirsad, https://api.github.com/search/repositories?q=arabic+name+matching+sanctions and https://api.github.com/search/repositories?q=iraq+aml+screening

### Q4. Similar-sounding products in AML and KYC (names ending in Check, or beginning with Pro or Proto)

Names ending in "Check":

- [PRIMARY] MemberCheck. Maker: MemberCheck, "a Nexiant brand" (logo alt text on the vendor home page). Chatswood, New South Wales, Australia (CB Insights). It screens against sanctions, PEP and watchlists, and does AML, CTF and KYC work. Sources: https://membercheck.com/ and https://www.cbinsights.com/company/membercheck/
- [SECONDARY] NameCheck. Maker: Cleversoft Group, listed on the SoftwareOne Marketplace. It screens clients against PEP and sanctions lists for AML and KYC, by API or as standalone software. The listing says it checks "over 180–200" lists. Country and people are not shown. Source: https://platform.softwareone.com/product/namecheck/PCP-5815-8271
- [INFERENCE] "Member check" may be a mishearing of "name check", the usual phrase for name screening, which NameCheck matches. Reason: the two phrases sound alike, and the owner's words were heard, not read. Source: https://platform.softwareone.com/product/namecheck/PCP-5815-8271
- [PRIMARY] World-Check. Maker: LSEG (formerly Refinitiv). Named in the Central Bank of Iraq AML controls of 2025 and sold in Iraq through Iraqna IBTS (2020). Source: research/18 lines 196 and 599 (the LSEG press release is cited there and was not reopened in this run).
- [SECONDARY] Compliance-Checker. A GitHub project by the account PrettyEvangelin, created 2026-08-26. Its author describes it as "a small compliance screening prototype" that normalizes sanctions data from several sources. It is not a company. Source: https://github.com/PrettyEvangelin/Compliance-Checker
- [SECONDARY] memberchecker and member-checker. Several GitHub and GitLab repositories have these names. None is AML (see Q3). Source: https://api.github.com/search/repositories?q=membercheck

Names beginning with Pro or Proto:

- [SECONDARY] ProgressSoft (Amman, Jordan). Payment gateways (PS-GatePay) at nine Iraqi banks in 2019. Not AML. Source: research/18 line 231 (vendor page PS-GATEPAY, not reopened).
- [SECONDARY] protocheck. Eleven GitHub repositories, none about AML (protobuf validators, clinical-trial protocol review, type checkers). Source: https://api.github.com/search/repositories?q=protocheck
- [SECONDARY] No AML or KYC software named Provenir, Protecht, ProKYC, ProType, Proto Type or Protype was found in any search. The closest KYC listings were KYC Management and KYC Global Technologies. See "Not found" for the queries. Source: https://www.capterra.com/p/10030289/KYC-Management/alternatives/ (search result)

Nearby names (they do not match the pattern):

- [PRIMARY] NameScan (Chatswood, New South Wales). Its footer address matches the Chatswood address that CB Insights gives for MemberCheck. Source: https://namescan.io/
- [SECONDARY] Sanctum, a Product Hunt launch of sanctions screening for AML and KYC aimed at startups. A search summary names its developer as "Ali" and says the product is still collecting feedback. Source: https://www.producthunt.com/p/sanctum-5/sanctum-5 (search result; not opened)

### Q5. Candidate scorecard (name, maker, country, people, what it does, evidence, confidence)

- **MemberCheck (confidence: medium).** Maker: MemberCheck, "a Nexiant brand" (logo); Chatswood, New South Wales, Australia. People: no founder or CEO is named on the vendor page, on CB Insights or on the logged-out LinkedIn page. Five staff are named on LinkedIn, and none is Mohammed. What it does: the vendor says it screens "customers and entities against global sanctions, PEP and watchlist data". Evidence: [PRIMARY] https://membercheck.com/; [SECONDARY] https://www.cbinsights.com/company/membercheck/ and https://www.linkedin.com/company/membercheck. Why medium: the name and the category both match; no Mohammed, prototype or Iraq link was found.
- **NameCheck (confidence: low).** Maker: Cleversoft Group (SoftwareOne Marketplace listing). Country and people: not shown. What it does: PEP and sanctions screening for AML and KYC, by API or standalone. Evidence: [SECONDARY] https://platform.softwareone.com/product/namecheck/PCP-5815-8271. Why low: a close sound-alike if "member check" was "name check"; no Mohammed or Iraq link.
- **NameScan (confidence: low).** Maker: NameScan, Chatswood, New South Wales, Australia (namescan.io footer). People: none named on namescan.io. What it does: name screening against watchlists, per a TrustRadius summary (not opened). Its parent is given as the Neurocom group by a search summary; unverified. Evidence: [PRIMARY] https://namescan.io/; [SECONDARY] a TrustRadius NameScan comparison page (search result; the page fetched returned HTTP 403). Why low: a sister brand with a different name.
- **World-Check (confidence: low).** Maker: LSEG (formerly Refinitiv). Country and people: not checked in this run. What it does: risk-screening data (research/18). Evidence: [PRIMARY, as recorded in research/18] research/18 lines 196 and 599. Why low: named by the Central Bank of Iraq; the name ends in "Check", but no source ties it to the owner's words.
- **Memeos135 sanctions-screening repository (confidence: low).** Maker: GitHub account Memeos135; the display name begins with "Mohammed" (flag: Mohammed); location Jeddah, Saudi Arabia. What it does: a Python sanctions-screening project with no description, created and last updated 2026-07-13. Evidence: [SECONDARY] https://github.com/Memeos135/vibeCodingPractice_SanctionScreeningPrototype and https://api.github.com/users/Memeos135. Why low: the only source found where "prototype" and "Mohammed" sit on one account, but the name is not "member check", there is no Iraq link, and the repository name suggests a practice project [INFERENCE].
- **Compliance-Checker, PrettyEvangelin (confidence: low).** Maker: GitHub account PrettyEvangelin (display name is not Mohammed); country not shown. What it does: "a small compliance screening prototype" that normalizes sanctions data. Evidence: [SECONDARY] https://github.com/PrettyEvangelin/Compliance-Checker and https://api.github.com/users/PrettyEvangelin. Why low: "prototype" and "Checker" match, but there is no Mohammed and no Iraq link.
- **Telegram membership-check API, prime8489 (confidence: low).** Maker: GitHub account prime8489 (the repository names contain "rehan"); country not shown. What it does: a Node.js API that checks Telegram channel membership (route GET /check-membership). Not AML. Evidence: [SECONDARY] https://github.com/prime8489/rehan-membercheck2.0-api, https://github.com/prime8489/MemberCheckerbyrehan and https://github.com/prime8489/Rehan-membercheck-api. Why low: a literal "member check" built as small prototype-style servers, but not AML and no Mohammed.
- **MemberCheck API Pre-Billing Tool, "AppAML" (confidence: low).** Maker: GitHub account ByteToHex (display name not checked); country not shown. What it does: retrieves MemberCheck API scan data by quarter for billing. An AML purpose is inferred from the app name only. Evidence: [SECONDARY] https://github.com/ByteToHex/MemberCheck-API-Pre-Billing-Tool. Why low: it shows a third party using the MemberCheck API, but it is not a prototype.
- **Pio-Tech (confidence: low).** Maker: Pio-Tech, Amman, Jordan (research/18). People: a CEO is named in intaj.net press items (Tariq Al-Saffarini, spelled two ways, undated). LinkedIn names Nazmi Al-Saffarini (development director) and Zena Haddadin (senior regional director). None is Mohammed in these sources. What it does: Bank-BI data warehouse, AML, goAML, RBA and a Names Bilingual Translator for Arabic names; 29 Iraqi bank references (research/18). Evidence: [PRIMARY] https://www.pio-tech.com/reference and https://www.pio-tech.com/products/names-bilingual-translator (both as cited in research/18, not reopened); [SECONDARY] https://intaj.net/?p=29821 (search result). Why low: "prototype" could be a sound-alike of "Pio-Tech" [INFERENCE], but no source ties the owner's words to Pio-Tech.
- **Capital Banking Solutions, CapitalCompliance (confidence: low).** Maker: Capital Banking Solutions (Lebanese-linked; Paris-dated releases, per research/18); people not checked. What it does: AML transaction monitoring, name screening, FATCA, CRS and reporting (vendor claims). Iraq: UR Islamic Bank is to implement it (2021 press); Sumer and Trans Iraq Bank per research/18. Evidence: [SECONDARY] https://fintech.global/?p=66393; [PRIMARY, vendor claim as cited in research/18] CBSOL-COMPLIANCE and CBSOL-SUMER (research/18 lines 654 and 656). Why low: a real vendor sold to Iraqi banks, but not local-built and with no name match.
- **mirsad, 0xfaisl (confidence: low).** Maker: GitHub account 0xfaisl (display name is not Mohammed; location Saudi Arabia); country not shown. What it does: an Arabic and Latin sanctions name-screening engine, MIT licence, created 2026-06-02. Evidence: [SECONDARY] https://github.com/0xfaisl/mirsad and https://api.github.com/users/0xfaisl. Why low: an Arabic name-screening project, but no Iraq mention, no "member check" name and no prototype label.
- **Sanctum (confidence: low).** Maker: a Product Hunt launch; the developer is named "Ali" in a search summary; country not shown. What it does: sanctions screening for AML and KYC, aimed at startups; still collecting feedback (search summary). Evidence: [SECONDARY] https://www.producthunt.com/p/sanctum-5/sanctum-5 (search result; not opened). Why low: an early-stage product, but no name match and no Mohammed.
- **Not AML (confidence: very low).** fancypantsinc/membercheck (GitLab) is a moon-scan plugin; ulxng/membercheck-bot is a Telegram bot; the membership and club repositories in Q3 are not AML. Evidence: [SECONDARY] https://gitlab.com/fancypantsinc/membercheck and https://github.com/ulxng/membercheck-bot. Why very low: the name matches, the purpose does not.

### Q6. The single most useful detail the owner could supply

- [INFERENCE] A screenshot of the page or document where the owner saw the name, with the web address and the logo or title text visible. Reason: the logo or address settles the vendor at once. The MemberCheck logo reads "a Nexiant brand", and NameCheck is listed under Cleversoft Group. A "prototype" label on the screen would test that clue. Sources: https://membercheck.com/ and https://platform.softwareone.com/product/namecheck/PCP-5815-8271

## Not found

- Arabic query "نظام فحص الأسماء للبنوك وشركات الصرافة العراق مكافحة غسل الأموال برنامج محلي": no Iraqi name-screening system, vendor or CBI screening rule. The results were explainers and news. Address: https://www.getfocal.ai/ar/knowledgebases/aml-name-screening
- Arabic query "شركة بروتوتايب برمجيات مصرفية مكافحة غسل الأموال العراق": no company called بروتوتايب. The results were Rafidain Bank, K2 Integrity and Qi items. Address: https://www.iraq-businessnews.com/2025/06/24/qi-and-k2-integrity-to-elevate-iraqs-financial-standards/
- Arabic query "شركة عراقية تطور نظام مكافحة غسل الأموال وفحص الأسماء للمصارف وشركات الصرافة": no Iraqi vendor. The Rafidain partner is a US consultancy (K2 Integrity), and the Qi item does not say whether Qi is Iraqi. Address: as in the previous line.
- Arabic query "نموذج أولي نظام فحص الأسماء ومكافحة غسل الأموال رسالة جامعة العراق المصارف": no thesis or prototype. One journal record on customer-acceptance criteria, with no title. Address: https://search.mandumah.com/Record/803018
- Arabic query "ممبر تشيك فحص الأسماء غسل الأموال نظام": no Iraqi use of the term. The results describe MemberCheck through Zoftwarehub, and Czech Republic items (تشيك also means Czech). Address: https://zoftwarehub.com/ar-ae/products/membercheck/overview
- English query "exchange house money transfer software Iraq sanctions list name check developer Baghdad Erbil": no Iraqi developer of exchange-house or remittance software. The results were sanctions listings and Iraqi cross-border rules.
- English query "software company Baghdad Iraq develops anti-money laundering name screening system for banks and exchange companies": none. The nearest match is Capital Banking Solutions (UR Islamic Bank press). Address: https://fintech.global/?p=66393
- English query "Mohammed founder name screening sanctions software startup prototype AML member check": no Mohammed founder found. Sanctum's developer is "Ali" (search summary). Address: https://www.producthunt.com/p/sanctum-5/sanctum-5
- English query "member check prototype Iraq bank Mohammed AML name screening": no member-check prototype and no Mohammed link. Background only on Iraqi AML. Address: https://cbi.iq/news/print_news/388 (search result)
- English query "member check AML sanctions screening demo youtube prototype project": no YouTube demo or prototype write-up. All results are MemberCheck directory listings. Address: https://us.fitgap.com/products/045072/membercheck
- English queries "Nexiant MemberCheck" and "Neurocom Group Australia founder AI risk analysis regulated institutions Nexiant CEO": no founder or owner named. The Capterra Nexiant page returned HTTP 403 (https://www.capterra.com.de/software/1107091/Nexiant). A different Neurocom (telecom software, neurocom.eu) and a US Nexiant (socaltech.com) appeared.
- English query "MemberCheck founder Chatswood Sydney AML KYC company LinkedIn": no founder name on the pages reached (CB Insights and logged-out LinkedIn).
- English query "Proto Type OR ProType software company banking AML compliance Jordan Lebanon UAE Iraq": no ProType or Proto Type company. The results were Fortent, Pio-Tech, FOCAL (Mozn) and other regional vendors. Address: https://sourceforge.net/software/aml/middle-east/
- English query "Provenir Protecht AML sanctions screening software product vendor": no Provenir or Protecht AML product in the results.
- English query "ProKYC AML KYC screening software vendor company": no ProKYC. The closest listings were KYC Management and KYC Global Technologies (RiskScreen). Address: https://www.capterra.com/p/10030289/KYC-Management/alternatives/
- English query "sanctions check PEP check AML KYC screening product name vendor Check software": no other product named Check besides NameCheck and MemberCheck. World-Check was not in the results. Address: https://platform.softwareone.com/product/namecheck/PCP-5815-8271
- GitHub repository API searches: "membercheck" with aml, sanctions or kyc each returned 0 (https://api.github.com/search/repositories?q=membercheck+aml, https://api.github.com/search/repositories?q=membercheck+sanctions, https://api.github.com/search/repositories?q=membercheck+kyc). The aml query limited to README text (in:readme) also returned 0. "%22member check%22 sanctions" returned 0 (https://api.github.com/search/repositories?q=%22member+check%22+sanctions). The web search for "member check" with aml returned no matches (https://github.com/search?q=%22member+check%22+aml&type=repositories).
- GitHub API searches "procheck" with aml and "iraq aml screening" each returned 0 (https://api.github.com/search/repositories?q=procheck+aml and https://api.github.com/search/repositories?q=iraq+aml+screening).
- npm and PyPI: no package named membercheck (https://registry.npmjs.org/-/v1/search?text=membercheck&size=20 returned zero packages; https://pypi.org/pypi/membercheck/json returned HTTP 404).
- Fetches that failed: the Capterra Nexiant page (HTTP 403), a TrustRadius NameScan comparison page (HTTP 403, https://www.trustradius.com/compare-products/namescan-vs-spycloud), the erembusiness article (HTTP 403, https://www.erembusiness.com/technology/mc3p945) and the arid.my paper page (site name only, https://portal.arid.my/en/Publications/Details/21976).
- Gap: the display names of the 24 GitHub owners of the membercheck repositories were not checked. Only their logins were checked, for "moh", "muham" and "mhd", and none matched. The GitHub unauthenticated API limit was reached.
- Gap: a Neurocom Group founder or leader was not found.
- Gap: no Iraqi university thesis or prototype of a name-screening system was found, and no LinkedIn post or YouTube demo of a "member check" prototype was found.

## Contradictions and surprises

- The brief's CB Insights line names the owner as "Member Check Pty Ltd". The vendor's site never uses "Pty Ltd" and says "a Nexiant brand". No page reached gives a legal name. Sources: https://membercheck.com/ and https://www.cbinsights.com/company/membercheck/
- Founding year: 2007 (CB Insights and LinkedIn), "2007/2008" (Zoftwarehub) and 2008 (a Capterra profile, per a search summary). Sources: https://www.cbinsights.com/company/membercheck/, https://www.linkedin.com/company/membercheck and https://zoftwarehub.com/ar-ae/products/membercheck/product-details
- Parent company: Zoftwarehub (Arabic) says MemberCheck is a subsidiary of the Neurocom group, now part of Nexiant. The vendor's home page links to neither Nexiant nor Neurocom and mentions Nexiant only in its logo text. Sources: https://zoftwarehub.com/ar-ae/products/membercheck/product-details and https://membercheck.com/
- The Chatswood address appears on namescan.io, in CB Insights and on LinkedIn, but not in the membercheck.com HTML (searched 2026-10-11). Sources: https://namescan.io/, https://www.cbinsights.com/company/membercheck/ and https://membercheck.com/
- The brief's TrustRadius link is a development host (tr-web.dev.trustradius.com). This run did not use it.
- The brief's api.membercheck.com does not appear in the vendor's home page links. The links show app login hosts only (app.membercheck.com, app.eu, app.id and app.z). Source: https://membercheck.com/
- Iraqi deployment counts for Capital Banking Solutions differ: "over 22 installations in Iraq" (fintech.global, dated 2021-11-25) and "more than 25 Iraqi retail, commercial and islamic banks" (a 2022 vendor release, as cited in research/18). Sources: https://fintech.global/?p=66393 and research/18 line 656.
- "Nexiant" is also the name of a US company in 2007 and 2011 news items on socaltech.com. No link to the AML brand is shown. Sources: https://socaltech.com/nexiant_names_ceo/s-0011819.html and https://socaltech.com/nexiant_names_new_ceo/s-0033570.html
- "member check" appears in 26 GitHub repositories. None is AML. Three are Telegram membership checkers (prime8489), and one calls the MemberCheck API (ByteToHex). The app name "AppAML" is not evidence of AML by itself. Source: https://api.github.com/search/repositories?q=membercheck
- The ulxng bot's "aml" and "kyc" matches sit inside base64 image data, so they are false matches. Source: https://github.com/ulxng/membercheck-bot
- Each of the three prime8489 repositories lists a .env file in its file listing. The file was not opened. Source: https://api.github.com/repos/prime8489/Rehan-membercheck-api/contents
- Pio-Tech's CEO name is spelled two ways in the press items, and their dates are not shown. Source: https://intaj.net/?p=29821

## Sources

- https://membercheck.com/
- https://www.cbinsights.com/company/membercheck/
- https://www.linkedin.com/company/membercheck
- https://zoftwarehub.com/ar-ae/products/membercheck/product-details
- https://zoftwarehub.com/ar-ae/products/membercheck/overview (search result)
- https://www.capterra.com.de/software/1107091/Nexiant (HTTP 403 when fetched)
- https://www.capterra.com/p/154148/MemberCheck/reviews/ (search result, not opened)
- https://www.crunchbase.com/organization/membercheck (linked from the vendor home page; not opened)
- https://www.capterra.com/p/10030289/KYC-Management/alternatives/ (search result)
- https://namescan.io/
- https://www.trustradius.com/compare-products/fraudblock-vs-namescan (search result, not opened)
- https://www.trustradius.com/compare-products/namescan-vs-spycloud (HTTP 403 when fetched)
- https://us.fitgap.com/products/045072/membercheck (search result)
- https://platform.softwareone.com/product/namecheck/PCP-5815-8271
- https://socaltech.com/nexiant_names_ceo/s-0011819.html (search result, not opened)
- https://socaltech.com/nexiant_names_new_ceo/s-0033570.html (search result, not opened)
- https://www.producthunt.com/p/sanctum-5/sanctum-5 (search result, not opened)
- https://sourceforge.net/software/aml/middle-east/ (search result)
- https://www.getfocal.ai/ar/knowledgebases/aml-name-screening (search result)
- https://search.mandumah.com/Record/803018 (search result)
- https://www.iraq-businessnews.com/2025/06/24/qi-and-k2-integrity-to-elevate-iraqs-financial-standards/ (search result)
- https://fintech.global/?p=66393
- https://www.intellinews.com/iraq-s-al-rafidain-bank-launches-online-banking-service-with-anti-money-laundering-systems-376254/ (search result, not opened)
- https://portal.arid.my/en/Publications/Details/21976 (returned only the site name)
- https://www.erembusiness.com/technology/mc3p945 (HTTP 403 when fetched)
- https://opensanctions.org/entities/iq-aml-48efffcc448e58c1893d609f429ee34697d2c1ad (search result, not opened)
- https://cbi.iq/news/print_news/388 (search result, not opened)
- https://intaj.net/?p=29821 (search result, not opened)
- https://www.pio-tech.com/reference (cited in research/18, not reopened)
- https://www.pio-tech.com/products/names-bilingual-translator (cited in research/18, not reopened)
- https://github.com/search?q=membercheck&type=repositories (WebFetch; first 10 of 26)
- https://github.com/search?q=%22member+check%22+aml&type=repositories (WebFetch)
- https://api.github.com/search/repositories?q=membercheck (all 26 results)
- https://api.github.com/search/repositories?q=membercheck+aml
- https://api.github.com/search/repositories?q=membercheck+sanctions
- https://api.github.com/search/repositories?q=membercheck+kyc
- https://api.github.com/search/repositories?q=membercheck+aml+in:readme
- https://api.github.com/search/repositories?q=%22member+check%22+sanctions
- https://api.github.com/search/repositories?q=member-check
- https://api.github.com/search/repositories?q=prototype+sanctions+screening
- https://api.github.com/search/repositories?q=prototype+aml+screening
- https://api.github.com/search/repositories?q=procheck+aml
- https://api.github.com/search/repositories?q=protocheck
- https://api.github.com/search/repositories?q=arabic+name+matching+sanctions
- https://api.github.com/search/repositories?q=iraq+aml+screening
- https://api.github.com/repos/{owner}/{repo}/readme (README reads for the 26 membercheck repositories)
- https://api.github.com/users/Memeos135
- https://api.github.com/users/PrettyEvangelin
- https://api.github.com/users/0xfaisl
- https://api.github.com/repos/prime8489/Rehan-membercheck-api/contents
- https://github.com/guidosch/membercheck
- https://github.com/itsukison/Membercheck
- https://github.com/flottorp/tf-membercheck
- https://github.com/nhsccpgh/msr-membercheck
- https://github.com/Memeos135/vibeCodingPractice_SanctionScreeningPrototype
- https://github.com/PrettyEvangelin/Compliance-Checker
- https://github.com/0xfaisl/mirsad
- https://github.com/ByteToHex/MemberCheck-API-Pre-Billing-Tool
- https://github.com/ulxng/membercheck-bot
- https://github.com/prime8489/rehan-membercheck2.0-api
- https://github.com/prime8489/MemberCheckerbyrehan
- https://github.com/prime8489/Rehan-membercheck-api
- https://gitlab.com/fancypantsinc/membercheck
- https://gitlab.com/api/v4/projects?search=membercheck&simple=true&per_page=20
- https://registry.npmjs.org/-/v1/search?text=membercheck&size=20
- https://pypi.org/pypi/membercheck/json
- /home/haithem/Projects/AKR-TMS/research/18-vendor-mena-iraq-market.md (read-only: lines 185 to 238; source tags at lines 478 to 658; research/18 line 196, 231, 599, 654 and 656 are cited above)
