# 16 — Vendors: KYC and client lifecycle management (Fenergo, Pega, Moody's Passfort, Encompass, Uqudo, Onfido, Sumsub, IDnow, Jumio) and e-KYC in Iraq

Status: done (2026-10-06)

Part 3 of 4 of the vendor research (screening engines, risk data providers, KYC tools). Sister files: 14 screening engines,
15 risk data providers. Requirement IDs use the prefix VC-xx and are numbered across files 14, 15 and 16.
Cross-references: file 01 (section 2b CDD Instructions No. 1 of 2023; section 2d CBI 2025 controls on risk rating and KYC refresh;
section 7 Iraqi ID documents), file 04 (section 2q Wolfsberg digital customer lifecycle; 2h SoW/SoF; 2n PEPs), file 26 (KYC, CDD
and risk rating, when written).

## Contents

1. What KYC and CLM tools must do in an Iraqi bank (baseline from files 01 and 04)
2. Fenergo
3. Pega Client Lifecycle Management and KYC
4. Moody's Passfort
5. Encompass
6. Uqudo
7. Onfido (Entrust)
8. Sumsub
9. IDnow
10. Jumio
11. e-KYC in Iraq: the national unified card, local providers and what a bank can actually verify
12. Cross-cutting: customer risk rating engines
13. Cross-cutting: periodic and event-driven reviews
14. Cross-cutting: evidence vaults and document management
15. Comparison table
16. What it means for AKR-TMS
17. Derived requirements (VC-xx)
18. Sources

(Each vendor section covers: what it does, deployment, Arabic and MENA presence, pricing signals, what it does right, what it does
wrong, lessons for AKR-TMS, AKR-TMS hooks.)

## Findings in brief

1. **No CLM vendor fits the deployment brief**: Fenergo and Moody's are SaaS only, the others are cloud-first or unverified, and CLM systems expect write access as the system of record. AKR-TMS is the read-only KYC risk, review and evidence layer around the CBS, so these vendors are design references and optional inputs.
2. **Uqudo is the only vendor documenting the Iraqi national card**: mother's and both grandfathers' names, family number, Kurdish-to-Arabic standardised names, MRZ checksum, with on-premise and in-country hosting; Iraqi chip reading and registry checks are not stated.
3. **The CBI's e-payment AML controls set the remote-onboarding bar**: face login, device binding, one account per user, recorded video calls, AI document forensics and biometric comparison with "original sources".
4. **No public evidence of a bank-facing check against the Iraqi civil registry** was found; this is an open question for the CBI.
5. **Copy**: Fenergo's policy engine and materiality filter, Pega's configurable questionnaires, Encompass's attribute-level lineage and audit trail, Uqudo's Iraqi ID field and name model.
6. **No vendor documents write-once evidence storage, retention by record class or an examiner export**; the stack (MinIO with object lock) makes an on-premises vault feasible.
7. **Price signals**: Sumsub USD 1.35 and 1.85 per verification (public); Fenergo-class public notices EUR 0.8 to 2.0 million (attribution unverified).
8. 21 derived requirements (VC-66 to VC-86).

---

## 1. What KYC and client lifecycle management tools must do in an Iraqi bank

**Method and limits.** Vendor pages were read directly on 2026-10-06 (curl and the fetch tool), the CBI's scanned e-payment AML controls were read
from rendered page images, and procurement records were read through the TED API. The shared web-search budget ran out part-way through the
vendor research, and Onfido's own site and the review sites block automated access (Entrust's page, which replaced Onfido's, was read from an
Internet Archive copy). Where a fact could not be verified it is marked. File 26 already covers customer risk rating methods, triggers,
EDD, beneficial ownership registers in Iraq and a proposed rating model; this file adds what the KYC and identity vendors say and do, and cross-refers
instead of repeating.

| Duty | Basis | What a tool must support | Where covered |
|---|---|---|---|
| Customer identification and verification from Iraqi documents (national card, civil status ID, residence card, ration card, family card, passport) | CDD Instructions No. 1 of 2023 (file 01 s.2b); Iraqi ID facts (file 01 s.7) | Arabic and Kurdish OCR, mother's and grandfather's names, family number, MRZ check | s.6 to s.11 |
| Risk rating High/Medium/Low (up to five levels), dynamic, reviewed at least every two years; KYC form refreshed at least yearly (MLRO duty) | CBI controls 2025 ch. 2 art. 2 and ch. 3 (file 01 s.2d) | Explainable rating, triggers, review calendar | s.12, s.13; file 26 |
| EDD for PEPs (relatives to the second degree), non-residents, high-risk countries and professions; senior-management approval | CBI PEP circular; e-payment controls p.12-14 | EDD workflow with approvals and evidence | s.13; file 26 s.7, s.9 |
| Beneficial owner at 20 percent; control; aggregation for sanctions | CDD instructions; OFAC rule (file 05 s.1e) | Full shareholder tables, registry evidence | file 15 s.13; file 26 s.8 |
| Screening of customers, owners, signatories at onboarding and continuously | CBI controls ch. 5 art. 5 | Hand-off to the screening layer (files 14 and 15) | file 14 |
| Records kept at least 5 years after the relationship, quick retrieval, off-site backup; OFAC expects 10 years for sanctions records | Law 39/2015 art. 11; CBI controls; OFAC final rule 2025 (file 05 s.1e) | Evidence vault with retention rules | s.14 |
| Remote and mobile onboarding (e-payment providers): face verification, device binding, recorded video call, document forensics | CBI AML controls for e-payment providers, 20 Apr 2025 (read from the scan, s.11) | Liveness, biometric match, forensic checks | s.11 |
| Everything stays in the CBS; AKR-TMS reads it and never writes | AKR-TMS brief | A tool that needs write access to the CBS or to onboarding is out of scope | s.16 |

**Position of these vendors in the AKR-TMS design.** Onboarding stays in the CBS (brief). Fenergo, Pega, Passfort and Encompass are CLM and
onboarding systems of record; Uqudo, Onfido (Entrust), Sumsub, IDnow and Jumio are identity-proofing services. AKR-TMS neither replaces nor wraps them.
It reads what the CBS holds, scores and monitors it, runs the review and EDD workflows that the CBS does not have, and keeps the evidence. The value of
studying them is to learn the workflow and evidence design, to know what data such tools produce (so the mapping layer can read it from the CBS), and
to decide where AKR-TMS must be better (Iraqi content, white-box rating, read-only, on-premises).

---

## 2. Fenergo

### 2.1 What it is (Fenergo pages, 2026)

Irish client lifecycle management (CLM) and KYC vendor; since 2021 owned by the private equity firms **Astorg and Bridgepoint** (company page); offices include
Dubai (DIFC) and Melbourne (contact page). Products: Fenergo CLM (client onboarding, KYC, account opening, loan origination), KYC and transaction
compliance, and the "Fen-AI" layer: **Fen-X** (the "Legal Entity System of Record"), **KYRA** (governed AI agents for onboarding, reviews and monitoring) and a "Continuous
System of Control". Facts:

| Topic | Fact |
|---|---|
| Delivery | "An API-first, SaaS solution"; "highly scalable, cloud-native"; "multi-region support"; Integration Hub and no-code Integration Flows; "99.9% uptime"; SOC 2 Type II on request under NDA |
| Rules | "Pre-packaged with KYC data, document, and ownership and control rules for all entity types and natural persons across **120+ jurisdictions**"; a **Policy Manager** "dynamically determines the KYC and AML requirements ... during every stage of the client lifecycle" |
| Rating | **Risk Engine** "dynamically responds to inputs to a client's KYC profile and automatically calculates client risk assessment"; "multiple risk models" called by "dynamic risk scoping rules, as defined by the business" (file 26 s.12) |
| Ownership | **Related Party Manager** to "identify, create and visualize complex entity relationships", UBOs and persons with significant control; ownership data through Moody's Entity Verification API and LexisNexis, LSEG partnerships |
| Reviews | "Continuous monitoring of the client profile by identifying changes from integrated data and screening providers, assessing the impact of those changes and determining materiality"; KYRA agents run due diligence "continuously, not just at onboarding" |
| Claims | Projects "delivered in under 12 weeks"; "faster client onboarding journeys compared to industry average" (figure missing on the page); "7 in 10 financial institutions lost a client to slow onboarding" |
| References | BNP Paribas "One KYC" (Celent Model Bank Award 2022), Northern Trust, US Bank Investment Services, TMF Group |
| Arabic, Iraq | No mention on the pages read; Dubai office only |
| Pricing | None public. TED full-text matches for "Fenergo": National Asset Management Agency (Ireland), EUR 1,980,000, notice 100252-2021, "software programming and consultancy services"; Central Bank of Ireland, EUR 800,000, notice 721899-2022, "software maintenance". The contractor field was not readable, so attribution to Fenergo is unverified |

### 2.2 What it does right

1. **Jurisdiction rule packs and a policy engine** that decide what is required for each client type, product and stage; this is the CDD instruction logic that AKR-TMS must reproduce for Iraq (identification, BO at 20 percent, PEP, EDD).
2. **Materiality filter on change**: changes in data are assessed and non-material updates are processed by policy, which keeps perpetual KYC from drowning analysts.
3. **A related-party model** that treats ownership and control as first-class, with visualisation.
4. **Audit-ready record**: "every action, decision, source and rationale is carried forward in the record as work happens" (KYRA page).
5. **Ecosystem**: data and screening integrations (LexisNexis, LSEG, Moody's), Salesforce for relationship managers.

### 2.3 What it does wrong (evidence)

1. **SaaS only**: no on-premise statement; customer data in Fenergo's cloud regions. For an Iraqi bank that keeps data in its data centre, this fails the default deployment.
2. **System of record with write access**: Fenergo becomes the master of client data and workflow; the AKR-TMS brief forbids write access to the CBS and keeps onboarding in the CBS.
3. **No Iraqi evidence**: no Iraqi rule pack, Arabic naming, family number or CBI profile on the pages; 120+ jurisdictions is a count, not a list.
4. **Large-institution and wealth focus** (BNP Paribas, Northern Trust, private banking, asset servicing): implementation and licence scale are out of line with Iraqi bank budgets; public-sector procurement values in Ireland (unverified attribution) are EUR 0.8 to 2 million.
5. **Agentic AI positioning**: "agents execute, humans stay in the loop" with claims that are vendor-only; governance and validation evidence are not published.

### 2.4 Lessons and hooks for AKR-TMS

- **Copy** the rule-pack idea as an "Iraqi regulatory profile": configurable requirements by client type (individual, sole trader, company, NGO, endowment, non-resident, PEP) and stage (onboarding, periodic, trigger), each requirement linked to its source clause in the CBI rules.
- **Copy** the materiality filter for event-driven refresh, and the related-party graph for ownership.
- **Beat** with a white-box rating, read-only integration and an on-premises deployment.

**AKR-TMS hooks**
- **Regulatory profile object**: requirement id, client type, stage, source clause (CBI, Law 39/2015, instructions), evidence required, approver level, review interval.
- **Materiality rules** for KYC change events (name, address, owner, activity profile, PEP status, high-risk country link), configurable and logged.

---

## 3. Pega Client Lifecycle Management and KYC

### 3.1 What it is (Pega page, 2026)

Pega (founded 1983, per its own page) sells a low-code process and decision platform; **Pega CLM and KYC** are applications on it. Facts: "CLM KYC is a solution for managing customer onboarding, due diligence, and ongoing KYC requirements across the customer lifecycle"; **party-centric**; "rules and questionnaires are configurable"; "business users [can] configure rules, data requirements, and questionnaires without deep technical changes"; "dynamic regulatory rules engine, global master view clients, intent led-processes, and pre-configured best-practice KYC rules that cover AML/CTF, FATCA, CRS, FINRA, MiFID II, IIROC, EMIR, and Dodd-Frank"; out-of-the-box connectors named "kyc.com (Markit), Clarient, Avox and World-Check" (an older list); **perpetual KYC**: "shifts from periodic reviews to ongoing, event driven monitoring ... triggering reviews when relevant changes occur"; AI for "document processing, guidance, and summarization while maintaining auditability". The page quotes an analyst: Pega "has expanded its KYC functionality with perpetual KYC monitoring and low-code capabilities differentiators" (attribution not shown).

### 3.2 Arabic, Iraq, deployment, price

No Arabic, Iraq or Middle East statement. Deployment options (Pega Cloud, client-managed cloud, on premise) were not confirmed because the Pega pages guessed for deployment returned 404; Pega Platform is generally deployable in the customer's environment, **unverified here**. No public price.

### 3.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| A flexible low-code engine: bank staff change rules and questionnaires; one platform for CLM, KYC and case work | A heavy platform needing implementation partners; connector list on the page is dated (Markit, Clarient, Avox) |
| Supports perpetual KYC triggers and global reuse of due-diligence data | No Iraq or Arabic content; no published latency or rating logic |
| Rules for many regimes (AML/CTF, FATCA, CRS, MiFID II) | Regulatory rule content is Western; CBI and Iraqi instructions would be a build |

### 3.4 Lessons and hooks

- Pega shows that **configurable questionnaires and rules** are a requirement, not a luxury; AKR-TMS needs a form designer for Arabic KYC and EDD questionnaires with versioned rule sets.
- **Hook**: questionnaire and rule designer (Arabic and English, right-to-left) with version history, approval and effective dates, used for EDD, SoW/SoF and periodic review forms.

---

## 4. Moody's Passfort and Maxsight (Moody's for Compliance)

### 4.1 What it is (Moody's pages, 2026)

Passfort (UK onboarding and CLM, owned by Moody's; acquisition date not verified here) and **Maxsight** (Moody's "unified risk platform") are the workflow side of Moody's; data is Orbis and Grid (file 15 s.4). Facts: "configurable workflows that bring data collection, verification checks, risk profiling, and case management into a single, transparent process"; CDD and EDD paths; KYB with ownership research "where available through connected sources"; risk tiers "for example low/medium/high — based on your internal definitions"; "policy-led routing for review, escalation, or offboarding"; **pKYC**: "customer and counterparty data are updated as relevant changes occur, rather than only at set intervals"; case management to "document alerts, review actions and customer-determined outcomes"; AI "to support name matching and screening workflows". Maxsight use cases: digital onboarding, sanctions screening ("sanctions by extension"), third-party risk, supply chain.

### 4.2 Arabic, Iraq, deployment, price

No Iraq or Arabic statement. Delivery is SaaS (customer logins for Passfort and kompany; Maxsight "platform"); on-premise not stated. No public price for the workflow products; data prices are in file 15 s.14.

### 4.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Workflow joined to the largest ownership database and a risk database; policy routing; pKYC; no-code | "Risk tiers ... based on your internal definitions" means the product leaves the rating logic to the bank; no Iraqi factor set |
| Entity Verification API reaches registries in 200+ countries | Iraqi company coverage unverified; registries access shrinking (Moody's own note) |
| Useful for KYB of foreign holding companies in Iraqi customers' chains | SaaS only; per-entity data licences |

### 4.4 Lessons and hooks

- Use Moody's-type data as an optional **ownership enrichment** for foreign chains (file 15 VC-58); AKR-TMS's own workflow should work without it.
- **Hook**: "customer-determined outcomes" are a risk: every EDD or review outcome in AKR-TMS must record the rule that applied, the evidence and the approver, not only the decision.

---

## 5. Encompass (EC360, Corporate Digital Identity)

### 5.1 What it is (Encompass pages, 2026)

Corporate KYC automation: the **EC360** platform builds **Corporate Digital Identity (CDI)** profiles from "regulators, corporate registries, ultimate beneficial ownership (UBO) registries and stock exchanges across the world", "normalized, deduped and merged on retrieval". Features: **data attribution lineage** ("Encompass records and retains the origin of every piece of information, ensuring any decisions can be easily explained and confidently defended"), a time-stamped **digital audit trail** "of every action taken ... available within a complete digital risk profile along with original source documents and full data provenance", UBO verification with automation rules and visualisation, PEP, sanctions and adverse media screening, **perpetual KYC** "replacing static, periodic reviews with dynamic, continuous monitoring", intelligent document processing, KYC managed services and remediation, an MCP interface. Claim: "KYC dissatisfaction among corporate treasurers reaches 95%" (its own survey).

### 5.2 Arabic, Iraq, price

No Arabic, Iraq, GCC or Middle East statement. No public price; a TED full-text match for "Encompass" and "KYC" at Sweden's export credit agency (notice 360865-2023, SEK 3,200,000, "data analysis services") is unverified as to contractor.

### 5.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| **The clearest attribute-level lineage and audit trail** of the vendors read: every attribute carries its source, retrieved evidence and time | Corporate and institutional focus; retail and SME onboarding in Iraq are not its use case |
| Automated registry retrieval across jurisdictions for company evidence | No evidence that the Iraqi Companies Registration Department portal or the CBI BO platform is connected; the data is only as good as the registries |
| Perpetual KYC with source-attributed changes | SaaS; no Arabic content |

### 5.4 Lessons and hooks

- **Copy** attribute-level lineage: each KYC attribute in AKR-TMS (name, owner, address, activity) stores its source (CBS field, document, registry extract, vendor), date and who verified it; a rating or alert can show it.
- **Hook**: **attribute provenance table** (attribute, value, source class, source reference, retrieved at, verified by) shared by KYC, ownership and screening, which also feeds the evidence vault (s.14).

---

## 6. Uqudo (UAE; Arabic and Kurdish identity documents)

### 6.1 What it is (uqudo.com and docs.uqudo.com, 2026)

Dubai-based identity, fraud and compliance platform for the Middle East and Africa. Claims: "50+ enterprises across MEA", "20M+ verifications performed", "14+ countries
served"; Iraq appears in its 37-country list. Capabilities: document scanning ("thousands of document types ... 99.5% accuracy"), "Process **Arabic, Kurdish, and Latin scripts**", NFC chip reading
("156+ passport types and national ID cards across 22 countries"), liveness against "presentation attacks, image injection, and deepfakes", optional checks against "7+ government databases" (the UAE is the one named;
"first and only provider supporting Kurdish OCR and UAE government database integration"), plus AML screening, transaction monitoring and audit trails. Deployment: "SaaS, private cloud, on-premise, or hybrid";
"in-country hosting (UAE, KSA, Oman, Rwanda, etc.)" and "a dedicated environment in your cloud subscription". Claims: "80-90% fewer false positives", "50-70% total cost reduction", "90%+ onboarding conversion".

### 6.2 The Iraqi ID in Uqudo's documentation (IRQ_ID)

The Scan API returns, for the Iraqi national card of citizens: face image; **given name, surname, father's name, mother's name, paternal grandfather's name and maternal grandfather's name, each in Arabic, transliterated and
"Kurdish to standard Arabic" forms**; document number; personal number; gender; blood group; card access number; on the back the **family number**, date of birth (MRZ and full), place of birth, issue and expiry dates,
nationality, issuing authority, the MRZ text and **`mrzVerified`** (MRZ checksum result). A separate schema covers the **resident** card (passport number, sponsor, residency type, barcode number). The card access number
(CAN) is the ICAO PACE secret used to read a chip; the page does not say whether Uqudo reads the Iraqi chip by NFC (not verified).

### 6.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| The only vendor read that documents Iraqi ID fields and Kurdish handling: it keeps three forms of each name and standardises Kurdish letters to Arabic; family number and both grandfathers captured | No Iraqi government database check stated; Iraqi chip (NFC) support not stated; legacy documents (civil status ID, ration card, family card) are not in the list |
| Deployment includes on-premise and private cloud with in-country hosting | Accuracy figures are generic (99.5 percent) with no Iraqi test result; 50+ enterprise customers is a small base; no Iraqi customer named |
| MRZ checksum result returned; regional language and document focus | Pricing not public; liveness and forensic claims are vendor-only |

### 6.4 Lessons and hooks

- **Copy the three-form name model**: store each name part as original Arabic or Kurdish script, standardised Arabic, and transliteration, with the method; this feeds screening (file 14 s.11, file 22).
- **Copy the field set**: mother's name, both grandfathers, family number, personal number, place of birth, card access number and MRZ validity belong in the KYC data model and in the mapping layer's canonical customer.
- **Hook**: **KYC field validators** on the CBS data (MRZ checksum where the MRZ is stored, ID number format, family number format of 18 alphanumeric characters, name-chain completeness, consistent names across documents) with a defect queue per branch.

---

## 7. Onfido (now Entrust Identity Verification)

### 7.1 What it is

Onfido (UK identity verification) is "now Entrust Identity Verification Solutions", "built on the Onfido technology" (Entrust page, Internet Archive copy of 2026; the acquisition date is not verified here). Facts: AI biometrics and document intelligence, "device and behavioral signals", an Entrust Fraud Lab, no-code orchestration of onboarding journeys, SDKs, QES and eIDAS 2.0 positioning, extension to authentication, account recovery and digital signing, G2 badges (vendor statement). Onfido's own site and documentation return HTTP 403 to automated clients, so document coverage for Iraq could not be read; Iraq appears only in a country selector on the Entrust page.

### 7.2 Arabic, Iraq, price

No Arabic or Iraq-specific statement; no public price (Onfido and Entrust sell by quote).

### 7.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Strong fraud-engine and biometric positioning; orchestration without coding; part of a large security vendor | Iraqi document coverage unverified; no Arabic or Kurdish OCR claim; no government database link stated |
| Reuse of the onboarding identity for later authentication | Cloud SaaS orientation; no on-premises statement found; vendor-only metrics |

### 7.4 Lesson

Treat a global identity-proofing vendor as unproven for Iraqi documents until it passes a document-level test on real Iraqi national cards (Arabic and Kurdish, chip or no chip). **Hook**: a document-coverage test pack (sample images with consent or synthetic) in the vendor evaluation workspace (file 15 VC-49).

---

## 8. Sumsub

### 8.1 What it is (Sumsub pages, 2026)

Identity verification, KYB and AML platform: "Trusted by 4000+ clients". Facts: ID verification with liveness and face match; **reusable KYC** (verified data shared between companies by API with timestamps and audit logs); **global database verification** ("government, banking, and credit data in over 50 countries", "cover 5.5 billion people ... including major markets in North America, LATAM, UK, and Africa"; Middle East not mentioned); **AML screening** ("over 50,000 independent sources across 240+ countries and territories"), ongoing monitoring, case management with "FIU reporting"; **AML risk score** ("define risk factors (like geography or transaction size) and assign weights", "compliance managers [can] configure risk scoring models without waiting on developers"); transaction monitoring and Travel Rule; an expert moderation team for manual review.

### 8.2 Price (public)

**Basic USD 1.35 per verification** (minimum monthly commitment USD 149) and **Compliance USD 1.85 per verification** (minimum USD 299 a month, includes AML screening and proof of address), custom plan above that; 14-day free trial with 50 checks; charged for successful verifications only.

### 8.3 Arabic, Iraq

The supported-documents page loads its country and document list dynamically; Iraq could not be confirmed. Arabic is not mentioned on the pages read.

### 8.4 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Public per-verification prices; integrated KYC, KYB, AML, monitoring and case management; configurable risk score weights | SaaS; database verification excludes the Middle East in its own description; Iraqi document support unverified |
| Reusable KYC with audit log | Reusable KYC across companies raises confidentiality and CBI secrecy questions in a closed Iraqi banking market |
| Low entry price for small institutions | Rating logic is a weight table the customer sets; no Iraqi factor set |

### 8.5 Lesson

Per-verification pricing at USD 1.35 to 1.85 sets the market comparator for identity proofing in Iraq (a bank opening 100,000 accounts a year would pay about USD 135,000 to 185,000 at list price, before volume discounts, which is itself a reason Iraqi banks may prefer an on-premise or local alternative). **Hook**: a cost-per-onboarding line in the AKR-TMS total-cost view that includes identity-proofing fees even though onboarding stays in the CBS.

---

## 9. IDnow

### 9.1 What it is (idnow.io, 2026)

German identity-verification group, strongly European: "support official eIDs like German eID and Italian SPID", **EUDI wallet**-compatible solutions under eIDAS 2.0, a certified QTSP partnership for qualified signatures, document verification ("hundreds of checks"), biometric liveness, data verification against government records, credit bureaus and utility data, address and IBAN verification, AML screening ("global PEP, Sanctions and Adverse Media lists"), and **video identification by "775+ trained identification specialists, located in 12 centers across 8 countries"** for high-risk cases.

### 9.2 Arabic, Iraq, price

No Middle East, Arabic or Iraq statement on the pages read; no public price. Its human video-ident model needs staff who read Iraqi documents, which is not claimed.

### 9.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Human-in-the-loop video identification with trained specialists for high-risk cases; qualified-signature route; eID schemes | European regulatory focus (eIDAS, AMLR); no MENA or Iraqi evidence; video centres are not described as covering Arabic or Kurdish |
| Layered fraud checks (injection, deepfakes) | Cloud; price not public |

### 9.4 Lesson

A **human video review** option for high-risk onboarding is consistent with the CBI e-payment rule on recorded video calls (s.11). AKR-TMS does not onboard, but its EDD workflow can require and store a recorded verification call reference for high-risk digital customers.

---

## 10. Jumio

### 10.1 What it is (jumio.com, 2026)

US identity verification and AML platform: "supports more than 5,000 ID types across 200 countries and territories", "processed over 1 billion transactions", ID capture with authenticity checks, selfie with liveness, risk-based decision, reverification by selfie, **screening and monitoring** ("global/regional sanctions, PEP lists and adverse media", near-real-time screening of new online customers, ongoing alerts by e-mail or API), video verification.

### 10.2 Arabic, Iraq, price

Iraq is listed among the 200 countries on the global coverage page, with no document types or Arabic and Kurdish claims; no public price.

### 10.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Broad document library claim and one platform for identity, selfie, screening and monitoring | Coverage statement is a count; Iraqi national card, Kurdish script and chip reading not stated; cloud |
| Large processed volume (over 1 billion transactions) | No MENA reference on the pages read; no public price |

### 10.4 Lesson

The same as Onfido: a coverage count is not a test. **Hook**: the vendor evaluation workspace includes an identity-document test (Iraqi national card front and back, chip, Arabic and Kurdish names, family number, residents' card, legacy cards) with pass criteria on field accuracy and false rejects.

---

## 11. e-KYC in Iraq: the national card, local providers and what a bank can actually verify

### 11.1 What the CBI requires for remote and digital customers (AML controls for e-payment service providers, 20 April 2025)

Read from rendered images of the scanned Arabic text (document pages 12 to 16) and translated here. These controls bind **e-payment companies**; they are the only CBI text found that
specifies biometric and document-forensic methods, and banks' digital channels will be judged against the same expectations. The CBI's digital-bank licensing controls (April 2026) require
screening and AML systems linked to the core system (item 13, file 01 s.2e) but the text read has no onboarding method.

| Page | Requirement (translated) |
|---|---|
| 12 | EDD for complex or unusual transactions without clear purpose; **non-resident customers are high risk**: check that residence is valid in Iraq, obtain a copy of ID or travel document, and for legal entities the incorporation contract attested by the competent authority in the home country or the Iraqi embassy; EDD for designated non-financial professions (lawyers, precious metals and stones dealers, real estate agents, accountants) |
| 13 | A list of "high-risk senior position holders" and their relatives for EDD (President and deputies, ministers, party heads, Speaker and members of the Council of Representatives, Supreme Judicial Council, independent bodies, undersecretaries and inspectors, ambassadors, directors general, judges of all courts, senior security commanders, NGO heads and boards), each "and those of equivalent rank" |
| 14 | EDD for countries not applying FATF recommendations and for social-media celebrities and digital merchants; **prohibited**: dealing with anonymous or fictitious names, persons acting for others without licence, merchant categories the CBI bars, and persons on local and international ban lists |
| 15 | **Document verification**: visual verification of authenticity, validity and absence of tampering and security features; "verify that all documents contain the same basic information accurately without contradictions" (full name as in all documents, date of birth, ID numbers, issue dates, language differences); use modern software, including **AI image forensics** of document images and **comparison of biometric data (fingerprints or selfie) between presented documents and the original sources** |
| 16 | For mobile apps: **biometric verification (face) to secure login** against the stored image; the app works only on the customer's own device (phone number or device ID); **one unique account per user**; periodic monitoring of login activity; secure password and account controls; **recorded video calls** to verify the customer and identity when needed, with the customer told the call is recorded and kept; extra documents for large or unusual transfers |

### 11.2 What the Iraqi national card and registries offer

- The national unified card (compulsory since 2016, ten-year validity, ICAO chip and MRZ, about 40 million issued per trade press) carries given name, father's and paternal grandfather's names, surname or tribe, **mother's name and maternal grandfather's name**, **family number (18 alphanumeric characters)**, birth data and document numbers (file 01 s.7). Uqudo's scan schema returns these fields (s.6.2). Legacy documents still accepted by banks: civil status ID, nationality certificate, residence card, ration card, family card, passport.
- **Registries**: the CRD portal for companies (shareholders and nationalities; BO declarations to reporting entities for about USD 4, file 26 s.8b) and the CBI beneficial-owner platform (37,286 statements by May 2026).
- **Bank-facing verification of a person against the civil registry**: I found **no public evidence** of an API or service through which a bank verifies a national card or civil status record against the Ministry of Interior database. The Prime Minister's "Ur" portal offers "electronic inquiries" (nature not examined), the Ministry of Interior site returned a server error (HTTP 525) in the probe, and ZainCash's site returned 403. If a verification channel exists, it is not documented publicly; this is a question for the CBI and the Ministry (open item).

### 11.3 What the vendors claim for Iraq

| Vendor | Iraq evidence | Verdict |
|---|---|---|
| Uqudo | IRQ_ID citizens and residents schemas; Kurdish to Arabic standardised names; family number; MRZ checksum; Iraq in country list | Best documented; chip reading and registry checks for Iraq not stated |
| Entrust (Onfido) | Iraq in a country selector only; site blocked | Unverified |
| Jumio | Iraq in a country list of 200 | Unverified |
| Sumsub | Supported-documents page dynamic; database verification excludes the Middle East in its own description | Unverified |
| IDnow | No Middle East statement | Unverified |

### 11.4 Local actors (limited evidence)

International Smart Card Company (Qi Card) is Iraq's largest card issuer and acquirer (established 2007 with Al-Rafidain Bank; Qi Card site), and Eastnets lists "QI Card" as a screening customer (file 14 s.3); the Qi card pages read state no e-KYC method (a search snippet said the prepaid QiCard relies on fingerprints, not verified). First Iraqi Bank lists "Biometric Identification" among its security features. No local e-KYC vendor with a public product page was found; the CBI controls themselves expect banks and PSPs to use imported or in-house tools.

### 11.5 What it means for AKR-TMS

AKR-TMS does not onboard, so it does not need an identity-proofing vendor. It needs (a) **the data those tools leave in the CBS** to be validated and used, (b) **the onboarding method as a rating factor**, and (c) **detection of what remote onboarding makes easy**: duplicate and synthetic identities, shared devices, shell customers. The CBI's own rules give the checks: consistency of names and dates across documents, one account per user and device, and recorded verification calls.

**AKR-TMS hooks**
- **KYC field validators** (MRZ checksum where stored, ID number format, family number format, name-chain completeness, name and date consistency across the customer's documents), run on every CBS load with a defect queue per branch and a data-quality score (links to file 14 VC-32).
- **Onboarding evidence metadata** read from the CBS: channel (branch, e-KYC, video), vendor, liveness and match result, document-check result, officer; **non-face-to-face onboarding** as a rating factor and as a trigger for a ninety-day enhanced watch.
- **Shared-attribute detection**: the same national ID, family number, phone, device, address or (if stored) face hash across several customers raises a link and an alert (also the "ghost employee" and mule typologies, file 01 s.9).
- **Digital-channel binding monitor**: where the digital channel database is readable, breaches of "one account per user and device" and abnormal login patterns.
- **Open item for the CBI**: whether banks may verify identity against the Ministry of Interior's national card database and by what channel.

---

## 12. Cross-cutting: customer risk rating engines (summary; the method and the proposed AKR-TMS model are in file 26)

| Vendor | How the rating is built (vendor pages) | Gap for an Iraqi bank |
|---|---|---|
| Fenergo | Risk Engine with multiple models selected by dynamic scoping rules; recalculates on profile changes | Logic is configured, not shown; no Iraqi factors; SaaS |
| Pega | Configurable rules and decisions inside CLM and KYC; business users edit rules and questionnaires | Rating logic is a build; no Iraqi content |
| Moody's (Passfort, Maxsight) | Risk tiers (low, medium, high) from "your internal definitions", policy routing | The bank must define the logic; data is Moody's |
| Encompass | Digital risk profile from CDI data with configurable retrieval by policy | Corporate focus; rating is secondary to data |
| Sumsub | Risk score from factors and weights set by compliance managers, updated as new data arrives | A weight table; no CBI mapping |
| Uqudo, IDnow, Jumio, Entrust | Identity and fraud signals (document, liveness, device, behaviour); not an AML customer rating | Useful inputs only |

None of the vendor pages read describes a mapping to the CBI's three to five levels, reason codes by default, or a reproducible validation. File 26 (s.12 and s.14) sets out the white-box alternative: the same factor blocks (customer, geography, product, channel, behaviour), explicit floors (for example PEP), override limits and an explanation panel. This file adds one requirement: **identity-proofing outputs become rating inputs when they exist in the CBS** (document-check outcome, liveness and match results, onboarding channel), and the rating must show them.

**AKR-TMS hooks**
- **Identity-proofing factors** in the rating's channel block: onboarding channel, document check result, liveness and match scores (if stored), re-verification date.
- **Rating-configuration simulation before release** (NICE's feature, file 26 s.12), shared with the screening change sandbox (file 14 VC-25).

---

## 13. Cross-cutting: periodic and event-driven reviews

### 13.1 What the vendors claim

| Vendor | Claim (pages, 2026) |
|---|---|
| Fenergo | Continuous monitoring: detect changes from integrated data and screening providers, assess impact, "determine materiality"; non-material updates processed by policy |
| Pega | Perpetual KYC: review triggered "when relevant changes occur"; rules configurable |
| Moody's | pKYC: data updated "as relevant changes occur"; change-driven monitoring, flags that trigger EDD or investigation; case management |
| Encompass | pKYC replacing "static, periodic reviews with dynamic, continuous monitoring"; source-attributed changes |
| Sumsub | Ongoing AML monitoring; risk score updated as data and behaviour change; reusable KYC reports with fresh timestamps |
| Jumio, IDnow | Ongoing AML monitoring alerts by API or e-mail |

All of these depend on **external change feeds** (registries, screening lists, adverse media) and on the customer data the bank holds. In an Iraqi bank, two further constraints apply: the CBI sets calendar duties (classification review at least every two years, yearly KYC form update, file 01 s.2d) that perpetual KYC does not replace, and the registries that would signal ownership changes are weak (file 26 s.8b). File 26 s.6 holds the trigger catalogue (TR-01 to TR-26), the calendar by tier, the evidence on review quality, and the EU limits; it concludes that "triggers beat calendars" while keeping a calendar backstop.

### 13.2 Trigger sources available to AKR-TMS with a read-only CBS

| Source | Examples | Need from vendors? |
|---|---|---|
| CBS data | Profile amendments (name, address, phone, signatory, owner), account type changes, dormancy and reactivation, limit changes, unusual activity versus expected activity, new products | No |
| Screening layer | New list entries, PEP changes, adverse-media hits (files 14 and 15) | Optional data feeds |
| Registry data | CRD changes in shareholders or directors; BO declarations | CRD portal (about USD 4 per declaration) and commercial data |
| Case system | STR filed, RFI received, FIU or court request, internal investigation | No |
| Onboarding evidence | Document expiry, failed re-verification | CBS fields |

**AKR-TMS hooks**
- **Review queue generator**: calendar reviews (by tier) and trigger-based reviews in one queue, each with its trigger, due date and evidence required; a materiality filter decides whether a change opens a review, a data update or nothing.
- **Review completion evidence and metrics**: percentage on time, backlog age, triggers that led to a rating change, reviews closed without evidence (file 26 s.6e measures).
- **Registry-change adapter** for the CRD portal (manual or file-based at first) producing trigger events.

---

## 14. Cross-cutting: evidence vaults and document management

### 14.1 What the vendors offer

| Vendor | Evidence feature (pages, 2026) | Gap |
|---|---|---|
| Encompass | Attribute-level lineage ("origin of every piece of information"); time-stamped digital audit trail with source documents and provenance | No statement on immutability, retention schedules or export formats |
| Fenergo | "Every action, decision, source and rationale ... carried forward in the record"; SOC 2 Type II report on request | Same |
| Sumsub | Reusable KYC report with fresh timestamps and audit logs | Same |
| Moody's | Workflow history trail and case notes | Same |
| Uqudo | Audit trails; configurable data retention (not detailed) | Same |

None of the pages read describes **write-once storage, tamper evidence, legal hold, retention by record class, or an examiner export package**. These are what the CBI inspectors, the FIU (Egmont information requests, file 04 s.3) and USD correspondents (CBDDQ and RFIs, file 04 s.2c, s.2p) will ask for.

### 14.2 What an Iraqi bank's vault must hold and prove

Records to keep at least five years after the end of the relationship, quickly retrievable, with off-site backup (Law 39/2015 art. 11; CBI controls), ten years as an option for sanctions-related records (OFAC, file 05 s.1e). Contents: identity and ownership documents (Arabic, Kurdish, Latin), profile snapshots from data vendors (file 15 VC-45), screening results including no-hit, alert and case files, internal reports with the acknowledgment and written decision required by the CBI, STR packages and goAML receipts, RFI answers, and review and EDD records. Evidence must carry **who, when, what version and why** and resist later alteration.

**AKR-TMS hooks**
- **Vault on the stack's object store** (MinIO with S3 Object Lock in compliance mode, versioning on) for documents, snapshots and exports; per-object SHA-256, a signed manifest per case and per day, and optional external timestamping of manifests.
- **Retention by record class** with legal hold, expiry review (not auto-delete) and access logs; roles that prevent tipping-off (restricted case visibility).
- **Examiner and correspondent export**: one package per customer or case containing documents, attribute lineage, decisions, approvals, hash manifest and a human-readable index in Arabic and English.
- **Arabic and Kurdish document text layer**: OCR at ingestion with the original preserved, searchable by name chain and number.

---

## 15. Comparison table

| Vendor | Type | Delivery | Iraq and Arabic evidence | CDD, EDD and review engine | Rating | Evidence and lineage | Public price signal | Main caveat | Role for AKR-TMS |
|---|---|---|---|---|---|---|---|---|---|
| **Fenergo** | CLM and KYC system of record, AI agents (KYRA) | SaaS only (multi-region) | None; Dubai office | Policy Manager; rule packs for 120+ jurisdictions; Related Party Manager; materiality-based continuous monitoring | Risk Engine, multiple models | Action, decision, source and rationale carried in the record | None (unverified Irish public notices EUR 0.8 to 2.0 million) | SaaS and write-access master; no Iraqi pack; large-bank scale | Learn (policy and materiality); do not integrate as a dependency |
| **Pega CLM and KYC** | Low-code CLM and KYC | Not verified (Pega Cloud or client-managed) | None | Configurable rules and questionnaires; perpetual KYC triggers | Configurable rules | Auditability stated | None | Heavy platform; dated connector list | Learn (form and rule designer) |
| **Moody's Passfort and Maxsight** | Workflow joined to Orbis and Grid data | SaaS | None | Configurable CDD and EDD workflows; pKYC; case management | Tiers from internal definitions | Workflow history | None for workflow (data: file 15) | Logic left to the bank; Iraqi coverage unverified | Optional ownership enrichment; learn (workflow) |
| **Encompass** | Corporate KYC and CDI profiles | SaaS (not stated otherwise) | None | UBO verification; registry retrieval; pKYC | CDI risk profile | **Attribute lineage and digital audit trail** | None (unverified Swedish notice SEK 3.2 million) | Corporate focus; Iraqi registries not connected | Learn (attribute provenance) |
| **Uqudo** | MEA identity proofing, fraud, compliance | SaaS, private cloud, on-premise, hybrid; in-country hosting | **IRQ_ID schema, Kurdish OCR, Kurdish to Arabic names, family number, MRZ check** | Identity checks, screening, monitoring | Not an AML rating | Audit trails | None | Iraqi chip and registry checks not stated; small base | Learn (Iraqi ID model); candidate partner for banks needing e-KYC |
| **Entrust (Onfido)** | Identity proofing and fraud | Cloud | Iraq in country selector only; site blocked | Orchestration of checks | n/a | Auditable verification (stated) | None | Iraq unverified | Benchmark only |
| **Sumsub** | Identity, KYB, AML, monitoring | SaaS | Not confirmed | Case management, FIU reporting, ongoing monitoring | Weighted factor risk score | Reusable KYC reports with timestamps and audit logs | **USD 1.35 and 1.85 per verification** (public) | Database verification excludes the Middle East; confidentiality of reusable KYC | Price comparator |
| **IDnow** | EU identity proofing, video ident | Cloud | None | AML screening; specialist video review | n/a | Not stated | None | EU focus | Learn (human review for high risk) |
| **Jumio** | Identity proofing, screening | Cloud | Iraq in country list of 200 | Screening and monitoring alerts | n/a | Not stated | None | Count is not coverage | Benchmark only |

---

## 16. What it means for AKR-TMS

1. **None of the CLM vendors fits the deployment brief.** Fenergo and Moody's are SaaS only, Encompass and the identity vendors are cloud-first, Pega is not confirmed, and the CLM systems expect to be the system of record with write access. AKR-TMS (read-only, on-premises, Arabic-first) is not a CLM replacement; it is the KYC risk, review and evidence layer around the CBS, and the vendors are best treated as references for workflow and evidence design, and as optional data or identity inputs.
2. **Iraqi content is the gap every vendor leaves.** No CLM vendor shows an Iraqi regulatory profile (CBI controls, CDD instructions), the Iraqi ID model, family-number relationship detection or the CRD and CBI BO platforms. Only Uqudo documents the Iraqi national card in detail. The AKR-TMS regulatory profile object, three-form name model, KYC field validators and CRD adapter are therefore the differentiators.
3. **Copy five design ideas**: policy-driven requirement engine (Fenergo); configurable questionnaires and rules (Pega); materiality filter for change events (Fenergo, Moody's); attribute-level lineage and audit trail (Encompass); and the Iraqi ID field and name model (Uqudo).
4. **Make rating and review white-box.** Vendors sell "continuous" and "perpetual" KYC with configured weights or tiers; none shows logic or validation. File 26's white-box model with floors, reason codes and an explanation panel is the better answer for a CBI-approved assessor, and identity-proofing outputs should feed it when present.
5. **The CBI's e-payment controls set the remote-onboarding bar** (face login, device binding, one account per user, recorded video call, document forensics, comparison with original sources). AKR-TMS cannot enforce them in the onboarding tool, but it can check their evidence in the CBS and detect their failure modes.
6. **The evidence vault is a product feature none of the vendors documents** at the level an examiner will ask: write-once storage, retention by class, legal hold, export packages. The stack (MinIO with object lock) makes it feasible on-premises.
7. **Verification against the civil registry is an open question.** No public service for banks to verify the national card against the Ministry of Interior was found; asking the CBI is an action for the plan.
8. **Costs**: public price comparators are Sumsub's USD 1.35 to 1.85 per verification and the unverified public notices for Fenergo-class systems (EUR 0.8 to 2.0 million). Iraqi banks buying identity proofing from abroad would carry both cost and data-residency risk, which is the argument for local or on-premise options (Uqudo offers them).

---

## 17. Derived requirements (VC-xx, continuing from file 15, which ends at VC-65)

| ID | Requirement | Source |
|---|---|---|
| VC-66 | Iraqi regulatory profile object: requirement id, client type (individual, sole trader, company, NGO, endowment, non-resident, PEP), stage (onboarding, periodic, trigger), source clause (CBI controls, Law 39/2015, CDD instructions), evidence required, approver level, review interval; drives CDD and EDD tasks | Fenergo Policy Manager and rule packs (s.2); file 26 s.5 |
| VC-67 | Materiality rules for KYC change events (name, address, owner, signatory, activity, PEP status, high-risk country link): a change opens a review, updates data, or is logged as immaterial, with the rule and reason stored | Fenergo (s.2); Moody's pKYC (s.4) |
| VC-68 | Questionnaire and rule designer (Arabic and English, right to left) with version history, approval and effective dates, used for EDD, SoW/SoF and periodic review forms | Pega (s.3) |
| VC-69 | Every review or EDD outcome records the rule applied, the evidence and the approver, not only the decision | Moody's "customer-determined outcomes" (s.4); file 26 s.10 |
| VC-70 | Attribute provenance table (attribute, value, source class, source reference, retrieved at, verified by) shared by KYC, ownership and screening, shown on ratings and alerts | Encompass lineage (s.5) |
| VC-71 | Three-form name model for every name part (original Arabic or Kurdish script, standardised Arabic, transliteration, with method) and a canonical field set: mother's name, both grandfathers, family number, personal number, place of birth, card access number, MRZ validity | Uqudo IRQ_ID (s.6); file 22 |
| VC-72 | KYC field validators on CBS data (MRZ checksum where stored, ID number format, family number format of 18 characters, name-chain completeness, consistency of names and dates across documents) with a defect queue per branch | Uqudo (s.6); CBI e-payment controls p.15 (s.11) |
| VC-73 | Onboarding evidence metadata read from the CBS (channel, vendor, liveness and match results, document-check result, officer); non-face-to-face onboarding is a rating factor and opens an enhanced watch | CBI e-payment controls (s.11); file 26 s.3d |
| VC-74 | Shared-attribute detection across customers (national ID, family number, phone, device, address, face hash if stored) creating links and alerts | CBI "one unique account per user" (s.11); ghost-employee typology (file 01 s.9) |
| VC-75 | Digital-channel binding monitor (one account per user and device, login anomalies) where the channel database is readable | CBI e-payment controls p.16 (s.11) |
| VC-76 | Identity-document test pack in the vendor evaluation workspace (Iraqi national card front and back, chip, Arabic and Kurdish names, family number, residents' card, legacy cards) with pass criteria | Jumio and Entrust coverage counts (s.7, s.10) |
| VC-77 | Store a reference to the recorded verification call for high-risk remote customers; EDD workflow can require it | CBI e-payment controls p.16; IDnow video ident (s.9) |
| VC-78 | Ingest identity-proofing results (document check, liveness, face match score, source) as rating inputs when present in the CBS | s.12 |
| VC-79 | Review queue generator combining calendar reviews and trigger-based reviews, each with trigger, due date and evidence required; completion evidence; on-time and backlog metrics | Vendor pKYC claims (s.13); file 26 s.6 |
| VC-80 | Trigger adapters for registry changes (CRD portal), screening changes, adverse-media hits, PEP updates and case events, with source and timestamp | s.13.2; file 15 s.13 |
| VC-81 | Evidence vault on object storage with write-once retention (object lock), per-object hash, signed manifests, retention by record class (five years after the relationship as a minimum; ten-year option for sanctions-related records), legal hold, access logs and role restrictions against tipping-off | s.14; file 05 s.1e; file 01 |
| VC-82 | Examiner, FIU and correspondent export package per customer or case: documents, attribute lineage, decisions, approvals, hash manifest, bilingual index | s.14; Wolfsberg RFI guidance (file 04 s.2p) |
| VC-83 | Arabic and Kurdish OCR and text layer for evidence documents, original preserved, searchable by name chain and number | Uqudo and Encompass document processing (s.5, s.6) |
| VC-84 | Total-cost view that includes identity-proofing fees (public comparator USD 1.35 to 1.85 per verification) even when onboarding stays in the CBS | Sumsub (s.8) |
| VC-85 | AKR-TMS never writes to the CBS or to onboarding systems; tasks, forms, approvals and evidence live in AKR-TMS with a one-way change feed | AKR-TMS brief; file 26 s.12 |
| VC-86 | Integration policy for KYC and identity tools: read outputs only from the CBS or exported files; any connector that would send customer images or documents off premises requires explicit bank enablement and a residency flag | s.16; file 15 VC-61 |

---

## 18. Sources

Pages read 2026-10-06 unless a date is given. Items marked (search result) were found in a search result and not opened. Onfido's own site, Gartner Peer
Insights, G2, TrustRadius and Capterra block automated access; the web-search budget ran out during the work.

**Fenergo**
- Home: https://www.fenergo.com/ ; KYC: https://www.fenergo.com/know-your-customer-kyc-solution ; CLM: https://www.fenergo.com/client-lifecycle-management ; platform: https://www.fenergo.com/platform ; AI agents: https://www.fenergo.com/ai-agents ; integrations: https://www.fenergo.com/integrations ; clients: https://www.fenergo.com/clients ; trust: https://www.fenergo.com/trust-and-security ; contact (Dubai office): https://www.fenergo.com/contact ; company (Astorg and Bridgepoint): https://www.fenergo.com/company/discover-fenergo ; Moody's partnership: https://www.fenergo.com/partners/moodys
- TED notices 100252-2021 and 721899-2022 (full-text matches for "Fenergo"; contractor unreadable), via the TED API: https://api.ted.europa.eu/v3/notices/search

**Pega**
- Client Lifecycle Management: https://www.pega.com/industries/financial-services/client-lifecycle-management ; resource page: https://www.pega.com/insights/resources/pega-client-lifecycle-management

**Moody's**
- KYC and compliance: https://www.moodys.com/web/en/us/kyc.html ; pKYC: https://www.moodys.com/web/en/us/kyc/solutions/pkyc.html ; KYC and KYB automation: https://www.moodys.com/web/en/us/kyc/solutions/automation.html ; AML automation: https://www.moodys.com/web/en/us/kyc/solutions/aml-automation.html ; Maxsight: https://www.moodys.com/web/en/us/solutions/compliance-third-party-risk/maxsight.html

**Encompass**
- Home: https://www.encompasscorporation.com/ ; data sources: https://www.encompasscorporation.com/encompass-platform/kyc-data-sources/ ; data attribution lineage: https://www.encompasscorporation.com/encompass-platform/data-attribution-lineage/ ; digital audit trail: https://www.encompasscorporation.com/encompass-platform/digital-audit-trail/ ; perpetual KYC: https://www.encompasscorporation.com/encompass-platform/perpetual-kyc-due-diligence/ ; UBO verification: https://www.encompasscorporation.com/encompass-platform/ubo-verification/
- TED notice 360865-2023 (Exportkreditnämnden; contractor unreadable): https://ted.europa.eu/en/notice/360865-2023/pdf

**Uqudo**
- Home: https://www.uqudo.com/ ; identity: https://www.uqudo.com/identity/ ; compliance: https://www.uqudo.com/compliance/ ; IRQ_ID documentation: https://docs.uqudo.com/docs/kyc/uqudo-api/scan/country-specific-ids/irq_id-iraqi-id

**Entrust (Onfido), Sumsub, IDnow, Jumio**
- Entrust Identity Verification (Internet Archive copy, 2026): http://web.archive.org/web/2026/https://www.entrust.com/products/identity-verification
- Sumsub: https://sumsub.com/ ; pricing: https://sumsub.com/pricing/ ; AML risk score: https://sumsub.com/aml-risk-score/ ; reusable KYC: https://sumsub.com/reusable-kyc/ ; global database verification: https://sumsub.com/global-database-verification/ ; AML screening: https://sumsub.com/aml-screening/ ; supported documents: https://sumsub.com/supported-documents/
- IDnow: https://www.idnow.io/ ; identity verification: https://idnow.io/solutions/identity-verification/ ; document verification: https://idnow.io/solutions/identity-verification/document-verification/ ; AML screening: https://idnow.io/solutions/kyc-checks/aml-screening/
- Jumio: https://www.jumio.com/ ; identity verification: https://www.jumio.com/products/identity-verification/ ; screening: https://www.jumio.com/products/screening/ ; KYC and AML: https://www.jumio.com/use-case/kyc-aml-compliance/ ; global coverage: https://www.jumio.com/global-coverage/

**Iraq**
- CBI AML/CFT controls for e-payment service providers, 20 Apr 2025 (scanned, 48 pages; pages 12 to 16 read from images): https://cbi.iq (file in the project scratchpad: cbi-aml-epayment-2025.pdf; listing page https://cbi.iq/ under banking supervision)
- Qi Card (International Smart Card): https://qi.iq/en/about-us ; First Iraqi Bank: https://fib.iq/ ; Ur portal: https://ur.gov.iq/
- Files in this project: 01 (Iraq regulatory framework), 04 (Wolfsberg, Egmont, UN), 05 (OFAC), 14 and 15 (screening and data vendors), 22 (Arabic names), 26 (KYC, CDD and customer risk rating)

