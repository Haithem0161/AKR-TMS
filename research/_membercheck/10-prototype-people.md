# MemberCheck notes 10 — Prototype for Information Technology: company and people
Status: partial (2026-10-11)

## Summary
- No founder, owner, CEO or staff member is named in any source read. The site code and the LinkedIn page name nobody. The Facebook About page showed only its title in this run. So "who are they" has no named answer yet. See Q1.
- No Mohammed is tied to Prototype. The site code has no Mohammed, Mohamed, Muhammad or Mohammad, and no Arabic form of the name. File 08 names a Mohamed Al Hakim (co-founder of EQIQ). That was not re-checked here, and no source links him to Prototype. See Q1 and Not found.
- Legal identity: the only register match found is "PROTOTYPE FOR INFORMATION TECHNOLOGY LLC", a Michigan LLC in Okemos, MI (company number 900085947). No source links it to the Iraqi company. No Arabic legal name or Iraqi registration was found. See Q2.
- California address: LinkedIn lists "15442 Ventura Blvd., Ste 201-1772 California , USA" as a second location. No source says whether it is an office or a mailbox. The format alone suggests a mailbox number. See Q2.
- Age: the domain was registered on 2019-07-24 (RDAP), and a TLS certificate starts the same day (crt.sh). LinkedIn gives "Founded 2019". The first archived page with HTTP 200 dates from 2022-02-07, and the current site's first archived copy dates from 2025-12-23. No source gives the company's formation date. See Q3.
- Size: LinkedIn shows 263 followers and "Company size 2-10 employees". Both are self-reported. No named staff was found. See Q4.
- Relationships: the company's own site lists six partners: Namescan, DigiCert, SumSub, Auth0, TeleSign and Online AML. These are company claims only. NameScan's homepage does not mention Prototype. No vendor, regulator or bank was found to confirm a relationship. See Q5.
- Member Check: no source puts "Member Check" or "MemberCheck" next to Prototype. The one screening vendor the company names is "Namescan" (NameScan), in its partner list. [INFERENCE] That may be the screening the owner heard about, but no source says so. See Q7.
- Hostnames: prototype-iq.com redirects to site.prototype-iq.com, which returned HTTP 200 on 2026-10-11. A wildcard certificate exists for *.prototype-iq.com, but the full subdomain list could not be obtained because the wildcard crt.sh query returned HTTP 502. See Q6.
- Contact details (the Baghdad and California addresses, phone and email) are not in the downloaded site code. The client count is computed at run time, and the code refers to a third-party backend. None of these can be checked from the code. See Q2, Q5 and Contradictions and surprises.

## Findings

### Q1. Who founded, owns or runs Prototype?
_No person is named so far. The site code and the LinkedIn page HTML name nobody. See Not found._

### Q2. Legal identity: names, registration and the California address
- Names in use. The home page title is "Prototype IQ - Elite Software Development & Cybersecurity Solutions". The site code uses the brand "Prototype Tech" in the header logo, in an About section label ("About Prototype Tech") and in a line that reads "Secured by Prototype Tech". Source: https://site.prototype-iq.com/ and https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY]
- The LinkedIn page carries the long name. Quote: "Prototype for Information Technology | LinkedIn" (page title). Source: https://www.linkedin.com/company/prototype-for-information-technology/ [SECONDARY]
- LinkedIn gives the primary location as Baghdad. Quote: "Baghdad , Al YArmouk , Near Modn , Baghdad 10075, IQ" (listed under Locations; spacing normalised). Source: https://www.linkedin.com/company/prototype-for-information-technology/ [SECONDARY, company-entered]
- LinkedIn also lists a California location. Quote: "15442 Ventura Blvd., Ste 201-1772 California , USA" (listed under Locations; spacing normalised). Source: https://www.linkedin.com/company/prototype-for-information-technology/ [SECONDARY, company-entered]
- [INFERENCE] The suffix "201-1772" has the shape of a mailbox number, not a plain suite number. Reasoning: the format only. Whether the address is a real office is not found yet. Source: https://www.linkedin.com/company/prototype-for-information-technology/
- A US company with the same English name exists in Michigan. OpenCorporates lists "PROTOTYPE FOR INFORMATION TECHNOLOGY LLC", company number 900085947, type "Domestic Limited Liability Company", registered address "2222 W Grand River Ave Ste A, Okemos, MI, 48864". Source: https://opencorporates.com/companies?q=prototype+for+information+technology [SECONDARY, aggregator copy of the Michigan register; the official record was not opened]
- [INFERENCE] The Michigan LLC may be linked to the Iraqi company, but the name is the only link. Reasoning: no source ties the two, and the California address is not in that record. Sources: https://opencorporates.com/companies?q=prototype+for+information+technology and https://www.linkedin.com/company/prototype-for-information-technology/
- The Facebook page title is in Arabic and English: "النموذج الاول لتكنلوجيا المعلومات / Prototype Technology" (gloss: "the first model of information technology / Prototype Technology"). Source: https://www.facebook.com/prototypeiq/about [SECONDARY]. This is a page name, not a registered name.
The legal name in Arabic, and the place and date of registration in Iraq, are not found. See Not found.

### Q3. Age and history: earliest traces
- The domain was registered on 2019-07-24. Quote: "2019-07-24T20:51:58Z" (RDAP event "registration"). Source: https://rdap.org/domain/prototype-iq.com (resolves to https://rdap.verisign.com/com/v1/domain/prototype-iq.com) [PRIMARY]
- Domain status and dates: status "active", expiry "2027-07-24T20:51:58Z", last change "2026-06-24T00:12:25Z". Source: https://rdap.verisign.com/com/v1/domain/prototype-iq.com [PRIMARY]
- Registrar and name servers: "One.com A/S"; "NS01.ONE.COM" and "NS02.ONE.COM". Source: https://rdap.verisign.com/com/v1/domain/prototype-iq.com [PRIMARY]
- The earliest TLS certificate for the name has a start time on the registration day. Quote: "2019-07-24T21:12:35" (not_before; GoDaddy certificate, crt.sh id 1704098079, valid to "2020-07-24T21:12:35"). Source: https://crt.sh/?q=prototype-iq.com&output=json [PRIMARY]
- [INFERENCE] The certificate starts about 20 minutes after the RDAP registration time of "2019-07-24T20:51:58Z", so the name was set up on its first day. Reasoning: the two timestamps. Sources: https://rdap.verisign.com/com/v1/domain/prototype-iq.com and https://crt.sh/?q=prototype-iq.com&output=json
- The earliest archived capture of any page under the domain is "20211222010816" (robots.txt, HTTP 301 redirect). Source: http://web.archive.org/cdx/search/cdx?url=*.prototype-iq.com&output=json&fl=timestamp,original,statuscode&limit=60 [PRIMARY]
- The first archived page that loaded with HTTP 200 is "20220207094157" (https://prototype-iq.com/, 34900 bytes). Source: http://web.archive.org/cdx/search/cdx?url=prototype-iq.com&output=json&fl=timestamp,original,statuscode,mimetype&limit=60 [PRIMARY]
- Older archived HTTP 200 pages exist on the root domain: "20240514223613" and "20240909154404". Source: http://web.archive.org/cdx/search/cdx?url=prototype-iq.com&output=json&fl=timestamp,original,statuscode,mimetype&limit=60 [PRIMARY]
- The first archived copy of the site subdomain is dated 2025-12-23. Quote: "20251223174304" (https://site.prototype-iq.com/58, HTTP 200). Source: http://web.archive.org/cdx/search/cdx?url=*.prototype-iq.com&output=json&fl=timestamp,original,statuscode&limit=60 [PRIMARY]
- LinkedIn gives the founding year as 2019. Quote: "Founded 2019" (spacing normalised). Source: https://www.linkedin.com/company/prototype-for-information-technology/ [SECONDARY, company-entered; no register confirms it]
- [INFERENCE] The 2019 registration and the LinkedIn founding year agree, but no register confirms when the company was formed. The current site's first archived copy is from 2025-12-23, yet the root domain had pages in 2022 and 2024. Archive coverage is patchy, so the current site may have launched earlier. Reasoning: the archive dates. Sources: https://rdap.verisign.com/com/v1/domain/prototype-iq.com and http://web.archive.org/cdx/search/cdx?url=prototype-iq.com&output=json&fl=timestamp,original,statuscode,mimetype&limit=60

### Q4. Size: staff count and named staff
- LinkedIn shows "263 followers on LinkedIn" (page read on 2026-10-11). Followers are not staff. Source: https://www.linkedin.com/company/prototype-for-information-technology/ [SECONDARY]
- LinkedIn gives the company size as 2-10 employees. Quote: "Company size 2-10 employees" (spacing normalised). Source: https://www.linkedin.com/company/prototype-for-information-technology/ [SECONDARY, self-reported band]
- [INFERENCE] A 2-10 band looks small for a company that claims many bank clients (see Q5). Reasoning: the claimed client count is far larger than the staff band. The band is self-reported and may be out of date. Sources: https://www.linkedin.com/company/prototype-for-information-technology/ and https://site.prototype-iq.com/assets/index-BVHNoGNr.js

Named staff: none found on the logged-out LinkedIn page. See Not found.

### Q5. Relationships: NameScan, MemberCheck, SumSub, DigiCert, Online AML, Central Bank of Iraq
- The company's own partner list names "Namescan" with the description "Global sanctions and PEP screening". Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY, company claim; not confirmed by the vendor]
- The same list names "DigiCert" with "Official SSL Certificate Reseller #1407277". Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY, company claim; the reseller number was not confirmed by DigiCert]
- The same list names "SumSub" with "Identity verification and KYC solutions". Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY, company claim]
- The same list names "Online AML" with "Anti-money laundering solutions". Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY, company claim]
- The same list names "Auth0" with "Identity and access management platform" and "TeleSign" with "Digital identity and communications". Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY, company claim]
- The site code makes a client claim. Quote: "We proudly serve major financial institutions across Iraq and the Middle East, delivering mission-critical technology solutions." Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY, company claim; no bank or regulator confirms it yet]
- The client counter is computed in code, so its value is not in the static text. The label beside it reads "Banking Clients". The lead's read shows 31. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY]
- The client count cannot be checked from the static code, and no bank has confirmed it. The earlier note in file 08 gives 28 banks and 2 insurers. Neither figure appears in the code. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY]
- [INFERENCE] The site code refers to a third-party backend host, "aweqtaahuaydqalxasvm.supabase.co". It was not opened. The client counter and the contact details may load from that backend at run time. Reasoning: neither is in the static code. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- NameScan: the company claim above names "Namescan". The NameScan homepage, read through WebFetch, names no partner or reseller page and does not name Prototype, Iraq or any Iraqi partner. Source: https://namescan.io/ [PRIMARY, vendor page]
- DigiCert: a search for the reseller number "1407277" found no page linking it to DigiCert or to Prototype. The claim is unverified. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY, company claim]
- Central Bank of Iraq: no CBI document was found that names Prototype or a licence for it. The search returned secondary guides on the licence for electronic payment providers, and the page was not opened. Source: https://regalert.today/guides/fintech/IQ [SECONDARY, search summary only]

Online AML is not identified. Two searches found no company of that name. See Not found.

SumSub: the partner listing was not checked. See Not found.

### Q6. Public hostnames under prototype-iq.com
- prototype-iq.com (apex). It appears in the crt.sh exact-name list and in the archive and RDAP records. The apex page redirects to https://site.prototype-iq.com/. Source: https://crt.sh/?q=prototype-iq.com&output=json and https://prototype-iq.com/ [PRIMARY]
- *.prototype-iq.com. A wildcard name in the crt.sh exact-name list. The hosts it covers are not listed there. Source: https://crt.sh/?q=prototype-iq.com&output=json [PRIMARY]
- site.prototype-iq.com. The live site returned HTTP 200 on 2026-10-11, titled "Prototype IQ - Elite Software Development & Cybersecurity Solutions". Source: https://site.prototype-iq.com/ [PRIMARY]
- www.prototype-iq.com. LinkedIn names it as the website. It is not in the archive or certificate name lists. It was not checked further, and no DNS query was made. Source: https://www.linkedin.com/company/prototype-for-information-technology/ [SECONDARY]
- Third-party hosts linked from the site, outside the domain: calendly.com (booking link "https://calendly.com/prototype-iq-com"), static.webapp-portal.com (brochure PDF) and the Supabase host in Q5. None was opened. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY]
- The full list of names under the wildcard is not obtained. The crt.sh wildcard query returned HTTP 502. See Not found. Source: https://crt.sh/?q=%25.prototype-iq.com&output=json [PRIMARY]

### Q7. Do sources put "Member Check", "MemberCheck" or "NameScan" next to Prototype?
- Yes for NameScan, as the company's own claim. The company's site code lists "Namescan" among its partners, with "Global sanctions and PEP screening". Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY]
- "Member Check" and "MemberCheck" do not occur in the site code. A web search by this run for "Member Check" with Prototype and Iraq found nothing, and file 08 reports the same. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js [PRIMARY]
- No source found puts "Member Check" or "MemberCheck" next to Prototype or its people. The NameScan homepage does not name Prototype either. Source: https://namescan.io/ [PRIMARY]

## Not found
- Mohammed, Mohamed, Muhammad, Mohammad and the Arabic spelling: zero matches in the four downloaded site files. This run ran no name search for Mohammed. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- Founder, owner or CEO: the site code has no "founder" and no whole-word "CEO". A search for "Prototype Tech" with Iraq, CEO, fintech and cybersecurity found no page naming Prototype Tech or its people. A search for "prototype-iq.com" found no page from or about the domain. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- "Member Check", "MemberCheck" and "MemberScan": zero matches in the site code, and a web search for "Member Check" with Prototype and Iraq found nothing. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- Baghdad, Karada, Ventura, "15442" and "info@prototype": zero matches in the site code. The four downloaded site files were checked. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- Exact-URL archive query for site.prototype-iq.com: zero rows. Captures of site.prototype-iq.com paths appear only in the wildcard query. Source: http://web.archive.org/cdx/search/cdx?url=site.prototype-iq.com&output=json&fl=timestamp,original,statuscode,mimetype&limit=30
- Facebook page: https://www.facebook.com/prototypeiq returned HTTP 400 to curl. Source: https://www.facebook.com/prototypeiq
- Facebook About page: the fetch returned only the page title. No "Page created" date, location, category or staff name was shown. Source: https://www.facebook.com/prototypeiq/about
- Staff names: the logged-out LinkedIn company page has zero employee-name fields. A web search for "at Prototype for Information Technology" with Baghdad found profiles of other Iraqi IT firms, and none named Prototype in its snippet. Source: https://www.linkedin.com/company/prototype-for-information-technology/
- Brochure: the PDF at https://static.webapp-portal.com/resources/prototypeiq/lp/721526/0.pdf is encrypted (AESV2) and could not be read. No password was tried.
- OpenCorporates: one result for the name, the Michigan LLC. Its status and incorporation date are not shown, and the Iraqi company has no record there. Source: https://opencorporates.com/companies?q=prototype+for+information+technology
- Link between the Michigan LLC and the Iraqi company: a web search found no page that connects them. The Michigan state record itself was not opened.
- Arabic legal name: a web search for "بروتوتايب لتكنولوجيا المعلومات" returned only English and unrelated pages.
- Iraqi company register: not searched. No online register was reached in this run.
- California address: a web search for "15442 Ventura Blvd" with "Ste 201" found no page with that address. The nearest hits are different addresses (15422 and 14542 Ventura Blvd).
- Central Bank of Iraq: a web search for a CBI approval or licence naming Prototype found nothing. No CBI document was read.
- DigiCert: a search for the number "1407277" with DigiCert reseller found nothing. No DigiCert partner directory was reached.
- NameScan: the homepage names no partner. A web search for a NameScan reseller or partner with Iraq and Prototype found nothing. Source: https://namescan.io/
- SumSub partner list: not checked.
- "Online AML": two searches (Online AML; OnlineAML) found no company by that name.
- Registrant name in the domain record. The RDAP record lists only the registrar and an abuse contact, and no registrant. Source: https://rdap.verisign.com/com/v1/domain/prototype-iq.com
- crt.sh wildcard query (all names under prototype-iq.com): HTTP 502 on every attempt in three calls, so the full list of subdomains is not obtained. The exact-name query works (see Q6). Source: https://crt.sh/?q=%25.prototype-iq.com&output=json and https://crt.sh/?Identity=%25.prototype-iq.com&output=json

## Contradictions and surprises
- The lead's rendered-page read gives a Baghdad address, a California address, a phone number and info@prototype-iq.com. The downloaded bundle has none of the Baghdad, Karada, Ventura, 15442 or info@prototype strings, so these details may come from a runtime request. Not resolved. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- The lead's read shows Facebook, LinkedIn and WhatsApp links. The bundle has no link to facebook.com/prototypeiq. Its only Facebook URL is the template placeholder "facebook.com/yourpage". Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- The root domain redirects to the site subdomain. The final URL is https://site.prototype-iq.com/. Source: https://prototype-iq.com/
- Name variants: "Prototype IQ" (page title), "Prototype Tech" (brand in the code) and "Prototype for Information Technology" (LinkedIn). Sources: https://site.prototype-iq.com/ and https://www.linkedin.com/company/prototype-for-information-technology/
- The site links a booking page at https://calendly.com/prototype-iq-com and a brochure at https://static.webapp-portal.com/resources/prototypeiq/lp/721526/0.pdf. Both are third-party hosts. Neither was opened. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- The domain was registered in 2019, but the earliest archived page with HTTP 200 dates from 2022. Source: https://rdap.verisign.com/com/v1/domain/prototype-iq.com and http://web.archive.org/cdx/search/cdx?url=prototype-iq.com&output=json&fl=timestamp,original,statuscode,mimetype&limit=60
- The site shows "5+" next to "Years Experience" in one block and "15+" next to "Years Experience" in another. Source: https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- LinkedIn gives the Baghdad address as "Baghdad , Al YArmouk , Near Modn , Baghdad 10075, IQ" (spacing normalised). The lead's read gives "Karada". Two Baghdad addresses are in use. Sources: https://www.linkedin.com/company/prototype-for-information-technology/ and https://site.prototype-iq.com/
- The Facebook page's Arabic name is "النموذج الاول لتكنلوجيا المعلومات / Prototype Technology". The Arabic name used in this run's search was a transliteration, not a source name. Source: https://www.facebook.com/prototypeiq/about
- The English name in the US record is "PROTOTYPE FOR INFORMATION TECHNOLOGY LLC" (Michigan). The site's brand is "Prototype Tech" and its page title is "Prototype IQ". Sources: https://opencorporates.com/companies?q=prototype+for+information+technology and https://site.prototype-iq.com/
- LinkedIn gives 2-10 employees and a 2019 founding year. The site claims a large bank client count and 5+ or 15+ years of experience. These claims do not fit together well (see Q4 and Q5). Sources: https://www.linkedin.com/company/prototype-for-information-technology/ and https://site.prototype-iq.com/assets/index-BVHNoGNr.js
- The current site's first archived copy is dated 2025-12-23, but the root domain had pages in 2022 and 2024. Archive coverage is partial. Source: http://web.archive.org/cdx/search/cdx?url=*.prototype-iq.com&output=json&fl=timestamp,original,statuscode&limit=60

## Sources
https://namescan.io/
https://opencorporates.com/companies?q=prototype+for+information+technology
https://www.facebook.com/prototypeiq/about
https://static.webapp-portal.com/resources/prototypeiq/lp/721526/0.pdf
https://regalert.today/guides/fintech/IQ
https://crt.sh/?q=prototype-iq.com&output=json
https://crt.sh/?q=%25.prototype-iq.com&output=json
https://crt.sh/?Identity=%25.prototype-iq.com&output=json
https://rdap.org/domain/prototype-iq.com
https://rdap.verisign.com/com/v1/domain/prototype-iq.com
http://web.archive.org/cdx/search/cdx?url=prototype-iq.com*&output=json&limit=60&collapse=timestamp:6
http://web.archive.org/cdx/search/cdx?url=prototype-iq.com&output=json&fl=timestamp,original,statuscode,mimetype&limit=60
http://web.archive.org/cdx/search/cdx?url=*.prototype-iq.com&output=json&fl=timestamp,original,statuscode&limit=60
http://web.archive.org/cdx/search/cdx?url=site.prototype-iq.com&output=json&fl=timestamp,original,statuscode,mimetype&limit=30
https://prototype-iq.com/
https://site.prototype-iq.com/
https://site.prototype-iq.com/assets/index-BVHNoGNr.js
https://www.linkedin.com/company/prototype-for-information-technology/
https://www.facebook.com/prototypeiq
