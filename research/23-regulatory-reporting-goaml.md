# 23. Regulatory reporting from AKR-TMS, centred on goAML

Status: done (2026-10-07)

Requirement prefix: RP (RP-01 to RP-84). Written by the research agent for file 23. Cross-references use "file NN" for the other files in this folder. Anything not verified is marked NOT VERIFIED; people on sanctions lists are never named, and all example data is invented.

## Contents

1. goAML as a product: UNODC, modules, report types, XML schema, validation, how FIUs customise (1.1 to 1.6)
2. goAML in Iraq: the AML/CFT Office, its published XSD, manuals, UTR, the portal, figures, 2026 outreach, hooks (2.1 to 2.8)
3. What Iraqi law and the CBI require a bank to report, and how (3.1 to 3.6, including frozen assets and the 48-hour clock)
4. Other reporting a bank's AML function produces: CBI reports, FATCA, CRS, correspondents (4.1 to 4.5)
5. Report quality: good reports, rejection reasons, defensive filing, pre-submission measurement (5.1 to 5.4)
6. Foreign regimes for the optional regime packs: UAE, Jordan, KSA, Mauritius, US, UK, EU, Egypt (6.1 to 6.9)
7. Tools and vendors: Oracle, Pio-Tech and others, open-source tooling, a business-rule catalogue (7.1 to 7.4)
8. Design of the AKR-TMS reporting module: mapping, builder, validation, workflow, export, clocks, CTR, freeze reports, tipping-off, evidence (8.1 to 8.17)
9. What it means for AKR-TMS: ten findings and twelve open questions for the Office
10. Derived requirements (RP-01 to RP-84)
11. Sources
Appendix A. Three validated example reports (STR, CTR, UTR)

## 1. goAML as a product: UNODC, modules, report types, XML schema, validation, how FIUs customise

### 1.1 What UNODC says (unitego.un.org/goaml, Wayback copy of 6 Dec 2025)

- "The goAML application is the UNODC Software Products for Member States' response to money laundering and terrorist financing, and is available to Financial Intelligence Units of Member States." "More than 70 member states are users of goAML." goAML is "an integrated database and intelligent analysis system developed by the Enterprise Application Center Vienna (EAC-VN)". Three solutions: **Collection** (data submitted by banks and other reporting entities), **Analysis** (rule-based analysis, risk score, profiling by FIU analysts), **Data exchange** (with intelligence agencies, FIU.net, Egmont Secure Web). Product family: goAML SE and EE, goINTEL (interfaces to national databases, FIU.net, INTERPOL I-24/7, ESW), goPortfolio.
- **Report types UNODC lists as primary:** CTR (all cash transaction reports), STR ("can be uploaded or created manually"), **UTR** ("all unusual (cash) transactions that might be suspicious. After examination, a UTR can be dismissed as unsuspicious or raised to an STR"; "the system treats a file with multiple unusual transactions as separate reports"), IFT (international funds transfer), cross-border report, **AIF** (replies to analysts' requests for more detail on transactions, persons, accounts or entities). The description page adds EFT (electronic funds transfer, including international) and "list-based reports".
- **Collection component:** reports "can be submitted in the following three ways: offline through XML data exchange such as an e-mail attachment, CD, HD; online through direct upload of XML data messages; online through forms filled in and submitted on the FIU's secure goAML website". **goAML Web** gives reporting entities XML upload, online forms (including multi-party transactions), secure e-mail submission, a communication function (the **message board**) and the **Additional Information Report**: on request the reporting entity compiles the data "in XML format and transmit it electronically to the FIU". FIUs "can also customize goAML Web to local requirements".
- **Data Clean Up component (the FIU-side validation):** "automatically checks incoming electronic data for completeness and accuracy (compliance with XML schema). This process takes place in a staging area outside the main system environment (the DMZ). If incoming data is correct and complete, it is transferred to the main database. If it is not, it is returned to the reporting entity for correction." **XML Rules Editor:** "allows the creation of rules to accept or reject individual transactions in a report after XML validation ... Syntax is a normal validation/check against schema. Symantix [sic], which is defined by the user, checks transactions for pre-defined conditions such as non-alphabetical characters in names, age of transactor, etc. This tool permits the extraction of faulty transactions from a report and sends those transactions back to the reporting institution for correction or enquiry. The rest of the report is validated." The system also records "who is reporting (and who isn't), what they are reporting, and the standard of the reports" for FIU compliance statistics. So an FIU can reject a **single transaction inside** an accepted report, and its business rules are its own and are not in the XSD.
- **Downloads:** "FIUs with an active Service Level Agreement with UNODC for goAML have access to additional documents in the User Group Virtual Community platform". The official schema documentation is therefore not public; what is public are the XSDs and manuals that individual FIUs publish for their reporting entities (sections 1.3 to 1.5).

### 1.2 The base schema and the lookup lists

Files compared: the Office's XSD (`iraq-goaml-schema.xsd`); a UNODC-edited 5.0.2 XSD found on GitHub (munguambe-eng/kairos-lex `scripts/goAML_XML_Schema_5.xsd`, header "edited with XMLSpy v2021 rel. 3 by" a UNODC user, `goaml:schemaVersion="5.0.2"`; the FIU it belongs to is not stated); the UNODC "goAML XML Reporting Schema Description" as adapted by FIU Nepal (v3.0.0 of 2019, a 4.x schema); older XSDs of Sri Lanka's FIU (v1.1.4) and v2.0 (Sri Lanka validator).
- Both 5.0.2 files have 1,054 `xs:element` declarations and the same element tree. The differences are in **cardinality** (each FIU tightens or relaxes optional fields) and in **lookup enumerations**.
- Lookup lists are embedded as enumerations with the code's label in an XML comment after each value (for example `<xs:enumeration value="OA"><!--Corruption--></xs:enumeration>`). UNODC's own text (Nepal copy): "As per schema 4.0, all schema lookups are defined as enumerations. goAML application will introduce an option to update the schema automatically with the lookup codes defined by the FIU in the 'Lookup Master' screen. Reporting entities will not be able anymore to submit reports with undefined lookup codes. Most of the lookup enumerations mentioned below are just predefined suggestions." So **an unknown code makes the whole report fail schema validation**.

### 1.3 What FIUs publish, and what varies between them (nine FIUs and the UNODC base file compared, 2026-10-07)

Primary documents retrieved: Iraq (XSD 5.0.2, 2026); UNODC-edited 5.0.2 XSD (GitHub, FIU not stated); Nigeria NFIU XSD "NFIU_goAML_4_5_Schema_13012021" (Wayback) and `goAMLXValidator.exe`; Pakistan FMU XSD 4.0 and "goAML Guidelines for STR-Automation, Phase 1, version 2, 22 Dec 2025"; Sri Lanka FIU XSDs v1.1.4 and v2.0 (GitHub copies); UAE FIU "goAML Web Submission Guide v2.2" (14 Apr 2021, the Wayback copy is cut at 1 MiB, pages 1 to 31 recovered) and "goAML Report Types v1.2" (29 Apr 2024); Mauritius FIU "Tip of the month, June 2021" and notice of 26 Dec 2025; Swiss MROS "goAML Web manual" v2.3 (69 pp.), "goAML Web: Web Report Semi-Automation" v0.6 (UNODC, Mar 2020) and the Swiss transaction-upload XSD; FIU Nepal's adaptation of UNODC's "Standard XML Reporting Instructions and Specifications v3.0.0" (2019, 57 pages).

| FIU | Schema | Report codes in its XSD or guide | Narrative field (`reason`) | Notable local rules |
|---|---|---|---|---|
| **Iraq** AML/CFT Office | **5.0.2** (published, 2026) | AIF, BCR, BCR-2, CBI, CR, CTR, CTR-D (DNFBP), CTR-R (real estate), DBS, IRD, IRI, M, MLCFC, ORI, SAR, STR, TFR, TS (test), UTR | 8,000 characters, required | Mother's name required; Arabic and Latin allowed; indicator list with Iraqi codes in Arabic; real-estate bank-transfer transmode; see 2.5 |
| UNODC-edited 5.0.2 file (FIU unknown) | 5.0.2 | AIF, CAS, CTR, DCTR, DEFT, DIFT, EFT, IFT, IRD, IRF, IRI, M, MCTR, MEFT, MIFT, ORD, ORI, SAD, STD, STR, WCTR, WEFT, WIFT | optional | daily, weekly and monthly batch types of CTR, EFT and IFT |
| **Nigeria** NFIU | 4.5 (Jan 2021) | AIF, CTR, EFT, IFT, LRFI, M, SAR, STR, UTR | 4,000 | amount must be >= 0; transaction number <= 50 |
| **Pakistan** FMU | 4.0 (XSD); STR-automation guideline 2025 on a 5.x-style schema | AIFLP, AIFT, CTR, CTR-A, CTR-M, IRD, IRI, ORD, ORI, STR-A, STR-F, STR-M | 4,000 | `teller` re-used for branch code and `authorized` for the city name; transaction number format "[RE code][branch code]-[number]"; amounts in PKR |
| **Sri Lanka** FIU | 1.1.4 and 2.0 (2019 to 2020) | AIF, BCR, CTR, EFT, IFT, IRF, SAR, SAR-L, SAR-P, STR, STR-L, STR-P (and AIFA, AIFT, CFIR, LSTRT in v2.0) | n/a | rentity_id pattern 1 to 3 digits |
| **Egypt** EMLCU | 5.0.2 file (Feb 2024) with a 5.2 upgrade note | AIF, AIFNT, BNI, DTET, IRI, ORI, OSD, SAR, SARTF, STR, STRTF | `reason` and `action` both required | separate TF codes; a report type for designated-terrorist matches (DTET); XValidator distributed |
| **UAE** FIU | goAML Web; guide v2.2 (2021) | STR, SAR, AIF, AIFT, RFI, RFIT, HRC, HRCA, DPMSR, REAR, PNMR, FFR | free text | see section 6 |
| **Mauritius** FIU | goAML Web, upgraded production platform from 9 Jan 2026 | STR | | STR "not later than 5 working days after the suspicion arose" |
| **Switzerland** MROS (fedpol) | goAML Web manual v2.3; Swiss XML schema CH 1.1.4 (upload schema read) | STR, SAR, AIF, AIFT | n/a | listed transactions limited to 100; report rejected if no transactions listed although the relationship has some; attachment categories mandatory; XML or ZIP up to 300 MB, attachments up to 20 MB; one XML per ZIP |
| **Nepal** FIU (UNODC text adapted) | 4.x | STR, SAR, TTR, FXT, IRD, STR-H/L/M/P (by risk and PEP) | 10,000 (action 4,000) | `report_code` values chosen by the FIU |

One historical datapoint on why FIUs adopt goAML: Jamaica's FID (overview deck, 2016) ran PDF and XML forms from 2008 to 2011 and listed their problems as a "quasi-manual process", "data quality" and "slow analysis"; it signed the goAML funding agreement in 2013, piloted in April to June 2016, and lists the benefits as an XML validator that analyses reports "before submission is allowed", mandatory fields, automatic delivery receipts, two-way secure communication through the message board and automatic compliance status of reporting entities. Its goAML kit was not on the live site; the 2013 pre-goAML XSD (`POCA-XML-2013-ver1.1.xsd`) is only a national XML.

What is therefore **standard** across all of them: the envelope (`rentity_id`, `report_code`, dates, reporting person, location, `reason`, `action`), the transaction structure (bi-party From and To, or multi-party `party`), the three party types (account, person, entity) with "my client" variants, the `report_indicators` list, goAML Web with XML upload (single file or ZIP with attachments), the Message Board and AIF replies, `fiu_ref_number` for linking a reply to a request. What **varies**: schema version (4.0 to 5.0.2, with 5.1 and 5.2 elements flagged `supportedIn`); the list of report codes and what each means; required versus optional fields (each FIU decides which fields of the "my client" types are mandatory); lookup lists (every enumeration is the FIU's own "Lookup Master"; an undefined code fails the file); narrative length (4,000 to 10,000); name and character patterns; which standard fields are re-purposed (Pakistan); deadlines (set by national law, not by goAML); thresholds; FIU-side business rules (not in the XSD).

### 1.4 The goAML XML schema: versions, main elements, what is standard

**Versions seen in the documents** (the schema version is separate from the application version): 1.1.4 and 2.0 (Sri Lanka FIU, 2019 to 2020); 3.0 (UNODC text adapted by Nepal: "as per Schema 3.0 the conductor could be different from the source person"); **4.0** (entity can be a direct party in bi-party transactions; all lookups defined as enumerations; "Lookup Master" screen in the application); 4.5 (Nigeria, Jan 2021); **5.0.2** (Iraq 2026, UNODC-edited file, `vc:minVersion="1.1"`); elements flagged for **5.1** (25 distinct elements, 41 occurrences: `prev_rejected_ref_number`, `reporting_user_code`, `schema_version`, `transaction_is_suspicious`, `is_suspected` flags, `account_category`, `account_type`, `related_persons`, `related_accounts`, `account_funds`, `institution_country`, `transaction_address`, `agent_name`, ...), for **5.2** (29 distinct elements, 97 occurrences: `additional_information`, `peps`, `sanctions`, `network_devices`, `geo_location`, `previous_names`, `full_name_frn` (foreign-script name), `social_media`, `employment_history`, `entity_identifications`, `means_of_transport`, `person_my_client`, `entity_my_client` and `account_my_client` as direct parties in activity-based reports, `urls`, `emails`, address detail lines) and for **5.0** (`activity`, `related_entities`, `report_date`); deprecation notes inside the XSD say that `submission_date`, `personal_account_type` and `signatory` "will be deprecated in Schema 6" in favour of `report_date`, `account_type` and `related_persons`. Egypt documents an upgrade to 5.2; Iraq's portal footer shows application 5.5.7.52405.

**Main structure (all versions):**
- `report`: envelope (entity id, branch, submission code, report code, references, dates, local currency), `reporting_person`, `location`, `reason`, `action`, then `transaction`+ or `activity`, then `report_indicators`+. "An XML report is linked to one Reporting Entity but may contain multiple transactions. An uploaded report can be from one report type." (UNODC text).
- `transaction`: number, internal reference, location, description, date and time, teller and authoriser, late-deposit pair, value date, mode (`transmode_code`), `amount_local`; **bi-party** (From and To, each "my client" or not, each an account, a person or an entity, with funds code, country, optional foreign currency, optional conductor) or **multi-party** (`involved_parties/party` with a role, significance 0 to 10 and the party as person, account or entity); `goods_services` for property; "Bi-party is enforced in goAML": "One of the nodes t_from_my_client or t_from should be provided. Both CANNOT be present together" (UNODC text).
- **Types:** `t_person` and `t_person_my_client`, `t_entity` and `t_entity_my_client`, `t_account` and `t_account_my_client`, `t_conductor`, `t_address`, `t_phone`, `t_person_identification`, `t_foreign_currency`, `t_party`, `report_party_type`, `t_trans_item`. The "my client" types carry the stricter set of mandatory fields: Pakistan's FMU says it "introduced [t_account_my_client] to impose additional requirements when the account is hosted by the reporting entity ... fields that are optional in the t_account schema may be defined as mandatory"; the principle: "whenever an involved account or person is classified as a direct client of the RE, additional details may be required" because the bank holds them under KYC.
- **Activity-based reports** (SAR, UTR without a transaction, AIF without transactions): `activity/report_parties/report_party+`.
- **Attachments are not in the XSD.** They accompany the XML (web upload, or a ZIP with the XML and the documents: UAE guide).

### 1.5 Standard validation and where the real rules live

1. **Schema validation (XSD 1.0 constructs in a file that declares 1.1):** structure, order, lengths, enumerations, date-times, decimals, a few patterns. UNODC's Data Clean Up component runs this "in a staging area outside the main system environment (the DMZ)"; failures are returned to the reporting entity.
2. **FIU business rules (the XML Rules Editor):** conditions defined by each FIU ("non-alphabetical characters in names, age of transactor, etc."); can reject individual transactions; the rules are not published. The only public glimpses are the FIUs' guides (UAE "to avoid rejection"), validators built by FIUs for their reporting entities (Nigeria and Egypt distribute `goAMLXValidator.exe`; Sri Lanka's data-cleansing validator lists 40 flags) and error messages seen in the web application.
3. **Web-form rules (goAML Web):** mandatory fields marked with an asterisk; dependent fields (late deposit and posting date; "other" transaction mode needs a comment); auto-filled reporting entity and reporting person.
4. **Operational checks by FIU analysts:** grading (Iraq), return for completion, requests for more information.

**The lifecycle in a documented goAML Web deployment (Swiss MROS manual v2.3, read in full; Iraq's own configuration is NOT VERIFIED but the application is the same product).** An uploaded XML file or web report "undergoes two automated data validation checks within goAML": (1) "The XML scheme check is a technical check on the report structure (completeness, mandatory fields, data formats, data logics)"; (2) "The Business rejection rules consist in business checks (e.g. if an account is active and belongs to a reported subject, then both account balance and balance date must be submitted)". If either fails, the status in "Submitted reports" is **Rejected**, and the reasons arrive on the **Message Board** (by e-mail alert, "without any client data"); the user opens the status, presses **Revert**, and the report returns to "Drafted reports" for correction and resubmission. If both pass, the status is **Processed** (technical success, notified on the Message Board); then **FIU personnel review** the content and either accept (a separate confirmation; in Switzerland the legal processing deadline starts only at acceptance) or reject with reasons. Other stated facts: "There is no automated upload of data from the internal systems of the reporting entity to goAML directly. The automation is therefore only between the internal systems of the reporting entity and the new application that has to be created in-house by the reporting entity." Limits in that deployment: one XML per report, a ZIP may hold only one XML plus attachments, 300 MB per XML or ZIP, 20 MB per attachment, unlimited attachments. **Retention in the portal is short:** reports transmitted to the FIU, accepted or rejected, are deleted 7 days after transmission; drafts after 14 days; reverted reports 7 days after recovery; Message Board items after 30 days, so "for record keeping purposes the reporting entity may want to ensure that these reports are saved onto its own internal IT environment". **Semi-automation:** goAML Web can also upload and download partial transactions as XML wrapped in `<reportdata><transactions><transaction>`; such files "do not have to be valid, i.e. the data does not have to be complete"; "being validated against the XML schema is the first step of getting a report accepted. The content will be checked against the business rules after report submission" (UNODC "Web Report: Semi-Automation", v0.6, Mar 2020); the "XML Report Validator function is only available in the pilot environment and not in the production environment"; uploading a transaction number that already exists in the report simply creates two transactions with that number.
Result for a bank: **XSD validity is necessary and nowhere near sufficient**; an export module needs its own business-rule layer and a feedback loop that records what the FIU actually rejected.

### 1.6 AKR-TMS hooks (section 1)

- Treat goAML as a **family of per-FIU profiles**, not a format (RP profile requirements): each profile = XSD + patches + lookup maps + business rules + emit flags + report-type list.
- Load lookups and cardinalities from the profile's XSD at start-up; generate the mapping UI's dropdowns from it; fail closed on unknown codes.
- Keep **XSD validity, FIU business rules, and quality** as three separate scores, because they fail for different reasons and have different owners (IT, compliance, MLRO).
- Keep an "FIU answers" corpus (real rejection messages, anonymised, with the fix) per profile; it is the only way to learn the unpublished business rules.
- Support goAML 5.1 and 5.2 elements behind per-profile emit flags (peps, related persons, `full_name_frn`, additional information, `transaction_is_suspicious`) so the bank can start using them when the Office confirms; the Egyptian FIU's 5.2 notes show the direction.

## 2. goAML in Iraq: the AML/CFT Office, its schema, manuals, report types, figures

Iraq's financial intelligence unit is the Anti-Money Laundering and Counter-Terrorist Financing Office (مكتب مكافحة غسل الأموال وتمويل الإرهاب, "the Office"), part of the Central Bank of Iraq's structure and the FIU that joined Egmont in July 2023. Its reporting platform is UNODC goAML. This section is built from the Office's own site, API, XSD, manuals and portal, read on 2026-10-07; earlier files only knew that the platform was goAML (file 01).

### 2.1 The Office publishes its goAML XML schema (found 2026-10-07)

The AML/CFT Office's public site (https://aml.iq, the "IQTFS" platform) has a page "goAML System" (route `/goaml`) with a description and one attachment. The site's JSON API (https://api.aml.iq/api/GoAmlSystem and https://api.aml.iq/api/GoAmlSystem/attachments, both answered HTTP 200 on 2026-10-07 without login) returns:

- description (updated 2026-09-17): "goAML is a specialized electronic platform for receiving, managing, and analyzing reports and financial information, supporting financial analysis processes and case management in accordance with approved procedures."
- one attachment, "goAML Schema": "the technical schema that defines the format, structure, and content of the data exchanged with the goAML system", file https://aml-iq.fra1.cdn.digitaloceanspaces.com/GoAmlSystem/2a5f02fd-9c66-4714-9a57-5434ed489f0c.xsd (217,567 bytes, 5,997 lines, uploaded 2026-09-17). Saved locally as scratchpad/src/rp/iraq-goaml-schema.xsd.
- The schema header reads "goAML XML Schema 5.0.2" (edited with XMLSpy v2021 rel. 3), `vc:minVersion="1.1"`, `goaml:schemaVersion="5.0.2"`. The Iraqi FIU therefore runs goAML schema **5.0.2**, the same version as Oracle's goAML application supports (file 09 section 6: "Currently, the application supports only v5.0.2"). The schema loads in libxml2 (XSD 1.0 engine) with no error: it uses no `xs:assert`.
- Its `report_type` list has 19 codes: AIF, BCR, BCR-2, CBI, CR, CTR, CTR-D, CTR-R, DBS, IRD, IRI, M, MLCFC, ORI, SAR, STR, TFR, TS, UTR.
- `submission_type`: E, M (and "-"); `rentity_type`: RE, SH, FIU, SUP, IND.

(Details of the schema are in 1.2 to 1.4 and 2.5; the comparison with other FIUs' schemas is in 1.3.)

### 2.2 The Office also publishes its guidance manuals (found 2026-10-07)

The same API (https://api.aml.iq/api/regulatorydocument?pageNo=N&documentType=T; T=0 laws, 1 regulations and instructions, 2 guidance manuals) lists the Office's public documents with PDF links on https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/. Files uploaded 10 Aug to 10 Sep 2026. Downloaded and read (scratchpad/src/rp/aml/):

| Document (English title on the site) | Pages | Use in this file |
|---|---|---|
| Guidance Guide on Reporting Suspicious Transactions and Activities (الدليل الإرشادي الخاص بالإبلاغ عن العمليات والأنشطة المشبوهة); bilingual cover "Guidelines for Reporting Suspicious Transactions and Activities", www.aml.iq | 29 | The 2025 STR/SAR manual that the CBI's 26 Aug 2026 circular cites: report types, timing, who reports, best practice |
| Guidance Guide on Reporting Unusual Transactions (الدليل الإرشادي الخاص بالإبلاغ عن المعاملات غير الاعتيادية), first edition 2026 | 15 | The external UTR (section 2.4) |
| Guidance Guide on Suspicious Indicators ... Related to Banking Transactions | 11 | Indicator catalogue for banks |
| Guidance Guide on the Prompt Implementation of Targeted Financial Sanctions; Guidance Manual on the Implementation of TFS | 15; 25 | Freeze and match reporting |
| Terrorists' Funds Freezing Regulation No. 6 of 2023 | 31 | Freeze reports, to whom |
| Anti-Money Laundering and Counter-Terrorist Financing Law No. 39 of 2015 (Arabic official text) | 33 | Reporting articles |
| Customer Due Diligence Rules for FIs and DNFBPs | 38 | Reporting links |
| Also: Due Diligence Controls No. 2 of 2023 (PEPs, scan), Council Statement No. 1 of 2023, cross-border declaration instructions, CTF guide, high-risk countries guide, evasion guide, virtual currencies guide, beneficial owner guide (57 pp.) | | Context |

(Arabic text layers use presentation forms; normalised with Unicode NFKC before reading. Dates of the files: STR/SAR manual PDF created 2025-07-20; UTR guide 2026-04-13; TFS prompt-implementation guide 2026-09-07; TFS implementation manual dated March 2023 (PDF July 2023); Freezing Regulation published in the Gazette 2023-09-18 (PDF modified 2024-01-22; the Office's 2026 guide calls it "as amended" and the amendments were not found); CDD Instructions published 2023-10-02.)

### 2.3 The Office's 2025 STR/SAR manual: what it says (read in full, 2026-10-07)

Source: "الدليل الإرشادي الخاص بالإبلاغ عن العمليات والأنشطة المشبوهة" / "Guidelines for Reporting Suspicious Transactions and Activities" (29 pages, www.aml.iq; the manual the CBI circular of 26 Aug 2026 calls "the AML/CFT Office's 2025 guidance manual").

**Report types named by the Office** (its glossary, "types of reports sent electronically to the Office"):

| Code | Meaning (Office's own wording, translated) |
|---|---|
| STR | reports of suspicion of money laundering cases |
| SAR | reports of suspicious activities |
| TFR | reports of suspicion of terrorist financing cases |
| CTR | daily reports of cash transactions that exceed the set ceiling |
| BCR | reports containing declaration forms for funds, currency, bearer instruments, precious metals and stones, received from the General Authority of Customs |
| AIF | reports containing the financial statements of the person or entity under suspicion (additional information file) |

And, in a separate 2026 guide, **UTR** (unusual transaction report, section 2.4 below). All of these codes exist in the `report_type` list of the published XSD (AIF, BCR, CTR, SAR, STR, TFR, UTR; the others are BCR-2, CBI, CR, CTR-D, CTR-R, DBS, IRD, IRI, M, MLCFC, ORI, TS).

**Legal basis quoted by the manual:** FATF R.20; Law 39 art. 12/Fifth/A ("notify the Office immediately of any transaction suspected to involve ML or TF, whether or not it was carried out, on the notification form the Office prepares"); art. 26/First/f (supervisors notify the Office immediately); CDD Instructions No. 1 of 2023 art. 36(2) ("notify the Office immediately of all suspicious operations, transactions or activities ... including attempts, regardless of value"). Art. 9/First sets the Office's tasks: receive and obtain reports; analyse and demand additional information from reporting entities "within the period it specifies"; **suspend execution of a transaction for up to 7 working days** if proceeds may be smuggled out or analysis harmed (art. 9/First/c); refer to the Public Prosecution and notify the parties concerned.

**Duty and standard of suspicion:** report "without delay" any transaction and/or activity suspected to be linked to ML or TF, or where there are "reasonable grounds to suspect" the funds are wholly or partly proceeds of crime (ML and the related predicate offence, or TF), "regardless of the amount". "It is not necessary to obtain evidence that a predicate offence actually occurred", nor to know the underlying crime; "any reasonable ground to suspect any criminal activity is sufficient". Suspicion attaches to behaviour, not the person; may rest on one indicator or a group. Context decides; transactions that do not fit the customer's profile and declared information are contributing factors.

**Form and content:** report "using the special electronic form (goAML)"; entities must "prepare the means to send suspicion reports through goAML"; "fill in all fields ... and enter all information available". The report must contain the essential data: (1) reporting entity data, (2) customer data (natural, legal, or both), (3) data on the suspect transaction or activity, "including a description of the transaction and the reasons and grounds of suspicion, precisely and clearly", (4) the related predicate offence if any. If several transactions: include the details of each. The report **must state the action taken by the reporting entity** (examples: enhanced monitoring of the account, account suspension, no cash deposits accepted, transaction not executed). If the subject was reported before: give **the reference number and date of the earlier report**.

**Timing:** art. 12/Fifth/A "immediately". First line (front-line staff) must notify the second line immediately on suspicion and make all supporting documents available; the reporting officer (second line) reviews, does the initial scrutiny, then decides one of three things: **send the report immediately without delay to the Office; collect more evidence; or keep (archive) the case with the decision justified through the necessary documents**. Priorities: terrorism or TF, or a person on international or local ban lists, first; then ML suspicions or predicate offences. The manual sets **no numeric deadline** (no "within 3 days"); the standard is "immediately / without delay".

**Who files:** the MLRO (second line) or the deputy; the entity must give the Office the contact details of the MLRO and the deputy and notify any change. Supervisors (the CBI is one of seven named: CBI, Insurance Diwan, Securities Commission, Real Estate Registration Department, Central Organisation for Standardisation and Quality Control, Bar Association, Accountants Association) are "the third line" and must also notify the Office of unreported suspicions they find in inspections.

**Best practice for content (section 12):** answer who, what, when, where, how before filing. Who: describe the suspect, the source, beneficiary and account holders; occupation, position inside a legal person; list of beneficial owners, directors, officers and signatories; ownership of the legal person; the role of each party and the relations between them; "all available information should be included", for example addresses, passport numbers, phone numbers. When: date of the transaction or when the activity was noticed; the duration; **each individual transaction in chronological order, with its own date, amount and currency, not only the total**; the time of completion or attempt. Where: addresses of the branches involved; foreign country, institutions, account numbers and names; locations of origin and destination. What: value, currency, any related property or proceeds; for example for cash deposits and cheques followed by outgoing transfers, a narrative that covers each (dates, amounts, currencies, accounts, beneficiaries, correspondent accounts); the instruments and channels used (wires, cash, LCs, cards, facilities; counter, ATM, internet, phone); whether funds crossed borders. How: the suspicious pattern, the source or movement of funds; **whether the suspicion arose from an automated system alert**; the suspected ML, TF or predicate offence (corruption and illicit enrichment are the examples given).

**Indicator catalogue (section 12, bank-relevant):** cash (repeated large cash deposits unrelated to activity; deposits just under the reporting limit; small repeated amounts adding to large totals; business income deposited in personal account; deposits followed at once by withdrawals or transfers), CDD (minimal or unverifiable data, forged or altered documents, copies only, same address for several companies, many signatories without a clear relation, accounts of other persons, alias use, request to cancel when CDD is attempted), reporting-avoidance (enquiries about reporting limits; "the transaction is clean"), account behaviour (inactive account reactivated then drained; use of remote branches; minors' accounts for commercial use; commercial transfers from personal accounts; high-risk countries; foreign-issued cards), TF (donations without approval, conflict zones, UNSC-listed persons), real estate payments (cash payment into seller's account, price mismatch, third-party cheques, chains of powers of attorney). The banking-specific list is the separate 11-page guide on banking indicators (listed in 2.2).

**Penalties quoted (art. 39/Second):** up to 3 years' imprisonment and a fine of IQD 15 million to 50 million, or one of them, for anyone who refrains from reporting suspicious transactions or knowingly gives incorrect information, or who discloses to the customer, the beneficiary or anyone other than the competent authorities any reporting, investigation or examination step. Supervisors' measures (art. 45): stop the activity, withdraw the licence, warn, ban persons, restrict or replace management, fine IQD 250,000 to 5 million per violation. **Protection:** art. 47 (exemption for reporting a criminal agreement before the crime) and art. 48 (no criminal or disciplinary liability for good-faith reporting "even if it proves incorrect").

(AKR-TMS hooks for this section are collected at the end of section 2.)

### 2.4 A new external report: the UTR (unusual transaction report), first edition 2026

Source: "الدليل الإرشادي الخاص بالإبلاغ عن المعاملات غير الاعتيادية، النسخة الأولى 2026" (Guidance Guide on Reporting Unusual Transactions, first edition 2026; 15 pages), prepared after "a series of technical meetings with the reporting officers of financial institutions" under a **public-private partnership (PPP)** project (state banks, private banks, e-payment companies, exchange companies and DNFBPs are listed as participants).

- **What it is:** a report "submitted to the Office when a transaction or activity is observed that is unusual, based on the assessment of the reporting entity, **without requiring suspicion of a financial crime**". Unusual means: it departs from the customer's usual pattern; does not match the profile or declared activity; raises objective questions that call for analysis. An early-warning tool "within a risk-based approach"; not an accusation or a judicial act; used for analysis, monitoring and risk assessment (strategic and operational analysis, linking patterns across institutions, the national risk assessment, emerging risks).
- **What it is not for:** "purely administrative compliance gaps", with three examples: expired ID cards, incomplete customer files, delay in updating KYC. These stay inside the bank's compliance procedures.
- **Sequence (flow chart):** unusual or unjustified transaction detected, then UTR filed, then **enhanced monitoring** of the customer or activity; if indicators do not develop and no suspicion arises, the UTR already sent is enough; if indicators develop and suspicion arises, file an **STR/SAR**.
- **UTR versus STR/SAR:** UTR = abnormal behaviour outside the expected pattern "without sufficient indicators of a financial crime"; STR/SAR = abnormal behaviour "supported by indicators or suspicions that make a possible financial crime likely". The classification "depends on professional risk-based assessment"; "all elements of the scenario need not be present"; accumulation of indicators over time may change the classification.
- **Examples in the guide (11 for banks and non-banks):** import payments for goods inconsistent with the registered business; dormant account (six months) reactivated with moderate-value transactions and no clear risk indicators; large outward transfer out of line with the company's size or history even with invoices or contracts; sudden change of supplier or supplier country; **structuring** (repeated cash purchases of foreign currency at amounts near the statutory limit per transaction); a card issued for a low-income or student segment used for repeated or relatively large international e-transactions; repeated card re-issuance; sudden rises in transaction counts or amounts, rapid deposits and withdrawals, amounts just under product limits, international use after only domestic use, cash withdrawal right after top-up; merchant (POS or payment gateway) with unexplained volume jump or activity unrelated to the registered business; abnormal refunds and cancellations. For brokers: large illogical trading by a new client; several accounts used for matched trades. For insurers: large policy cancelled fast; illogical claims.
- **Channel and mechanism (section 8):** "all UTRs are submitted **exclusively through goAML**"; the entity enters the UTR in goAML, chooses the report type "according to the approved classification", and attaches "all available information **without requiring completeness of the suspicion**": a clear description of the transaction or activity, **the reasons for classifying the case as a UTR**, and any supporting documents or data. Principles: good faith under art. 48; **do not tell the customer**; **do not delay reporting until final verification**; **continue enhanced monitoring after filing**; **keep UTRs under art. 11** (five years).
- **Training:** reporting officers must run periodic training programmes on UTR indicators.

**Why it matters for AKR-TMS:** it adds a third outcome to the MLRO decision (file STR/SAR, file UTR, close with reasons) and a state, "UTR filed, enhanced monitoring", that must later be able to escalate to an STR/SAR that cites the earlier report's number and date. It also fits the CBI controls' own term "unusual-transaction reports" (file 01 sections 2d and 2e), which were internal reports to the MLRO: the bank now has an **internal** unusual-transaction report and an **external** UTR with different recipients.

### 2.5 The Iraqi XSD in detail (analysed and tested 2026-10-07)

Method: parsed `iraq-goaml-schema.xsd` with lxml (libxml2 2.15, XSD 1.0 engine; the schema declares `vc:minVersion="1.1"` but uses no XSD 1.1 construct), listed every element with its cardinality, wrote 3 sample reports with invented data, and ran 66 validations (the 3 complete samples, 38 field-level cases, 19 entity-name and e-mail cases, 6 cases on optional 5.x elements and Arabic text). (xmllint itself is not installed on this machine; lxml wraps the same libxml2.)

**Report envelope (order matters; XSD `sequence`):** `schema_version` (optional, flagged "supportedIn 5.1"), `rentity_id` (integer >= 1, assigned by the FIU), `rentity_branch` (required, up to 255), `submission_code` (E electronic, M manual), `report_code` (19 values, section 2.2), `entity_reference` (optional, 255: the bank's own report number), `fiu_ref_number` (optional: the FIU's reference, used in replies), `prev_rejected_ref_number` (optional, flagged 5.1: links a corrected re-submission to a rejected report), `submission_date` or `report_date` (xs:dateTime), `currency_code_local` (IQD), `reporting_person` or `reporting_user_code` (the MLRO, auto-filled by goAML Web from the login), `location`, **`reason` (required, maximum 8,000 characters: this is the narrative)**, `action` (optional, 8,000: action taken), then **either one or more `transaction` nodes or one `activity` node**, then **`report_indicators` (required, one or more `indicator` codes)**, then (flagged 5.2) `additional_information`.

**Transaction node (bi-party or multi-party):** `transactionnumber` (<= 100), `internal_ref_number`, `transaction_location` (required), `transaction_description` (required, <= 8,000), `date_transaction` (xs:dateTime), `teller`, `authorized`, `late_deposit` (optional; required when `date_posting` is given), `date_posting`, `value_date`, `transmode_code` (required lookup), `transmode_comment`, `amount_local` (decimal, up to 13 fraction digits), then **either** `involved_parties/party+` (multi-party) **or** a bi-party pair: (`t_from_my_client` | `t_from`) and (`t_to_my_client` | `t_to`); each side has `*_funds_code` (lookup), optional `*_foreign_currency` (code, amount, rate), a conductor (`t_conductor`), exactly one of account, person or entity, and `*_country`. "My client" variants are the stricter types: they require data a bank holds on its own customers. `goods_services/item` carries property (vehicles, real estate, jewellery). **Activity node:** `report_parties/report_party+` (person, account or entity, each with optional `significance` 0 to 10, `reason`, `comments`) plus optional `goods_services`: this is how an SAR, UTR or other activity-based report names subjects when there is no transaction.

**Iraq-specific tightening (diff against the UNODC-edited 5.0.2 file, 2,846 path differences collapsing to about 45 rule differences; "Iraq-specific" means "differs from that file", which may itself carry another FIU's choices, but the name patterns, mother's name rule and e-mail rule are not in it):**

| Rule in the Iraqi XSD | Effect on a bank |
|---|---|
| `mothers_name` required (may be empty text) on `t_person`, `t_person_my_client`, `t_conductor_my_client`; optional on `t_conductor` and on the reporting person | Mother's name is the Iraqi secondary identifier (file 22); the report builder must carry it for every natural person, including counterparties; the empty element passes, so unknown values must be flagged by the quality check, not by the schema |
| Person `first_name`, `last_name`, `mothers_name`: a pattern that forbids ASCII punctuation (exact character class quoted below the table) | No apostrophe, dot, comma, parentheses or slash in person names: "Dr. Ali" and "Al-Ka'abi" are rejected by the schema (hyphen and digits are allowed); strip or transliterate before export |
| `last_name` required on every person; `birthdate`, `id_number`, `nationality1`, `occupation`, `source_of_wealth`, `phones` and `addresses` required on `t_person_my_client` (`phones` and `addresses` must be present but pass as empty elements; `identification` is optional, tested) | A customer with no occupation or source of wealth on file cannot be reported without a placeholder; the CBS often lacks these (file 25) |
| Address: `city` and `state` required (min length 1) on every address | Iraqi addresses are district and neighbourhood based; map governorate to `state` and district to `city` |
| `tph_number` required on every phone | |
| Account (my client): `opened`, `beneficiary`, `client_number`, `is_primary`, `role` required; `institution_name` plus `institution_code` or `swift` | |
| Entity (my client): `incorporation_legal_form`, `incorporation_number`, `incorporation_date` required | Company registry data must be in the customer master |
| `reason` and `report_indicators` required; `transaction_location` required | Narrative and at least one indicator are structural, not only business rules |
| Email TLD limited to 2 to 8 letters (`ahmed@example.technology` rejected); no Arabic local part or IDN domain | |
| `sql_date` is `xs:dateTime` with minimum 1753-01-01: a date without time ("2026-09-30") fails | Always emit `YYYY-MM-DDThh:mm:ss` |
| Boolean flags `is_primary`, `deceased`, `business_closed`, `collection_account` carry `fixed="1"`: the literal `true` fails in libxml2 | Emit `1` or omit the element |
| Unknown lookup code (indicator, country, currency, funds, transmode, roles) fails the whole file; case-sensitive (`iqd`, `str` rejected); the UK is `GB` not `UK` | Lookups come from the XSD itself: load them at build time |

The exact person-name pattern (as in the XSD, `first_name`, `last_name`, `mothers_name`; the empty string also passes) is:

```text
[^!"#$%&'()*+,./:;<=>?@\[\\\]^_`{|}~]*||
```

**A defect in the published schema.** The entity `name` pattern is `^[a-zA-Zء-ي\s'&-]*[a-zA-Zء-ي][a-zA-Zء-ي\s'&-]*$`. In XSD regular expressions `^` and `$` are ordinary characters, not anchors. Tested with libxml2: **every ordinary entity name fails** ("Example Trading Company", "شركة النور للتجارة العامة"); only a string that literally begins with `^` and ends with `$` passes (`^Example Trading Company$` passed). Read as the author intended, the pattern would also reject digits, dots, commas, brackets, Arabic diacritics (U+064B to U+0652), and the Kurdish and Persian letters outside U+0621 to U+064A (for example "گوڵان بۆ بازرگانی"). goAML's server appears to be an ASP.NET Core application (the portal's `/Identity/Account/Login` path and `__RequestVerificationToken` cookie); if it validates with .NET's schema engine, which as far as I recall wraps XSD patterns in its own anchors, normal names may pass (NOT VERIFIED either way; no test environment was reachable). The product consequence is the same either way: a bank cannot run a strict XSD validator on the file as published, and entity names with digits, Latin dots ("Co."), brackets or Kurdish letters are at risk of rejection by the FIU.

**Validation test results (63 cases besides the 3 samples, libxml2 against the Iraqi XSD):** a complete STR with one transaction (customer account to third-party account, three indicators) validates once `is_primary` is `1`. Failing as expected: date without time; date before 1753; missing `reason`; missing `report_indicators`; unknown indicator, country or currency code; lowercase currency; apostrophe or dot in a person name; empty first name; missing `mothers_name` element; more than 13 decimals; thousands separator in the amount; narrative of 8,001 characters; `transactionnumber` of 101 characters; `late_deposit` as `True`; lowercase `str`; `rentity_id` 0; address without `state`; empty city or phone number; account number over 255; SWIFT of 12 characters; non-numeric balance. Passing, and therefore a trap for quality: Arabic text in the narrative; empty `mothers_name`; empty `id_number`; a negative amount; digits in a person name; Kurdish letters in a person name; `late_deposit` as `0`.

### 2.6 The Iraqi goAML deployment: what can be read from outside (2026-10-07)

- **Portal:** https://goaml.aml.iq/PROD/Home (Cloudflare-fronted, HTTPS, bilingual Arabic `ar-IQ` and English `en-GB`). The footer reads "© 2026 UNODC, All rights reserved. **Version 5.5.7.52405**": the application is goAML **5.5.7**, while the published XML schema carries `schemaVersion="5.0.2"` with the 5.1 and 5.2 elements flagged. The login page offers username and password, "register", and "forgot password"; the home page says that "to access the system you must first register as a **reporting entity** through the option 'register as an institution'; after successful registration you can log in with the credentials set during registration", gives a support e-mail (obfuscated by Cloudflare) and links a registration guide (`before_login.pdf`, returned 404 on 2026-10-07: NOT READ).
- **How a bank gets in:** register the institution on goAML; give the Office the contact details of the MLRO and the deputy and report any change (2025 manual); the MLRO or deputy is the filer. The separate IQTFS site (aml.iq, mobile apps) has its own registration for liaison officers (user types: citizen, government employee, private-sector employee, lawyer, accountant, jeweler, **liaison officer**, broker; registration status pending, approved, rejected, under review). Two IQTFS services are "liaison only": reporting a name match and declaring frozen assets.
- **Launch and use:** the Office's e-reporting system was piloted in the second half of 2022 and "all banks and a large number of FIs" were registered by the end of 2022 (MER paras 179, 261); bank STRs rose from 82 (2022) to 672 (2023) and 451 (2024). The CBI deck (May 2026, file 01 section 5): 7,020 goAML reports in 2023-24, 415 connected entities, 242,884 CTRs in 2023-25, 503 "SAR" activity reports in 2023-25. MER para 252: in 2022 CTRs (cash deposits and withdrawals of USD 10,000 or more, any currency) came as 32,380 files with 76,369 transactions "via an automated system", "a mechanism imposed by administrative authorities in the absence of a legal text imposing such obligation", for banks only; the Office's 2025 manual calls the CTR a **daily** report.
- **What the Office does with reports (MER paras 259, 266, 267):** receipt and acceptance acknowledgement and "its outcome"; **instruction to refrain from carrying out the transaction (86 STRs)**; requests for additional information about clients and accounts; requests to follow up EDD; notice of poor quality to the supervisor; since 2017 manual grading, automated since goAML; priority to good, moderate and acceptable STRs; poor STRs go back to the reporter. Law 39 art. 9/First/c allows the Office to suspend a transaction for up to 7 working days.
- **Deadlines:** STR/SAR/TFR "immediately" (art. 12/Fifth/A); the 2025 manual and the UTR guide set no numeric day count; CTR daily; TFS freeze reports 48 hours and two months (section 3); replies to Office requests "within the period it specifies" (art. 9/First/b). The bank's own governance clocks come from the CBI controls (liaison officers' monthly report by the 10th; MLRO decision recorded with reasons).
- **Not found:** a numeric STR deadline; the goAML code table in prose (the XSD comments serve as one); the Office's own STR grading form; the format of the "AIF" request; the 48-hour freeze report channel; whether the Office's server rejects single transactions inside an accepted report (UNODC says it can).

### 2.7 The Office's 2025-2026 outreach on report quality (news items read from its API)

Source: https://api.aml.iq/api/news/getall/1 to 5 (43 items, 18 Aug 2025 to 6 Oct 2026, English and Arabic). Items that bear on reporting:

| Date (event) | What the Office said |
|---|---|
| 28 Jan 2026 | Public-private partnership (PPP) meeting with banks on "indicators of suspicion related to unusual financial transactions" and how to "distinguish them from suspicious transactions" (the origin of the UTR) |
| 29 Jan 2026 (workshop) | "Enhance feedback mechanisms between the Office and reporting officers in banks ... to improve the quality of suspicious financial transaction reports and submissions filed through the goAML system"; held "in implementation of the action plan and remedial measures derived from the impact assessment of the analytical study issued by the Office concerning the improvement of reports received from reporting entities"; topics: practical scenarios on submitting reports, "the main technical challenges and obstacles faced by banks during data entry and report submission", "common errors that affect the quality of information", adoption of the Office's reporting guideline; quality to be aligned with FATF standards |
| 12 Feb 2026, 1 Mar 2026 | The same feedback workshop for insurance companies and for exchange companies |
| 18 Feb 2026 and others | Training on the Office's smartphone application for instant access to local and international freezing lists and "implementation of the decisions of the Terrorists' Assets Freezing Committee" |
| 26 Jun 2026 (lecture) | "Reporting mechanisms through officially approved channels and their integration with banking systems to enhance the efficiency of reporting suspicious transactions"; "an introduction to Unusual Transaction Reports (UTRs), presentation of the relevant guidance manual, ... key differences between UTRs and STRs"; AI and OSINT for detecting emerging crime |
| 3 Sep 2026 (workshop) | Local and global sanctions lists for banks' reporting officers: handling alerts, "true matches and false positives", "procedures for reporting to the Terrorist Funds Freezing Committee and the Office, as well as documenting the actions taken" |
| 10 Sep 2026 (workshop) | **"Improving the Quality of Reports, Preparing Suspicious Transaction Reports, and Feedback"**: distinguishing unusual from suspicious in context; "the concept of a high-quality report"; essential elements; "formulate the grounds for suspicion in a professional manner based on facts and indicators, rather than unsupported conclusions"; "a chronological sequence of transactions and fund movements, and linking related events"; customer and related parties "including the beneficial owner"; "effective use of supporting documents ... attaching them to the report"; feedback; "the most common observations and errors in reports". Stated aim: "the objective of developing the reporting system is not merely to increase the number of reports, but primarily to enhance their quality and analytical value" |
| 22-23 Sep 2026 | Workshop covering "reporting suspicious operations and transactions, and the quality of information provided in suspicious transaction reports" |
| 4 Dec 2025 | The Committee for Freezing Terrorist Funds issued a clarification: its Decision 61 of 2025 (Gazette 4848, 17 Nov 2025, freezing a list on Malaysia's request under UNSCR 1373) had included names "not linked to any terrorist activities with the aforementioned organizations" because the list was "published before revision"; the Gazette text was to be corrected |

Two things follow. The Office's analytical study "Improving Reports Submitted by Reporting Entities" and the list of "common errors" are not public (NOT READ), so banks will get them only through the feedback sessions; and a published list can be corrected after the event, so a match, a freeze and a report built on it must be reversible with a documented release (3.4).

### 2.8 AKR-TMS hooks (section 2)

- Ship the **Iraqi XSD (5.0.2) as the reference profile** with a profile loader that reads cardinalities and lookups from the XSD file itself; refresh from https://api.aml.iq/api/GoAmlSystem/attachments (public, no login) on a schedule and on demand, show a diff to the MLRO and the bank's IT, and block exports until a changed profile is tested (RP schema-version requirements).
- Report types in scope: **STR, SAR, TFR, UTR, CTR (and CTR-D, CTR-R for non-bank groups), AIF**, and read-only support for the Office's request types (IRD, IRI) so that incoming instructions are logged. The product does not produce BCR (customs) or CBI/DBS/MLCFC/CR (purpose not found).
- The MLRO decision has **four** outcomes recorded with reasons: file STR/SAR, file TFR, file UTR (with enhanced monitoring), close without filing (reasons mandatory, CBI controls and 2025 manual); every outcome has a timestamp and the case's evidence snapshot.
- Clocks: "immediately" has no number, so AKR-TMS tracks **time from alert to MLRO decision to filing** as management information and lets the bank set an internal target (for example 3 working days from the date suspicion was formed; Mauritius uses 5 working days, the US 30 days); the date suspicion formed is a separate field (as in the Jordanian form).
- Display the Office's grade (if it is ever returned to the bank) and the instruction to refrain as first-class events on the case.
- Treat the Office's feedback sessions as an input channel: a place to record "common errors" and "technical obstacles" the Office names, turn each into a rule or a template change, and show the bank the before and after rejection and grade statistics (RP-84).

## 3. What Iraqi law and the CBI require a bank to report, and how

### 3.1 The reporting map

| # | Report | Trigger | To whom | Channel | Clock | Source |
|---|---|---|---|---|---|---|
| 1 | **STR** (ML), **SAR** (activity, no transaction), **TFR** (TF suspicion) | Suspicion or reasonable grounds to suspect proceeds of a predicate crime (every felony or misdemeanour), ML or TF; executed or not; **attempts included**; **any amount**; also when CDD cannot be completed (Law art. 10/Fifth; CDD Instr. art. 15) | the AML/CFT Office | goAML (XML upload or web form), filed by the MLRO or deputy | "immediately" (فوراً), no day count; internal liaison report to MLRO immediately | Law art. 12/Fifth/A; CDD Instr. 1/2023 arts. 34, 36; Office manual 2025; CBI circular 26 Aug 2026 |
| 2 | **UTR** | Unusual transaction or activity, no suspicion needed; not for administrative gaps (expired ID, incomplete KYC) | the Office | goAML only | "do not delay reporting until final verification"; then enhanced monitoring; escalate to STR/SAR if suspicion arises | Office UTR guide, first edition 2026 |
| 3 | **CTR** | Cash deposits or withdrawals of USD 10,000 or more (any currency); banks only | the Office | goAML, daily batch | daily | MER para 252 and box 3.1; Office manual 2025; CBI deck |
| 4 | **Freeze report** | Freeze of funds of a listed person or any measure taken to comply, including attempted transactions | the **Committee for Freezing Terrorists' Funds**, copy to the Office | "approved channels and forms" (not public); IQTFS has a "frozen assets and funds volume" declaration form for liaison officers | **48 hours**, update **two months** later | Reg. 6/2023 art. 4/Fifth; Office guide 2026 arts. 12, 13, 16 |
| 5 | **Match or similarity report** | Current, former or prospective customer listed or linked to a listed person; unresolved name similarity (the Technical Committee decides) | the Committee and the Office, "immediately" | as above; IQTFS "Report Name and Entity Matches" form (name, reason, evidence, notes, attachment) | immediately | Office TFS implementation manual (Mar 2023) section 6.2; IQTFS |
| 6 | **Reply to the Office's requests** (AIF, information on clients and accounts, EDD follow-up) | A request from the Office "within the period it specifies" | the Office | goAML (AIF is the reply type in the UNODC model; Iraq lists AIF "financial statements of the person or entity under suspicion") | deadline in the request | Law art. 9/First/b; CDD Instr. art. 34/3; MER para 266 |
| 7 | **Instruction to refrain from executing** (inbound) | The Office orders a transaction held, up to 7 working days | from the Office to the bank | via goAML message or correspondence (not public) | up to 7 working days | Law art. 9/First/c; MER para 266 (86 STRs) |
| 8 | **Compliance reports to the CBI** | PEP policy copy "within compliance reports"; inspection requests; the CBI's annual assessment (D2 auditor) | the CBI | CBI templates and inspection | per CBI instruction (not public) | CBI PEP circular 26 Aug 2026; CBI assessment guide |

### 3.2 Internal chain (CBI AML/CFT controls, 8 Jan 2025; file 01 section 2d)

Employee or system raises an unusual-transaction report to the **branch liaison officer**, who sends it to the MLRO (branch management is excluded); the MLRO issues a **written acknowledgment with a tipping-off reminder**, examines, and decides: file with the Office on its form (goAML), close with **written reasons**, or gather more; the employee gets a **written decision notice**; the liaison officer's **monthly report by the 10th** lists cases and latest actions; the MLRO reports to the board through the audit committee; copies of reports sent to the Office, internal reports and **records of closed cases** are kept at least five years with an off-site backup. The 2025 manual adds the three-way decision (send now, gather more evidence, keep with justification) and the priority rule (terrorism and listed persons first). The CBI circular of 26 Aug 2026: SARs go through goAML, "a coherent, detailed narrative", "missing information is acceptable if the maximum available is given and what is unknown is stated", the head of the AML department files, the customer is not alerted, observations are documented precisely, and banks must build automated scenarios for restructuring, splitting and repeated patterns.

### 3.3 Confidentiality and tipping-off (what the report module must enforce)

- **Law 39 art. 12/Fourth:** no disclosure to "the customer, beneficiary or any other person except competent authorities" of any step taken on suspected ML or TF; **art. 39/Second:** up to 3 years' imprisonment and a fine of IQD 15 to 50 million for failing to report or giving knowingly false information, and for tipping off; **art. 48:** good-faith reports carry no criminal or disciplinary liability "even if proven incorrect"; **art. 47:** exemption for early reporting of a criminal agreement.
- **CDD instructions art. 15:** if CDD would tip off, stop it and file immediately; the CBI controls: the MLRO has "full confidentiality" of internal reports and STRs; the UTR guide and TFS guides repeat "do not inform the customer" for every report type and for freezes ("a policy prohibiting staff from informing the customer or any third party that a freeze will be applied").
- Product consequences (file 30 section 8 has the access model): reports, drafts, XML files, acknowledgements and the case timeline are **suspicion tier**; export of any report needs step-up, purpose and approval and is logged; a filed report's existence is never shown to branch users; neutral wording for holds and tasks; the report module writes no AML reason to the CBS.

### 3.4 Frozen assets and sanctions matches: three overlapping duties, and a clock that changed

Sources read 2026-10-07: Terrorists' Funds Freezing Regulation No. 6 of 2023 (Iraqi Gazette no. 4737, 18 Sep 2023; in force 22 Aug 2023; it repeals Regulation No. 5 of 2016, art. 22); the Office's "Guidance Guide on the Prompt Implementation of Targeted Financial Sanctions" (uploaded 10 Sep 2026, 15 pp.); the IQTFS site's report forms (aml.iq SPA).

| Duty | Rule | Source |
|---|---|---|
| Freeze | "without delay" from the time of listing; the Office's 2026 guide defines "without delay" as **within 24 hours** of the listing decision of the UNSC committee or of the Committee for Freezing Terrorists' Funds; freeze extends to funds owned or controlled, directly or indirectly, jointly, derived, and of persons acting for or at the direction of the listed person; follow list updates on the Office's site and the UN site "even during official holidays" | Reg. 6/2023 art. 4 (First, Second); guide arts. 5 and 13-15 |
| **First report: 48 hours** | "All implementing entities must direct a report to the Committee **within 48 hours** of freezing any funds or other assets, or any measures taken to comply with the prohibition, **including transactions that were attempted**. The report covers the value and type of the funds frozen, the date and time of the freeze, and any other attempted transactions" | Reg. 6/2023 art. 4 (Fifth) |
| **Second report: two months** | "a second report to the Committee within two months from the date of applying the freeze, with an update on the status of the frozen funds or assets" | same |
| Copy to the Office | The Office's guide: report "to the Committee for Freezing Terrorists' Funds, **with a copy to the AML/CFT Office, through the approved channels and forms**" (arts. 12, 13 and 16). Contents (art. 16): data of the listed person or entity; source of listing; date and time the match was discovered; date and time the freeze was executed; type of funds frozen; value or estimated value; account, document or reference number; measures taken; any other relevant information | Guide arts. 12-13, 16 |
| Match states | **True match, potential match, false match** (art. 12). True match: freeze at once, block any disposal, document, report, do not tell the person. Potential match: further review of the data, compare identifiers (the guide's list: four-part name and surname, alias, **two-part mother's name**, date of birth, place of birth, nationality, residence, ID number, nature of activity, address), document the result and the reasons, do not tell the customer. False match: document, keep supporting documents, **record the reason for excluding the match**, relationship continues under risk management | Guide arts. 10-12 |
| Records (art. 31) | Copies of the lists used; search logs; match and verification results; freeze decisions and measures; **the reports sent to the Committee and the copy to the Office**; documents supporting "true" or "false" decisions; list-update logs; training records. Retention per the legal periods in force | Guide art. 31 |
| Screening system must (art. 9) | receive and update lists periodically; search full or partial names; handle different spellings in Arabic and foreign languages; alias, titles and variant spellings; screen beneficial owners and linked parties; keep a search log and results; produce reports proving screening was done; protect screening data; role-based access | Guide art. 9 |
| Registration | Each reporting entity registers in IQTFS and in the notification platform of the Office's site (art. 8/3) and names a person responsible for TFS (art. 8/2) | Guide art. 8 |
| Where the Office takes data | The IQTFS portal (aml.iq) has two forms open to registered liaison officers only ("liaisonOnly" in the app): **"Report Name & Entity Matches"** (fields: name, reason, evidence, notes, attachment) and **"Frozen Assets & Funds Volume"** declaration (fields: individual or entity, name, freezing decision number, freezing decision date, freezing amount, last transaction date, estimated value of frozen immovable assets, freezing exceptions) | aml.iq site code (SPA), 2026-10-07 |

**Conflict with an earlier research note.** File 25 (section 15.7 and requirement C-63) records a "three-day Committee-notification clock", taken from the CBI sanctions guidelines of 1 August 2023, which quote the old Regulation No. 5 of 2016. Regulation No. 6 of 2023 (Gazette 18 Sep 2023, in force from 22 Aug 2023) replaced it and sets **48 hours** plus a **two-month update**. AKR-TMS should use 48 hours and flag the change in file 25. (The CBI guideline was dated before the new Regulation took effect; whether the CBI re-issued it is NOT VERIFIED.)

**List corrections happen.** The Committee's Decision 61 of 2025 needed a clarification (4 Dec 2025) because names were "published before revision" (2.7); the match and freeze records must therefore be reversible, with a documented release and a notice that the earlier report is superseded.

**Open point (NOT VERIFIED):** the Office's guide says "approved channels and forms" and does not name them. The IQTFS portal has the match and frozen-asset forms, but whether the 48-hour report to the Committee is made on those forms, by letter or e-mail to the Committee secretariat (General Secretariat of the Council of Ministers), or in goAML (for example as a TFR) is not stated in anything public. The product must therefore make the channel a per-bank setting and produce the report as a controlled document (PDF plus CSV) that a named person can lodge by whichever channel the Committee prescribes.

**Clock conflict within the Office's own documents.** The March 2023 implementation manual says freeze "immediately ... and within a maximum of **8 hours** of the publication of sanctions lists", the September 2026 prompt-implementation guide says "within **24 hours**" of the listing decision, and Regulation 6/2023 says "without delay". AKR-TMS defaults to the strictest (8 hours) and the bank can relax it with a recorded legal basis.

### 3.5 Records (Law art. 11 and CBI controls ch. 4; file 30 section 12)

STR copies and related documents: at least five years from the report date or from a court ruling; records of unusual-transaction reports and their review; records of closed cases; training records; secure storage with a copy at another location; retrievable at speed. The UTR guide cites art. 11 for UTR retention. The TFS guide (art. 31) adds copies of the lists used, search logs, match results, freeze decisions, the reports to the Committee and the copy to the Office, and supporting documents for true and false matches. The retention clocks of file 30 (R6 cases and STRs, R7 screening and sanctions records) apply; the reporting module adds the XML as filed, the acknowledgement, and the version history as legal records.

### 3.6 AKR-TMS hooks (section 3)

- A **report register** with one row per outbound or inbound item: type (STR, SAR, TFR, UTR, CTR, AIF, freeze report, match report, inbound request, instruction to refrain), case, clock start and due, status, channel, FIU reference, approvers, evidence snapshot hash, retention class.
- **Clock engine** with profiles: Iraq (immediately; 48 hours and two months for freeze reports; 7 working days for FIU holds; daily CTR; internal targets), plus reference profiles for the US (30 and 60 days), the UK (DAML 7 working days notice), the EU (5 working days to reply to an FIU), Mauritius (5 working days), the UAE (five days for FFR and PNMR; three working days for HRC).
- **Inbound-request desk:** Office requests (AIF), holds (up to 7 working days) and Committee communications are registered in a restricted register with a case-level access list, due dates, and a reply builder that assembles statements and documents from the evidence vault (file 04 E-01 and E-07).
- **Matches:** true, potential and false matches stored with reasons (the Office's three states); a "freeze" task with an 8-hour default clock and a 48-hour report task with the contents list of art. 16 of the Office's guide; second report at two months.
- **Attempted transactions** (the duty covers them, any amount): posted ledger tables rarely show them, so the report module relies on the integration reading rejected, reversed and abandoned items and the branch observation log (file 03 INR.20 note; file 25), and lets an analyst add an attempt to a case as a transaction with status "incomplete" (`transaction_status` or a comment, profile permitting).

## 4. Other reporting a bank's AML function produces

### 4.1 Reports to the CBI (supervisor)

| Report | Content and frequency | Source | Status of detail |
|---|---|---|---|
| **Compliance officer's AML/CFT report** | Reviewed by the CBI's offsite department: 258 to 269 a year for banks in 2018 to 2022 (MER table 6.2; the counts equal the onsite inspection reports, so the reports appear to be tied to inspections; frequency NOT VERIFIED); feeds the frequency of inspections ("quality of detailed compliance reports") | MER para 646 and annex criterion 26.5 (para 386) | Template not public |
| **Auditor's or external commissioner's annual report** with AML/CFT compliance level (78 a year, one per bank) | Annual | MER para 646 | Template not public |
| **Remedial-action reports** | After inspection findings; banks submitted 39 in 2018 to 2022 (10, 8, 3, 12, 6) | MER para 659 | Template not public |
| **Independent AML assessment (standard D2)** | By a CBI-approved external auditor; submission was due 30 September 2026; gaps fixed within six months; the auditor tests rule logic, alert generation, thresholds, permissions, audit-trail integrity, integration of KYC, monitoring and sanctions, and "whether the data used to detect and report suspicious activity can be traced accurately" | CBI Detailed Assessment Guide (Arabic, p. 45-46); file 01 | Public in outline |
| **Three standard internal-control reports** | Monthly current-account movements and balances (customers and staff accounts: all movements, month-end and average balance, number of operations); transfer reports (all incoming and outgoing, domestic and foreign, amount, currency, payment method per customer); foreign correspondent account reports (transfers, beneficiary bank, number and volume per foreign bank, changes) | CBI controls 2025 ch. 15 (file 01) | Content public; submission route not stated (inspectors on request) |
| **PEP statistics** | Customers by category; PEPs found by screening versus self-declared; relationships needing senior approval and how many refused or restricted; data updates and re-assessments; SARs on PEPs and associates; test results and remediation | CBI PEP circular 26 Aug 2026 (file 01) | Content public; "available to the CBI on request" |
| **Behavioural-indicator training records and tests** | Annual plan approved by senior management; competency tests; records for the CBI | CBI circular 26 Aug 2026 | Public |

### 4.2 Internal governance reports (inputs for inspection and the D2 assessment)

Liaison officers' **monthly reports to the MLRO by the 10th** (cases reported and latest action); MLRO's **detailed periodic reports to the board through the audit committee** on unusual and suspicious transactions and actions taken, with copies for inspection; MLRO review of the monthly reports; a correspondent-bank policy and register (correspondents, ratings, ML/TF risk, due diligence done); KYC update tracking (annual default); training register. All are in file 01 section 2d; this file adds only that the same report engine should render them (section 8).

### 4.3 FATCA and CRS

- **FATCA:** the US Treasury's table "FATCA Agreements and Understandings by Jurisdiction" (read live 2026-10-07) lists **Iraq: "Agreement in Substance", Model 2, "treated as having an IGA in effect" from 30 June 2014**. Under Model 2 an Iraqi financial institution registers with the IRS (GIIN) and reports its US accounts directly to the IRS, with the Iraqi authority's role limited to the agreement; the product should therefore treat FATCA as an optional bank-side report (US indicia, W-8/W-9 status, Form 8966 data) outside the FIU channel. Whether each Iraqi bank holds a GIIN and files is NOT VERIFIED; vendor material shows FATCA modules sold in Iraq (Pio-Tech, Capital Banking Solutions, file 18).
- **CRS:** Iraq's participation in the OECD automatic exchange of financial account information **could not be verified** (the OECD AEOI pages answered 403; the one archived Global Forum page retrieved lists Iraq only in a country navigation and shows no AEOI commitment). The only Iraqi statement found is a vendor release (Capital Banking Solutions, Aug 2022) saying its compliance product was extended "to include GoAML and CRS ... as per the Central Bank of Iraq regulation" (the CBI regulation was not found). Treat CRS as a configurable module, off by default, and ask each bank at discovery whether it has a CBI or tax-authority CRS instruction.

### 4.4 What correspondents request

Correspondent banks (file 04 sections 2c, 2o, 2p; file 01 section 2d on correspondent evidence) ask for: the **Wolfsberg CBDDQ v1.4** (132 questions, 14 sections) and **FCCQ v1.2**; an independent AML assessment by an international firm; audited statements; a credit rating; a Bankers Almanac profile; answers to **RFIs on individual payments** (Wolfsberg Transaction Monitoring RFI Best Practice Guidance, Aug 2022, 30 common questions); sanctions-screening test evidence; statistics on STRs filed and RFI response times. These are answers drawn from system facts; file 04's requirement W-06 (CBDDQ and FCCQ evidence pack generated from system facts) and the RFI register apply. The reporting module adds the **STR statistics** that CBDDQ asks for, **without disclosing any individual report** (aggregates with small-number suppression).

### 4.5 AKR-TMS hooks (section 4)

- A **regulatory calendar** per bank: every recurring report (monthly liaison reports, monthly CBI internal-control reports, annual compliance officer's report, D2 assessment, PEP statistics, training plan, CTR daily batch, FATCA annual, CBDDQ refresh) with owner, due date, status, and the evidence behind each figure.
- **Report templates in the report engine** (same maker-checker and evidence snapshot): the three CBI standard reports (data from the replica), liaison monthly report, MLRO board pack, PEP statistics pack, STR statistics pack for correspondents, D2 evidence pack (rule list with owners, thresholds, change history, alert-to-report traceability).
- **Traceability ("report to source"):** every figure in an outbound report links to the underlying transactions through the evidence snapshot (hash of the extract), as the D2 auditor will test that "the data used to detect and report suspicious activity can be traced accurately".

## 5. Report quality: what a good STR is, why reports fail, and how to measure quality before sending

### 5.1 What the standard-setters and FIUs say a good report contains

| Source | Quality principles (quoted or close paraphrase) |
|---|---|
| **Iraq, AML/CFT Office** (2025 manual, section 2.3; MER para 259) | All fields filled; "the description of the transaction and the reasons and grounds of suspicion, precisely and clearly"; the **action taken**; the earlier report's number and date; who, what, when, where, how; every individual transaction in chronological order with its own date, amount and currency "instead of only the total"; whether the suspicion came from an automated alert; the predicate offence if any. The Office's own grading form (since 2017, automated in goAML) looks for "real indicators of suspicion" (value or frequency not in line with the suspect's activity and income; equal amounts to several people in different countries), "supported by all documents and measures taken by the reporting entity" (MER). |
| **FinCEN** (SAR narrative guidance, Nov 2003; XML guide) | Five elements, who, what, when, where, why, plus how; three parts: **introduction** (purpose, type of activity, earlier SARs and why, sanctions-list links, internal case numbers, red-flag summary), **body** (all relevant facts on the parties, accounts and transactions in chronological order, flows broken into credits and debits, relationships between transactors, branch and channel of cash transactions), **conclusion** (follow-up actions such as closure or monitoring, contacts, information available to law enforcement). "Some SAR forms are submitted with blank narratives." Do not write "see attached". Narrative up to 20,000 characters in five blocks. |
| **UK NCA/UKFIU** (v7.0, Feb 2023) | See 6.6: structured fields first, narrative <= 8,000, glossary codes, "UNKNOWN", intended action at the end, chronological sequence, plain language, up to three previous SAR references. |
| **FINTRAC** (Canada, "Reporting suspicious transactions", live 2026-10-07) | "As soon as practicable" is defined by process: the report is due once the measures that establish "reasonable grounds to suspect" are complete, and "the greater the delay ..., the greater the need for a suitable explanation"; narrative sections ("Details of suspicion", "Action taken") must "avoid jargon or non-public references" so "an outsider can easily understand"; transaction detail goes in structured fields, not only in the narrative; email addresses, IP addresses, aliases and phone numbers matter because FINTRAC "has been able to identify networks ... through pieces of information such as email addresses and secondary identifiers"; entities should "review a sample of your Suspicious Transaction Reports to assess the timeliness" and "the quality of information reported". |
| **Egmont Group** (Operational Guidance 2024, annex on improving STR quality; para 41-44) | Quality is assessed on "volume, timeliness and relevance"; FIUs should "ensure STR formats are consistent", "ensure STRs capture all relevant information" including "IP addresses if accounts are accessed online, telephone numbers and email addresses", unique identifiers and geolocation; FIUs grade cases high (full analysis and dissemination), medium (ongoing analysis, more information) or low (no immediate action). |
| **AUSTRAC** (Australia, SMR page via Wayback) | "High-quality, accurate and timely SMRs give us the best chance to detect, deter and disrupt"; numeric clocks: **24 hours** if the suspicion relates to terrorism financing, **3 business days** for other matters such as money laundering; disclosure that an SMR was filed "may be tipping off" |
| **FATF** (file 03: R.20, R.21, INR.20; IO.6) | Report "promptly", all suspicious transactions "including attempted ones, regardless of amount", reporting "direct and mandatory"; assessors judge **STR quality and timeliness** and the response to FIU requests (IO.6); the Office's grading is Iraq's own implementation of that test |
| **SAMA** (KSA, section 8.9) | Each STR is accompanied by a technical report: six months of statements, the CDD documents, and an analysis of the account. |
| **EU draft ITS (2026)** | A report is a set of typed data points (technically mandatory, mandatory if available, optional, dependent); includes urgency, period of analysis, amount, indicators, offence type, action taken, related earlier reports. |

**Common points to turn into checks:** (1) structured fields and narrative both filled and consistent; (2) the narrative states the observed pattern with numbers (counts, amounts, period, comparison with the expected activity), not the rule name; (3) each party's role is stated; (4) the grounds are facts, not conclusions ("structuring" is a conclusion; "14 deposits of IQD 9.5m in 6 days at 5 branches against a declared salary of IQD 1.2m a month" is a fact); (5) the action taken is stated; (6) earlier reports are referenced; (7) identifiers that link networks (phone, email, device, IP, address, account numbers in standard format) are present; (8) unknown data is marked as unknown, in one agreed way; (9) no jargon or internal codes; (10) timeliness is explained.

### 5.2 Why FIUs reject or downgrade reports

| Layer | Examples found in the sources |
|---|---|
| **Syntax and schema (rejected at upload)** | Not well formed; wrong element order; undefined lookup code (UNODC: "Reporting entities will not be able anymore to submit reports with undefined lookup codes"); date format; value over a length or pattern limit; FinCEN fatal errors (missing records, invalid characters, invalid line terminator, duplicate sequence numbers, invalid TCC). The 63 field-level cases in section 2.5 reproduce the commonest. |
| **FIU business rules (rejected or partly rejected after schema validation)** | UNODC's XML Rules Editor can "accept or reject individual transactions in a report after XML validation" and "send those transactions back"; UAE: STR/SAR without internal report number or description, no reason selected, AIF without the FIU reference, source of funds missing above AED 75,000, Emirates ID missing for a resident; Pakistan: specific re-use of `teller` and `authorized` and the transaction number format. FinCEN: **three outcomes**: rejected ("significant errors, fix and re-submit"), accepted with errors to be corrected after acknowledgement ("E02"), and accepted. |
| **Substance (accepted by the machine, then judged by people)** | Swiss MROS: staff review passed reports and may reject them, for example when no transactions are listed although the relationship has transactions, when subjects with roles are missing, when required attachments are missing, when the factors that aroused suspicion are not selected; Iraq: grades good, moderate, acceptable, poor; **poor reports are returned to the reporter to complete and the supervisor is notified** (MER para 259; 2% to 5% of bank STRs 2018 to 2022, 90% good in 2022); FinCEN: blank or incomplete narratives; UK: poor-quality SARs may be referred to the AML supervisor; wrong or missing glossary codes; DAML requests that are open-ended. |
| **Timing** | Iraq: "immediately"; late filing is itself a violation (FinCEN guidance: "late filings, absence of supplementary SARs, and/or inaccuracies ... have an impact"). |

### 5.3 Defensive filing and over-reporting: evidence

- Iraq's problem is the opposite: **too few** STRs (451 bank STRs in 2024 across about 76 banks, file 01 section 5), not too many; so "reduce volume" is not a design goal, "raise the share of good reports" is. The new UTR (section 2.4) is the Office's own instrument for the grey zone: report the unusual without having to claim suspicion, then monitor and escalate.
- Where defensive filing exists the cure has been to **separate the signals**: the UK offers a DAML (consent) route with a refusal mechanism; the US told banks in October 2025 that near-threshold activity alone does not require a SAR and that a no-SAR decision need not be documented (file 05 section 1b); the Office's UTR plays the same role. The product should therefore classify each case into: file STR/SAR, file UTR, close with reasons, and let the bank tune the mix by outcome data (FIU grade, FIU feedback, law-enforcement use), not by volume.
- Not found: a published figure for what share of Iraqi alerts convert to STRs (file 01: "large numbers of alerts are archived daily against very few STRs").
- The Wolfsberg Group's 2024 statement on effective monitoring (file 04 section 2a) gives the counter-evidence on volume: SAR volumes rise while value does not, "no SAR left behind" tuning and defensive filing are named causes, and the example given is the German FIU in 2022 (337,186 STRs, 15.3% forwarded to law enforcement). Its proposed measures (alert productivity per scenario, completeness of SAR information, false-negative assessment of SARs that came from other sources) are in the analytics of 8.16.

### 5.4 Measuring and improving quality before submission (AKR-TMS hooks for section 5)

1. **Pre-submission quality score (0 to 100)** computed from rule checks that mirror the Office's grading criteria and the sources above: (a) schema valid and lookup valid (gate, not scored); (b) completeness of structured party data (name parts, mother's name, date of birth, ID, address, phone) with "unknown" counted separately from "missing"; (c) every transaction has date, amount, currency, channel, counterparty; (d) narrative has the six mandatory parts (summary, pattern with numbers, parties and roles, chronology, action taken, intended follow-up) and meets length limits; (e) at least one indicator code and a predicate-offence code (or "none identified" with a reason); (f) expected-versus-actual comparison present when the pattern is a profile mismatch; (g) supporting documents attached or listed; (h) earlier reports linked; (i) timeliness: time from first alert to filing and a reason when above a threshold; (j) consistency: amounts in the narrative equal the sum of the transactions in the structured part; dates in the narrative are inside the transaction range. The score is advice to the analyst and MLRO; the MLRO decides.
2. **Narrative drafting with AI under the governance rule:** the model drafts from the evidence snapshot only, cites the transaction identifiers it used, never invents amounts (a checker recomputes every number), flags unsupported statements, and a named analyst edits and signs. The draft is stored as a version with its model, prompt and evidence hash (file 29).
3. **Feedback loop:** record the FIU's response per report (acknowledgement, grade if shared, request for more information, instruction to refrain), and report quality over time by analyst, branch, typology and scenario. This is what the CBI can ask for as "effectiveness" evidence (file 05 R-03).
4. **Sample review (FINTRAC practice):** a monthly random sample of filed reports re-scored by a second reviewer, compared with the automatic score, so the score itself is validated (calibration).

## 6. Foreign regimes for the optional regime packs (US, UK, EU, UAE, KSA, Jordan, Egypt and others)

### 6.1 United Arab Emirates (FIU goAML, documents of 2021 and 2024)

Source: UAE FIU "goAML Report Types" v1.2 (29 Apr 2024) and "goAML Web Submission Guide" v2.2 (14 Apr 2021), both via Wayback (the live site answers 403 to automated clients).

| Report | Trigger and rule |
|---|---|
| STR | Suspected ML, fraud or TF in a transaction, "promptly" (the 2021 guide: "at the earliest") |
| SAR | Suspected activity or an **attempted (non-executed) transaction** |
| AIF / AIFT | Answer to an FIU request received on the **Message Board** about an earlier STR or SAR, without or with transactions; the reply quotes the FIU's reference number |
| RFI / RFIT | Request to several reporting entities, not only the original reporter |
| HRC / HRCA | Transactions or activities linked to **high-risk countries** designated by the national committee; "may only be executed **three working days** after reporting such to the FIU, and if the FIU does not object" |
| **PNMR** | **Partial Name Match Report**: suspension measures taken on partial name matches to the Local Terrorist List or UN Consolidated List, to be reported through goAML **within five days** |
| **FFR** | **Funds Freeze Report**: freezing measures on confirmed name matches, **within five days**, through goAML |
| DPMSR, REAR | Dealers in precious metals and stones, and real estate: cash of AED 55,000 or more, or virtual-asset payments |

Field-level rules in the 2021 guide: "Internal STR/SAR #" and "Description of the Report" must be filled for STR and SAR "to avoid rejection of the report by the system"; AIF/RFI need the FIU reference; "a minimum of one reason for reporting must be selected to avoid rejection"; Emirates ID mandatory for UAE residents; source of funds mandatory when the amount is above AED 75,000; "late deposit" is for banks only; "indemnified for repatriation" only for fraud STRs; the web form asks for My Client versus Not My Client on each side and offers bi-party and multi-party transactions; a "significance" score 0 to 10 per involved party; the MLRO or deputy files; registration requests are approved by the regulator before on-boarding; XML files may be uploaded singly or in a ZIP with attachments; web access for banks is through the central bank services portal with one-time passcodes.

### 6.2 Jordan (AMLU, aml.gov.jo, pages read 2026-10-07)

The Unit "cooperated with UNODC to provide the Unit with goAML" and says the system gives reporting entities "an electronic portal on the internet" to send reports, follow the status of reports sent, and exchange messages and files with the Unit. The Unit's own guidance on filling the notification form (still published for entities not connected to goAML) says: entities connected to goAML submit through it; others deliver a signed and sealed form by hand, or for urgent cases by fax or password-protected e-mail with the original sent without delay; fill in all signatories of a legal person; state the **actions taken to verify the suspicion** ("searching government and commercial databases, verifying the purpose of the transactions against the activity"); attach copies of all available documents; **an amendment to an earlier report fills in only the changed data on a blank form, marked as an amendment to the original report, quoting its date and the suspect's name**. The paper STR form (financial activities) has: reporter data; natural or legal suspect; **the suspect's relationship with the reporting entity** (customer, employee, shareholder, lawyer, accountant; whether employment related; resigned, terminated, suspended or still in post); beneficial owner; **date of the transaction and separately the date of suspicion**; value in currency and in dinar equivalent; type of transaction and payment method; type of suspicion (market manipulation, fraud, embezzlement, forgery, illicit arms trafficking, TF, theft, organised crime, drug trafficking, human trafficking, other); brief description; reasons and grounds of suspicion. No deadline other than "promptly" was found on the pages read; the CBJ instruction text was not retrieved (NOT VERIFIED).

### 6.3 Saudi Arabia (SAMA AML/CTF Guide, section 8, in force since 17 Nov 2019)

Read in full at rulebook.sama.gov.sa. Key rules: report "immediately and directly" to the SAFIU on suspicion (8.3); report all suspicious transactions including **unsuccessful attempts, regardless of value** (8.4); **wire transfers** on suspicion (8.5); no reports "based on speculation" (8.6); if CDD would tip off the customer, skip CDD and report with the reasons (8.7); file "according to the reporting mechanism and form approved by the SAFIU", **plus a detailed additional report** with all data on the transaction and the parties (8.8); **a technical report on each reported case: account statement or transactions for six months, the CDD documents, and a technical report examining the account** (8.9); decisions not to report must be documented in a detailed and sufficient manner with the levels of approval (8.10); keep copies of every report and the internal reports not filed, separately, with named persons allowed to access (8.11); use SAFIU feedback to update indicators and typologies (8.12); notify SAMA at once of accounts or transactions involving names on UN committee lists 1267/1989/2253 and 1988 and the national 1373 list (8.14). SAFIU receives STRs on goAML (file 05 section 4b).

### 6.4 Mauritius (FIU, notice of 26 Dec 2025 and tip of June 2021)

STR "not later than 5 working days after the suspicion arose" (Saturdays, Sundays and public holidays excluded); the platform is goAML; tipping off and failure to report carry up to 10 years' imprisonment and a fine of up to MUR 5 million; the FIU announced a migration to an "upgraded goAML Production platform" on 9 Jan 2026 (down from 3:30 pm on 7 Jan; STR cut-off 3:15 pm on 7 Jan; credentials unchanged). Platform upgrades happen on the FIU's timetable, and a bank's export must follow a schema change (see RP requirement on schema versions).

### 6.5 United States (FinCEN BSA E-Filing)

Sources: FinCEN "FinCEN SAR XML Schema User Guide" (XML Schema 2.0, v1.6, Aug 2021, 182 pp.); "FinCEN SAR Electronic Filing Requirements" v1.4 (Mar 2015, fixed-width legacy layout, 134 pp.); "FinCEN CTR Electronic Filing Requirements" v1.5 (Mar 2015, 83 pp.), all from bsaefiling.fincen.treas.gov (retrieved live 2026-10-07). Base rules (30 or 60 day SAR clock, USD 5,000 threshold, CTR over USD 10,000) are in file 05 section 1b.
- Channels: discrete (web form) or **batch XML** through the BSA E-Filing System; the batch file carries a transmitter, transmitter contact, filing institution and contact office, then one `<Activity>` per SAR; **at most 60 MB per batch**; schemas `EFL_SARXBatchSchema.xsd` and `EFL_BatchAcknowledgementSchema.xsd` are public on fincen.gov. FinCEN "strongly encourages that filers first test their XML batch files for schema compliance using the schemas available ... otherwise, it will be at risk of being rejected."
- **Two validation levels, "Form & Fit"**: form = XSD (element names with the `fc2:` prefix, correct order, closing tags); fit = business rules per element ("The batch submission will be rejected if any of the following conditions are true ..."). Rules a bank can test locally: no leading or trailing spaces; no non-printable characters (carriage return, line feed, tab) inside values; no duplicate entries inside one activity unless allowed (a subject may have one legal name and up to 99 alternate names); unique `SeqNum` attributes; TIN must be 9 digits for TIN types A and B (legacy layout); prior report BSA ID is 14 digits (14 zeros if unknown).
- **Narrative:** up to **five blocks of 4,000 characters (20,000 in total)**, each block an `ActivityNarrativeInformation` with a sequence number 1 to 5; the guide asks for "a clear, complete, and concise description of the activity, including what was unusual or irregular that caused suspicion"; transaction listings that are too long go in an attachment (one file; "Do not include any other supporting documentation"; describe other documents in the narrative).
- **Acknowledgement:** FinCEN returns an XML acknowledgement file with, for each `<Activity>`, the **BSA Identifier (14 digits)** and any errors, status code "A" (Acknowledged); error codes are listed in an attachment (the legacy requirements document has a 13-page error code list and correction instructions); a rejected report is corrected and re-filed.
- **Continuing activity and corrections:** the form's item 1 distinguishes initial, correct/amend, continuing activity and joint filings, and a prior-report BSA ID links them (file 05 section 1b for the 90-day timeline).
- CTR: the legacy CTR record layout has a transaction summary, foreign cash-in and cash-out, persons involved and account numbers, and an "aggregated transactions" flag; the filing clock for CTRs is 15 calendar days (31 CFR 1010.306; stated from general knowledge, NOT VERIFIED in the files read).

### 6.6 United Kingdom (NCA UKFIU, SAR Portal)

Sources: NCA SAR page and "SARs Reporter Booklet June 2026" (live, 2026-10-07); "Guidance on submitting better quality SARs" v7.0 (Feb 2023; the live link is withdrawn, Wayback copy read in full, 30 pp.).
- **Scale and channel:** the UKFIU "receives more than 850,000 SARs a year" and holds "over 4.5 million" on its database; SARs go through the **SAR Portal** (24/7, "you will receive an acknowledgement and reference number following submission"); post is slower and gets no acknowledgement. Reporters "are not routinely provided with updates on their SARs"; feedback comes as sanitised case studies in the Reporter Booklet and the "SARs in Action" magazine (issue 37 current). A **defence against money laundering (DAML)** request is made on the same SAR by ticking "consent"; it must name a specified activity ("not open-ended"); the UKFIU may refuse (7 working day notice period, then a moratorium) and the case studies show refused DAMLs leading to account freezing orders within days.
- **Statutory content (POCA s330/331, TACT):** the reason for suspicion, a description of the property including value where possible, the prohibited act the reporter wishes to carry out; for the regulated sector also the identity of persons and the whereabouts of the property. Failure to report: up to five years.
- **What "good" means, from the guidance (use as the quality rubric in section 5):**
  1. **Structured fields first, narrative second.** "It is important that details are completed within the appropriate SAR Online fields and not solely placed within the 'reason for suspicion' field ... it enables UKFIU systems to link the same entities submitted by a variety of reporters, especially bank account details."
  2. **Narrative of at most 8,000 characters** (about 1,500 words) answering who, how, what is the criminal property, what is its value, where is it, when, and why suspicious; "the bulk of analysis focuses upon the free text searching of keywords in this field."
  3. **Four-step narrative:** start with a glossary code (for example XXD9XX), then a brief summary of the suspicion, then other useful information in chronological order, and **conclude with the intended action** (exit, monitor, continue).
  4. **Glossary codes** are typed into the narrative in the form XX...XX (for example XXV2XX vulnerable persons, XXV3XX child exploitation, XXF9XX fraud against the private sector, XXOICXX organised immigration crime, XXDRUXX drugs, XXFIREXX firearms, XXPROPXX property, XXS99XX consent request); "if in doubt ... better to include one than not"; they let the UKFIU "identify high risk cases for immediate action". The NCA also publishes a note titled "Incorrect use of SAR glossary codes" (title seen in the NCA listing, not read).
  5. **"UNKNOWN" convention:** "Use the word 'UNKNOWN' to make it clear if you do not know the information ... Do not use * (asterisk), ? (question mark), . (dot) or leave blank."
  6. Clear simple language, no acronyms or jargon, no capital letters, paragraphs, a chronological sequence of events, standard account formats (sort code and account number), postcode and country on every address, date of birth "a vital field", up to **three previous SAR reference numbers** for a repeat subject (not internal numbers), no attachments (everything in the SAR).
  7. Say how the activity deviates from the customer's normal activity; explain why each included transaction is suspicious; summarise cash amounts at the end.
- The UKFIU "may also refer poor quality SARs to the relevant AML supervisor", the same pattern as the Iraqi Office's grading and notification of the CBI (file 01 section 5).

### 6.7 European Union (AMLR Article 69 and AMLA's draft reporting-format standard, July 2026)

Sources: Regulation (EU) 2024/1624 Article 69 (text in the scratchpad); AMLA consultation page, consultation paper (29 pp.), interpretative note (89 pp.) and annexes (95 pp.) of the **draft ITS under Article 69(3) AMLR**, opened 2 July 2026, closed 20 September 2026, public hearing 9 September 2026; AMLA will submit to the Commission by **30 November 2026**.
- **Duties (Art. 69):** report "promptly" on own initiative, **including attempted transactions and suspicions from failed CDD**, regardless of amount; **reply to FIU requests within 5 working days** (FIUs may shorten to under 24 hours, or extend); the compliance officer transmits to the FIU of the Member State where the entity is established; protection of the compliance officer against retaliation; a partnership for information sharing may designate one member to file a joint report that names all participants. AMLA indicator guidelines are due by 10 July 2027; the obligations apply from 10 July 2027.
- **The draft format (content and form):** "Current reporting practices across the EU display a high degree of heterogeneity" (the same problem as goAML's per-FIU variants). The ITS defines a **common core data set** (172 distinct data points in the annexes) with **templates per obliged entity type** (annex I.1 financial and credit institutions; I.2 credit intermediaries; I.3 crowdfunding; I.4 notaries; I.5 real estate; I.6 auditors, accountants and legal professionals; I.7 precious metals, high-value and cultural goods; I.8 gambling; I.9 football agents and clubs) and **templates for transaction records** (annex II.1 banking activities, II.2 money remittance, II.3 correspondent services; a CASP template is also mentioned). It "does not specify the technical language to be used nor the file format, while requiring the use of a machine-readable format"; the templates must be implemented by all FIUs "and by all obliged entities wishing to internally automate the process for reporting instead of using the reporting platforms of FIUs". Two phases: two years of testing and gap analysis by FIUs, then technical implementation.
- **Data point treatments (a useful vocabulary):** technically mandatory (the report cannot be submitted without it), technically required, mandatory if available, optional, dependent on another data point, and FIU-required. Report-level data points in annex I.1 include: reporting-entity activity and ID; report reference; **submission type and related previous reports (subsequent, allegedly connected, amending, substituting)**; report type; **urgency (flag, type, latest-reaction date, description)**; cross-border and correspondent-services flags; **suspicion description and the indicators or red flags that triggered it; possible criminal offence type (ML, TF, PF, other) and predicate offence; period of analysis (start and end date); amount reported in euro; content tags; action taken (flag plus description)**. Natural-person data points start with the person's connection to the reporting entity and onboarding date.
- **Transaction-record template (annex II.1, 42 data points, ISO 20022 style):** statement reference, from and to date, account owner, account type, booking and value dates, amount, currency, equivalent in local currency, credit or debit indicator, internal transaction type, nature of transaction, related-party names, countries, IBANs, addresses, device ID (FIU-required), start and end balance. This is the shape of a statement a bank should be able to produce for an FIU's request (AMLR Art. 69(1)(b)) and is the closest published analogue to an Iraqi AIF.
- goAML is used by several EU FIUs (file 13 sections 6 and 9 list Ireland, Germany, Sweden, the Netherlands, Luxembourg in the Hummingbird claims); the draft ITS means a bank with EU exposure will meet a **common data set that is not goAML** by about 2028 or later. AKR-TMS's canonical report model should therefore be wider than any one FIU's XML.

### 6.8 Egypt (EMLCU, mlcu.org.eg, goAML page read 2026-10-07)

Egypt publishes the most complete goAML kit among the FIUs found: "goAML Schema V5" (XSD, Feb 2024, header `goaml:schemaVersion="5.0.2"`), "goAML v5.2 documentation" (a 20-page Arabic upgrade note, most content as images: new reports, suspicion indicators, new fields, new lookups), the UNODC **goAML XValidator** (a Windows executable of 2013; byte-identical to the file Nigeria's NFIU distributes, SHA-256 `1f868561...fbe`, 19,968 bytes, a .NET 2.0 Windows Forms program with a schema chooser; not run here because no .NET or Mono runtime is installed), "Operations, their types and parties", an "EMLCU goAML Explanatory Note" (Aug 2016: the Unit asks banks' compliance and IT departments to cooperate, read the schema documentation, study the "EMLCU goAML Scenarios" paper, and write to the Unit about data gaps and lookups), and per-sector STR forms (xlsx) for entities not yet on goAML.
- **Report codes in Egypt's XSD (11):** AIF, AIFNT (non-transactional), BNI (bearer negotiable instruments), **DTET (Designated Terrorist Entities and Terrorists)**, IRI, ORI, OSD (outgoing spontaneous disclosure), SAR, **SARTF**, STR, **STRTF**. TF has its own STR and SAR codes, and matches against the designated-terrorist lists have a goAML report type (DTET). Iraq's XSD has TFR but no equivalent of DTET, FFR or PNMR.
- **Version:** the v5.2 note lists the 5.1 and 5.2 additions (an "is suspect" flag, previously rejected report reference, reporting user code, related persons for accounts, signatories, entities, directors and persons, additional information, new lookups such as virtual cards (VCN), PayPal, charitable organisation, political party, single-person company). These are the elements the Iraqi XSD already carries, flagged `supportedIn 5.1` or `5.2`. Egypt documents a 5.2 upgrade; Iraq's portal footer reports application 5.5.7, newer than 5.2, while its published XSD keeps `schemaVersion="5.0.2"` with the flagged elements as optional (they validate in libxml2; whether Iraq's server accepts and uses them is NOT VERIFIED).

### 6.9 Comparison and hooks for the regime packs

| Regime | Report types for banks | Deadline | Channel and format | Acknowledgement and feedback |
|---|---|---|---|---|
| **Iraq** | STR, SAR, TFR, UTR, CTR (daily), AIF; freeze report to the Committee | "immediately"; freeze report 48 hours then two months; FIU hold up to 7 working days | goAML Web, XML 5.0.2 profile (app 5.5.7) | on-screen and message board; grade shared with the supervisor; outcome sometimes returned |
| **US** | SAR, CTR, (FBAR etc. not banks' AML) | SAR 30 days (+30 if no suspect, max 60); CTR 15 days | BSA E-Filing, discrete or batch XML (SAR XML 2.0), <= 60 MB | XML acknowledgement with 14-digit BSA ID; errors fatal, rejected, or accepted-with-errors |
| **UK** | SAR, DAML, DATF | as soon as practicable; DAML: 7 working days notice, then a moratorium (31 days under POCA; the figure is not in the pages read) | SAR Portal (web) | acknowledgement and reference number; no routine feedback |
| **EU** | STR per Member State; common ITS format coming (draft July 2026) | "promptly"; FIU replies in 5 working days | national platforms; several use goAML; format-agnostic ITS | national |
| **UAE** | STR, SAR, AIF(T), RFI(T), HRC(A), PNMR, FFR, DPMSR, REAR | "promptly"; PNMR and FFR five days; HRC three working days before execution | goAML Web, XML upload single or ZIP | message board |
| **KSA** | STR with a technical report (6 months of statements, CDD documents, account analysis) | "immediately and directly" | SAFIU on goAML | per SAFIU |
| **Jordan** | STR via goAML; paper form for entities not connected | "promptly" (not stated numerically) | goAML Web; paper by hand, fax or password-protected e-mail | portal status and messages |
| **Egypt** | STR, SAR, STRTF, SARTF, DTET, AIF, AIFNT, BNI, IRI, ORI, OSD | not retrieved | goAML 5.2, XML | per EMLCU |
| **Mauritius** | STR | 5 working days from suspicion | goAML, upgraded platform Jan 2026 | per FIU |
| **Pakistan** | STR-A/F/M, CTR-A/M, AIF; STR automation (XML transaction trail) from 1 Apr 2026 | not retrieved | goAML Web plus XML transaction upload; test environment | per FMU |

**Hooks:** the regime packs are **profiles** (8.15) plus a **clock set** plus a **narrative template set** plus an **output adapter**: goAML profiles (UAE, KSA, Jordan, Egypt, Mauritius, Pakistan share the engine; each needs its own XSD, lookups and rules), FinCEN SAR XML 2.0 (a separate serialiser with five 4,000-character narrative blocks, BSA ID acknowledgement parsing and the three error classes), UK SAR Portal (a field-ordered form export with glossary codes inserted in the narrative and "UNKNOWN" convention), and an EU data-point export (the 172-point core set) when the ITS is final. Each pack is switched on per bank and evidenced in the assessor pack. Other numeric clocks seen, for reference only: Australia 24 hours (TF) and 3 business days (other), Mauritius 5 working days, US 30 days. None is needed for an Iraqi-only bank; all matter for banks with foreign branches, group reporting or correspondents that ask for "equivalent" reporting practice (file 05 section 5a).

## 7. Tools and vendors: how AML products produce goAML, and open-source tooling

### 7.1 Commercial products (from files 09, 13, 18 and new reading)

| Product | What it does for goAML | Evidence and limits |
|---|---|---|
| **Oracle FCCM, Regulatory Reporting Cloud** (goAML STR application; user guide G58777-01, 14 Aug 2026, 71 pp., read in full) | A report is created from a case by the action "Generate goAML STR" (web service from Case Investigation) or manually; lifecycle statuses **Open, Reopened, Requested for Approval, Rework, Approved, Canceled, E-File Generated, Submitted, Acknowledged, Rejected from E-File**; analyst requests approval, supervisor approves, reworks or cancels (comments up to 255 characters, all audited); on approval the application produces the final STR document; **an authorised user generates the E-File and downloads it; the "Submit" action only records that the file went to the FIU and asks for the "FIU Reference Number (an acknowledgment number received from the regulator)"**; due date = action date + N calendar days (default 5), editable only if an administrator enables it; roles Analyst1, Analyst2, Supervisor, Auditor; per-country XSD templates, version 5.0.2 only (file 09) | Oracle itself does **not** upload to the FIU; the human uploads the XML in goAML Web and types back the reference number. This is the pattern the AKR-TMS plan assumes (the goAML portal belongs to the FIU). No Iraq template is documented |
| **Pio-Tech goAML management system** (Jordan; Iraqi references in file 18: International Islamic Bank live Apr 2024, Iraqi Islamic Bank Feb 2024) | "Dynamic scenario builder and rules engine", "utilities to adjust, monitor and validate reports"; data from the Bank-BI warehouse ("goAML reports ... require automation and preparation of data that is either uncollected or resides in multiple business areas") | The product pages are client-side rendered; features are vendor claims; no schema detail, no quality scoring found |
| **Capital Banking Solutions CapitalCompliance** | "extended ... to include GoAML and CRS" at Sumer Commercial Bank (Aug 2022, vendor release, file 18) | Vendor release only |
| **Flagright** | 7-stage filing workflow with regulator acknowledgment log and locked versions; "goAML-ready" for "30+/35+/70+" jurisdictions (file 13 section 6) | The number of real schemas is unclear; no Iraq |
| **Hummingbird** | "Thousands of validation tests before filing, ensuring they're never rejected"; prepared reports for goAML FIUs (Ireland, Germany, Sweden, the Netherlands, Luxembourg, Bermuda, Ghana, Nigeria); complete record from internal approval to receipt acknowledgment (file 13 section 9) | No Middle East FIU listed |
| **FinCEN-style vendors** (NICE Actimize, SAS, Verafin, etc.) | US SAR and CTR batch XML | Not goAML (files 08, 10, 11) |
| UNODC **goAML XValidator** (Windows executable, 2013; .NET 2.0 Windows Forms, byte-identical copies from Nigeria and Egypt) | A small tool that lets the user pick a schema and an XML file and checks the file against it (inferred from its resource names); distributed by the FIUs of Nigeria and Egypt. Being a .NET program it presumably uses the .NET schema engine, the same family as the FIU server (inference) | Binary only, dated 2013; not run or inspected here (the file was only unpacked) |

### 7.2 Open-source and public tooling found on GitHub (searched 2026-10-07; none is maintained at product level)

| Repository | What it is | Use for AKR-TMS |
|---|---|---|
| `eradesa/goAML_py_scripts` | A Python (tkinter) "goAML Submission Validator" with a flag catalogue ("Description of Validator Flags.xlsx", 40 flags), config file, SWIFT code list, audit SQLite database, manual; written for Sri Lanka's CTR and EFT reports | Best public catalogue of **business-rule checks** beyond the XSD; reused as the rule list in 7.3 |
| `KasunHewagamaDFCC/GOAML_Validator` | Java validator with Sri Lanka FIU XSD v2.0 and CTR samples | Shows XSD validation in a batch tool |
| `PingExchange/XmlGoaml` | .NET 7 library: generate XML from parameters and validate against a supplied XSD | Minimal; confirms there is one XSD per FIU ("There are various versions of goAML XSD schema") |
| `Aruna-SITS/goAMLProject` | Java project generating XML from the Sri Lanka FIU schema v1.1.4 | Older schema |
| `huxiaoming274/goaml-xml-generator` | Browser form that generates goAML 5.x XML for the Luxembourg CRF; English, French, Chinese; its README says "Each FIU publishes its own indicator list with different numbering" | Confirms the per-FIU indicator lists |
| `manojrevenkar12-web/aml-investigator` | LangGraph investigator whose output is schema-validated goAML STR XML with four-eyes and hash chain; its README admits the schema is "a representative subset ... not the official FIU schema, which is distributed to registered" reporting entities | Same governance idea as AKR-TMS (AI drafts, human decides); the **Iraqi FIU publishes its XSD openly**, so a real-schema test is possible here |
| `munguambe-eng/kairos-lex` | Python generator that writes goAML 5 XML (report types DEFT, DIFT, DCTR) from a database | Example of batch CTR/EFT generation |

No maintained general-purpose goAML library (Rust, Java or Python) with FIU profiles was found. A Rust implementation on `quick-xml` with schema-driven code generation and a separate validation layer would be AKR-TMS's own work; libxml2 (through the `libxml` crate or `xmllint` as a sidecar) is the reference engine for XSD 1.0, but see 2.5 for the pattern defect that needs a compatibility shim.

### 7.3 Catalogue of business-rule checks to run after the XSD (from the Sri Lanka validator, adapted)

1. **My client versus not my client consistency:** an account whose institution SWIFT equals the reporting bank's must be `my_client`; a `my_client` account's institution must be the bank itself; a person who looks like a customer but is sent in a "not my client" node is flagged.
2. **Identity checks for own customers:** nationality present; national ID or passport present and, for Iraqis, valid in format (Iraq: the unified national card number and the civil status document numbers: format to be agreed with each bank; NOT VERIFIED), foreigners' passports in `passport_number`; directors of entities listed with nationality and ID.
3. **Entity checks:** invalid characters in names (as set per FIU); incorporation number present for entity types that need one; business description must not contain placeholder words ("not found", "not available", "dummy"); own-customer entities need directors.
4. **Account checks:** no leading "/" or "\", no characters such as "*", "$", "^", no spaces; implausible account numbers (mostly letters); the amount must not equal the account number; an account has either an entity owner or signatories, not both; **internal GL accounts of the bank must not be reported as parties** (such transactions go as multi-party).
5. **Transaction checks:** transaction date on or before the submission date; **more than 31 calendar days between transaction and submission is a late submission flag**; location mandatory for branch and agent modes; amounts below a floor suggest an unconverted foreign amount; CTR amounts that are extreme or odd (not round) suggest an EFT sent as a CTR; cash transactions cannot have accounts on both sides; EFT sides must both be accounts; IFT needs one foreign country; CTR countries must be domestic.
6. Plus the Iraqi schema rules in 2.5, the Office's guidance in 2.3 and the quality score in 5.4.

### 7.4 AKR-TMS hooks (section 7)

- Build the exporter and validator in-house on the Office's XSD; no incumbent shows an Iraqi template, and the open-source tools are small and unmaintained. The competitive claim is "tested against the Office's published schema, with the Iraqi business rules and a quality grade".
- Follow Oracle's boundary (generate, approve, hand over, record the FIU reference) and go further on validation, rejection handling and the inbound request desk.
- Offer Pio-Tech-warehouse banks a **reader** for their goAML data (replays of what was filed) so AKR-TMS can learn each bank's historical reports and FIU responses without replacing their filing tool on day one (file 18 positioning).
- Package the Sri Lanka-derived rule catalogue (7.3) as the seed of the Iraqi rule set, with every rule documented and owned; cite the source as a public validator, not as an Iraqi requirement.
- Keep the regression corpus and the "FIU answers" corpus as bank-owned assets (they contain real rejection messages) stored in the suspicion tier.

## 8. Design of the AKR-TMS reporting module

### 8.1 Principles

1. **AKR-TMS prepares, a named person files.** The goAML portal belongs to the FIU; AKR-TMS holds no goAML credentials and writes nothing to the CBS. It produces an approved, validated, hash-sealed package that the MLRO or deputy uploads, and it records the upload and the answers. This matches Oracle's own pattern (section 7.1: generate E-File, human submits, human records the FIU reference). A machine submission adapter is possible only if the Office offers an interface (not found; NOT VERIFIED).
2. **One canonical report model, many serialisations.** The report is stored as typed data (the model in 8.3) and rendered to goAML XML (Iraq profile), to a PDF in Arabic and English, to CSV for the IQTFS forms, and, for optional regime packs, to FinCEN SAR XML 2.0, the UK SAR Portal field layout, the EU draft ITS data points, and other goAML profiles. The model is wider than any one FIU's schema.
3. **The FIU's published schema is the contract**, loaded from the XSD file (profile), never hard-coded; lookups and cardinalities come from it.
4. **AI drafts and ranks; a named human decides.** The narrative draft, the quality score and the suggested indicators are advice; the maker, the checker and the MLRO each sign.
5. **Every figure traces to source.** Every number in a report links to rows in an evidence snapshot (hash), because the D2 auditor tests traceability "of the data used to detect and report suspicious activity".
6. **Tipping-off by design.** Reports and their metadata live in the suspicion tier (file 30 section 8).

### 8.2 Components and data flow

```text
CBS replica (read-only) -> canonical model (file 25 s.13) -> alert -> case (file 28) -> evidence snapshot
                                                                            |
                      MLRO decision: STR | SAR | TFR | UTR | close with reasons
                                                                            v
 report builder (maker) -> pre-submission validation (L0-L5) -> quality score -> checker -> MLRO approval (step-up)
        |                                                                                      |
        | AI narrative draft (local model,                                                     v
        | evidence-bound, numbers re-checked)                                       package (XML + PDF + attachments
        v                                                                           + manifest + validation report)
 FIU profile registry (Iraq XSD 5.0.2 +                                                         |
 patches; lookups; code maps; lint rules)                                    MLRO uploads in goAML Web (manual)
                                                                                                |
 clocks (8.10) <- report register -> acknowledgement / rejection / FIU request desk (8.9) <-----+
        |
 evidence vault (object lock) + audit trail + retention classes (8.14)
```

Placement: a bounded context `reporting` beside `case` and `kyc` in the Rust workspace (file 32 maps the stack): `akr-reporting` library (profile loader, mapper, serialiser, validators, scorer), jobs in `akr-worker` (CTR batch, clocks, calendar), API under Axum, UI in React with RTL. XML is written with a streaming writer (a Rust XML writer crate such as quick-xml is the natural choice; NOT VERIFIED as a project decision); validation uses libxml2 as the reference engine (sidecar or binding) plus the compatibility shim of 8.6, with the profile's own regex facets re-implemented in Rust for speed.

### 8.3 Data mapping: canonical model to goAML (Iraq profile)

**Envelope (`report`)**

| goAML element | Source | Rule |
|---|---|---|
| `rentity_id` | bank setting (assigned by the Office at registration) | integer >= 1 |
| `rentity_branch` | branch of the main subject; default "Head Office" | required text; Arabic or English |
| `submission_code` | constant `E` | |
| `report_code` | decision outcome | STR, SAR, TFR, UTR, CTR, AIF (profile list) |
| `entity_reference` | AKR-TMS report number `RP-<type>-<year>-<seq>` | unique forever per bank; <= 255; also the key shown to the MLRO |
| `fiu_ref_number` | for AIF replies: the Office's request reference | |
| `prev_rejected_ref_number` | when re-filing a rejected report (flagged 5.1; emit only if the profile enables it) | |
| `submission_date` (or `report_date`, 5.0) | export time, local time (UTC+3, no DST), format `YYYY-MM-DDThh:mm:ss` | `submission_date` "will be deprecated in Schema 6"; profile flag chooses |
| `currency_code_local` | `IQD` | |
| `reporting_person` | MLRO or deputy profile: name, birth date, ID, occupation "MLRO", phone, e-mail | or `reporting_user_code` (5.1) if the profile enables it; goAML Web fills it automatically for web reports |
| `location` | bank head office address (`city` and `state` required) | |
| `reason` | final narrative, <= 8,000 characters | structure in 8.5; Arabic text allowed |
| `action` | action taken (enhanced monitoring, hold, refusal, closure) | the Office's manual requires it |
| `transaction`+ or `activity` | STR, CTR, TFR: transactions; SAR, UTR without a transaction, AIF without transactions: activity | |
| `report_indicators` | indicator codes from the Iraqi list (8.3 below) | at least one |

**Person (`t_person_my_client` for customers, `t_person` for others)**

| goAML | Canonical source | Iraqi rule |
|---|---|---|
| `first_name`, `middle_name`, `last_name` | `party_name` components: given, father, grandfather, family or tribe | first = given name; middle = father and grandfather (and great-grandfather); last = family or tribe name; if no family name, last = grandfather and middle = father; no ASCII punctuation (the forbidden class is quoted in 2.5); the full Arabic name is also kept in `alias` or `full_name_frn` (5.2) for matching by the Office |
| `mothers_name` | `party.mother_name` (two-part where held) | required; empty allowed by the schema but flagged by the quality score |
| `birthdate` | `party.date_of_birth` | `YYYY-MM-DDT00:00:00`; if only the year is known, `YYYY-01-01T00:00:00` plus a comment "year only" |
| `birth_place`, `country_of_birth` (5.2) | place of birth | |
| `gender`, `title` | party | `M` or `F`, else `-` |
| `id_number` | national ID number | required for customers |
| `identification[]` | `party_identifier`: national card (A), unified card, civil ID, nationality certificate, residence card; passport (C) with `passport_number` and `passport_country` | `issue_country` required (IQ) |
| `nationality1..3`, `residence` | party | ISO 3166 codes from the XSD list |
| `phones` | `party_contact` phone | `tph_country_prefix` `964`, number without leading zero; contact type P/B/O/R, communication M/L |
| `addresses` | structured address | governorate to `state`, district to `city`, neighbourhood and street to `address`; both required |
| `email` | contact | up to 5; TLD <= 8 letters, ASCII only |
| `occupation`, `employer_name` | customer KYC | `occupation` required for customers |
| `source_of_wealth` | KYC source of wealth | required for customers; "UNKNOWN" convention only with a flag |
| `peps` (5.2) and indicator `PEPL` or `PEPI` | PEP status | |
| `related_persons` (5.2) | `party_relationship` | emit only if the profile enables |

**Entity (`t_entity_my_client` or `t_entity`)**: `name` (legal name; pattern shim, 8.6), `commercial_name`, `incorporation_legal_form` (map to the Iraqi list: A Private Limited, A5 Joint Stock, A6 Cooperative, A14 Governmental institution, A17 Branch, S Sole proprietorship, ...), `incorporation_number` (Companies Registration Department number), `business` (ISIC text), `incorporation_date`, `incorporation_country_code`, phones and addresses (required for customers), `director_id[]` (each a `t_person` plus role from the entity-role list: A Director, A1 Shareholder, A2 CEO, A10 Chairman, UBO, RBO, ...), `tax_number`.

**Account (`t_account_my_client` or `t_account`)**: `institution_name`, `swift` or `institution_code` (CBI bank code), `branch`, `account` (CBS account number), `iban` (Iraqi IBAN), `currency_code`, `client_number` (CIF), `personal_account_type` (map product family: B current, C savings, J term deposit, D foreign-currency account, E mortgage, G personal, A business, ...), `signatory[]` (`is_primary` as `1`, person, role: A1 main holder, A2 co-holder, A3 to A5 authorised, A8 power of attorney, A9 guardianship, ...), `opened`, `closed`, `balance` and `date_balance` (from `balance_snapshot` at the transaction date), `status_code` (A active, B inactive, C dormant, D closed by customer, E closed by bank, F closed for inactivity, G no transactions within a year, H new), `beneficiary` (<= 50, required for customers). For accounts at other banks: name, SWIFT, account or IBAN, account name, country, from `payment_message_party` or `counterparty`.

**Transaction**: `transactionnumber` = CBS reference (<= 100; prefix with bank code if the FIU requires, as Pakistan does), `internal_ref_number` = AKR-TMS transaction id, `transaction_location` = branch name or "online", `transaction_description` = canonical type plus narrative, `date_transaction` = posting time, `teller` and `authorized` = operator and authoriser ids (<= 50), `late_deposit` (1 when posting is later than the transaction date; then `date_posting`), `value_date`, `transmode_code` from the canonical channel and type (A in-branch, B ATM, C electronic, AT wire transfer, AU withdrawal, AD deposit, AP transfer to other party, AQ transfer to own other account, AF domestic bank to bank, AB currency exchange, AG loan, AH loan repayment, AA credit-card purchase, **1 bank transfer for real-estate sale**, AR transport across border), `amount_local` (IQD, two decimals), `from_funds_code` and `to_funds_code` (K cash, P cheque, B electronic funds transfer, E bank draft, L from account, LL credit card, O mobile payment, J securities, A e-currency, F money order, ...), foreign-currency block when the currency is not IQD (code, foreign amount, rate and its source), country of origin and destination, `t_conductor` for the person who physically conducted the transaction, `transaction_is_suspicious` (5.1; emit if enabled) per transaction selected by the analyst as part of the suspicion.

**Multi-party transactions** (`involved_parties/party`, used when the event has no clear From and To, for example a property sale with several buyers, or an internal GL movement): each party has a role from the Iraqi list of 46 codes (A payee or sender, B beneficiary or receiver, C conductor, D bought on behalf of, E buyer, AD seller, AF source party, AG suspected beneficiary or on whose behalf, AH suspected conducting or acting on behalf, BB associated with, G conducting or acting on behalf of, I destination party, XAH account holder, XSH source primary account holder, XDH destination primary account holder, ...), optional funds code and country, significance 0 to 10 and a reason.

**Report indicators (Iraqi list, 62 codes)**: detection method (IDA automated rules-based monitoring, IDB teller identified, IDC manual account monitoring, IDD manually identified); predicate offence (OA corruption, OB drugs, OC fraud, OD money laundering, OE organised crime, OF proliferation, OG tax evasion and smuggling, OH terrorism and TF, OI human trafficking and migrant smuggling, OJ sexual exploitation, OK illicit arms, OL stolen goods, OM counterfeit currency, ON counterfeiting and piracy, OO environmental crime, OP murder and injury, OQ kidnapping, OR robbery or theft, OS extortion, OT forgery, OU piracy, OV insider trading); typology (TA activity does not match client profile, TB purchase of securities or high-value goods, TC smurfing, TD structuring, TE trade-based ML, TF casinos, TG nominees and trusts, TH hawala, TI offshore services, TJ shell companies, TK family members and third parties, TL gatekeepers, TM new payment technologies, TN denomination conversion); PEP (PEPL local, PEPI international); LOFAC (appears on OFAC or other list); adverse media (RADVC commercial databases, RADVI international, RADVL local press); insider (BRI no longer affiliated, BRIS still affiliated); MIS (material impact on the bank); Iraqi codes: **3 listed in local freezing lists, 4 amount seized, 5 amount attached, 6 human trade, 7 human smuggling, 1 seizing, 2 declaration, A1 wire transfers, A2 internal transfers, A3 counterfeit cheques, A4 cheque endorsement, A5 use of the foreign-exchange window**. AKR-TMS maintains a map from its scenario catalogue (file 27) and typologies to these codes, and the analyst can add or remove.

### 8.4 Report types and what builds each

| Type | Built from | Notes |
|---|---|---|
| **STR / TFR** | case, selected suspicious transactions (bi-party, with my-client flags), parties, narrative, indicators, action | TFR is the TF variant (a match that is not a pure freeze, a TF suspicion) |
| **SAR** | case without a transaction (attempted, refused onboarding, behaviour): `activity/report_parties` | includes "STRs filed because CDD could not be completed" (MER: 132 by banks) |
| **UTR** | case with outcome "unusual, no suspicion yet"; lighter template (description, reasons for classifying as UTR, supporting data) | sets a watch policy (enhanced monitoring, default 90 days, bank-set) and a review date; escalation creates an STR or SAR that cites the UTR number and date |
| **CTR** | daily batch (8.11) | no case |
| **AIF** | the Office's request (inbound register) | transactions and statements assembled from the replica and vault; `fiu_ref_number` = the Office's reference; the EU annex II.1 layout is the model for a statement |
| **Freeze report, match report** | match record and freeze task (8.12) | PDF and CSV; channel per bank setting |
| **Internal unusual-transaction report** | staff observation form to the liaison officer | not an external report; acknowledgement and decision notice are generated |

### 8.5 The report builder (Arabic and English)

- **Layout:** a stepper that mirrors the goAML structure so the MLRO recognises it: (1) decision and type, (2) subjects (persons, entities, accounts with my-client versus other), (3) transactions (selectable table with totals, "suspicious" flag per row, chronological order), (4) narrative, (5) indicators and predicate, (6) action taken and previous reports, (7) attachments, (8) validation and quality, (9) preview, (10) approvals. RTL-first with English toggle (file 32 section on the UI stack); Arabic labels use the Office's own wording (for example "الإجراء المتخذ", "أسباب ودواعي الاشتباه").
- **Narrative template (Arabic default, English available), following the Office's 5W+H and FinCEN's three parts:** (a) one-sentence summary and type of suspicion; (b) the pattern with numbers: count, total, period, comparison with the declared profile, channels and branches; (c) parties and roles, relationships, beneficial owners; (d) chronology (the system inserts a generated table of the selected transactions; the analyst writes only the interpretation); (e) what the bank checked and found (documents, adverse media, screening, customer contact without tipping off); (f) action taken and intended follow-up; (g) previous reports (number and date) and whether the suspicion arose from an automated alert (the Office asks for this). Maximum 8,000 characters, counted live; the builder warns at 90% and refuses to export above the limit.
- **AI drafting:** the local model receives the evidence snapshot only; each sentence carries tags for the transactions or documents it relies on; a checker recomputes every number, date and name against the snapshot and blocks export if one does not match; unsupported claims are highlighted; the draft, its prompt, model version and evidence hash are stored as a version (file 29). The analyst edits; the system never auto-files.
- **Text hygiene:** UTF-8; Unicode NFKC normalisation of Arabic presentation forms; strip bidi control characters and non-printable characters (the FinCEN guide rejects such characters; goAML behaviour NOT VERIFIED); Western digits for amounts and dates inside structured fields; Arabic-Indic digits allowed in free text but a warning is shown because keyword search by the Office may be digit-sensitive.
- **Side-by-side version compare** (FinCEN-style locked final versions, Flagright pattern): every approval locks a version; a correction creates a new version linked to the old one.

### 8.6 Pre-submission validation: six layers

| Layer | What | Engine | Outcome |
|---|---|---|---|
| L0 | Canonical completeness: required canonical attributes present for every party and transaction selected (mother's name, birth date, ID, address parts, phone, source of wealth, account opened date, beneficiary) | rules in Rust over the case snapshot | list of gaps with "fill from CBS", "mark UNKNOWN with reason", or "cannot report" |
| L1 | **XSD validation** against the Iraq profile (libxml2 reference) | profile loader | hard block |
| L1a | **Pattern compatibility shim**: the entity-name pattern in the Office's XSD cannot be satisfied under XSD 1.0 rules (section 2.5); the profile carries an override that treats `^` and `$` as anchors and, until the Office confirms its server's behaviour, also applies a conservative sanitiser (letters, digits, space, hyphen, ampersand, apostrophe; Latin and Arabic letters; others transliterated or removed with a logged change) | profile patch | warning with the changed value shown to the maker |
| L2 | **Iraqi business rules** (8.6.1) | rule engine, versioned, testable | hard block or warning per rule |
| L3 | **Quality score** (5.4) | scorer | advice; MLRO may override with a reason |
| L4 | **Consistency**: narrative numbers equal structured totals; dates in range; each transaction's accounts present in the party list; the "suspicious" flags sum equals the amount reported; the narrative language matches the setting | checker | hard block for number mismatches |
| L5 | **Package sanity**: file size, number of transactions per file, ZIP structure, attachment types (PDF, images), antivirus scan, no active content | packager | hard block |

**8.6.1 Iraqi business rules (initial set; sources in brackets)**: `reason` and at least one indicator present [XSD]; predicate offence code or "not identified" with a reason [Office manual item 4]; **action taken present** [manual]; **previous report reference present when the subject was reported before** [manual]; each transaction has date, amount, currency, channel [manual 5W]; every natural person has a mother's name [XSD, TFS guide]; `late_deposit` consistent with posting date; transaction date not after submission date; 31-day lateness flag (Sri Lanka validator pattern) shown as a timeliness warning, because the Iraqi legal standard is "immediately"; account-number hygiene (no spaces, no leading slash); internal GL accounts must not appear as parties; my-client consistency (account of this bank must be my-client); entity and signatories not both on one account; CTR consistency (cash on one side, account on the other; both domestic; amount at least the threshold); amount not equal to the account number; names free of forbidden punctuation; IQD and USD only unless a transaction says otherwise; email shape. Each rule has an ID, a severity, an owner, a version and tests; a rule change goes through maker-checker like a scenario change.

**8.6.2 Test corpus and the FIU's test environment.** The product ships the three validated samples of this file (appendix A) and 63 negative and edge cases as regression tests; it can run a "dry run" file through the Office's test environment if one exists (Pakistan and others provide one; Iraq's is NOT VERIFIED) and records the result. Every XSD refresh re-runs the corpus.

### 8.7 Workflow, maker-checker and MLRO approval

States: `DRAFT` (maker) -> `READY_FOR_REVIEW` (validation passed or overridden) -> `IN_REVIEW` (checker) -> `REWORK` or `APPROVED_BY_CHECKER` -> `MLRO_APPROVED` (step-up authentication, signs the manifest hash) -> `PACKAGE_READY` -> `FILED` (upload recorded) -> `ACKNOWLEDGED` or `REJECTED` or `RETURNED` or `ACCEPTED_WITH_ERRORS` -> `CLOSED`; side states `CANCELLED` (with reason), `REOPENED`, `SUPERSEDED` (a correction filed). Oracle's statuses are the closest public analogue (Open, Requested for Approval, Rework, Approved, E-File Generated, Submitted, Acknowledged, Rejected from E-File, Canceled). Rules: maker is not checker; checker is not MLRO unless the bank has only one compliance officer (then a recorded exception and a second signature by a named board-level person for TFR and STR above a bank-set size); the MLRO or deputy only can approve for filing (the Office accepts reports from the MLRO or deputy); all transitions carry reason text, timestamps and the evidence hash; the approval is bound to the exact manifest hash, so a changed file needs a new approval.

### 8.8 Export, upload and the package

`package-<entity_reference>.zip` holds: `<entity_reference>.xml` (UTF-8, no BOM); `attachments/` (PDF, images; names ASCII-safe; the goAML ZIP convention is FIU-specific and NOT VERIFIED for Iraq, the UAE guide says XML plus attachments may be zipped); `report.pdf` (Arabic and English rendering, watermarked for internal use, not for upload); `manifest.json` (SHA-256 of each file, profile and rule versions, validation report id, score, approvals); `validation.json`. Limits are profile settings (Swiss goAML Web: 300 MB per XML or ZIP, 20 MB per attachment, one XML per ZIP; the UAE guide allows several XML files in one ZIP; Iraq's limits are open questions). A second output mode, **semi-automation**, writes only the `<reportdata><transactions>` wrapper with the selected transactions for upload into a report the MLRO creates in goAML Web (the UNODC semi-automation note; Pakistan's STR-automation phase 1 uses the same idea). The MLRO downloads it with step-up and a purpose, or the bank's file share receives it in an outbox; the MLRO uploads it in goAML Web (New Reports, XML upload) from the bank's workstation; AKR-TMS records "filed" with the uploader, time and any on-screen reference. No automated driving of the portal is planned: it would need the MLRO's goAML credentials inside AKR-TMS and sits outside the read-only, no-credentials principle (the portal's terms of use were not read).

### 8.9 Acknowledgement, rejection and the Office's requests

**Capture first.** goAML Web may delete submitted reports after days (Swiss configuration: 7 days after transmission, drafts 14 days, Message Board items 30 days), so the module asks the MLRO to store the as-filed XML, the on-screen receipt and every Message Board item in the evidence vault at filing time and daily after, warns before any portal purge window set in the profile, and treats a missing capture as an exception.

Inbound events are entered by the MLRO (paste of the message text, upload of the goAML receipt, or a mailbox rule if the bank allows) and become typed records:
- **Processed (technical acceptance)** and **Acknowledged / accepted by the FIU (content):** FIU reference stored; the report is locked; two separate states because a report can pass the automated checks and still be returned or rejected by FIU staff.
- **Rejected at upload (schema or business rejection rules):** error text parsed into field-level findings against the stored XML; the MLRO presses Revert in goAML Web (the bank's draft in goAML is a copy; the master stays in AKR-TMS), a corrected version is created with `prev_rejected_ref_number` (if the profile allows) and a new `entity_reference` suffix; the rejection and the fix are both kept.
- **Accepted with transaction-level rejections** (UNODC Rules Editor behaviour): the returned transactions become a sub-task.
- **Returned for completion ("poor")**: a task with the Office's text, a due date, and a link to the quality score; the supervisor notification is outside the bank's control.
- **Request for information (AIF or other):** registered in the restricted request register with the Office's deadline, an assembled draft (statements, documents), the same maker-checker path.
- **Instruction to refrain (up to 7 working days):** a task to bank operations with neutral wording and a clock; AKR-TMS cannot block the payment (read-only) so the task is confirmed by the operations user and later verified from the replica (no debit after the instruction); the clock end triggers a "release or extend" decision recorded.
- **Feedback or outcome shared by the Office:** stored on the case; feeds the quality and effectiveness reports.

### 8.10 Deadline clocks

Clock types and defaults (bank-configurable, with a recorded legal basis for any relaxation): **A.** alert to MLRO triage (1 working day); **B.** triage to decision (3 working days); **C.** decision to filing: "immediately" (same day target, escalation at 24 hours); **D.** suspicion-formed date as an explicit field that starts the "as soon as practicable" explanation burden (FINTRAC logic: "the greater the delay, the greater the need for a suitable explanation"); **E.** freeze: 8 hours default (24 hours and "without delay" are the other published texts), report within 48 hours, update at two months; **F.** Office request: the date it states; **G.** hold: 7 working days; **H.** CTR daily at a fixed time after the CBS business-date close; **I.** liaison monthly report by the 10th; **J.** regulatory calendar items (4.5). Working days follow the Iraqi public holiday calendar (reference data, effective-dated; weekend Friday and Saturday; holidays vary with lunar dates and government announcements: a manual override is needed). Alerts to the MLRO and a deputy at 50%, 80% and 100% of each clock, with notification text that reveals nothing (file 30 section 8).

### 8.11 CTR batch generation from CBS data

- **Input:** `transaction` rows with the `cash_event` flag, cash deposits and withdrawals, business date D, converted at the CBI rate of day D (`amount_usd` stored with the rate source) to test "USD 10,000 or more in any currency"; MER para 252 describes the practice as per transaction ("cash transactions, withdrawals and deposits equal to or exceeding USD 10,000"); whether linked or same-day transactions are aggregated is not stated, so aggregation is a bank-visible option with the default off and a note to ask the Office.
- **Run:** a worker job after the replica has the business-date close (completeness check: row counts and control totals against the CBS end-of-day cash GL), idempotent per (bank, D, run id), produces one or more XML files `CTR` (the Office's CTR code; CTR-D and CTR-R are for DNFBP groups), each with at most a configurable number of transactions (default 2,000; the Office's limit is NOT VERIFIED), one `transaction` per cash movement: deposit = `t_from` (person or entity conductor, funds code K) to `t_to_my_client` (the account); withdrawal = `t_from_my_client` (account) to `t_to` (person, funds code K); `transmode_code` A or B; `teller` and branch; `late_deposit`.
- **Control:** reconciliation report (count, sum by currency, by branch; tie to the CBS cash GL; list of excluded rows and why), second-person approval of the batch (a checker confirms the control totals, not every row), same package and upload path, acknowledgement recorded per file; late or missing days are a flagged exception, as are days where the replica was incomplete; correction files for corrected CBS data ("replacement" semantics are NOT VERIFIED for Iraq; default is a new file marked as correction in `entity_reference` and `reason`).
- **Quality hooks:** CTRs with extreme or non-round amounts, cash on both sides as accounts, or foreign country are flagged by the Sri Lanka-derived rules (7.3); a conductor without identification is blocked or marked UNKNOWN with a reason.
- Scale: volumes are small: 76,369 CTR transactions in 32,380 files in 2022 for all banks together (about 210 transactions a day), and 242,884 CTRs in 2023 to 2025 (the CBI deck does not say whether files or transactions); file size is not the constraint, completeness, timeliness and correctness are.

### 8.12 Frozen-asset, match and TFS reports

- **Match record:** states `potential`, `true`, `false` (the Office's wording) with reasons; a true match creates a **freeze task** (bank operations; neutral wording; clock 8 hours default) and a **report task** for the Committee with a copy to the Office (clock 48 hours), then a **second report** task at two months (status of the frozen funds); content per art. 16 of the Office's guide: listed person data, source of listing, discovery date and time, freeze date and time, type and value of funds, account or reference, measures taken, other information. Output: a controlled PDF (Arabic and English) and a CSV with the IQTFS "Frozen Assets & Funds Volume" fields (individual or entity, name, freezing decision number, decision date, amount, last transaction date, estimated value of frozen immovable assets, exceptions); the lodging channel is a bank setting (IQTFS form, letter, e-mail to the Committee secretariat, or goAML TFR) because the Office does not publish it.
- **Verification:** the replica is read after the freeze time to confirm the hold exists on every affected account and that no debit, including fees, occurred (file 05 R-52); an exception is a P1 alert to the MLRO.
- **Attempted transactions** by listed persons are included in the report and also trigger a TFR or SAR decision.
- **Delisting and releases:** a release instruction from the Committee closes the freeze with documentation (art. 25 of the Office's guide: update systems, lift restrictions, record date and time, keep the record).

### 8.13 Tipping-off controls specific to the reporting module

Report objects, versions, XML, attachments and acknowledgements carry the suspicion tier and per-case access lists; the report register is excluded from global search and counts; any staff member's access to a report or to a case with a filed report passes the relationship-conflict check (file 30 section 8.5); exports need purpose and approval, are watermarked, and are logged; the package outbox is encrypted at rest and purged after confirmed upload plus a bank-set retention; notification text is generic ("task waiting"); the PDF carries no bank-wide "STR filed" banner on customer-facing systems; the module never writes an AML reason to the CBS; a "tipping-off test suite" attempts to learn from the UI, API, search, counts, timings and error messages whether a report exists for a given customer, and fails the build if it can.

### 8.14 Retention and the evidence pack

Per filed report the vault holds, under object lock with the case retention class (R6, file 30 section 12.3: floor five years from the report date or the ruling, default ten years): the XML exactly as uploaded, attachments, `report.pdf`, `manifest.json` with hashes, every narrative and report version with author and time, the alert list and scoring at decision time, the evidence snapshot (transactions, party profiles, list-match details, scenario versions), the MLRO decision and reasons, maker, checker and MLRO approvals with step-up evidence, the upload record, acknowledgements, rejections, the Office's communications, the quality score and the reviewer's calibration, and the clock log. **Evidence-pack views:** (1) the **D2 auditor view** (list of reports with decision times, reasons for no-report decisions, traceability from each report to source rows, rule and threshold history); (2) the **inspector view** (a single report with its complete trail, suspicion tier, MLRO release); (3) **Office reply view** (only what the Office asked for, assembled per request). Closed-without-filing decisions are kept in the same way (the CBI requires records of cases the MLRO closed).

### 8.15 Profile governance (schema versions and the Office's changes)

A **profile** = the XSD file + patches + lookup maps + rule set + report-type list + emit flags (`report_date` versus `submission_date`, `account_type` versus `personal_account_type`, `related_persons` versus `signatory`, 5.1 and 5.2 elements) + attachment convention + size limits, versioned and signed. The Iraqi default profile is "IQ-5.0.2-2026-09" (XSD uploaded 17 Sep 2026). A watcher fetches the Office's attachment list (public API) daily, compares a hash, and on change opens a task: diff (elements, cardinalities, lookups), run the corpus, mark profile "candidate", test in the Office's test environment if available, then promote with maker-checker. Reports in flight keep the profile version they were drafted under until re-validated. Deprecation notes inside the XSD ("will be deprecated in Schema 6") are shown as warnings.

### 8.16 Reporting analytics (management information)

Counts and times by type, branch, analyst, scenario and typology: alerts to cases to UTR, STR, SAR, TFR, closures; median and 90th percentile times for triage, decision and filing; share of reports passing validation first time; rejection reasons; quality score distribution against the Office's grades; FIU responses (acknowledgement, requests, holds, returns); freeze clocks; CTR runs and exceptions; backlog and ageing; all for the board pack and the CBI, with small-number suppression for non-compliance roles. These are the "effectiveness evidence" (file 05 R-03) and the metrics Wolfsberg says matter (file 04 section 2a: alert productivity, STR completeness, false-negative assessment).

### 8.17 What the module will not do

It will not submit to goAML on its own; it will not store goAML credentials; it will not decide to file; it will not change CBS data; it will not guess the Office's undocumented rules (it lists them as open questions and tests them); it will not replace the bank's legal advice on tipping-off.

## 9. What it means for AKR-TMS

**Findings that change the product design (in order of weight):**

1. **The Iraqi FIU publishes its goAML XML schema, and it can be tested today.** The XSD (5.0.2, uploaded 17 Sep 2026) is public at the Office's site API. None of the vendors found in file 18 (Pio-Tech, Capital Banking) or in file 09 (Oracle) publishes a test against it. A sample STR, CTR and UTR built for this file validate against it; 63 test cases show which errors the schema catches. This is a concrete, demonstrable advantage for the plan.
2. **The published XSD has a defect and several Iraqi tightenings that break generic exporters.** Mother's name is required on every natural person; address `city` and `state` are required everywhere; person names reject apostrophes and dots; email top-level domains over eight letters fail; dates need a time; `is_primary` must be `1`; unknown lookup codes fail the whole file; and the entity-name pattern `^...$` cannot be satisfied under XSD 1.0 (the `^` and `$` are literals), so a strict validator rejects every entity name. A reporting module must carry a profile with a compatibility layer and a business-rule layer; a "goAML-ready" export that only checks the XSD will fail in two ways.
3. **There is a new report, the UTR (first edition 2026, PDF dated April 2026), and a three-way MLRO decision.** The Office wants unusual-but-not-suspicious activity reported early through goAML, followed by enhanced monitoring and escalation. Case management must model it (file 28 should add the state).
4. **The deadline is "immediately", the Office's manual gives no number, and the freeze clocks conflict across its own documents** (8 hours in the March 2023 manual, 24 hours in the September 2026 guide, "without delay" in the Regulation, whose definitions clause also sets it at 24 hours from the listing decision (checked in the Arabic text by the lead, 2026-10-07); report to the Committee within 48 hours and a second report at two months; FIU holds up to 7 working days). The product needs a clock engine with a recorded legal basis, and file 25's "three-day Committee-notification clock" (requirement C-63) should be updated to 48 hours.
5. **The Office grades every STR (good, moderate, acceptable, poor), automatically since goAML, returns poor ones to the reporter and tells the CBI.** Bank STRs rated good rose from 51% in 2018 to 90% in 2022 (of 84 reports that year). A pre-submission grade that mirrors the Office's criteria (real indicators; documents; measures taken) protects the bank on exactly the metric the supervisor sees. The Office told banks on 10 Sep 2026 that the aim "is not merely to increase the number of reports, but primarily to enhance their quality and analytical value", and it has run feedback sessions with banks, insurers and exchange companies on "common errors" since January 2026 (2.7).
6. **The Office's 2025 manual is a content checklist:** who, what, when, where, how; each transaction in order; action taken; previous report reference; whether an alert raised it. The same checklist (with FinCEN's three-part narrative and the UK's "structured fields first") is the quality rubric.
7. **The portal belongs to the FIU, and nobody submits to it by machine.** The Swiss FIU's manual says plainly that "there is no automated upload of data from the internal systems of the reporting entity to goAML directly", and Oracle generates the E-File and records the FIU reference typed in by a person. AKR-TMS follows that pattern: package, approve, hand over, record, with no portal automation and no stored goAML credentials.
8. **Freeze reports go to the Committee, not (only) goAML.** The Iraqi XSD has no FFR or PNMR type (the UAE has both, Egypt has DTET). The channel is unpublished; the product produces a controlled document and the data for the IQTFS forms, and keeps the channel a bank setting.
9. **CTRs are a daily bank-wide batch, and small:** 76,369 transactions in 32,380 files in 2022 for all banks together (about 210 a day) and 242,884 CTRs in 2023 to 2025 (files or transactions, the deck does not say); the 2024 MER found no legal text behind the obligation in 2022 (an administrative mechanism), and whether the 2023 instructions changed that was not found. The generator needs completeness checks against CBS cash control totals and a replay path, not heavy scaling.
10. **The EU is moving to a common data set that is not goAML** (172 data points, draft July 2026); FinCEN SAR XML has its own acknowledgement and three error classes; the UK wants structured fields and glossary codes. A canonical report model wider than one FIU's schema is the safe design.

**What the product should not promise:** acceptance by the FIU (its business rules are unpublished and it can reject single transactions), a specific STR outcome, or that every Iraqi data field exists in the CBS (mother's name, source of wealth, occupation and address parts are often missing; the product must show gaps before the MLRO is at the deadline).

**Open questions to put to the AML/CFT Office and the CBI (list for the pilot bank's letter):**
1. Numeric STR/SAR/TFR/UTR deadline, if any; is the CTR file per day per bank, and the maximum size and transactions per file?
2. Does the server accept the elements flagged 5.1 and 5.2 in the XSD (`full_name_frn`, `peps`, `related_persons`, `transaction_is_suspicious`, `prev_rejected_ref_number`, `reporting_user_code`)? Which report date element (`report_date` or `submission_date`)?
3. How does the server treat the entity-name pattern, and which characters are really forbidden in entity and person names (Kurdish letters, digits, dots)?
4. Is there a pilot environment with the XML Report Validator that UNODC describes (it exists only in pilot environments), and a published list of rejection messages and business rejection rules? Does the server reject single transactions inside an accepted report? Does the portal purge submitted reports and Message Board items after a fixed number of days (the Swiss configuration does)?
5. Attachment convention for XML upload (ZIP structure, naming, size, types).
6. Duplicate `transactionnumber` across reports (a UTR escalated to an STR on the same transactions).
7. The channel and form for the 48-hour freeze report, the second report, and match reports; is a TFR expected for a true match?
8. How to reply to AIF requests and holds (message board format); the form of the instruction to refrain.
9. Whether CTR aggregation (same customer, same day) is expected; correction file semantics.
10. The code table descriptions for the Iraqi additions (indicators 1 to 7, A1 to A5; transmode `1`), and the meaning of report codes CBI, CR, DBS, MLCFC.
11. The format of the compliance officer's annual AML/CFT report to the CBI and the remedial-action report.
12. CRS and FATCA obligations of Iraqi banks under CBI or tax-authority instruction.

**Owner's answers (2026-10-07, from the product owner's knowledge; not yet confirmed in writing by the Office):**
- Q1: no fixed number of hours or days beyond "immediately". The internal target stays: file the same day as the decision, escalate at 24 hours (RP-47).
- Q2: the server accepts all the newer elements (`full_name_frn`, `peps`, `related_persons`, `transaction_is_suspicious`, `prev_rejected_ref_number`, `reporting_user_code`); the Iraqi profile emits them (RP-05). Which report-date element the server expects is still open.
- Q3: the schema's character rules apply as written (dots and apostrophes rejected in person names); company names are accepted normally, so the compatibility layer (RP-04) passes them and only logs.
- Q4: there is no test (pilot) environment; AKR-TMS relies on its own validation layers L0 to L5 (RP-27) and the regression corpus (RP-06); the dry-run path (RP-33) stays off in the Iraq profile. The list of rejection messages and the portal's purge period are still open.
- Q5: no set rules for attachments to an XML upload; AKR-TMS uses safe defaults (one ZIP per report, PDF and image files, size limits from the profile, RP-82).
- Q6: an STR that follows a UTR may reuse the same transaction numbers; the STR links back to the UTR (RP-19).
- Q7: the 48-hour freeze report goes to the Committee by official letter or email, not through goAML; the freeze report builder (RP-56) produces the letter and its annex, and the channel setting defaults to letter or email. Whether a TFR is also expected for a true match is still open.
- Q8: replies to the Office's information requests and holds go by official letter; the request desk (RP-44) and the instruction-to-refrain handling (RP-45) produce a signed letter, not a message-board post.
- Q9: CTR amounts are added up per customer per business day before the USD 10,000 test; the Iraq profile switches aggregation on by default (RP-51).
- Q10: the meaning of the Iraqi code-table additions is not known to the owner; it stays on the list for the Office.
- Q11: there is no set format for the MLRO's annual AML/CFT report or the remedial-action report; AKR-TMS drafts both from system facts in its own layout (RP-68).
- Q12: Iraqi banks have both FATCA and CRS duties; both modules ship, each switched on per bank (RP-69).

**Still open for the pilot letter:** Q10 (code tables), plus parts of Q2 (which report-date element), Q4 (rejection messages, purge period) and Q7 (whether a TFR is expected for a true match).

## 10. Derived requirements (prefix RP)

| ID | Requirement | Source |
|---|---|---|
| **Profiles and schema** | | |
| RP-01 | A goAML **profile registry**: each profile is an XSD plus patches, lookup maps, business rules, report-type list, clock set, emit flags and size limits, versioned and signed. The default Iraqi profile `IQ-5.0.2` is loaded from the Office's published XSD | 1.3; 2.1; 2.5; 8.15 |
| RP-02 | Cardinalities, lengths, patterns and lookup values (with the comment labels) are read from the profile's XSD at load time, never hard-coded; an unknown lookup code is a hard block (an undefined code fails the whole file) | UNODC spec (Nepal copy) s.5; 2.5 tests |
| RP-03 | A **profile watcher** fetches the Office's attachment list (api.aml.iq `GoAmlSystem/attachments`) at least daily, hashes the XSD, and on change produces a diff (elements, cardinalities, lookups), runs the regression corpus, and requires maker-checker promotion; reports in flight keep their profile version until revalidated | 2.1; 8.15 |
| RP-04 | A **pattern compatibility layer** for facets that cannot be met under XSD 1.0 rules (the entity-name pattern with literal `^` and `$`), a logged sanitiser for entity names, and an "open question" entry until the Office confirms its server's behaviour | 2.5 |
| RP-05 | Per-profile **emit flags** for elements flagged 5.0, 5.1 and 5.2 and for deprecated versus new elements (`submission_date`/`report_date`, `personal_account_type`/`account_type`, `signatory`/`related_persons`); default off until confirmed by the Office | 1.4; XSD notes |
| RP-06 | A **regression corpus** (three validated samples, the 63 negative and edge cases, and the anonymised FIU-answers corpus) runs on every profile or mapping change and in CI | 2.5; Appendix A |
| RP-07 | A **value normaliser** before serialisation: `YYYY-MM-DDThh:mm:ss` dates, `1` for fixed-1 flags, plain decimals, no thousands separators, trimmed values, NFKC for Arabic presentation forms, removal of bidi and non-printable characters, ASCII digits in structured numeric fields | 2.5; FinCEN XML guide s.3.2 |
| RP-08 | Several profiles can be active in one installation (for example Iraq plus a group branch in another country), each with its own clocks, templates and numbering | file 09 VA-30; 6.9 |
| RP-09 | An **open-questions register** per profile (the list in section 9), with the Office's answer, date and source once received | 2.6; 9 |
| RP-10 | **Code maps** per bank from CBS codes to goAML lookups (funds, transaction mode, account type and status, legal form, party roles, identifier types, indicators), effective-dated, maker-checker, with an unmapped-code coverage report | 8.3; file 25 |
| **Report types and content** | | |
| RP-11 | Report types **STR, SAR, TFR, UTR, CTR, AIF** plus the freeze report and match report; inbound Office requests (AIF requests, IRD, IRI, holds) registered read-only; CBI-specific types (BCR, CBI, CR, DBS, MLCFC) not produced | 2.3; 2.4; 3.1 |
| RP-12 | The MLRO decision has four outcomes (file STR/SAR/TFR; file UTR with enhanced monitoring; keep or close with written reasons; gather more evidence), each with timestamp, reasons and evidence snapshot; a UTR can be escalated to an STR or SAR that cites the UTR's number and date | 2.3; 2.4; CBI controls (file 01 s.2d) |
| RP-13 | The **internal chain**: staff observation to liaison officer to MLRO, written acknowledgment with the tipping-off reminder, written decision notice, liaison monthly report by the 10th (auto-drafted), all timestamps kept | file 01 s.2d; 3.2 |
| RP-14 | Every STR carries a chronological transaction table (each transaction with its own date, amount, currency, channel, counterparty) and a per-transaction "suspicious" marker | Office manual 2025 (when, what) |
| RP-15 | Content gates before approval: narrative present; action taken present; at least one indicator; predicate-offence code or a stated "not identified" reason; reference and date of any previous report on the subject; statement whether an automated alert raised the suspicion | Office manual 2025; 8.6.1 |
| RP-16 | **Activity-based reports** (`activity/report_parties`, significance 0 to 10, reason and comments per party) for SAR, UTR without transactions and AIF without transactions | 1.4; XSD |
| RP-17 | Mapping of parties, accounts and transactions per section 8.3, including Iraqi name splitting, mother's name, year-only birth dates, governorate to `state` and district to `city`, phone prefix 964, national ID in `id_number` and `identification`, signatory roles, account status codes, funds and mode codes | 8.3; files 22 and 25 |
| RP-18 | A maintained map from the scenario catalogue and typologies to the Iraqi **62 report-indicator codes**, with automatic detection-method codes (IDA, IDB, IDC, IDD) and PEP and list flags; the analyst can edit | XSD lookups; file 27 |
| RP-19 | **Related reports** stored as structured links (relation: previous, amending, superseding, escalated from UTR) and rendered in the narrative's first lines with number and date | Office manual; UK guidance; EU DP0006 |
| RP-20 | An **AIF and statement builder** (transactions, statements, documents from the replica and vault) with the Office's request reference, due date, maker-checker, and an ISO-20022-style statement layout (EU annex II.1) as the internal model | 3.1; 6.7 |
| **Builder, narrative, AI** | | |
| RP-21 | Arabic-first RTL builder with English toggle, steps mirroring the goAML structure, Arabic labels from the Office's wording, live character counter (8,000), refusal to export above the limit | 8.5; XSD |
| RP-22 | Narrative templates in Arabic and English (5W+H; FinCEN introduction, body, conclusion; UK four steps) with an auto-generated chronology table | 5.1; 8.5 |
| RP-23 | **AI-assisted drafting** under the governance rule: evidence snapshot only, sentence-level source tags, automatic recomputation of every number, date and name, unsupported-claim highlighting, versioned drafts with model, prompt and evidence hash, no auto-filing | file 29; 5.4; 8.5 |
| RP-24 | Locked report versions at every approval; side-by-side comparison; corrections create linked versions | Flagright and FinCEN patterns (file 13); 8.5 |
| RP-25 | Attachments with type and size checks, antivirus scan, hashing, and a list inserted in the narrative; no "see attached" in place of narrative | FinCEN guidance; 8.5 |
| RP-26 | A configurable **UNKNOWN convention** per field (value, reason code) that distinguishes unknown from missing and feeds the quality score | UK guidance; 8.6 |
| **Validation and quality** | | |
| RP-27 | Six validation layers L0 to L5 (canonical completeness, XSD, compatibility, Iraqi business rules, quality, consistency, package) with severities, stored results and an override that needs a reason | 8.6 |
| RP-28 | A **business-rule catalogue** (Iraqi and the Sri Lanka-derived checks) with IDs, owners, versions and tests; rule changes through maker-checker | 7.3; 8.6.1 |
| RP-29 | A **quality score 0 to 100 and a four-level grade** (good, moderate, acceptable, poor) mirroring the Office's criteria (real indicators of suspicion; documents; measures taken) plus the rubric of 5.1; monthly second-review sample to calibrate the score | MER para 259; 5.4 |
| RP-30 | A **consistency checker**: narrative numbers equal structured totals; dates inside the transaction range; every transaction's accounts in the party list; "suspicious" sums equal the reported amount | 8.6 |
| RP-31 | **Timeliness metrics**: first alert, triage, decision, filing, and a mandatory explanation field when the interval exceeds a bank-set threshold | FINTRAC guidance; 8.10 |
| RP-32 | A **feedback register** per report: acknowledgement, grade if returned, requests, holds, outcome; used for quality and effectiveness reports | MER paras 259, 266; file 04 s.2a |
| RP-33 | A **dry-run path** to the FIU test environment where one exists, with results stored | 8.6.2 |
| **Workflow and approvals** | | |
| RP-34 | State machine of 8.7 (draft, review, rework, checker-approved, MLRO-approved, package ready, filed, acknowledged, rejected, returned, accepted with errors, closed, cancelled, reopened, superseded); maker is not checker; only the MLRO or deputy approves for filing; approval is bound to the manifest hash and needs step-up | 8.7; Oracle guide |
| RP-35 | MLRO and deputy registry with contact details as notified to the Office; a task to notify the Office and the CBI on any change | Office manual; file 01 s.2d |
| RP-36 | Exception rule for banks with one compliance officer: recorded exception plus a second named signatory for TFR and large STRs | 8.7 |
| RP-37 | Roles for the reporting module (maker, checker, MLRO, deputy, auditor read-only, none for vendor staff), with branch users excluded entirely | file 30 s.7, 13 |
| **Export, upload, acknowledgement** | | |
| RP-38 | A **package** per report: XML (UTF-8), attachments, bilingual PDF, manifest with SHA-256 and versions, validation report | 8.8 |
| RP-39 | **No goAML credentials stored and no portal automation**; manual upload recorded (uploader, time, on-screen reference); an adapter interface for a future machine channel if the Office provides one | 7.1; 8.1; 8.8 |
| RP-40 | Controlled download or outbox with step-up, purpose, watermark, logging and purge rules | 8.8; file 30 s.8 |
| RP-41 | Acknowledgement capture (reference, receipt upload) that locks the report | 8.9 |
| RP-42 | **Rejection handling**: error text parsed into field findings against the stored XML; corrected version with a new `entity_reference` and `prev_rejected_ref_number` where the profile allows; rejection and fix both retained; transaction-level rejections create sub-tasks | 8.9; UNODC Rules Editor |
| RP-43 | A **returned-for-completion** task type (from a "poor" grade) with the Office's text, due date and link to the score | MER para 259 |
| RP-44 | An **inbound request desk** for the Office's AIF requests, holds and Committee communications: restricted register, case-level access list, due dates, reply builder, maker-checker | 3.1; file 04 E-01, E-07 |
| RP-45 | **Instruction-to-refrain** handling: task to bank operations with neutral wording, a 7-working-day clock, confirmation by an operations user, post-check from the replica that no debit occurred, release or extend decision recorded | Law art. 9/First/c; MER para 266 |
| **Clocks** | | |
| RP-46 | A **clock engine** with per-profile clocks, working-day calendars (Iraqi weekend Friday and Saturday, manual holiday override), pause and resume with reasons, and alerts at 50%, 80% and 100% with content-free notification text | 8.10; file 30 s.8 |
| RP-47 | Iraqi default clocks: STR/SAR/TFR/UTR "immediately" (internal target and escalation at 24 hours), freeze 8 hours (24 hours and "without delay" selectable with a recorded basis), freeze report 48 hours, update two months, FIU hold 7 working days, CTR daily, liaison report by the 10th | 3.1; 3.4 |
| RP-48 | A **suspicion-formed date** field on every case, separate from transaction and filing dates | Jordan form; FINTRAC; 8.10 |
| RP-49 | A **regulatory calendar** of recurring reports (CBI reports, D2 assessment, PEP statistics, training plan, CTR runs, FATCA, CBDDQ refresh) with owner, due date, status and evidence | 4.5 |
| **CTR** | | |
| RP-50 | A **daily CTR job** after the business-date close, with replica completeness check and control totals tied to the CBS cash general ledger; idempotent per bank, business date and run | 8.11 |
| RP-51 | Threshold test on the **USD equivalent at the CBI rate of the day** (rate and source stored), any currency, deposits and withdrawals; aggregation as an option, default on in the Iraq profile (owner, 2026-10-07; was default off), noted as an open question | MER para 252; 9 |
| RP-52 | File splitting by a configurable transaction count, a reconciliation report (count and sum by currency and branch, exclusions with reasons), checker confirmation of control totals, correction files, exception log for missing or late days | 8.11 |
| RP-53 | CTR-specific quality rules: cash on one side and an account on the other, domestic countries, conductor identified, amount sanity flags | 7.3; 8.11 |
| **Frozen assets, matches, TFS** | | |
| RP-54 | **Match states** (true, potential, false) with reasons stored; a false match keeps its reason for exclusion and documents | Office TFS guide arts. 10-12 |
| RP-55 | A **freeze task** to bank operations with an 8-hour default clock, neutral wording, and post-freeze verification from the replica (hold present on every affected account; no debit including fees); exceptions raise a P1 alert | 8.12; file 05 R-52 |
| RP-56 | A **freeze report builder** with the contents of art. 16 of the Office's guide, bilingual PDF and CSV, a channel setting, a 48-hour clock and a second-report task at two months | 3.4; 8.12 |
| RP-57 | A CSV and PDF export that carries the fields of the IQTFS "Frozen Assets & Funds Volume" and "Report Name & Entity Matches" forms | 3.4 |
| RP-58 | A **name-similarity referral** record for matches the bank cannot resolve, addressed to the Committee's technical committee, with the identifiers compared (four-part name, mother's name, birth date and place, nationality, ID, activity, address) | Office TFS manual s.6.2-6.3; guide art. 11 |
| RP-59 | **Delisting and release** workflow that updates screening data, lifts holds only on a recorded decision, and keeps date, time and approver | Office guide arts. 24-25 |
| **Tipping-off and access** | | |
| RP-60 | All report objects (drafts, versions, XML, attachments, acknowledgements, register rows) in the **suspicion tier** with per-case access lists, relationship-conflict check and existence masking | file 30 s.8; 8.13 |
| RP-61 | Exports of reports need purpose and approval, are watermarked per user, logged, and hidden from branch roles; the report register is excluded from global search and counts | 8.13 |
| RP-62 | A **neutral-wording library** for tasks, notifications and holds; no AML reason ever written to the CBS; an automated **tipping-off inference test** (UI, API, search, counts, timing, errors) that fails the build | file 30 s.8.9; 8.13 |
| **Retention and evidence** | | |
| RP-63 | An **evidence pack per report** under object lock with the case retention class: as-filed XML, attachments, PDF, manifest, all versions, alert list and scoring, evidence snapshot, decision and reasons, approvals, upload record, acknowledgements, Office communications, clock log | 8.14; file 30 s.12 |
| RP-64 | **Source-to-report traceability**: every figure in a report links to the evidence snapshot rows (hash); a trace view for the D2 auditor | CBI assessment guide; 8.14 |
| RP-65 | Records of **cases closed without filing** are retained like filed reports and reportable by period, analyst and reason | CBI controls; file 01 s.2d |
| RP-66 | Three views: D2 auditor, inspector, Office reply (only what was asked) | 8.14 |
| RP-67 | Report objects follow the retention engine (floors, holds, deletion certificates, no resurrection after restore) | file 30 s.12.4-12.5 |
| **Other regulatory reporting** | | |
| RP-68 | Built-in templates: the three CBI standard reports (monthly account movements incl. staff accounts, transfers register, correspondent account activity), liaison monthly report, MLRO board and audit-committee pack, PEP statistics pack, training and test records, remedial-action tracker, D2 evidence pack | 4.1; 4.2 |
| RP-69 | **FATCA** as an optional module (Model 2 status, US indicia, GIIN, Form 8966 data) outside the FIU channel; **CRS** as an optional module too, switched on per bank (owner, 2026-10-07: Iraqi banks have both FATCA and CRS duties) | 4.3 |
| RP-70 | A **correspondent statistics pack** (STRs filed, RFI response times, screening and training facts) as aggregates with small-number suppression, feeding the CBDDQ and FCCQ evidence pack; no individual report is ever disclosed | 4.4; file 04 W-06 |
| **Regime packs** | | |
| RP-71 | Regime packs are profile plus clock set plus narrative templates plus output adapter, switchable per bank and listed in the assessor pack | 6.9 |
| RP-72 | **US pack:** FinCEN SAR XML 2.0 serialiser (five 4,000-character narrative blocks, `fc2:` namespace, no leading or trailing spaces, no non-printable characters), batch limit 60 MB, acknowledgement parser (BSA ID, fatal, rejected, accepted with errors), CTR layout; clocks 30/60 days and 15 days | 6.5 |
| RP-73 | **UK pack:** SAR Portal field-ordered export, glossary codes inserted in the narrative, "UNKNOWN" convention, consent (DAML) flag with a 7-working-day notice clock, up to three previous SAR references | 6.6 |
| RP-74 | **EU pack:** export of the AMLA core data points once the ITS is final; 5-working-day reply clock for FIU requests; transaction-record statement export | 6.7 |
| RP-75 | **Gulf and regional packs:** UAE (PNMR and FFR within five days, HRC held three working days), KSA (technical report with six months of statements, CDD documents and account analysis), Jordan, Egypt (STRTF, SARTF, DTET), Mauritius (5 working days), each as its own goAML profile | 6.1-6.4; 6.8 |
| **Operations and analytics** | | |
| RP-76 | **Reporting analytics** of section 8.16 (volumes, times, first-time pass rate, rejection reasons, grades, FIU responses, freeze clocks, CTR runs, backlog) with small-number suppression for non-compliance roles | 8.16 |
| RP-77 | A **bank discovery checklist** for reporting: goAML registration and version, XML or web filing today, current export tool, who files, MLRO and deputy, CTR process, freeze channel, FATCA and CRS status, sample rejections received | file 18 VD-33; 2.6 |
| RP-78 | A **pilot letter** to the Office and the CBI built from the open-questions register, and a record of the answers as product configuration | 9 |
| RP-79 | **Capture portal content before it is purged:** the as-filed XML, the on-screen receipt, rejection texts and every Message Board item are stored in the evidence vault at filing time and daily; the profile can define purge windows (Swiss goAML Web: reports 7 days after transmission, drafts 14, Message Board items 30) and the module warns before them and logs a missing capture as an exception | Swiss goAML Web manual s.13-14; 8.9 |
| RP-80 | A **status model that mirrors goAML Web**: Draft, Submitted, Processed (technical), Rejected (automatic; reasons by message), Reverted, Accepted by FIU (content), Rejected by FIU (content), plus the bank's own states; any legal clock that starts at acceptance is modelled per profile | Swiss manual s.9.7.4, 12; 8.9 |
| RP-81 | **Semi-automation output:** transaction-only XML in the `reportdata/transactions` wrapper for upload into a web-created report, valid or partial, as a second export mode next to the full report XML | UNODC "Web Report: Semi-Automation" v0.6; Pakistan FMU STR automation; 8.8 |
| RP-82 | **Profile limits** enforced at packaging: maximum transactions listed per report, XML and ZIP size, attachment size and types, XML files per ZIP, one-report-per-file rules; defaults from the profile, open questions for Iraq | Swiss manual (300 MB, 20 MB, 100 transactions); UAE guide; 8.6 L5 |
| RP-83 | A **business-rejection-rule corpus** per profile (for example: an active account of a reported subject needs balance and balance date; a multi-party dummy transaction must have amount zero in the Swiss configuration), each rule with a test and the FIU message that revealed it | Swiss manual s.9.7.4; 7.3 |
| RP-84 | A **feedback-loop register** for the Office's quality sessions and written feedback: common errors and technical obstacles named by the Office, the rule or template change made, date, and the effect on first-time pass rate and grades | Office news items 29 Jan, 12 Feb, 1 Mar, 10 Sep 2026 (2.7) |

## 11. Sources

All read 2026-10-07 unless stated. Local copies are in the session scratchpad (`src/rp/`). "NOT READ" or "blocked" marks what could not be reached.

**Iraq: the AML/CFT Office and goAML**
- AML/CFT Office web platform (IQTFS), React single-page app, code read for routes and form fields: https://aml.iq (copy `aml-index.js`)
- Office API, goAML system page and attachments (public): https://api.aml.iq/api/GoAmlSystem ; https://api.aml.iq/api/GoAmlSystem/attachments
- Office API, news items (43 items, workshops and statements): https://api.aml.iq/api/news/getall/1 (pages 1 to 5)
- Office API, list of laws, regulations and guidance manuals: https://api.aml.iq/api/regulatorydocument?pageNo=1&documentType=0 (laws), `documentType=1` (regulations and instructions), `documentType=2` pages 1 and 2 (guidance manuals)
- **goAML XML schema 5.0.2 published by the Office** (217,567 bytes, uploaded 2026-09-17): https://aml-iq.fra1.cdn.digitaloceanspaces.com/GoAmlSystem/2a5f02fd-9c66-4714-9a57-5434ed489f0c.xsd
- Iraqi goAML production portal (home and login pages; footer "Version 5.5.7.52405"): https://goaml.aml.iq/PROD/Home ; https://goaml.aml.iq/PROD/Identity/Account/Login ; registration guide link https://goaml.aml.iq/prod/before_login.pdf (404, NOT READ); probes of /TEST, /UAT, /PILOT, /DEMO, /TRAINING paths returned 404 (no test environment found); the host root redirects to /owa/
- Guidelines for Reporting Suspicious Transactions and Activities (Office, PDF created 2025-07-20, 29 pp.): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/ecd23f30-19dd-4268-84e2-cb2d14cce9be_20260811085852.pdf
- Guidance Guide on Reporting Unusual Transactions, first edition 2026 (15 pp.): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/40c281c6-d5b7-480d-919d-2a88c454b2e3_20260811093728.pdf
- Guidance Guide on Suspicious Indicators, banking transactions (11 pp., scanned; first pages read): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/9d615ba7-d34a-4802-8458-4205c54e95a8_20260811093513.pdf
- Guidance Guide on the Prompt Implementation of Targeted Financial Sanctions (PDF created 2026-09-07, 15 pp.): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/e6fde006-431a-4fc7-8c9b-fdd61b99cd12_20260910121330.pdf
- Guidance Manual on the Implementation of Targeted Financial Sanctions (March 2023, 25 pp.): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/493a0741-b3e6-4d93-bca0-124ed67e57bb_20260811084732.pdf
- Terrorists' Funds Freezing Regulation No. 6 of 2023 (Iraqi Gazette 4737, 18 Sep 2023; the Office's guide calls it "as amended", amendments not found): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/24d01159-7ec5-4274-bb22-cbc75a129214_20260810234152.pdf
- Customer Due Diligence Rules for FIs and DNFBPs, Instructions No. 1 of 2023 (Gazette 4739, 2 Oct 2023): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/ab7d4fa5-7205-4b1a-9a08-c1846bc3a167_20260811083840.pdf
- AML/CFT Law No. 39 of 2015, Arabic official text (text layer unreadable; articles quoted through the Office's manuals): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/79dff203-9081-4d89-af1c-52b4dccc9bd6_20260810234003.pdf
- Mutual Evaluation Report of Iraq, May 2024 (paras 179, 245, 252, 259, 261, 266, 267, 646, 659, table 6.2): local copy `iraq-mer-2024-en.txt` (source URL in file 01)
- CBI "Strategic Achievements 2020-2025" deck, slide 51: https://cbi.iq/static/uploads/up/file-177935854666319.pdf (via file 01 section 5)
- CBI circulars of 26 Aug 2026 (behavioural indicators; PEP due diligence): https://cbi.iq/static/uploads/up/file-178963291760044.pdf (via file 01 section 2c)
- CBI AML/CFT/CPF supervisory controls, 8 Jan 2025: https://cbi.iq/static/uploads/up/file-173666439114818.pdf (via file 01 section 2d)
- CBI Detailed Assessment Guide (D2 test), local copy `cbi-guide.txt` p. 45-46 (via file 01)
- CBI sanctions guidelines, 1 Aug 2023 (scan; via file 25 and file 30): https://cbi.iq/static/uploads/up/file-169554063119385.pdf

**UNODC and the goAML schema**
- UNODC goAML pages (Wayback copy of 6 Dec 2025): https://web.archive.org/web/20251206043416/https://unitego.un.org/goaml/ and `/content/reports`, `/description`, `/components`, `/interfaces`, `/compliance`, `/approach-and-benefits`, `/downloads`
- UNODC "goAML XML Reporting Schema Description / Standard XML Reporting Instructions and Specifications v3.0.0" (as adapted by FIU Nepal, 2019): https://raw.githubusercontent.com/manoj-pantha/goAML-XML/main/goAML%20Standard%20XML%20Reporting%20Instruction%20and%20Specification%20for%20Insurance%20Version%201.0.0%202020-12-15.pdf
- UNODC-edited goAML 5.0.2 XSD (FIU not stated): https://raw.githubusercontent.com/munguambe-eng/kairos-lex/main/scripts/goAML_XML_Schema_5.xsd

**Other FIUs**
- UAE FIU, goAML Report Types v1.2 (29 Apr 2024): https://www.uaefiu.gov.ae/media/adperv4o/different-types-of-reports-on-goaml-v1-2-29-apr-2024.pdf (Wayback 20260513133948; live site 403)
- UAE FIU, goAML Web Submission Guide v2.2 (14 Apr 2021; Wayback copy truncated at 1 MiB, pages 1-31 recovered with Ghostscript): https://www.uaefiu.gov.ae/en/media/pdf/guidance/goAML%20Web%20-%20Report%20Submission%20Guide%20V2.2%20-%2014%2004%202021.pdf (Wayback 20210926221457)
- Jordan AMLU, goAML page, stages of notification, forms page: https://www.amlu.gov.jo/Ar/Pages/%D9%86%D8%B8%D8%A7%D9%85_%D8%A7%D9%84%D8%A5%D8%AE%D8%B7%D8%A7%D8%B1%D8%A7%D8%AA_go_AML ; https://www.amlu.gov.jo/AR/Pages/%D9%85%D8%B1%D8%A7%D8%AD%D9%84_%D8%A7%D9%84%D8%A5%D8%AE%D8%B7%D8%A7%D8%B1 ; https://www.amlu.gov.jo/AR/List/%D9%86%D9%85%D8%A7%D8%B0%D8%AC_%D8%A7%D9%84%D8%A5%D8%AE%D8%B7%D8%A7%D8%B1 ; forms under `https://www.amlu.gov.jo/EBV4.0/Root_Storage/AR/EB_List_Page/` (guidance on filling the notification form; form for financial activities)
- Egypt EMLCU goAML page and kit: https://mlcu.org.eg/ar/2075/%D9%86%D8%B8%D8%A7%D9%85-goAML ; https://mlcu.org.eg/upload/uploadeditor/files/go_aml/goAML%20Schema%20V5.zip ; https://mlcu.org.eg/upload/uploadeditor/files/go_aml/goAML%20v5_2%20Documentation.pdf ; https://mlcu.org.eg/upload/uploadeditor/files/go_aml/goAML_XValidator.zip ; EMLCU goAML explanatory note (Aug 2016)
- Pakistan FMU, goAML Guidelines for STR-Automation Phase 1, v2 (22 Dec 2025): https://www.fmu.gov.pk/docs/2025/01-FMU-STR-Automation-Project-Schema-Guidelines-Version-2-FInal.pdf ; goAML schema 4.0: http://www.fmu.gov.pk/FMU_goAML_Schema_4.0.xsd ; also listed: https://www.fmu.gov.pk/docs/2024/FMU-goAML-STR-TBML-GuideBook.pdf (not read)
- Nigeria NFIU, goAML 4.5 schema and XValidator (Wayback): http://nfiu.gov.ng/images/Downloads/downloads/schema.zip ; http://nfiu.gov.ng/images/Downloads/downloads/goAMLXValidator.zip (live 403)
- Mauritius FIU, Tip of the Month June 2021: https://www.fiumauritius.org/fiu/wp-content/uploads/2021/07/Tip-of-the-Month-June-2021-Filing-of-STR-on-goAML-2.pdf ; notice on migration to the upgraded goAML platform, 26 Dec 2025: https://www.fiumauritius.org/fiu/wp-content/uploads/2025/12/NoticeGoAML261225.pdf ; test STR scenarios guide (196 pp., not read): https://www.fiumauritius.org/fiu/wp-content/uploads/2022/08/goAMLWebguide-2018-Test-STR-Scenarios_v6-30Sep2019-final.pdf
- Kenya FRC goAML registration guide (listed, not read): https://www.frc.go.ke/wp-content/uploads/2025/11/goAML-Registration-Guide.pdf ; Bangladesh BFIU goAML manual (listed, not read): https://www.bfiu.org.bd//pdf/goaml_user_manual.pdf
- Saudi Arabia: SAMA AML/CTF Guide section 8 (reporting) and section 7: https://rulebook.sama.gov.sa/en/section-8-reporting-suspicious-transactions ; https://rulebook.sama.gov.sa/node/3657 ; SAFIU website unreachable (no connection)
- Luxembourg CRF goAML generator (README on indicators): https://github.com/huxiaoming274/goaml-xml-generator

**United States, United Kingdom, European Union, Canada, Egmont**
- FinCEN SAR XML Schema User Guide v1.6 (Aug 2021): https://bsaefiling.fincen.treas.gov/docs/XMLUserGuide_FinCENSAR.pdf ; FinCEN SAR Electronic Filing Requirements v1.4: https://bsaefiling.fincen.treas.gov/docs/FinCENSARElectronicFilingRequirements.pdf ; FinCEN CTR Electronic Filing Requirements v1.5: https://bsaefiling.fincen.treas.gov/docs/FinCENCTRElectronicFilingRequirements.pdf ; Guidance on Preparing a Complete and Sufficient SAR Narrative (Nov 2003): https://www.fincen.gov/sites/default/files/shared/sarnarrcompletguidfinal_112003.pdf
- US Treasury, FATCA Agreements and Understandings by Jurisdiction (Iraq: Agreement in Substance, Model 2): https://home.treasury.gov/policy-issues/tax-policy/foreign-account-tax-compliance-act
- UK NCA, Suspicious Activity Reports page: https://www.nationalcrimeagency.gov.uk/what-we-do/crime-threats/money-laundering-and-illicit-finance/suspicious-activity-reports ; SARs Reporter Booklet June 2026: https://www.nationalcrimeagency.gov.uk/who-we-are/publications/817-sars-reporter-booklet-june-2026/file.pdf ; Guidance on submitting better quality SARs v7.0 (Feb 2023; Wayback copy of https://www.nationalcrimeagency.gov.uk/who-we-are/publications/446-guidance-on-submitting-better-quality-sars-1/file ; the current glossary-code document returned 404 and was not read)
- EU Regulation (EU) 2024/1624 (AMLR), Article 69: local copy `amlr-2024-1624.txt`; AMLA consultation on the draft ITS under Article 69(3) (opened 2 Jul 2026, closed 20 Sep 2026): https://www.amla.europa.eu/policy/public-consultations/consultation-draft-its-format-reporting-suspicions-and-providing-transaction-records_en (consultation paper, interpretative note and annexes downloaded from the page's links)
- FINTRAC, Reporting suspicious transactions: https://fintrac-canafe.canada.ca/guidance-directives/transaction-operation/str-dod/str-dod-eng ; AUSTRAC live pages unreachable (read through Wayback)
- Swiss MROS (fedpol), goAML Web manual v2.3 (69 pp.): https://www.fedpol.admin.ch/dam/data/fedpol/kriminalitaet/geldwaescherei/aml/goaml-web-manual-e.pdf ; goAML Web Transactions Upload Specification (UNODC "Web Report: Semi-Automation" v0.6): https://www.fedpol.admin.ch/dam/data/fedpol/kriminalitaet/geldwaescherei/aml/goaml-web-transactions-upload-specification.pdf ; transaction-upload XSD: https://www.fedpol.admin.ch/dam/data/fedpol/kriminalitaet/geldwaescherei/aml/goAML_ReportTransactions_Upload_Schema_CH.xsd ; Swiss XML schema document https://www.fedpol.admin.ch/dam/data/fedpol/kriminalitaet/geldwaescherei/aml/goaml-xml-schema.pdf (not read); all via Wayback (live host returned 502 to the client)
- AUSTRAC, Suspicious matter reports (Wayback copy): https://web.archive.org/web/2026/https://www.austrac.gov.au/business/core-guidance/reporting/suspicious-matter-reports-smrs (live host not reachable from the client)
- Jamaica FID goAML overview (2016): https://www.fid.gov.jm/html/wp-content/uploads/2017/09/Goaml-overVIEW-ODT.pdf (Wayback); pre-goAML XML kit https://www.fid.gov.jm/html/wp-content/uploads/2016/06/POCA-XML-2013-ver1.1.zip (Wayback)
- Egmont Group Operational Guidance for FIUs (2024) and Principles (2025): local copies `egmont-opguidance-2024.txt`, `egmont-principles-2025.txt` (file 04 section 3)

**Vendors and open source**
- Oracle OFS RR Cloud Service goAML User Guide G58777-01 (14 Aug 2026): https://docs.oracle.com/en/industries/financial-services/ofs-analytical-applications/regulatory-reporting-cloud/26.08.01/goamluserguide/goaml-user-guide-cs.pdf
- Pio-Tech goAML management system and Iraqi references (pages are client-side rendered; details from file 18): https://www.pio-tech.com/products/goaml-management-system ; https://www.pio-tech.com/news/iraqi-islamic-bank-integrates-go-aml-solution ; https://www.pio-tech.com/news/imtb-announces-successful-go-live-of-go-aml-system-from-pio-tech
- Capital Banking Solutions, release of 26 Aug 2022 (Wayback): https://web.archive.org/web/20221001015216/https://www.capital-banking.com/en/sumer-commercial-bank-selects-capital-banking-solutions-for-its-core-and-digital-banking-solutions/
- Flagright and Hummingbird filing features: see file 13 sections 6 and 9
- GitHub: https://github.com/eradesa/goAML_py_scripts ; https://github.com/KasunHewagamaDFCC/GOAML_Validator ; https://github.com/PingExchange/XmlGoaml ; https://github.com/Aruna-SITS/goAMLProject ; https://github.com/manojrevenkar12-web/aml-investigator ; https://github.com/munguambe-eng/kairos-lex ; https://github.com/manoj-pantha/goAML-XML ; GitHub repository search API queries "goaml", "goaml xsd", "goaml validator", "goaml xml generator"
- Wolfsberg and Basel material cited through file 04; Wolfsberg effective-monitoring statement through file 04 section 2a.

**Not reached or not read:** FATF site (blocked by the session rules; FATF text taken from file 03 and the Office's manual); OECD AEOI commitments page (403; no archived list with Iraq found); the SAFIU site; AUSTRAC's SMR content guidance (only the timing page was read); the Office's goAML registration guide; the Office's analytical study "Improving Reports Submitted by Reporting Entities" (not public); the Swiss XML schema document (listed, not read); the Jordanian CBJ AML instruction; goAML guides of Ghana's FIC and the Seychelles FIU (none found in the archived listings; Jamaica's FID has only a 2016 overview); the Nigerian NFIU schema description (404); Mauritius test-STR guide (196 pp.), Pakistan TBML guidebook, Kenya and Bangladesh manuals (listed only); the NCA glossary-code document (404); the Office's STR grading form and any test environment; the CBI circulars' full text beyond file 01's reading; Iraq's goAML 5.5 release notes.

## Appendix A. Validated example reports (invented data; each validated 2026-10-07 against the Office's XSD with libxml2 2.15 through lxml)

Names, numbers and banks are invented. 'Example Bank for Investment' and its SWIFT code do not exist. Test method: `etree.XMLSchema(etree.parse('iraq-goaml-schema.xsd')).validate(doc)`; result VALID for all three. Validation by the Office's server is not implied (section 2.5 and section 9 list what the XSD cannot show).

### A.1 STR, one transaction, my-client account to a third-party account

```xml
<?xml version="1.0" encoding="UTF-8"?>
<report>
  <rentity_id>1234</rentity_id>
  <rentity_branch>Baghdad Main Branch</rentity_branch>
  <submission_code>E</submission_code>
  <report_code>STR</report_code>
  <entity_reference>STR-2026-000123</entity_reference>
  <submission_date>2026-10-07T10:30:00</submission_date>
  <currency_code_local>IQD</currency_code_local>
  <reporting_person>
    <first_name>Example</first_name>
    <last_name>Officer</last_name>
    <birthdate>1980-01-01T00:00:00</birthdate>
    <passport_number>A0000000</passport_number>
    <occupation>MLRO</occupation>
  </reporting_person>
  <location>
    <address_type>B</address_type>
    <address>Karrada, Baghdad</address>
    <city>Baghdad</city>
    <country_code>IQ</country_code>
    <state>Baghdad</state>
  </location>
  <reason>Customer received 14 cash deposits of IQD 9,500,000 each in 6 days at 5 branches, each just under the CTR line, and moved 133,000,000 to a third party account the same day. Activity does not match the declared occupation (salaried teacher).</reason>
  <action>Account placed under enhanced monitoring; no new cash deposits accepted pending review.</action>
  <transaction>
    <transactionnumber>TXN-20260930-000771</transactionnumber>
    <internal_ref_number>CBS-9981-000771</internal_ref_number>
    <transaction_location>Baghdad Main Branch</transaction_location>
    <transaction_description>Outgoing transfer to third party account</transaction_description>
    <date_transaction>2026-09-30T13:05:00</date_transaction>
    <late_deposit>false</late_deposit>
    <transmode_code>AP</transmode_code>
    <amount_local>133000000.00</amount_local>
    <t_from_my_client>
      <from_funds_code>L</from_funds_code>
      <from_account>
        <institution_name>Example Bank for Investment</institution_name>
        <swift>EXBKIQBA</swift>
        <branch>Baghdad Main Branch</branch>
        <account>IQ00EXBK000000000001</account>
        <currency_code>IQD</currency_code>
        <client_number>C-100045</client_number>
        <personal_account_type>C</personal_account_type>
        <signatory>
          <is_primary>1</is_primary>
          <t_person>
            <first_name>Ahmed</first_name>
            <last_name>Example</last_name>
            <birthdate>1985-04-12T00:00:00</birthdate>
            <mothers_name>Fatima Hassan</mothers_name>
            <id_number>199000000001</id_number>
            <nationality1>IQ</nationality1>
            <phones>
              <phone>
                <tph_contact_type>P</tph_contact_type>
                <tph_communication_type>M</tph_communication_type>
                <tph_country_prefix>964</tph_country_prefix>
                <tph_number>7700000000</tph_number>
              </phone>
            </phones>
            <addresses>
              <address>
                <address_type>P</address_type>
                <address>Al Mansour, Baghdad</address>
                <city>Baghdad</city>
                <country_code>IQ</country_code>
                <state>Baghdad</state>
              </address>
            </addresses>
            <occupation>Teacher</occupation>
            <identification>
              <type>A</type>
              <number>199000000001</number>
              <issue_country>IQ</issue_country>
            </identification>
            <source_of_wealth>Salary</source_of_wealth>
          </t_person>
          <role>A1</role>
        </signatory>
        <opened>2019-03-01T00:00:00</opened>
        <balance>1250000.00</balance>
        <date_balance>2026-09-30T23:59:00</date_balance>
        <status_code>A</status_code>
        <beneficiary>Ahmed Example</beneficiary>
      </from_account>
      <from_country>IQ</from_country>
    </t_from_my_client>
    <t_to>
      <to_funds_code>L</to_funds_code>
      <to_account>
        <institution_name>Other Bank</institution_name>
        <swift>OTHRIQBA</swift>
        <account>IQ00OTHR000000000002</account>
        <account_name>Third Party Trading</account_name>
      </to_account>
      <to_country>IQ</to_country>
    </t_to>
  </transaction>
  <report_indicators>
    <indicator>TA</indicator>
    <indicator>TD</indicator>
    <indicator>IDA</indicator>
  </report_indicators>
</report>
```

### A.2 CTR, one cash deposit (t_from person, t_to_my_client account)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<report>
  <rentity_id>1234</rentity_id>
  <rentity_branch>Head Office</rentity_branch>
  <submission_code>E</submission_code>
  <report_code>CTR</report_code>
  <entity_reference>CTR-20261006-0001</entity_reference>
  <submission_date>2026-10-07T01:30:00</submission_date>
  <currency_code_local>IQD</currency_code_local>
  <reason>Daily cash transaction report, business date 2026-10-06: cash deposits and withdrawals of USD 10,000 or more.</reason>
  <transaction>
    <transactionnumber>CASH-20261006-000045</transactionnumber>
    <internal_ref_number>TLR-0203-000045</internal_ref_number>
    <transaction_location>Baghdad Main Branch</transaction_location>
    <transaction_description>Cash deposit at teller</transaction_description>
    <date_transaction>2026-10-06T11:20:00</date_transaction>
    <teller>T0203</teller>
    <late_deposit>0</late_deposit>
    <transmode_code>A</transmode_code>
    <amount_local>18500000.00</amount_local>
    <t_from>
      <from_funds_code>K</from_funds_code>
      <from_person>
        <first_name>Mohammed</first_name>
        <last_name>Example</last_name>
        <birthdate>1978-02-10T00:00:00</birthdate>
        <mothers_name>Zainab Ali</mothers_name>
        <nationality1>IQ</nationality1>
        <identification>
          <type>A</type>
          <number>198800000002</number>
          <issue_country>IQ</issue_country>
        </identification>
      </from_person>
      <from_country>IQ</from_country>
    </t_from>
    <t_to_my_client>
      <to_funds_code>L</to_funds_code>
      <to_account>
        <institution_name>Example Bank for Investment</institution_name>
        <swift>EXBKIQBA</swift>
        <branch>Baghdad Main Branch</branch>
        <account>IQ00EXBK000000000009</account>
        <currency_code>IQD</currency_code>
        <client_number>C-200311</client_number>
        <personal_account_type>B</personal_account_type>
        <signatory>
          <is_primary>1</is_primary>
          <t_person>
            <first_name>Mohammed</first_name>
            <last_name>Example</last_name>
            <birthdate>1978-02-10T00:00:00</birthdate>
            <mothers_name>Zainab Ali</mothers_name>
            <id_number>198800000002</id_number>
            <nationality1>IQ</nationality1>
            <phones><phone><tph_contact_type>P</tph_contact_type><tph_communication_type>M</tph_communication_type><tph_country_prefix>964</tph_country_prefix><tph_number>7800000001</tph_number></phone></phones>
            <addresses><address><address_type>P</address_type><address>Zayouna, Baghdad</address><city>Baghdad</city><country_code>IQ</country_code><state>Baghdad</state></address></addresses>
            <occupation>Trader</occupation>
            <identification><type>A</type><number>198800000002</number><issue_country>IQ</issue_country></identification>
            <source_of_wealth>Business income</source_of_wealth>
          </t_person>
          <role>A1</role>
        </signatory>
        <opened>2015-06-01T00:00:00</opened>
        <balance>25800000.00</balance>
        <date_balance>2026-10-06T23:59:00</date_balance>
        <status_code>A</status_code>
        <beneficiary>Mohammed Example</beneficiary>
      </to_account>
      <to_country>IQ</to_country>
    </t_to_my_client>
  </transaction>
  <report_indicators>
    <indicator>-</indicator>
  </report_indicators>
</report>
```

### A.3 UTR, activity-based (no transaction); the `fiu_ref_number` value is only a placeholder, because how the Office wants a previous report referenced is an open question (section 9)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<report>
  <rentity_id>1234</rentity_id>
  <rentity_branch>Head Office</rentity_branch>
  <submission_code>E</submission_code>
  <report_code>UTR</report_code>
  <entity_reference>UTR-2026-000031</entity_reference>
  <fiu_ref_number>UTR-PRIOR-0007</fiu_ref_number>
  <submission_date>2026-10-07T10:30:00</submission_date>
  <currency_code_local>IQD</currency_code_local>
  <reason>Dormant for six months, the account received 3 transfers of IQD 4,000,000 in 4 days and then paid out 11,500,000 in cash. No indicator of crime has been established. Enhanced monitoring started. If activity continues the case will be escalated to an STR.</reason>
  <action>Enhanced monitoring applied for 90 days.</action>
  <activity>
    <report_parties>
      <report_party>
        <person>
          <first_name>Sara</first_name>
          <last_name>Example</last_name>
          <mothers_name>Huda Karim</mothers_name>
        </person>
        <significance>4</significance>
        <reason>Account holder</reason>
      </report_party>
    </report_parties>
  </activity>
  <report_indicators>
    <indicator>TA</indicator>
    <indicator>IDA</indicator>
  </report_indicators>
</report>
```
