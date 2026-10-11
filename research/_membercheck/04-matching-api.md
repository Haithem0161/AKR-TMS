# MemberCheck notes 04 — How: matching, API, deployment, security
Status: done (2026-10-11)

## Summary
- Who: the product is MemberCheck, run by MEMBER CHECK PTY LIMITED (ACN 129 012 344). The Australian Business Register shows it as active from 21 Dec 2007, main location NSW 2067. The vendor says it "operates under Neurocom Group". Its homepage logo says "a Nexiant brand". [PRIMARY] Q8, Contradictions.
- Matching: the vendor names only "fuzzy matching and phonetic algorithms". It publishes no match types, percentages, defaults, score scale or recommended threshold. Users set "matching thresholds", and each match carries a score. [PRIMARY] https://membercheck.com/faq/ and https://membercheck.com/ (Q1 to Q3)
- Secondary identifiers: the homepage illustration shows "Secondary identifiers" with "Date of birth, country" beside "Match score" and "Above threshold". No page says how these change a result. [PRIMARY] https://membercheck.com/ (Q2)
- Outcomes and false positives: the vendor's design uses three outcomes: match, no match, and "cannot determine". Whitelists are built from decisions marked "false match". [PRIMARY] https://membercheck.com/about-us/ and https://membercheck.com/faq/ (Q3, Q5)
- Arabic: no Arabic support is claimed. The software is in English, French and Japanese. No page covers Arabic-script input, transliteration or Arabic name parts. [PRIMARY] https://membercheck.com/faq/ (Q4)
- API: the public pages withhold base URLs, authentication, endpoints, version and rate limits. The vendor says the reference and a sandbox come on request. It describes a single "call at the moment" mode and a batch mode. It names no ready-made CRM or core-banking connector. Implementation takes three to four weeks on average. [PRIMARY] https://membercheck.com/api-integration/ and https://membercheck.com/faq/ (Q6, Q7)
- Hosting: cloud only, "our cloud servers". Sub-processors include Amazon Web Services and Google, both listed as United States. The vendor leaves data location to the buyer. No on-premise, private-cloud or offline option is named. [PRIMARY] https://membercheck.com/sub-processors/ and https://membercheck.com/api-integration/; the cloud-only conclusion is [INFERENCE] (Q8)
- Security: the vendor says it holds ISO/IEC 27001 certification and is GDPR compliant, with no certificate details. It names MFA, user roles, deletion after subscription and "secure servers". It publishes no encryption method, SSO, IP allow-list, uptime commitment or status page. [PRIMARY] https://membercheck.com/faq/ and https://membercheck.com/privacy-policy/ (Q9)
- Watchlist origin: the privacy policy says database data comes "from a commercial supplier of AML/CTF watchlists". The supplier is not named. The sister brand NameScan says its PEP data is in "our proprietary database". [PRIMARY] https://membercheck.com/privacy-policy/ and https://namescan.io/ (Q8)
- Open items: the owner's "Mohammed" is not identified. The About page names no leaders, and the web search found no Mohammed linked to MemberCheck or Neurocom. The owner's "prototype" most likely means "MemberCheck" [INFERENCE]. See Not found.

## Findings

### Q1. Match types and algorithms
- Vendor FAQ: the filters "use fuzzy matching and phonetic algorithms" and no other match type is named. "Our intelligent filters use fuzzy matching and phonetic algorithms to filter out noise in the database and cut down false positives." [PRIMARY] https://membercheck.com/faq/
- Features page (marketing): "Sophisticated algorithms detect non-exact matches and name variations, so subtle links to money laundering or terrorism financing aren't missed." No match types, percentages or defaults are given. [PRIMARY] https://membercheck.com/features-sets/
- Homepage: screening uses "configurable matching that cuts noise, not coverage." The page does not list what is configurable. [PRIMARY] https://membercheck.com/
- Products page: screening runs "with configurable matching and ongoing monitoring." [PRIMARY] https://membercheck.com/products/
- Glossary, generic definition of the concept, not MemberCheck's engine: "Name matching that allows for spelling variants, transliteration and nicknames rather than requiring an exact match, so genuine hits are not missed." [PRIMARY] https://membercheck.com/glossary-of-aml-terms/
- Not stated on any page read: exact match, exact with middle name, percentage similarity, initials, name-order handling, which phonetic method is used, or any default. See Not found. [PRIMARY] https://membercheck.com/features-sets/ (silence)

### Q2. Secondary identifiers
- Homepage illustration of a screening record shows "Secondary identifiers" with "Date of birth, country", beside "Match score" and "Above threshold". It is an illustration, and the page does not say these fields change the score. [PRIMARY] https://membercheck.com/
- Homepage lists input fields: "Full Name", "Date of Birth", "Nationality", "Government ID", "Address", "Business No.", and "+10 more fields". The page does not say how each one is matched. [PRIMARY] https://membercheck.com/
- Privacy policy lists stored fields: "names, date of birth, residential address, a client number allocated to the individual and gender". This describes stored data, not matching rules. [PRIMARY] https://membercheck.com/privacy-policy/
- About page: every match "carries its source list, its score and the identifiers that were compared". The list of identifiers is not given. [PRIMARY] https://membercheck.com/about-us/
- Integration page, advice to banks rather than a product feature: "Name similarity without secondary identifiers is the single largest source of false positives". It does not say how the engine uses them. [PRIMARY] https://membercheck.com/api-integration/
- Date of birth and country are probably usable to narrow matches, because the homepage illustration labels them as secondary identifiers. Only that one illustration supports this, and no rule is stated. [INFERENCE] https://membercheck.com/
- NameScan cross-check (sister brand, not MemberCheck's rule): its sample request sends "name" and "dob": "1974", a year-only value, and selects datasets "pep", "sanctions" and "media". [PRIMARY] https://namescan.io/
- Not found: how a year-only date of birth, nationality, gender, ID number or address changes a result. See Not found. [PRIMARY] https://membercheck.com/features-sets/

### Q3. Match score or match rate, and recommended threshold
- Homepage: before screening, users "Set risk rules, matching thresholds, and customer context". The threshold is a user setting. [PRIMARY] https://membercheck.com/
- Homepage: potential matches are routed "with confidence scoring, evidence, and assignment". The scale of the confidence score is not given. [PRIMARY] https://membercheck.com/
- About page: every match "carries its source list, its score and the identifiers that were compared, rather than just a result." The score's scale, formula and meaning are not given. [PRIMARY] https://membercheck.com/about-us/
- About page: "cannot determine" is "a first-class outcome and not a variant of "no match"." This makes the design three-way: match, no match, cannot determine. [PRIMARY] https://membercheck.com/about-us/
- Integration page: "Give your reviewers a third option. Match, not a match, and cannot determine." This is advice to the bank's reviewers. [PRIMARY] https://membercheck.com/api-integration/
- Integration page: "Matching thresholds are a control." It asks who can change one, which suggests the bank can change it. The control's default is not given. [PRIMARY] https://membercheck.com/api-integration/
- Glossary, generic definition: "The sensitivity setting that decides how close a name must be to a listed name before an alert is raised." It gives no value for MemberCheck. [PRIMARY] https://membercheck.com/glossary-of-aml-terms/
- Homepage marketing figure: "75%" with the label "Fewer false positives". No method, base or test is given. [PRIMARY] https://membercheck.com/
- Recommended threshold, match-rate definition and score scale: none found. See Not found. [PRIMARY] https://membercheck.com/about-us/

### Q4. Non-Latin scripts (Arabic)
- FAQ on software languages: "The software is currently available in English, French and Japanese, and its documentation in English and French, with Spanish to follow." Arabic is not listed. [PRIMARY] https://membercheck.com/faq/
- Features page: "Access the platform in multiple languages to support regional teams." It names no languages. [PRIMARY] https://membercheck.com/features-sets/
- Silence: the word "Arabic" appears on none of the 14 MemberCheck pages read. No page covers Arabic-script input, transliteration in matching, Arabic name parts (Al, Bin, Abu, Abd) or original-script search. The Iraq page covers Iraqi rules and is also silent. [PRIMARY] https://membercheck.com/aml-cft-legislation-in-iraq/
- NameScan's homepage has Arabic-script marketing copy, a phrase reading roughly "check anyone" (my reading). It is site copy, not a matching feature. [PRIMARY] https://namescan.io/

### Q5. False-positive handling
- FAQ defines whitelists: "You can create whitelists from due diligence decisions marked as a "false match" during the screening process, then apply them to future scans to eliminate false positives and reduce workload." [PRIMARY] https://membercheck.com/faq/
- Features page: "Reduce false positives by whitelisting trusted entities, so your team can focus resources on genuine risk." [PRIMARY] https://membercheck.com/features-sets/
- Features page: "Custom watchlists and blacklists" with "Build targeted lists to support your own AML/CTF compliance approach." [PRIMARY] https://membercheck.com/features-sets/
- Glossary, generic definition: "A list of names excluded from future screening because the compliance team has validated them as false matches." [PRIMARY] https://membercheck.com/glossary-of-aml-terms/
- Not stated on any page read: who may approve a whitelist entry, whether an entry expires, whether it is re-checked when the list record changes, and whether whitelist changes are logged. [PRIMARY] https://membercheck.com/faq/ (silence)
- Vendor advice to banks: "A match is not a decision", and "Never auto-reject on a name match alone." [PRIMARY] https://membercheck.com/api-integration/

### Q6. API
- Vendor: "the endpoint-level reference is not published here — base URLs, authentication, request and response shapes, error codes and rate limits." The page was "Last updated 28 August 2026". [PRIMARY] https://membercheck.com/api-integration/
- Access to the reference is on request: "and we will get you the current reference along with a sandbox." I did not contact the vendor, use its forms, request a demo or create an account. [PRIMARY] https://membercheck.com/api-integration/
- Two calling modes: a call "at the moment" for "One subject", used for "Onboarding, a payment about to be released, any decision a person is waiting on"; and "Batch" for "The whole portfolio", used for "Re-screening the existing book, list-change sweeps, remediation". [PRIMARY] https://membercheck.com/api-integration/
- Homepage: the platform screens "against global sanctions, PEP, and watchlist data in real time." [PRIMARY] https://membercheck.com/
- FAQ: "Yes — our API lets you automate your AML/CTF compliance programme and reduce time wasted on manual checks." [PRIMARY] https://membercheck.com/faq/
- Not on any page read: callbacks or webhooks, corporate, ID-verification or business-verification endpoints, result-retrieval endpoints, rate limits, SDKs, version numbers, or an endpoint list. [PRIMARY] https://membercheck.com/api-integration/ (silence)
- api.membercheck.com/docs and demo.api.membercheck.com/docs both returned HTTP 401 to an unauthenticated GET. The 401 suggests the hosts exist and need credentials. [INFERENCE] https://api.membercheck.com/docs and https://demo.api.membercheck.com/docs
- membercheck.com/api and docs.membercheck.com returned HTTP 403 to curl, as did nine guessed paths and a nonexistent test path. The 403 says nothing about whether those pages exist. [INFERENCE] https://membercheck.com/api and https://docs.membercheck.com
- NameScan cross-check: https://api.namescan.io/doc/ returned HTTP 401 to an unauthenticated GET, so its reference could not be read. [PRIMARY] https://api.namescan.io/doc/
- NameScan Developer Centre: "The NameScan API is organised around REST." "each account has a test and production key." "Requests made with test API Key do not use your credits and are not saved in your scan history." It lists no endpoints, request fields, match parameters or rate limits. I did not register for its free API key. [PRIMARY] https://namescan.io/pepandsanctionapi
- NameScan homepage sample: "// POST /v3/person", and "The API documentation gives the endpoints and the fields." [PRIMARY] https://namescan.io/

### Q7. Ready-made integrations and bank core connection
- The API page names no ready-made integration. The terms "CRM", "core bank", "webhook", "callback" and "SDK" appear on none of the MemberCheck pages read, except "Suite CRM", a sub-processor used for sales. [PRIMARY] https://membercheck.com/api-integration/ and https://membercheck.com/sub-processors/
- Features page: "Connect MemberCheck to your existing systems for seamless, real-time screening." This is the only integration statement on the site. [PRIMARY] https://membercheck.com/features-sets/
- FAQ: "This depends on your organisation's technical resources; the average implementation takes between three and four weeks." [PRIMARY] https://membercheck.com/faq/
- FAQ: after the Client Subscription Agreement is signed, a production account can be set up "within 24 to 48 business hours." [PRIMARY] https://membercheck.com/faq/
- Privacy policy names "Freshworks, our customer relationship system". This is the vendor's own CRM for website enquiries, not a client integration. [PRIMARY] https://membercheck.com/privacy-policy/
- A bank core system would most likely connect by calling the API from its own side, because the API is the only integration route named. No core-banking connector appears on any page. [INFERENCE] https://membercheck.com/features-sets/

### Q8. Deployment and data location
- Operator: the privacy policy says "Member Check Pty Limited ACN 129 012 344" is "the owner and operator of the MemberCheck service". [PRIMARY] https://membercheck.com/privacy-policy/
- Australian Business Register (government register, read via WebFetch): MEMBER CHECK PTY LIMITED, ABN 64 129 012 344, ACN 129 012 344, "Active from 21 Dec 2007", entity type "Australian Private Company", main business location "NSW 2067". [PRIMARY] https://abr.business.gov.au/ABN/View/64129012344
- OpenCorporates (aggregator, read via WebFetch): same name, status "Registered", address "CHATSWOOD", "2067", "New South Wales", "Australia". Incorporation date is behind a login. [SECONDARY] https://opencorporates.com/companies/au/129012344
- Group: "MemberCheck operates under Neurocom Group, established in 1992, which also includes NameScan, GPayments and Creative Digital Technology." [PRIMARY] https://membercheck.com/about-us/
- Hosting: the FAQ says "clients retain full ownership of the data stored on our cloud servers." [PRIMARY] https://membercheck.com/faq/
- Hosting: the privacy policy says "MemberCheck databases are held and stored by MemberCheck or a cloud services provider." [PRIMARY] https://membercheck.com/privacy-policy/
- Cloud providers: the sub-processor page lists "Amazon Web Services Inc" (United States, "Cloud services") and "Google LLC" (United States, "Cloud services"). It does not say which one holds the screening databases, or in which regions. [PRIMARY] https://membercheck.com/sub-processors/
- Other sub-processors and locations: Kyckr Ireland Ltd (Ireland), Realyou Limited (New Zealand), Acuris Risk Intelligence Limited (England), Australian Securities & Investments Commission (Australia), Bing Search Engine (Microsoft) (Australia), Hotjar Ltd (Malta). They "assist with the processing of data submitted to our platform, which may include personal information." [PRIMARY] https://membercheck.com/sub-processors/
- Watchlist origin: the privacy policy says database data comes "from a commercial supplier of AML/CTF watchlists" and does not name it. The sub-processor list names Kyckr and Acuris under "Data analysis", but does not say either supplies watchlists. [PRIMARY] https://membercheck.com/privacy-policy/ and https://membercheck.com/sub-processors/
- Data location: the vendor leaves it to the buyer: "Where is the data processed and stored? A jurisdictional question with contractual consequences; settle it before integration rather than during procurement." [PRIMARY] https://membercheck.com/api-integration/
- Regions: the sign-in selector offers "Oceania & Rest of World", "Europe", "Southeast Asia" and "Middle East (GCC)". The page says the region "is the region your account was set up in". The data location for each region is not stated. [PRIMARY] https://membercheck.com/api-integration/
- Cookies: the privacy policy says website data "may be stored or processed overseas, including in the United States." This covers website cookies and analytics, not the screening databases. [PRIMARY] https://membercheck.com/privacy-policy/
- No on-premise, private-cloud, self-hosted or offline option is named on any page read. The offer appears to be cloud SaaS only, because the vendor describes "cloud servers" and names no other deployment. The CONTEXT review-site line about no local offline servers is consistent with this (not re-read). [INFERENCE] https://membercheck.com/faq/ and https://membercheck.com/api-integration/
- Coverage: the country page says "195 Countries screened for sanctions, PEP and watchlists". The FAQ says the database has "more than 1,000 sanctions lists, PEP lists, and wanted/watched lists from 195+ countries." [PRIMARY] https://membercheck.com/country-coverage/ and https://membercheck.com/faq/
- Iraq: the country page lists Iraq under the Middle East, with "Central Bank of Iraq (supervisor); Office of Combating Money Laundering and Terrorist Financing (FIU)". [PRIMARY] https://membercheck.com/country-coverage/
- Iraq page: screening is "against global sanctions, PEP, and adverse-media data". No Iraq-specific list is named. Its figures are "1,000+" "Global watchlists screened", "24h" "Watchlist refresh cycle" and "195" "Countries covered". [PRIMARY] https://membercheck.com/aml-cft-legislation-in-iraq/
- NameScan cross-check: its homepage says it runs "a name against extensive official government databases worldwide", and that the Sapphire tier is "Sourced from reliable third parties", naming no provider. [PRIMARY] https://namescan.io/

### Q9. Security and operations
- ISO: the FAQ says "Yes. ISO 27001 is an information security standard that governs good IT practice and IT security, and MemberCheck holds this certification." No certificate body, number or scope is given. [PRIMARY] https://membercheck.com/faq/
- Footer badges: "ISO/IEC 27001:2022", "ISO/IEC 27701:2019", "GDPR" and "DORA". None links to a certificate or explanation. [PRIMARY] https://membercheck.com/api-integration/
- GDPR: "Yes. MemberCheck is GDPR compliant, and clients retain full ownership of the data stored on our cloud servers." [PRIMARY] https://membercheck.com/faq/
- MFA: the features page lists "Multi-factor authentication" and says "Protect every account against unauthorised access." The method is not given. [PRIMARY] https://membercheck.com/features-sets/
- Roles: "We offer unlimited multi-user access, with simple and advanced permission settings and pre-defined user roles." [PRIMARY] https://membercheck.com/faq/
- Data handling: "Store and access client and transaction data securely, with the ability to delete it when required." [PRIMARY] https://membercheck.com/features-sets/
- Storage: "All personal information held electronically is kept on secure servers and protected by firewall security systems." No encryption method is named. [PRIMARY] https://membercheck.com/privacy-policy/
- Retention: personal information "is retained only for the duration of the client’s subscription and for a period of time afterwards where we have an ongoing business need to retain it." Then "we will make sure it is deleted or anonymised." No period is given. [PRIMARY] https://membercheck.com/privacy-policy/
- Audit: "It is why a decision records who made it." The Iraq page says "a clear audit trail behind every decision." [PRIMARY] https://membercheck.com/about-us/ and https://membercheck.com/aml-cft-legislation-in-iraq/
- Reports: "you can create and download reports in PDF, Word, and Excel". [PRIMARY] https://membercheck.com/faq/
- Monitoring and speed: ongoing monitoring runs "through automatic daily screening" (FAQ). The Iraq page gives "24h" for the watchlist refresh cycle and "<1s" for "Average screening response". No test method or conditions are stated for the speed figure. [PRIMARY] https://membercheck.com/faq/ and https://membercheck.com/aml-cft-legislation-in-iraq/
- Not found on any page read: uptime or availability commitment, SLA, status page, encryption method, IP allow-listing, SSO, backup or disaster recovery, breach notification, penetration testing, and a retention period in numbers. See Not found. [PRIMARY] (silence) https://membercheck.com/features-sets/ and https://membercheck.com/privacy-policy/

## Not found
- Match-type definitions, defaults, percentages, phonetic method used, and initials or name-order rules: searched the 14 MemberCheck pages listed in Sources for fuzzy, threshold, score, phonetic, transliteration, exact, initials, middle name and match rate. Only the Q1 to Q3 statements exist. [PRIMARY] https://membercheck.com/features-sets/ and https://membercheck.com/glossary-of-aml-terms/
- Recommended threshold, match-rate definition and score scale: none on the pages read. "Match score" appears only as a label in the homepage illustration. [PRIMARY] https://membercheck.com/about-us/ and https://membercheck.com/glossary-of-aml-terms/
- Secondary identifier rules (year-only date of birth, country, nationality, gender, ID number, address): none. [PRIMARY] https://membercheck.com/privacy-policy/ and https://membercheck.com/features-sets/
- Arabic-script input, transliteration rules, Al/Bin/Abu/Abd handling and original-script search: none. The word "Arabic" appears on none of the 14 pages. [PRIMARY] https://membercheck.com/faq/ and https://membercheck.com/aml-cft-legislation-in-iraq/
- Help centre, scan-settings and match-rate articles on membercheck.com: none found. The homepage and footer links include Guides, Resources, Glossary, FAQ and Compliance Q&A. The Buyer's Guide, Compare and Case Studies pages were not read. [PRIMARY] https://membercheck.com/
- API base URLs, version, authentication, endpoint list, rate limits, SDKs, callbacks and webhooks: withheld on the API page. Also tried: https://api.membercheck.com/docs (HTTP 401), https://demo.api.membercheck.com/docs (HTTP 401), https://membercheck.com/api (HTTP 403), https://docs.membercheck.com (HTTP 403), and https://membercheck.com/security/, /trust-centre/, /help/, /help-centre/, /status/, /docs/, /developers/, /api-documentation/, /integrations/ (all HTTP 403, the same as a nonexistent test path). [PRIMARY] https://membercheck.com/api-integration/ and https://membercheck.com/faq/
- NameScan API reference at https://api.namescan.io/doc/: HTTP 401. The NameScan Developer Centre at https://namescan.io/pepandsanctionapi has no endpoint list, request fields, match parameters, rate limits or response fields. [PRIMARY] https://api.namescan.io/doc/ and https://namescan.io/pepandsanctionapi
- Ready-made integrations (CRM, accounting, onboarding, core banking, loan origination): none named. [PRIMARY] https://membercheck.com/api-integration/ and https://membercheck.com/features-sets/
- Core-banking connector or bank integration guide: none. [PRIMARY] https://membercheck.com/features-sets/
- On-premise, private-cloud, self-hosted, offline or air-gapped deployment: none. The terms were searched on all pages. [PRIMARY] https://membercheck.com/faq/ and https://membercheck.com/api-integration/
- Data-residency choice or in-country hosting (Iraq or any region): none. Only the regional sign-in selector exists. [PRIMARY] https://membercheck.com/sub-processors/ and https://membercheck.com/api-integration/
- Named cloud regions: none. The sub-processor list gives only "United States" for AWS and Google. [PRIMARY] https://membercheck.com/sub-processors/
- Which cloud provider holds the screening databases: not stated. [PRIMARY] https://membercheck.com/sub-processors/ and https://membercheck.com/privacy-policy/
- Encryption at rest or in transit ("encrypt", "encryption", "TLS", "AES"): none. [PRIMARY] https://membercheck.com/privacy-policy/
- IP allow-listing ("IP address", "allow-list", "allowlist"): none. [PRIMARY] https://membercheck.com/features-sets/
- SSO ("SSO", "single sign"): none. MFA method: none. [PRIMARY] https://membercheck.com/features-sets/
- An audit-log feature by that name ("audit log"): none. The phrase "audit trail" is used (Q9). [PRIMARY] https://membercheck.com/about-us/
- Retention period in years or days: none. [PRIMARY] https://membercheck.com/privacy-policy/
- Uptime, SLA, status page or response-time commitment ("uptime", "SLA", "status page", "response time"): none. Only the "<1s" figure on the Iraq page. [PRIMARY] https://membercheck.com/faq/ and https://membercheck.com/aml-cft-legislation-in-iraq/
- Backup, disaster recovery, breach notification and penetration testing: none. [PRIMARY] https://membercheck.com/privacy-policy/ and https://membercheck.com/faq/
- ISO 27001 certificate number, certification body and scope: not on any page read. Not searched on the web. [PRIMARY] https://membercheck.com/faq/
- Watchlist supplier name: not named by MemberCheck. Not searched on the web. [PRIMARY] https://membercheck.com/privacy-policy/ and https://membercheck.com/sub-processors/
- Leadership, founders and CEO: none on the About page ("CEO", "founder", "director", "chief", "managing", "head of", "president" found on no page). Web search "MemberCheck Neurocom founder Mohammed" returned no page naming a Mohammed linked to MemberCheck or Neurocom. The owner's "Mohammed" is not identified. [PRIMARY] https://membercheck.com/about-us/ and [SECONDARY] WebSearch snippets for the founder query (pages not opened)
- "Prototype" as a product or company name: found on no vendor page or search result for this product. [PRIMARY] https://membercheck.com/
- Named data provider on NameScan's homepage ("Dow Jones", "World-Check", "Refinitiv", "LSEG", "LexisNexis"): none. A search summary had said NameScan uses Dow Jones; the vendor page does not support that. [PRIMARY] https://namescan.io/
- A primary (vendor or Nexiant) source naming Nexiant as parent: none. The vendor's homepage logo text is the only primary mention. Nexiant appears in one secondary page (see Contradictions). [PRIMARY] https://membercheck.com/about-us/ and [SECONDARY] https://zoftwarehub.com/en-ae/products/membercheck/product-details
- Client list or case study naming an Iraqi bank: not found on the pages read. The case-studies page was not read. [PRIMARY] https://membercheck.com/country-coverage/
- CB Insights profile (CONTEXT lead): not opened by me, so its 2007 date is only a CONTEXT claim. [SECONDARY] https://www.cbinsights.com/company/membercheck/ (not opened)

## Contradictions and surprises
- CONTEXT says "Member Check Pty Ltd (Australia)". The privacy policy and the Australian Business Register give the full name MEMBER CHECK PTY LIMITED, ACN 129 012 344. The name and country agree; the ACN and ABN are new. [PRIMARY] https://membercheck.com/privacy-policy/ and https://abr.business.gov.au/ABN/View/64129012344
- Founding year: CONTEXT (CB Insights, not opened) says 2007. The vendor's About page says "MemberCheck was established in 2008". The ABN has been active from 21 Dec 2007. A likely reading is that 2007 is the company or ABN date and 2008 the start of the service. [INFERENCE] https://membercheck.com/about-us/ and https://abr.business.gov.au/ABN/View/64129012344
- Parent company: the vendor's About page says it "operates under Neurocom Group". The homepage logo says "MemberCheck, a Nexiant brand". NameScan's footer says "NameScan, a member of the Neurocom group." A secondary page says "The broader group, now part of Nexiant," (fragment, read via summary). The Nexiant relation is therefore not confirmed by a primary source. [PRIMARY] https://membercheck.com/ and https://membercheck.com/about-us/; [SECONDARY] https://zoftwarehub.com/en-ae/products/membercheck/product-details
- Headquarters: the Zoftware Hub page says "HQ Location: Sydney, Australia" [SECONDARY] https://zoftwarehub.com/en-ae/products/membercheck/overview; the ABN record gives the main business location as NSW 2067, which OpenCorporates lists under CHATSWOOD 2067 [PRIMARY] https://abr.business.gov.au/ABN/View/64129012344. Not a true conflict, but the city differs.
- NameScan data source: a web-search summary said NameScan screens "against Dow Jones watchlists". The NameScan homepage names no provider, and calls its PEP data "our proprietary database". The search summary is not supported by the vendor page. [PRIMARY] https://namescan.io/
- Name collision: a web search for "Neurocom" also returned a telecom software company with the website neurocom.eu. The search gave nothing linking it to MemberCheck. Do not treat it as related without evidence. [SECONDARY] search snippet only, https://neurocom.eu (not opened)
- Openness: the MemberCheck API reference is withheld, but NameScan's Developer Centre describes REST, test and production keys, and a free key. The two group brands differ in how openly they describe their APIs. [PRIMARY] https://membercheck.com/api-integration/ and https://namescan.io/pepandsanctionapi
- Languages: MemberCheck lists English, French and Japanese. NameScan's homepage contains Arabic-script copy. MemberCheck does not claim an Arabic interface. [PRIMARY] https://membercheck.com/faq/ and https://namescan.io/
- Sub-processor page: the first column header reads "Data Controller", above rows that are processors. The page says "Last updated 14 September 2026" and also "Modified on: 25.07.2023". [PRIMARY] https://membercheck.com/sub-processors/
- Overseas storage: the privacy policy's "stored or processed overseas, including in the United States" sentence is about website cookies and analytics. Do not read it as the screening database location. [PRIMARY] https://membercheck.com/privacy-policy/
- Coverage wording differs: the FAQ says the solutions are "supported in every country"; the country page says "195 Countries"; the FAQ database claim says "195+ countries". [PRIMARY] https://membercheck.com/faq/ and https://membercheck.com/country-coverage/
- Unexplained marketing figures: "75%" "Fewer false positives" (homepage), "60M" "Scans performed" (homepage) and "40%" "Faster customer onboarding" (products page). None has a method or date. [PRIMARY] https://membercheck.com/ and https://membercheck.com/products/
- Owner wording: the owner's "prototype member check" matches no page or search result. The product name is "MemberCheck". [INFERENCE] The owner most likely meant MemberCheck; the words sound alike and "Prototype" is never used for this product. https://membercheck.com/
- The Iraq page cites "Law No. 39 of 2015" and "Instructions No. 1 of 2023". These are the vendor's citations, not checked against the law text here. [PRIMARY] https://membercheck.com/aml-cft-legislation-in-iraq/

## Sources
https://membercheck.com/
https://membercheck.com/api-integration/
https://membercheck.com/about-us/
https://membercheck.com/sub-processors/
https://membercheck.com/faq/
https://membercheck.com/features-sets/
https://membercheck.com/products/
https://membercheck.com/privacy-policy/
https://membercheck.com/company-policy/
https://membercheck.com/country-coverage/
https://membercheck.com/aml-cft-legislation-in-iraq/
https://membercheck.com/glossary-of-aml-terms/
https://membercheck.com/resources/
https://membercheck.com/guides/
https://api.membercheck.com/docs
https://demo.api.membercheck.com/docs
https://membercheck.com/api
https://docs.membercheck.com
https://membercheck.com/security/
https://membercheck.com/trust-centre/
https://membercheck.com/help/
https://membercheck.com/help-centre/
https://membercheck.com/status/
https://membercheck.com/docs/
https://membercheck.com/developers/
https://membercheck.com/api-documentation/
https://membercheck.com/integrations/
https://namescan.io/
https://namescan.io/pepandsanctionapi
https://api.namescan.io/doc/
https://abr.business.gov.au/ABN/View/64129012344
https://opencorporates.com/companies/au/129012344
https://zoftwarehub.com/en-ae/products/membercheck/overview
https://zoftwarehub.com/en-ae/products/membercheck/product-details
https://www.cbinsights.com/company/membercheck/ (CONTEXT lead only, not opened)
WebSearch result lists (snippets only, pages not opened individually): "Neurocom Group MemberCheck Nexiant NameScan"; "Nexiant Neurocom Group parent company MemberCheck"; "Member Check Pty Limited ACN 129 012 344"; "MemberCheck Neurocom founder Mohammed"
