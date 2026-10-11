# MemberCheck notes 08 — Is "Prototype" a company or product in Iraq
Status: done (2026-10-11), with the verification gaps listed in the Summary

## Summary
- Direct answer: the only company named Prototype with an Iraq link is Prototype IQ (also written Prototype Tech). It says it is based in Baghdad, offers software development and cybersecurity, and claims AML and sanctions-screening products and bank clients. Its own site is the only source for most of this. Address: https://site.prototype-iq.com/ [PRIMARY]
- What is verified in the company's code: the page title, the sentence "We proudly serve major financial institutions across Iraq and the Middle East", and a list of partner entries (DigiCert, Namescan, SumSub, Auth0, TeleSign, Online AML). I downloaded the bundle and read these directly.
- What is not verified: the AML product (AGIES AI for AML/CFT), the bank client list (28 banks and 2 insurers), the Baghdad and California addresses and the name "Prototype Tech". These come only from a rendered-page read, not from the code. No bank, regulator, press item, directory, LinkedIn or Crunchbase profile confirms any of it.
- Member check: not found. No product called Member Check, or close to it, appears on the company's site, in the App Store Iraq storefront, in the readable Google Play results or in any search. The closest app name is an event check-in app (MemberScan) from DACdb, LLC.
- Founder named Mohammed: not found. The company's site names no people. The one Mohamed Al Hakim who came up is a co-founder of the venture fund EQIQ, and no source ties him to Prototype.
- Directories and ecosystems: none of the directory pages I could read (KAPITA's homepage, the 2021 Takween participant list, the iraqtech.io startup news page) lists Prototype. The Station's listing could not be read.
- Reading (b), a developer named Mohammed with a screening, KYC or AML system: not found. The search tool dropped the first name from my queries, so this check is weak.
- Confidence that the owner meant Prototype IQ: medium that it is the Prototype company the owner heard of, because it is the only Iraq-linked company with that name that claims AML and KYC products. Low that it matches the full description, because no member-check product and no Mohammed founder is supported by any source.
- Gaps left open: the Iraqi company register and the domain owner (WHOIS) for prototype-iq.com were not checked, and the rest of the bank list needs a bank-side check. The owner could say which bank showed them the system, which would settle the question.

## Findings

### Q1. Companies, startups, agencies or teams named Prototype in Iraq or selling to Iraq
- Direct answer: one company found, Prototype IQ (also written Prototype Tech). It says it is based in Baghdad and offers software development and cybersecurity to financial institutions. This rests on its own site. No other company named Prototype was found in Iraq, or selling to Iraq from Jordan, the UAE, Lebanon or Turkey. [PRIMARY] Address: https://site.prototype-iq.com/
- Prototype IQ presents itself as a software development and cybersecurity firm for banks and financial institutions in Iraq and the Middle East. Quote from the site code: "We proudly serve major financial institutions across Iraq and the Middle East, delivering mission-critical technology solutions." Addresses: https://site.prototype-iq.com/ (reached by redirect from https://prototype-iq.com), and the site's JavaScript bundle https://site.prototype-iq.com/assets/index-BVHNoGNr.js (checked by download). [PRIMARY] Quote (page title, checked in the downloaded HTML): "Prototype IQ - Elite Software Development & Cybersecurity Solutions".
- Unverified: a page summary says the company is also called "Prototype Tech" and "Prototype for Information Technology" in its About section. The static code does not contain either name, so treat both as unconfirmed. Address: https://site.prototype-iq.com/. [PRIMARY]
- The domains prototype.iq and protoype.tech do not resolve, and prototypeiq.com redirects to a for-sale page. The live site is site.prototype-iq.com. Addresses: https://prototype.iq; https://protoype.tech; https://prototypeiq.com; https://prototype-iq.com. [PRIMARY] (direct DNS and HTTP results). See Not found.
- The contact area of the rendered page reads: "IRAQ Baghdad ,Karada / California , USA, 15442 Ventura Blvd., Ste 201-1772". This text is not in the static code, so it is unverified. Address: https://site.prototype-iq.com/. [PRIMARY]
- Services on the rendered services page: Compliance & Audit Excellence, Core Banking Systems, SWIFT Integration Services, Advanced Cybersecurity, FinTech Development and Managed Security Provider. This text is not in the static code, so it is unverified. Address: https://site.prototype-iq.com/services. [PRIMARY]
- Other companies named Prototype appear in the search results, but none is linked to Iraq or to Iraqi banks in those results. Prototype Capital is a US venture firm in directory listings (https://altss.com/profile/prototype-capital-san-francisco; https://linkedin.com/company/prototype-capital). A directory profile for Prototype (https://www.datanyze.com/companies/prototype/551832198) places it in Connecticut, per the search summary. [SECONDARY]

### Q2. Links to banks, exchanges, payments, KYC, AML, sanctions or name or member checks
- Direct answer: yes, by the company's own account only. Prototype IQ says it serves major financial institutions across Iraq and the Middle East, and its rendered page names 28 banks. Its homepage text claims AML and sanctions screening, though its services page does not list them. It also lists KYC and sanctions vendors as partners, and its rendered page describes banking integrations for financial institutions and payment providers (unverified). No bank, regulator or third party in the results confirms any of this. Nothing on the site is called member check or name check. [PRIMARY] Address: https://site.prototype-iq.com/
- Unverified: the rendered page (read through the fetch tool, not in the static code) says: "Our portfolio includes AGIES AI for AML/CFT, ProtoAudit, ProtoBot, Fraud Intelligence solutions, Digital Identity platforms," (the tool cut the sentence off). Unverified. Address: https://site.prototype-iq.com/. [PRIMARY]
- Unverified: the rendered page (read through the fetch tool) shows the heading "Trusted by Leading Banks" (that heading is in the static code) and 31 entries (one name is listed twice): 28 distinct banks and 2 insurers (Al Batek Insurance Company and Asia Gate Insurance). The banks include IS Bank, Ziraat Bank, Vakif Bank, Rasheed Bank, Agricultural Bank, Nahreen Islamic Bank, Cihan Bank, Asia Bank, Rafidian Bank, Erbil Bank, Mosul Bank, Misk Islamic Bank, Sumer Commercial Bank, First Iraq Bank and Regional Commercial Bank (RTB). The static code has no bank names. It loads a "clients" table at runtime, so the names are not in the downloaded code. Self-reported, and not checked with the banks. Address: https://site.prototype-iq.com/. [PRIMARY]
- The company's own site lists six vendor or partner entries: DigiCert, Namescan, SumSub, Auth0, TeleSign and Online AML. The Namescan entry reads: "Global sanctions and PEP screening". Address: https://site.prototype-iq.com/assets/index-BVHNoGNr.js (the site's JavaScript bundle, checked by download). [PRIMARY]
- The rendered services page does not show AML, KYC, screening, watchlist or member-check services, although the rendered homepage text names AML and sanctions screening. This conflicts with the homepage. Address: https://site.prototype-iq.com/services. [PRIMARY] Unverified in the static code.
- [INFERENCE] The partner entry for Namescan lists the services Sanctions Screening, PEP Lists and Watchlist Management. If Prototype offers a name check, it may rely on Namescan, but the site does not say so. Reason: the screening services sit under Namescan, and the site names no name-check product of its own. Address: https://site.prototype-iq.com/assets/index-BVHNoGNr.js

### Q3. Is a founder, owner or lead developer named Mohammed?
- Direct answer: no. No founder, owner or developer of Prototype IQ is named in any source found, and no Mohammed or Mohamed is tied to it. [PRIMARY] Address: https://site.prototype-iq.com/
- The company's own site names no founder, owner, CEO or developer, in the static code or on the rendered pages. Address: https://site.prototype-iq.com/. [PRIMARY]
- A press profile of Mohamed Al Hakim (spelled Mohammed in some outlets) says he is "co-founder of Abu Dhabi-based venture capital fund and venture builder EQIQ." The profile does not mention Prototype, so no link to the company is shown. Address: https://web.archive.org/web/20250720031155/https://meatechwatch.com/2025/06/05/mohamed-al-hakim-driving-iraqs-digital-entrepreneurship-revolution/ (archived copy; the live page returned HTTP 403). [SECONDARY]

### Q4. App stores, web-app listings or domain names showing "Member Check" from an Iraqi publisher
- Direct answer: no. No app or domain named Member Check, or close to it, from an Iraqi publisher was found. The two closest names are check-in apps, neither shown as Iraqi: MemberScan (DACdb, LLC) and Member Lessons: Check-in. [PRIMARY] Address: https://itunes.apple.com/search?term=member+check&country=iq&entity=software
- No app named Member Check, or close to it, from an Iraqi publisher was found in the App Store Iraq storefront. Of 41 results for "member check", the two closest names are "MemberScan - Member Minder Pro" (DACdb, LLC) and "Member Lessons: Check-in" (a Chinese-language seller name). Neither is named Member Check, and neither is shown as Iraqi. The MemberScan description reads: "provides a no frills approach to managing the check in process for your DACdb and iMembersDB events." That is an event check-in app, not a screening product. I did not read the description of Member Lessons: Check-in. Address: https://itunes.apple.com/search?term=member+check&country=iq&entity=software [PRIMARY]
- Google Play search for "member check" (country IQ) shows no app titled Member Check in the readable part of the results. The page was cut off before the end. Address: https://play.google.com/store/search?q=member%20check&c=apps&gl=IQ [PRIMARY]
- Domains checked: prototype.iq does not resolve, prototype-iq.com is the company site, and prototypeiq.com is a for-sale page. None of them shows a Member Check product. Addresses: https://prototype.iq; https://prototype-iq.com; https://prototypeiq.com. [PRIMARY]

### Q5. Iraqi startup directories and ecosystems (KAPITA, The Station, Iraq Tech Ventures, Five One Labs, Takween, LinkedIn, Crunchbase)
- Direct answer: no. None of the directories or ecosystem pages checked lists Prototype or Prototype IQ. The Station's listing could not be read, and the Five One Labs annual report was not opened. [INFERENCE] Based on the page reads and searches listed here and under Not found. Addresses: https://kapita.iq/; https://iraqtech.io/12-iraqi-and-kurdish-startups-in-takween-accelerator/; https://www.crunchbase.com/hub/iraq-companies
- KAPITA: its homepage names no portfolio companies, startups, members or alumni, and it does not mention Prototype. It lists four partners: AsiaCell, GIZ, the Netherlands Embassy and the World Bank. Address: https://kapita.iq/ [PRIMARY]
- Takween Accelerator: a 2021 iraqtech.io article lists 12 participants: Dakakenna, One Connect, FAM Academy, FIX IT, IOT KIDS, KURDCOIN, Mr Erbil, Péleen's Organic, Rescue Project, Shopyobrand, SmartMed and The Workshop. Prototype is not among them. Address: https://iraqtech.io/12-iraqi-and-kurdish-startups-in-takween-accelerator/ (article dated 17 January 2021). [SECONDARY]
- No Prototype or Prototype IQ entry turned up in the Five One Labs, Iraq Tech Ventures, LinkedIn or Crunchbase results. The Station's listing page returned HTTP 403 (see Not found). [INFERENCE] Based on the searches and page reads listed under Not found. Address: https://www.crunchbase.com/hub/iraq-companies

### Q6. Reading (b): public trace of an Iraqi developer named Mohammed with a screening, KYC or AML system
- Direct answer: not found. Topic searches in English and Arabic, with GitHub, YouTube, university and hackathon angles, found no Iraqi developer's name-screening, KYC or AML project. The search tool dropped the first name from the queries, so the test for a developer named Mohammed is weak. [INFERENCE] Based on the topic-only searches listed under Not found. The closest Iraq-focused hits were vendor pages, not developer projects (for example https://shuftipro.com/iraq).

### Q7. Ranked candidates
- 1. Prototype IQ (also written Prototype Tech). Maker: the company, with no founder named. People: none named on the site. What it does, by its own account: software development and cybersecurity for financial institutions, with a claimed AML and CFT product (AGIES AI for AML/CFT, from the rendered page, unverified) and a partner entry for Namescan. Evidence: its own site [PRIMARY]; no third-party confirmation found. Confidence that it is the company the owner heard of: medium. Confidence that it matches the full description (a member-check system built by a Mohammed): low, because no member-check product and no Mohammed founder are supported. Address: https://site.prototype-iq.com/ [PRIMARY]
- 2. MemberScan - Member Minder Pro. Maker: DACdb, LLC. What it does: an event check-in app for DACdb and iMembersDB events, not a name check. Evidence: App Store record [PRIMARY]. Confidence: low (similar name only; not shown as Iraqi). Address: https://itunes.apple.com/search?term=member+check&country=iq&entity=software [PRIMARY]
- 3. Other companies named Prototype (Prototype Capital, and the company at aka-prototype.com). Maker: US companies, per directory snippets. What they do: Prototype Capital is a venture firm in directory listings; the other is not described in the snippets read. Evidence: directory and profile snippets [SECONDARY]. Confidence: none (no Iraq link found). Addresses: https://altss.com/profile/prototype-capital-san-francisco; https://www.datanyze.com/companies/prototype/551832198
- 4. AEGIS hackathon entry (different spelling, close to AGIES). Maker: a hackathon team, not named in the snippet. What it does: a hackathon submission described as an automated money-laundering investigation. Evidence: hackathon page snippet [SECONDARY]. Confidence: none (not shown as Iraqi; different spelling). Address: https://lablab.ai/ai-hackathons/band-of-agents-hackathon/agenticdeveloper/aegis-autonomous-financial-crime-investigation
- 5. Mohamed Al Hakim (spelled Mohammed in some outlets), co-founder of EQIQ. Evidence: press profile [SECONDARY]. No source read ties him to Prototype. Confidence: none (ruled out). Address: https://web.archive.org/web/20250720031155/https://meatechwatch.com/2025/06/05/mohamed-al-hakim-driving-iraqs-digital-entrepreneurship-revolution/

## Not found
- Q1 domain prototype.iq: WebFetch returned "getaddrinfo ENOTFOUND prototype.iq", so the name did not resolve from this environment. Address tried: https://prototype.iq
- Q1 domain protoype.tech (spelling from the brief): WebFetch returned "getaddrinfo ENOTFOUND protoype.tech". Address tried: https://protoype.tech
- Q1 search "Prototype" software company Iraq Baghdad Erbil (WebSearch, standard): no result names a company called Prototype in Iraq. The results were unrelated firms with no Prototype name.
- Q1 search بروتوتايب العراق شركة برمجيات (WebSearch, standard): no Arabic-language result and no result naming بروتوتايب. Results were English and unrelated (US prototyping firm pages, a Microsoft distributor story, a game studio listing).
- Q1 domain prototypeiq.com: WebFetch was redirected to a GoDaddy domain-for-sale URL (forsale.godaddy.com/forsale/prototypeiq.com), which returned HTTP 403 when fetched. The company does not appear to use this domain. Address tried: https://prototypeiq.com [INFERENCE] The redirect shows the domain is parked for sale.
- Q4 search "Member Check" Iraq bank OR Iraqi app KYC name check (WebSearch, standard): no result ties a product called Member Check to an Iraqi bank, app or publisher. Results were vendor KYC pages for Iraq and one bank app listing.
- Q2 search "Prototype" fintech OR AML OR KYC OR sanctions screening Iraq bank software company (WebSearch, standard): no result names Prototype. The only Iraq bank AML hit was about Capital Banking Solutions, which is already on the known vendor list.
- Q3 static-code check: the downloaded site code (https://site.prototype-iq.com/assets/index-BVHNoGNr.js) has no founder, CEO, director or team text, and no "Mohammed" or "Mohamed". The homepage has no team section in the static code.
- Q1 archive check: the Internet Archive snapshot for 2026 of the company homepage returned HTTP 404. Address tried: https://web.archive.org/web/2026/https://site.prototype-iq.com/
- Q3 rendered-page check (WebFetch of https://site.prototype-iq.com/): the page names no founder, CEO, director or team member, and its About section shows no people.
- Q2 search "AGIES" AML Iraq OR "ProtoAudit" OR "ProtoBot" Prototype (WebSearch, standard): no result mentions AGIES, ProtoAudit, ProtoBot or Iraq. The only close hit was an unrelated hackathon entry spelled AEGIS. No third party confirms these product names so far.
- Q5 LinkedIn and Crunchbase check (WebSearch, standard, "Prototype IQ" Iraq company profile LinkedIn Crunchbase): no LinkedIn or Crunchbase profile for Prototype IQ came back. The only Crunchbase hit was a generic hub of Iraq companies (https://www.crunchbase.com/hub/iraq-companies).
- Q4 App Store, Iraq storefront, term "prototype iq" (https://itunes.apple.com/search?term=prototype+iq&country=iq&entity=software): resultCount 0, so no app is listed under that name. [PRIMARY]
- Q4 App Store, Iraq storefront, term "member check" (https://itunes.apple.com/search?term=member+check&country=iq&entity=software): resultCount 41. The first 10 results are unrelated apps (Instagram, Facebook, WhatsApp and others). The full list was checked by download (see Q4).
- Q1 social pages: none found. The only Facebook link in the site code is a template placeholder (https://facebook.com/yourpage). Address checked: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- Q1 Arabic search بروتوتايب تكنولوجيا المعلومات بغداد الكرادة (WebSearch, standard): no result names Prototype (بروتوتايب) in Karrada or Baghdad.
- Q1 search "15442 Ventura Blvd" Prototype company (WebSearch, standard): no result mentions that address. The other Prototype-named companies in the results are US companies with no Iraq link.
- Q3 Mohamed Al Hakim article (https://meatechwatch.com/2025/06/05/mohamed-al-hakim-driving-iraqs-digital-entrepreneurship-revolution/): WebFetch and curl both returned HTTP 403. The archived copy was read and does not mention Prototype. See Q3.
- Q2 search "Prototype IQ" bank Iraq partnership OR contract OR launch OR go-live (WebSearch, standard): no result names Prototype IQ. The bank deals found are with other vendors (for example ICS Banks, BML Istisharat and Temenos).
- Q1 region search "Prototype" software company Jordan OR UAE OR Turkey OR Lebanon sells to Iraqi banks AML KYC (WebSearch, standard): no company named Prototype. The closest vendors were Azentio and Capital Banking Solutions. Capital Banking Solutions is already on the known vendor list. Address: https://ibsintelligence.com/ibsi-news/ur-islamic-bank-selects-capitalcompliance-to-support-its-clients-in-iraq
- Q5 KAPITA, The Station and Iraq Tech Ventures search (WebSearch, standard): no result mentions Prototype IQ or Prototype Tech. Results described KAPITA (https://kapita.iq/about) and The Station (https://superscout.co/incubator/the-station). Iraq Tech Ventures appeared only as an affiliation, and no standalone listing was found.
- Q5 Five One Labs, Takween and Iraq Tech Ventures search (WebSearch, standard): no result mentions Prototype. Five One Labs results were about the incubator itself (https://millercenterglobal.org/?p=67466).
- Q5 Takween search (WebSearch, standard): the accelerator was based at AUIS in Sulaimani. PitchBook labels it inactive (https://pitchbook.com/profiles/investor/482585-05). No Prototype in the results.
- Q5 directory page read: https://iraqtech.io/startups/ is a paginated news list (pages 1 to 7). Page 1 has no Prototype entry. Pages 2 to 7 were not read.
- Q6 English search "Mohammed Iraqi developer AML OR KYC OR name screening OR member check system project Iraq bank" (WebSearch, standard): topic-only results. The search tool's own summary says it dropped the first name from the query, so this search could not test for a developer named Mohammed. Results were about UR Islamic Bank and vendor pages.
- Q6 Arabic search for a graduation project on a name screening system and AML in Iraq (WebSearch, standard): no graduation project matched. Results were background only. The tool's summary again says it dropped the developer's first name.
- Q5 Iraq Tech Ventures: no standalone listing came up. The only mention was a 2021 article naming a Mohammed as an affiliate of Iraq Tech Ventures. No link to Prototype appears in the results. Address: https://www.iraq-businessnews.com/2021/11/02/new-venture-fund-focused-on-iraqi-startups-announced/
- Q5 Five One Labs: the search results are about the incubator and its annual reports. The 2025 annual report was not opened, so its list of companies is unchecked. Address: https://www.iraq-businessnews.com/2026/09/12/five-one-labs-2025-annual-report/
- Government registers: the Iraqi companies registry, the Kurdistan Region registry and the California Secretary of State business search were not searched. The California search needs a web form, which this task does not allow. Domain ownership (WHOIS) for prototype-iq.com was not checked.
- Q5 The Station (https://superscout.co/incubator/the-station): WebFetch returned HTTP 403, curl returned HTTP 403, and the 2025 archive copy returned HTTP 404. The listing could not be read, so The Station's list is unchecked.
- Q1 search "Prototype for Information Technology" Baghdad OR Karada company (WebSearch, standard): no company with that name came up. The results were other Baghdad IT firms, none named Prototype.
- Q6 English search, topic only: no Iraqi developer's KYC, AML or name-screening project was found. No YouTube demo came back. Vendors and foreign open-source projects were the only results.
- Q3 search "Prototype IQ" Baghdad founder CEO software cybersecurity (WebSearch, standard): no result mentions Prototype IQ. The people named in the results have no link to it.
- Q3 search "Mohamed Al Hakim" Iraq digital entrepreneurship company founder CEO (WebSearch, standard): the results tie him to EQIQ and Fedshi, not to Prototype. The archived article mentions EQIQ only.
- Q1 search "بروتو تايب" العراق شركة برمجيات أمن سيبراني مصارف (WebSearch, standard, the last search in the budget): no result names بروتو تايب or Prototype. The results were English and unrelated.

## Contradictions and surprises
- Services conflict: the rendered homepage text names AML and sanctions screening services, but the rendered services page lists none of AML, KYC or screening. Both pages come from the company's own site.
- Runtime content: the homepage loads its client list from a database table ("clients"), and the product list and the About text are not in the static code either. So the bank names, product names and address come only from a rendered-page summary. Treat them as unverified until another route confirms them. [INFERENCE] The static code queries a "clients" table at runtime.
- Preview build: the homepage HTML carries the marker data-artifact-kind="preview_commit_sha". This suggests a preview deployment, not necessarily a production site. [INFERENCE] Reason: the marker is on the page itself.
- Self-reported figures: the site shows "PCI DSS Certified" with the value "100%", "SWIFT CSP Ready" with "24/7", and "Uptime SLA" with "99.9%". These are the company's own claims and are not independently checked.
- Domains: prototype.iq does not resolve; prototype-iq.com redirects to site.prototype-iq.com; prototypeiq.com is a for-sale parked page. The live company address appears to be site.prototype-iq.com.
- Search tool: two WebSearch summaries said they had dropped the developer's first name from my queries. Name-based checks through this tool are therefore weak.
- Name clash: "Prototype" is a common name. Several US companies use it, and none is linked to Iraq in the results.
- California address: the one on the rendered page could not be matched to any company in search.

## Sources
- https://site.prototype-iq.com/
- https://prototype-iq.com
- https://prototype.iq
- https://protoype.tech
- https://prototypeiq.com
- https://forsale.godaddy.com/forsale/prototypeiq.com
- https://fintech.global/?p=66393
- https://shuftipro.com/iraq
- https://knowyourcustomer.com/developers/coverage/iraq/
- https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- https://web.archive.org/web/2026/https://site.prototype-iq.com/
- https://site.prototype-iq.com/services
- https://itunes.apple.com/search?term=prototype+iq&country=iq&entity=software
- https://itunes.apple.com/search?term=member+check&country=iq&entity=software
- https://meatechwatch.com/2025/06/05/mohamed-al-hakim-driving-iraqs-digital-entrepreneurship-revolution/
- https://altss.com/profile/prototype-capital-san-francisco
- https://linkedin.com/company/prototype-capital
- https://www.datanyze.com/companies/prototype/551832198
- https://lablab.ai/ai-hackathons/band-of-agents-hackathon/agenticdeveloper/aegis-autonomous-financial-crime-investigation
- https://www.crunchbase.com/hub/iraq-companies
- https://web.archive.org/web/20250720031155/https://meatechwatch.com/2025/06/05/mohamed-al-hakim-driving-iraqs-digital-entrepreneurship-revolution/
- https://play.google.com/store/search?q=member%20check&c=apps&gl=IQ
- https://iraqtech.io/startups/
- https://kapita.iq/about
- https://superscout.co/incubator/the-station
- https://millercenterglobal.org/?p=67466
- https://pitchbook.com/profiles/investor/482585-05
- https://ibsintelligence.com/ibsi-news/ur-islamic-bank-selects-capitalcompliance-to-support-its-clients-in-iraq
- https://iraqtech.io/12-iraqi-and-kurdish-startups-in-takween-accelerator/
- https://kapita.iq/
- https://www.iraq-businessnews.com/2021/11/02/new-venture-fund-focused-on-iraqi-startups-announced/
- https://www.iraq-businessnews.com/2026/09/12/five-one-labs-2025-annual-report/
- https://facebook.com/yourpage
- https://web.archive.org/web/2025/https://superscout.co/incubator/the-station
<!-- end of sources -->
