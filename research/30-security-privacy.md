# 30 — Security and privacy: Iraqi rules, standards, encryption, access control, tipping-off, insider threat, audit, retention, supply chain and continuity

Status: done (2026-10-06)

Stack alignment (2026-10-07, file 32 sections 12-13): the user chose **OpenBao** instead of HashiCorp Vault, **Valkey** instead of Redis, and to **keep MinIO** as a pinned, maintained and renamed build (Silo, built from source; Chainguard's fork as second source). Mentions below were aligned on that date; where the text describes the digital-banking-system (DBS), its own Vault and Redis are kept as they are.

Scope: what an AML and KYC platform that holds the most sensitive data in a bank (customers, transactions, suspicion reports, investigation notes) must do to be safe to install in an Iraqi bank, and how AKR-TMS should be built to meet it. Requirement IDs use the prefix SP. Cross-references: file 01 (section 8 hosting and data protection; chapter 4 record keeping; tipping-off; CBI 2026 risk-management controls), file 05 (OFAC 10-year records; NYDFS Part 504), file 25 (the read-only link), file 32 (stack choices), file 29 (governance), file 23 (goAML reporting).

## Contents

1. Scope, assets and threat model (including the insider threat shown by the October 2025 OFAC designations)
2. Iraqi rules: CBI cyber-resilience and IT governance controls, hosting, banking secrecy, tipping-off, data protection
3. Standards map: ISO 27001:2022, ISO 27701, ISO 22301, PCI DSS, SWIFT CSP, NIST, CIS
4. Trust zones, network segmentation and the data-flow boundary
5. Encryption in transit and at rest; key management
6. Identity, authentication and sessions
7. Authorisation: RBAC and ABAC, segregation of duties, maker-checker, branch scoping, break-glass
8. Tipping-off controls: who may see suspicion-report information
9. Insider threat, privileged access and "who viewed which customer"
10. Immutable audit logs and the bank's SIEM
11. Data minimisation, purpose limitation, classification, masking and pseudonymisation (PCI scope)
12. Retention and deletion (5, 7 and 10 years, legal hold, crypto-shredding)
13. Vendor access, remote support and operations
14. Secure development and supply chain (SBOM, signed images, dependency control)
15. Penetration testing and independent assurance
16. Business continuity and disaster recovery (RPO, RTO, what happens when AKR-TMS is down)
17. Tenant isolation if hosted
18. AI and ML security (self-hosted models only)
19. Privacy of data subjects within AML duties
20. What it means for AKR-TMS
21. Derived requirements
22. Sources

Evidence marks ([V], [S], [K]) are defined in section 1 and again in section 22; the points that could not be verified are collected in section 20.5. Derived requirements SP-01 to SP-102 are in section 21.

---

## 1. Scope, assets and threat model

Evidence marks: **[V]** read in a primary document, repository or registry during this research (the CBI documents were read from their PDFs; the scanned ones were read page by page from rendered images, the translations are ours); **[S]** secondary source; **[K]** practitioner knowledge to be confirmed.

### 1.1 What AKR-TMS holds, ranked by damage if it leaks or is altered

| Rank | Asset | Why it matters |
|---|---|---|
| 1 | **Suspicion information**: alerts, investigation notes, STR drafts and filed STRs, MLRO decisions, FIU correspondence, freeze orders | Disclosing that a report or investigation exists is a crime in Iraq (Law 39 of 2015, Articles 12 and 39; section 2.1); leaks endanger staff and the investigation |
| 2 | **Customer, account and transaction data** copied from the CBS (the whole customer base, years of postings) | Banking secrecy; a bank-wide data set of the highest commercial and personal value; target for theft and for sale |
| 3 | **Detection logic**: scenarios, thresholds, whitelists, model features, list-matching settings | Tells a launderer exactly how to stay under the radar; the enforcement record shows threshold changes and customer whitelists used to reduce alerts (file 20) |
| 4 | **Audit trail and evidence vault** | Their integrity is what lets the bank prove to the CBI, an assessor or a correspondent what it did and when |
| 5 | **Screening lists and decisions** (whitelists, false-positive clearances) | A tampered clearance silently lets a listed party through |
| 6 | **Keys and credentials**, including the read-only CBS accounts and the phase-2 signing key | The keys unlock everything above; the signing key authorises releases |
| 7 | **Availability** of monitoring and, in phase 2, of the pre-release assessment | An outage is a monitoring gap and, for payments, a stop or an unscreened release (file 25 section 15) |
| 8 | **Models and training data** | Intellectual property of the bank, and a route to learn the detection logic |

### 1.2 Adversaries

| Adversary | Iraq-specific note | Main paths |
|---|---|---|
| **Bank insiders with a motive to launder or to tip off** | OFAC's 9 October 2025 designation of three Iraqi bankers accused of using their positions to "generate revenue and launder money" for armed groups and the IRGC, "tens of millions" (file 05 section 1e); file 05 lists "insiders (bank staff and officers) who use their position to move funds" as a typology the system must see | Reading STR status, warning a customer or relative, suppressing an alert, changing a threshold, whitelisting a customer, deleting or editing a case |
| **Compliance insiders who tamper under pressure** | The enforcement record: thresholds set "based on a desire not to generate 'too many alerts'", a customer "Verified List" that switched monitoring off, falsified and backdated review evidence (file 20) | Configuration changes, whitelists, alert closure at volume, editing evidence after the fact |
| **Branch and operations staff** | CBI requires branch liaison officers and a confidential channel to the MLRO that branch management cannot see (file 01 section 2d) | Seeing an account hold reason, a case flag in a list, a notification, a printout |
| **IT insiders: DBAs, system administrators, developers, integrators** | CBI says direct database access is for DBAs only (cyber-resilience controls 12.1.3) | Reading the database or backups, extracting keys, changing the audit trail, altering code in production |
| **Vendor and integrator staff** | The CBS vendor and AKR-TMS support staff may be abroad; CBI classification rules limit remote access and non-Iraqi access to confidential data (section 2.4) | Support sessions, remote administration, update delivery |
| **External attackers** | Ransomware and espionage against banks; a hostile information environment around Iraqi banks and dollar access (file 01 section 4) | Phishing, exposed services, supply chain, VPN and remote-access abuse, credential theft |
| **Supply-chain attackers** | A product shipped to many banks is a single point of leverage | Malicious dependency, compromised build, tampered release bundle |
| **Coercion and legal-process abuse** | Armed groups are named in the US designations that reach Iraqi banks and bankers (file 05 section 1e); staff of a bank handling such cases can be pressed to disclose [K] | Pressure on staff to disclose; informal requests for data outside legal process |
| **Physical and environmental** | Data-centre theft, fire, power, conflict-related disruption (the CBI's continuity controls name wars and riots) | Theft of servers or backups, loss of site |

### 1.3 Trust zones and the data flow

```text
 Z0 CBS network (bank-owned)         Z1 CBS-facing zone                 Z2 Application zone
 CBS DB / replica / views  <-------- akr-ingestd (only process  ------> akr-worker, PostgreSQL,
 (read-only account;                 with CBS credentials/route)         object store, OpenBao
  kill switch = DBA)                                                      |
                                                                          |
 Z3 Analyst zone (bank LAN)  <-- edge proxy --> akr-api (BFF, Kratos)  ---+
 analysts, MLRO, liaison officers                                         |
                                                                          |
 Z4 Gateway zone                 <== mTLS ===> akr-assess (phase 2)  -----+
 payment hub / SWIFT interface                                            |
                                                                          |
 Z5 Management zone: jump host/PAM, Harbor, CI import, backup target, SIEM collector
 Z6 DR site: standby, offsite backups, second OpenBao
 Z7 Vendor: no standing path; time-boxed, bank-approved sessions only (section 13)
 Outbound: FIU goAML submission (file 23); list providers (if allow-listed); nothing else
```

### 1.4 Security objectives

1. **Confidentiality by need to know**, strongest for suspicion information (section 8).
2. **Integrity with proof**: any change to data, configuration or evidence is attributable and tamper-evident (section 10).
3. **Accountability**: every read, export and decision is attributable to one person (section 9).
4. **Availability with honest degradation**: outages become visible blind periods, never silent gaps (file 25 sections 5.4, 8.2, 15.6; section 16).
5. **Containment**: compromise of one component (the web tier, the CBS link, a vendor laptop) must not expose the rest; the CBS must be unaffected (read-only link, file 25).

### 1.5 Top threats and the controls that answer them

| Threat | Control (section) |
|---|---|
| A staff member looks up whether a relative or friend is under investigation | Restricted-customer tier, relationship-conflict check at access time, view logging, canary records (8, 9) |
| An analyst or manager suppresses alerts by editing a threshold or whitelisting a customer | Maker-checker with a different approver, expiring whitelists, volume-reduction flag, immutable change log (7, 10; file 29) |
| A DBA or sysadmin reads the database, a backup or a log | Field-level encryption with keys the DBA cannot reach, separate roles, encrypted backups, DBA activity monitored (5, 9) |
| The web tier is compromised | No CBS credentials in it; mTLS between components; least-privilege roles; zones (4) |
| The CBS link is abused to reach the CBS | Read-only account on bank-owned views, bank-side audit, kill switch, no per-customer queries (file 25 section 10) |
| A vendor support session exfiltrates data | No standing access; redacted bundles; recorded, time-boxed, bank-approved sessions (13) |
| A poisoned dependency or release | SBOM, signed images and bundles, admission policy, bank approval before change (14; file 32 section 16) |
| Ransomware encrypts the platform and its backups | Object-locked offsite backups, separate credentials, restore drills, blind-period accounting (16) |
| A forged RELEASE reaches the payment gateway | Signed, expiring decisions verified by the gateway; release-without-decision control (file 25 section 15) |
| An attacker learns the thresholds and structures below them | Thresholds hidden from branch roles and from non-compliance staff; logic changes audited (7, 8) |

---

## 2. Iraqi rules: confidentiality, tipping-off, cybersecurity, hosting and data protection

### 2.1 Anti-Money Laundering and Counter-Terrorism Financing Law No. 39 of 2015

Read from the English translation on the CBI site [V] (https://cbi.iq/static/uploads/up/file-150199913642139.pdf).

| Article | What it says | AKR-TMS consequence |
|---|---|---|
| 11 | Records are kept "for five (5) years after the business relationship with the customer has ended, the account was closed, or the transaction was carried out for an occasional customer, whichever is longer" and must be "available to competent authorities in a timely manner". They include copies of all due-diligence records, "all records of transactions, both domestic and international, attempted or executed... sufficiently detailed to permit the reconstruction of each individual transaction", "copies of suspicious transaction reports sent to the Office and documents related thereto for at least five years after the date the report was made or a ruling was issued in a lawsuit thereof", and records of the risk assessment | Retention classes with a relationship-end clock, attempted transactions kept, STR records kept at least five years from the report date (section 12) |
| 12, Fourth | Institutions must refrain "from disclosing to the customer, beneficiary or any other person except competent authorities... any legal procedures taken regarding transactions or financial activities suspected of money laundering or terrorism financing" | The tipping-off control set (section 8) |
| 12, Fifth | Report "without delay any transaction or attempted transaction" using the Office's form | Case workflow deadlines (file 28) |
| 39, First | A fine of IQD 25 million to 250 million for failing to keep records "for the period stipulated" | Retention is a compliance control with a penalty, not a convenience |
| 39, Second | Imprisonment of up to three years and a fine of IQD 15 million to 50 million "or any of these two" for anyone who "refrains from submitting STRs to the Office, or intentionally submits false information" or "discloses to the client, beneficiary or any party except the competent authorities and bodies, the implementation of the provisions hereof regarding any action of reporting, investigation or inspection taken with respect to financial transactions suspected to involve ML or FT, or the information related thereto" | **Disclosure is a criminal offence for the individual**; access control and view logging protect the staff as well as the bank |
| 40 | Board members, owners, directors and employees who violate the obligations "deliberately or through gross negligence" are punished by imprisonment and a fine | Gross negligence includes leaving suspicion data open to anyone; a design that restricts access is the defence |
| 48 | "No penal or disciplinary liability for any person who in good faith reports any suspicious transaction... even if it is proved incorrect" | Protects reporters; the reporter's identity must still be protected (section 8) |
| 52 | "Confidentiality provisions stipulated in any law shall not preclude the implementation of the provisions of the present Law" | Banking secrecy does not block AML reporting |
| 29 and 53 | Information exchanged between the Office and foreign counterparts "may not be used for purposes other than combating money laundering, terrorism financing and the predicate offenses"; Council and Office employees must not disclose what they learn, even after service | Purpose limitation on any data received from the FIU or abroad (section 11) |

### 2.2 Banking secrecy

Banking Law No. 94 of 2004, Article 49, protects customer confidentiality; the 2025 CBI controls state that it "must not obstruct AML/CFT controls" and Law 39 Article 52 overrides confidentiality provisions for its own purposes (file 01 sections 2d and 3). Two consequences. First, giving a **third party** (a vendor, an integrator, a cloud host) access to customer data is a disclosure that the secrecy regime does not automatically excuse, so access by outsiders needs a contract, a CBI-compatible outsourcing basis and technical limits (section 13). Second, AML duties do not create a general licence to share: AKR-TMS must keep data inside the bank and send reports only to the Office through goAML (file 23).

### 2.3 CBI sanctions guidelines (1 August 2023)

Read from the scanned Arabic PDF [V] (https://cbi.iq/static/uploads/up/file-169554063119385.pdf). Relevant to security design: the compliance programme policies must "prevent employees, directly or indirectly, from informing the customer or any third party that a freeze or any other measures will be implemented" (p. 15), which extends the tipping-off duty to sanctions freezes; lists are to be kept and updated continuously (24 hours) and the customer database screened daily (pp. 20 and 23, file 25 section 9); policies and training are reviewed at least annually and independent testing covers the systems (pp. 15 to 17).

### 2.4 CBI cyber-resilience controls for the financial and banking sector (first edition, June 2024)

The central Iraqi security rulebook for banks: 48 pages from the Information Technology and Payments Department, built on the NIST Cybersecurity Framework (identify, protect, detect, respond, test) and referencing ISO/IEC 27001, ISO/IEC 27005 and NIST CSF [V] (https://cbi.iq/static/uploads/up/file-171939658616261.pdf, Arabic; item numbers below are the document's own). Its structure is the structure of this file.

| CBI control | What it requires (translation ours) | AKR-TMS response |
|---|---|---|
| **Data classification** (pp. 19 to 22) | Four levels: **public**, **restricted** (مقيدة, including "personal information of persons protected under any law"), **confidential** (سري) and **top secret** (سري للغاية). Restricted: awareness before access, clear desk and clear screen, no printing or copying outside, storage only on bank devices or authorised external ones ("such as those used by third parties or cloud service providers"), removable media encrypted, data-leak prevention, e-mail only on the bank's system, wipe before reuse. **Confidential**: the list of authorised staff kept tight, approved and maintained, a signed non-disclosure undertaking, no printing or copying without approvals, storage "only on the institution's own information assets and technologies", **no remote access**, access "at a specific time", e-mail only encrypted with the password sent on another channel and a subject that does not reveal "the content or classification", devices destroyed at end of life, discussion only in secure meeting rooms. **Top secret**: vetting and "a register of all employees who accessed" the information, always encrypted at rest, "may not be stored on non-governmental cloud services", access by **prior written approval**, approvals "may not be granted to non-Iraqi individuals except by exception under international agreements by a justified written order", no electronic transfer by e-mail or file sharing, destruction logged and signed | Label every data class (11.3); suspicion information is at least *confidential* (8); no remote access and time-boxed access for that tier (7, 8, 13); view register (9); no vendor-hosted storage for confidential or top-secret classes (17); notification text that reveals nothing (8, file 32 section 11.5) |
| **8 Identity and access** (intro and 8.1) | "No one by virtue of rank or position" has "any inherent right of access to confidential data, applications, system resources or facilities"; access "only on the basis of need to use and during the period in which access is required"; contractors "closely monitored"; unique user ids; approved access forms; disable after failed logins; access updated with job status; unique logs; **regular reviews of access privileges** | No role sees everything by default, administrators included; time-boxed grants; quarterly access review report (6, 7) |
| **8.2 Passwords** | Strong controls; change at first login; at least **8 characters**; **three of four character classes**; maximum validity **90 days**; lock after **three** consecutive failures; password history of at least three; administrative passwords for the operating system, database and applications kept "in a safe place in a sealed envelope"; revoke on retirement, resignation or termination | Configurable policy meeting these minima on top of mandatory MFA; sealed-envelope break-glass credentials; instant offboarding (6) |
| **8.3 Data input control** | Session time-out per policy; audit log "with user id and date timestamp for data entry, deletion and modification"; "the software must not allow the same user to be both entering and verifying the same transaction" unless the authority allows otherwise; restrict access to sensitive fields | **Maker-checker is a CBI requirement** for entries; AKR-TMS applies it to configuration and decisions (7); field-level permissions (7, 11) |
| **8.4 Privileged access** | Strict vetting, strong authentication, strong remote-access controls, limit the number of privileged users, "need to access" basis, timely review of their activity, **ban shared privileged accounts** | PAM model (9, 13) |
| **12 Database security** | **12.1.3** "direct access to the database shall be limited to database administrators only; other users shall access the database through applications only"; **12.1.5** "migrating a critical system's database or moving it from the production environment to any other environment is prohibited"; 12.2.3 administrators reach the DBMS only over SSH, VPN or TLS; **12.3.6** stored databases "encrypted according to the institution's encryption and classification policies"; 12.4 DBMS event logs enabled and kept, "behaviour of database administrators" monitored and reviewed regularly | Analysts never touch the database; DBAs reach it through the bank's bastion; **no production data in development or test** (synthetic only; file 25 section 13.7); encryption at rest (5); pgAudit and SIEM export (10). The read-only CBS account is a named **service account for an application**, approved through the bank's access process and limited to bank-owned views (file 25 section 10), which keeps 12.1.3 intact |
| **13 Third parties** | Contracts bind the third party to the bank's cyber policy and make it "legally responsible for any breaches"; non-disclosure and "secure deletion of the institution's information by third parties at the end of the service"; vetting of third-party staff on critical systems; **MFA on critical systems**; revoke access at termination; periodic reviews; audit logs kept and "made available at the institution's request" | Vendor access model (13); contract annex (13) |
| **14 Application security** | At least three tiers; secure protocols; WAF for externally reachable applications; **MFA for web applications used by staff or customers reachable from the Internet**; session management including time-out; separate development and test; protected data in non-production masked or deleted; developers must not change production code or data; test accounts and data never in production; **security testing before go-live**, vulnerability scan before production, **independent penetration test periodically**, source-code security review with issues fixed before production; patches on time; OWASP Top 10 recommended | Secure development and assessment (14, 15) |
| **3 Network** | Documented design with security zones; VLAN separation; firewalls and intrusion detection and prevention; internal firewalls; encryption of sensitive data over WAN or public networks; SSH not Telnet; role-based and time-based ACLs; all default passwords changed; unused ports closed | Zones and policies (4) |
| **Detect** | Security monitoring and a SOC capability "in real time or near real time", logs protected and retained, detection of unauthorised changes to critical resources; internal audit of information systems at least yearly; independent audits; **compliance with PCI-DSS, EMV and SWIFT/CSP** (2.4) | SIEM export (10); standards (3) |
| **Respond** | Incident management with severity levels, escalation, root-cause and impact analysis; **notify the CBI "as soon as possible" in case of any disaster**; keep customers informed of major incidents; BCP and DRP with a geographically separate site (or near and far sites), RTO and RPO defined per system, annual tests with documented scope, plan and results; backup policy with encrypted off-site copies, one copy on site, tested restores, retention consistent with legal requirements | Section 16 |
| **Testing** | Regular vulnerability assessment, automated and manual, covering SQL injection and cross-site scripting; remediation verified; penetration tests of network infrastructure and internet-based systems "periodically or as needed" | Section 15 |

### 2.5 CBI standards handbook for private commercial and Islamic banks (2025)

[V, Arabic text] **B7 Infrastructure and data**: all banks must build "robust, secure and stable" infrastructure "capable of withstanding cyber attacks" and protect "all data from unauthorised access and tampering", with compliance on infrastructure and data encryption "ensured by a competent firm from the list of firms approved by the Central Bank"; the requirement is added to the data-storage requirements of Article 38 of the Banking Law. **B2** requires a comprehensive banking system (file 25 section 1.1). **B9**: documented and audited business-continuity and disaster-recovery plans, audited by an approved firm. **D2**: AML/CFT and sanctions tools verified by an approved firm (file 01 section 2e). Design consequence: AKR-TMS will be examined by CBI-approved assessors, so it ships an **assessor pack** (section 15).

### 2.6 CBI risk-management controls (13 July 2026)

[V, Arabic text, commercial-banks edition, Article 7 and the sections on IT risk and continuity] **Outsourcing** (section on service providers): documented, board-approved policies; due diligence; a sound contract "including data ownership and confidentiality and termination rights"; a control environment on both sides with the outsourced activities logged; contingency plans for termination; allow the bank's internal audit and CBI examiners "to assess the activities outsourced". The contract must be reviewed by the bank's legal department, be terminable by the bank "without any financial obligation", guarantee quality and emergency performance, guarantee "full protection of the confidentiality of the information" the provider obtains, ensure the provider makes "no changes to the outsourced service without prior approval of the bank", and keep the bank informed of the service and its performance. **IT risk**: the bank must follow the CBI's IT governance controls and cyber-resilience controls; tests "at least annually and whenever needed, including penetration tests". **Continuity**: board-approved plan from scenario analysis and business impact analysis; alternate site "far from the main site but easy to reach", in a stable area with utilities; continuous testing. **Internal control** includes segregation of duties, dual approval of transactions and technical controls "to prevent tampering" with information, documentation and accounting systems.

### 2.7 Earlier CBI frameworks

The ICT governance controls of 25 April 2019 (74 pages, based on COBIT 5, 17 chapters including reliability, availability and recovery, operational infrastructure security, data-centre protection, access control and e-payment security; vendors and service providers with transfer privileges to sensitive systems must be controlled) [V, scanned; read at chapter level only] and the business-continuity controls of 22 April 2018 (a letter to all licensed banks: board responsibility for a plan under ISO 22301, business impact analysis per ISO 22317, an alternate site meeting international standards, periodic tests that include the alternate site and the branches) [V, scanned]. The 2024 controls above supersede and detail them for cybersecurity.

### 2.8 Hosting, cloud and data protection

- **No data-localisation statute** was found (file 01 section 8), and **no comprehensive personal-data-protection law is in force** (a draft exists). The CMC's digital-platforms framework of February 2025 refers to "international best practices" and to forthcoming Iraqi law (file 01 section 8).
- **The CBI's own classification rules act as localisation rules for the most sensitive data.** Confidential data may be stored "only on the institution's own information assets and technologies" and top-secret data may not go to "non-governmental cloud services" (2.4). Restricted data (personal information) may be stored on "authorised external" devices including those of "cloud service providers". Therefore a vendor-hosted AKR-TMS could hold restricted data if the CBI and the bank agree, but **not suspicion information classified as confidential or above** (section 17).
- **Kurdistan Region**: the CBI supervises banks there under the same rules (file 01 section 9); no separate regional rulebook was found.

### 2.9 What could not be found or read

The CBI "assessment guide" that the handbook refers to (the detailed infrastructure and encryption specifications under B7) is not on the pages read; the text of Banking Law Articles 38 and 49 was not retrieved (quoted through the handbook and file 01); the ICT governance controls of 2019 were read only at chapter level (scanned, 74 pages); the CBI's e-payment AML controls (April 2025) and the digital-onboarding controls (2021, scanned with a defective text layer) were not read in detail. Each is a follow-up for the compliance workstream.

---

## 3. Standards map: ISO 27001, PCI DSS, SWIFT CSP, NIST, OWASP and the supply-chain standards

### 3.1 Which standard binds whom

| Standard | Why it matters here | AKR-TMS position | Evidence the bank can show |
|---|---|---|---|
| **ISO/IEC 27001:2022** and ISO/IEC 27002:2022 (published March 2022 with four control themes: organisational, people, physical, technological [V, Wikipedia]; 93 controls, 11 of them new [K]; the transition from the 2013 edition ended 31 October 2025 [K]) | The CBI cyber controls cite ISO/IEC 27001 and 27005 as references (2.4); banks will ask a vendor for its certificate; a regional CBS vendor announced its ISO/IEC 27001:2022 certification in 2025 [V, icsfs.com], so the market expects it | The **vendor's ISMS** (development, build, release, support) is certified before the first bank delivery; the product is designed so a bank can map it to its own Annex A (3.2) | Certificate and statement of applicability; Annex A mapping of the product; policies |
| **ISO 22301** (business continuity) and ISO 22317 (business impact analysis) | The CBI's 2018 controls require a plan "per ISO 22301" (2.7) | The product ships BIA inputs and DR runbooks; the vendor's own support continuity is documented | BIA worksheet, DR test reports (16) |
| **ISO/IEC 27005** (information-security risk management) | Cited by the CBI | Threat model of section 1 maintained per release | Risk register |
| **NIST Cybersecurity Framework** | The CBI controls are organised on it (identify, protect, detect, respond, test) | This file follows the same order | Mapping table 2.4 |
| **PCI DSS v4.0.1** | The CBI requires banks to comply with PCI DSS (2.4). PCI DSS covers entities that store, process or transmit cardholder data (primary account number, name, expiry, service code) and sensitive authentication data (full track data, card verification codes, PINs and PIN blocks) [K]; sensitive authentication data may not be stored after authorisation, a stored PAN must be unreadable, and a displayed PAN may show at most the first six and last four digits [K]. The PCI SSC document could not be downloaded here (the site returned an access page), so these points are [K] and must be checked against the current text | **Stay out of scope by design**: AKR-TMS never ingests, stores, processes or transmits a full PAN or any sensitive authentication data (11.4); the CBS views, the ingestion filter and the free-text guard enforce it. Systems "connected to" a cardholder-data environment can still be in scope, so the card-system boundary is documented | Data-flow diagram showing no PAN; ingestion scan reports; QSA or bank assessor sign-off |
| **SWIFT Customer Security Programme** (Customer Security Controls Framework, updated yearly) | The CBI requires compliance with SWIFT/CSP (2.4). The Swift pages were not retrievable, so scope rules are [K] | AKR-TMS touches Swift data in two ways: copies of messages the bank archives (outside the secure zone if the bank's policy allows), and in phase 2 an interposition next to the Swift interface. A component connected to that interface is treated as part of the bank's secure-zone scope under the architecture type the bank attested to. AKR-TMS therefore ships a hardening and patching baseline, MFA and logging for the assessment nodes and the message adapter, stores no Swift credentials, and treats Swift messages as *confidential* | The bank's own CSP attestation, with the AKR-TMS components listed in scope |
| **OWASP ASVS 5.0.0** (released 2025-05-30 [V, GitHub]) and OWASP Top 10 | CBI 14 recommends the Top 10 for important web applications | ASVS Level 2 as the baseline for the whole web application and API; Level 3 targets for authentication, session handling, the assessment API, key handling and the tipping-off controls [K, our target] | ASVS checklist per release |
| **NIST SP 800-218 SSDF** (version 1.1, February 2022 [V]) | Secure software development framework | Section 14 maps the build pipeline to it | SSDF mapping |
| **NIST SP 800-57** (key management) and **SP 800-63B** (digital identity) | Key hierarchy and authenticator policy | Sections 5 and 6 [K]. Note that SP 800-63B advises against forced periodic password changes while the CBI sets a 90-day maximum; AKR-TMS supports the CBI policy and treats phishing-resistant WebAuthn credentials as the preferred factor [K] | Policy settings |
| **CIS Benchmarks** (Ubuntu 24.04, PostgreSQL, Kubernetes, Docker) | Hardening baselines (the DBS production host is Ubuntu 24.04, `deploy/prod/README.md` [V]) | Install scripts apply and verify the baseline | Scan reports |
| **CycloneDX 1.7.2** (specification released 2026-09-17 [V]) and **SPDX 3.0.1** (2024-12-17 [V]); **SLSA** build levels | Software bill of materials and provenance | Each release ships a CycloneDX SBOM and signed provenance (14) | SBOM, attestation |
| **FIPS 140-3** validated cryptography | Some banks and state bodies ask for it [K] | The Rust RustCrypto crates the DBS uses (`aes-gcm`, `ed25519-dalek`, `hmac`, `sha2`) are not FIPS-validated modules; if a bank requires it, use a validated provider (an OpenSSL FIPS provider or AWS-LC FIPS) behind the same crypto trait [K] | Cryptographic module list |

### 3.2 ISO/IEC 27002:2022 controls the product helps a bank meet

Numbers are the 2022 numbering [K]. This is the mapping the product's documentation carries.

| Annex A control | AKR-TMS contribution |
|---|---|
| 5.12, 5.13 Classification, labelling | Data classes labelled to the CBI four levels (11) |
| 5.15 to 5.18 Access control, identity, authentication information, access rights | Cedar roles and attributes, Kratos, access reviews (6, 7) |
| 5.19 to 5.23 Supplier relationships, cloud services | Vendor access model, contract annex (13, 17) |
| 5.24 to 5.28 Incident management | Runbooks, evidence preservation, CBI notification template (16) |
| 5.29, 5.30 Continuity, ICT readiness | RTO and RPO, drills (16) |
| 5.33 Protection of records; 5.34 Privacy and PII | Retention engine, immutable audit, minimisation (10 to 12) |
| 6.1 to 6.6 People controls | Vetting inputs, non-disclosure and acknowledgement of tipping-off rules at first login (8, 9) |
| 8.2, 8.3, 8.5 Privileged access, information access restriction, secure authentication | PAM, field-level permissions, MFA (6, 7, 9) |
| 8.8, 8.9 Vulnerability and configuration management | Patch SLAs, baselines (14) |
| 8.10 Information deletion | Partition drop and crypto-shredding with proof (12) |
| 8.11, 8.12 Data masking, data-leak prevention | Masking, export controls, PAN scanning (11) |
| 8.13, 8.14 Backup, redundancy | pgBackRest, restic, replicas (16) |
| 8.15, 8.16, 8.17 Logging, monitoring, clock synchronisation | Immutable audit, SIEM export, NTP (10) |
| 8.20 to 8.22 Network security, segregation | Zones and policies (4) |
| 8.24 Use of cryptography | Key hierarchy (5) |
| 8.25 to 8.34 Secure development, testing, separation of environments, test information | Section 14 and the synthetic-data rule |

### 3.3 Certification roadmap

Before the first bank: vendor ISO/IEC 27001:2022 certification, an independent penetration test report, the assessor pack (15), a documented SDLC and a vulnerability-disclosure policy. Within the first year: ISO 22301 alignment of the vendor's own support continuity, an ASVS Level 2 verification report, SBOM and provenance on every release. Later and optional: ISO/IEC 27701 (privacy), SOC 2 Type II, and, for a hosted option, the cloud-specific ISO/IEC 27017 and 27018 [K].

### 3.4 AKR-TMS hooks

- **Compliance mapping documents** generated from the control catalogue in the repository (CBI control, ISO control, NIST function, implementing feature, test, evidence location) so an assessor can follow one line from rule to evidence.
- **PAN-free assertion test**: an automated scan of landing data and logs for card-number patterns that fails the build and raises a production incident if one appears.
- **Baseline scanners** (CIS and ASVS) as part of the installer pre-flight (file 32 section 16.7).

---

## 4. Trust zones, network segmentation and the read-only boundary

### 4.1 Zone rules

| Zone | Contains | Allowed inbound | Allowed outbound |
|---|---|---|---|
| Z0 CBS network | CBS database, replica, payment gateway database (bank-owned) | From `akr-ingestd` only, to the listener ports of the bank-owned views | n/a |
| Z1 CBS-facing | `akr-ingestd`, optional Debezium sidecar | From Z2 (job and health calls) | To Z0 listener ports; to Z2 PostgreSQL; nothing else |
| Z2 Application | `akr-worker`, PostgreSQL, object store (MinIO, Silo build), OpenBao, Valkey | From Z1, Z3 (through `akr-api` only), Z4 (assess), Z5 | To the FIU gateway (goAML) via the bank's proxy; allow-listed list providers if approved; SMTP relay; SIEM |
| Z3 Analyst | Edge proxy, `akr-api`, Kratos, Hydra | From the bank LAN on 443 | To Z2 |
| Z4 Gateway | `akr-assess` | From the payment hub or Swift interface by mTLS | To Z2 PostgreSQL and the cache |
| Z5 Management | Bank PAM or jump host, Harbor, CI import, backup target, SIEM collector | From the bank's admin network | To zones for administration only |
| Z6 DR | Standby, offsite backups, second OpenBao instance | Replication and backup flows | None |
| Z7 Vendor | Nothing standing | Nothing | Nothing (sessions are initiated by the bank, section 13) |

Mechanics: default-deny network policies (K3s `NetworkPolicy`, or host firewall rules in the compose profile); TLS 1.2 or later everywhere and mutual TLS between services and from gateways (file 32 section 13.3, internal PKI); no inbound from the Internet; default-deny egress with an explicit allow-list (the DBS decision D6); DNS and NTP only to bank servers; unused ports closed and default passwords changed (CBI network controls 3.1.19 and 3.1.20).

### 4.2 Direct database access

CBI 12.1.3 limits direct database access to DBAs. For AKR-TMS that means: analysts, administrators and support reach the product database only through the application; DBAs reach it through the bank's bastion with their own named accounts, and what they can read of personal data and suspicion information is ciphertext (section 5); service accounts are separate per binary (file 32 section 6.1). For the **CBS** database, the AKR-TMS account is a named application service account, approved through the bank's access process and limited to bank-owned views (file 25 section 10), and the bank's DBA audits it (file 25 section 10.5). A bank's CISO should record this as an approved exception to the "DBAs only" rule in the asset register.

### 4.3 No production data elsewhere

CBI 12.1.5 prohibits moving a critical system's database from production to any other environment, and 14.2.3 to 14.2.9 require masking or deleting protected data in non-production and forbid test data in production. Rules: development, test, demonstration and training environments, at the vendor and at the bank, use **synthetic data only** (file 25 section 13.7); a bank's UAT may use masked extracts if its CISO approves and the masking is verified; support bundles are redacted (section 13); backups restore only into production or into a labelled, access-controlled restore environment that is destroyed after the drill.

### 4.4 Remote administration

Administration only through the bank's PAM or jump host over SSH or VPN with MFA, recorded sessions and named accounts; Telnet and unencrypted management disabled; no vendor VPN into production (CBI 3.1.10, 8.4, 13.2.3).

### 4.5 AKR-TMS hooks

- **Network policy bundle** (Helm or compose) with a test that proves what each pod or container can and cannot reach.
- **Zone diagram and flow table** in the product documentation, kept in step with the manifests by a CI check.
- **Exception register template** for the bank's CISO (CBS service account, bastion access, etc.).

---

## 5. Encryption in transit and at rest; key management

### 5.1 In transit

| Link | Protection |
|---|---|
| Browser to edge proxy | TLS 1.3 preferred, 1.2 minimum, HSTS, certificates from the bank's CA (the DBS edge sets `Strict-Transport-Security`, `X-Content-Type-Options`, `X-Frame-Options: DENY` and a `frame-ancestors` policy, `deploy/prod/Caddyfile` [V]) |
| Edge to `akr-api`, between services, to PostgreSQL, to the object store | Mutual TLS from an internal PKI; PostgreSQL with `verify-full`; no plaintext hop inside the cluster for personal data |
| `akr-ingestd` to the CBS | TLS 1.2 or later validated against the bank's CA bundle, or the engine's native encryption if the bank already uses it (file 25 section 10.3) |
| `akr-assess` and the payment gateway | Mutual TLS with a per-gateway client certificate plus signed, expiring decisions (file 25 section 15.3) |
| OpenBao | TLS on the listener. The DBS `vault.hcl` runs `tls_disable = true` on the compose network and notes "Production terminates TLS here (DR-107)"; AKR-TMS enables TLS in every profile |
| Backups, replication, SIEM export, SMTP | TLS or an encrypted repository; no clear-text syslog |
| Administration | SSH or VPN only (CBI 12.2.3, 3.1.10) |

Protocol policy: TLS 1.0 and 1.1 disabled, AEAD cipher suites, certificate lifetimes of one year or less with automated renewal (cert-manager in K3s; the DBS edge renews automatically [V]); no wildcard certificates for internal services; certificate expiry on the ops cockpit (the DBS cockpit, module M15, specifies it; the first build records a value written by the platform layer rather than probing, `docs/plan/phase-10.md` [V]).

### 5.2 At rest

| Layer | Mechanism | Notes |
|---|---|---|
| Volumes | Full-volume encryption (LUKS or the bank's storage encryption) on database, object-store, OpenBao and backup volumes [K] | CBI 12.3.6 and the top-secret rule "always encrypted when stored electronically" (2.4) |
| PostgreSQL | No transparent data encryption in community PostgreSQL; rely on volume encryption **plus field-level encryption** below | Roles and views limit exposure further (file 32 section 6.1) |
| Fields | Envelope encryption: AES-256-GCM data keys wrapped by an OpenBao transit key, cached in process memory, never in the request path to OpenBao (the DBS pattern, `ARCHITECTURE.md` section 10 [V]) | Applied to suspicion information, notes, STR text, evidence attachments, secrets and, in the strict mode, identifiers |
| Search keys | Keyed HMAC-SHA256 hashes ("blind indexes") for exact-match lookups of national ID, phone and similar | A different key from the encryption keys |
| Object store | Evidence is encrypted before upload: envelope encryption with data keys wrapped by an OpenBao transit key, as for fields, on LUKS disks; no server-side encryption through MinIO KES, which is archived (file 32 section 12.6); plus object lock | |
| Backups | pgBackRest repository with `repo1-cipher-type=aes-256-cbc` and a passphrase held outside the server; restic repositories encrypted; OpenBao raft snapshots sealed by OpenBao's barrier (`deploy/prod/pgbackrest.conf`, `backup-restore.md` [V]) | CBI backup rule: encrypt before moving off site (2.4) |
| Process memory | Disable core dumps and swap for key-holding processes; zeroise data keys on drop [K] | |

**What not to encrypt at field level, and why.** Fuzzy name matching and analyst search need customer names in a searchable form (file 32 section 10). The default is therefore: names, addresses and similar attributes in disk-encrypted columns readable only by the application roles (the same data a DBA can already read in the CBS), and field-level encryption for **suspicion information and anything that is not a copy of CBS data**. Banks that want stronger protection can switch identifiers and names to encrypted columns and load an in-memory index at start, accepting the cost in search features and start-up time. The choice is a documented, per-bank setting.

### 5.3 Key hierarchy, rotation and custody

The hierarchy is the one in file 32 section 13.3: Shamir unseal (3 of 5 key holders, sealed envelopes in separate sites, which matches the CBI rule that administrative passwords be kept "in a sealed envelope"), KEK in transit, data keys per table family, a separate match-key HMAC key, Ed25519 signing keys, internal PKI, backup key held off-site.

| Key | Rotation | Notes |
|---|---|---|
| KEK (transit) | At least yearly; on demand after suspected exposure | Rewrap data keys; old versions kept for decryption |
| Data keys | A new key per **partition-month** for time-series tables, per tier for case data | Allows **crypto-shredding** of an expired partition by destroying its key (section 12) |
| Signing keys | Yearly with overlapping versions; version embedded in each artifact | Never raise `min_decryption_version` past a version that signed evidence (the DBS rule for QR keys) |
| TLS | One year or less, automated | |
| CBS credentials | Per the bank's policy, rotated by the bank's process (file 25 section 10.2) | |
| OpenBao tokens | Short-lived and renewable through Kubernetes or AppRole authentication | The DBS gives the API and worker 10-year tokens that `bootstrap.sh` replaces in their last 400 days (`deploy/prod/README.md` [V]); AKR-TMS should not copy that lifetime |

**Custody rules**: no key material in repositories, images or environment files in production (the DBS refuses to boot on a default secret in production [V]); keys are never shared across banks and the vendor holds no bank key; split knowledge and dual control for the unseal ceremony; a written compromise runbook (revoke tokens, rotate KEK, rewrap, re-sign anchors with a new key version, notify the bank's CISO and, where the incident is a disaster, the CBI as soon as possible per 2.4); optional HSM for the KEK or the unseal shares; destruction logged. The phase-2 service must survive a restart while OpenBao is sealed (file 32 section 13.4).

### 5.4 AKR-TMS hooks

- **Crypto trait** in `akr-common` (encrypt, decrypt, sign, verify, keyed hash) with the OpenBao implementation, an in-memory test implementation and room for a FIPS-validated provider.
- **Key inventory screen** in the cockpit: key, purpose, version, last rotation, next due, signer dependencies.
- **Quarterly sealed-state and key-compromise drills** recorded in the audit trail.

---

## 6. Identity, authentication and sessions

### 6.1 What the DBS already does

Kratos holds credentials and the API is the only client of its browser flows; registration is disabled and every identity is provisioned by an administrator; recovery is **admin-issued**, "no password-reset email means no email-interception path into a banking account"; the Have-I-Been-Pwned check is off because it is an outbound call; `required_aal` is `highest_available`, so a session that could reach AAL2 is not accepted at AAL1; passwords are hashed with argon2id (128 MB, 3 iterations, 4 lanes); cookie sessions are `httpOnly`, `SameSite=Strict` with an HMAC-bound CSRF token; signing-grade actions need a **single-use step-up assertion** consumed inside the action's transaction; a failure counter with exponential back-off protects login, in a store whose failure mode is documented; WebAuthn is a decided requirement that the DBS records as "implemented, not deleted" (DR-113; the status of the browser ceremony was not verified) (`deploy/compose/kratos/kratos.yml`, `.claude/rules/auth.md`, `domain/src/identity/mod.rs`, `docs/plan/research.md` [V]).

### 6.2 What the CBI adds

| CBI rule (2.4) | AKR-TMS setting |
|---|---|
| MFA for web applications used by staff or customers reachable from the Internet (14.1.4); MFA for third parties on critical systems (13.2.3.4) | MFA is **mandatory for every user**, internal or not; WebAuthn preferred, TOTP baseline |
| Password at least 8 characters, three of four classes, at most 90 days, lock after three consecutive failures, history of three (8.2) | Configurable policy whose defaults meet the CBI minima (the default minimum length is raised to 12); forced change flow (the DBS "confined session that owes a credential change"); lock after a configurable number of failures, three by default, with administrator unlock or a timed unlock the bank chooses (the DBS function takes `max_failures` as a parameter) |
| Session time-out per policy (8.3.1, 14.2.1) | Idle time-out 15 minutes and absolute lifetime 8 hours by default (the DBS dev configuration uses 12 hours); both bank-configurable; re-authentication for sensitive views |
| Unique user ids and logs (8.1.3, 8.1.7) | One identity per person; no shared accounts, no shared privileged accounts (8.4) |
| Revoke access on leaving (8.2.9, 13.2.3.5) | One-click offboarding that disables the identity, revokes sessions and tokens and releases or reassigns tasks (DBS 9.4 [V]) |
| Administrative passwords in a sealed envelope (8.2.8) | Break-glass credentials in sealed envelopes, alarm on use, rotation after use |

Where the bank federates login (file 32 section 14.3), these rules still apply to AKR-TMS-side sessions and step-up.

### 6.3 Assurance level per action

| Action | Required |
|---|---|
| Sign in; view the work queue and alert lists | Password plus second factor (AAL2) |
| Open a customer or an alert | AAL2 session; branch scope; purpose is logged |
| Open a restricted-tier customer or any suspicion-tier case | **Fresh step-up within 10 minutes**, a purpose code, relationship-conflict check, internal network origin (7, 8) |
| File an STR, close a case without an STR, clear a hold, approve a change request, export, change roles, run a backfill | **Single-use step-up assertion** consumed in the action's transaction |
| Break-glass | Two named approvers, each with step-up (7.6) |

### 6.4 Machine identities

Mutual TLS client certificates for gateways, one per gateway; scoped personal access tokens for scripted clients (the DBS `mpat_` tokens, always AAL1 and subject to the scope gate); service-to-service identities from the internal PKI; no shared service accounts; every token has an owner, an expiry and a last-used time shown in the admin screen.

### 6.5 Leave, rotation and dormancy

The CBI's know-your-employee rules require staff leave of two weeks and rotation (file 01 section 2d; file 27). The DBS already has a time-and-leave ledger (`docs/plan/time-leave-ledger.md`); AKR-TMS can **block sign-in for a person on recorded leave** and require a handover of open cases, which also removes the staff member's ability to tamper while away. Accounts unused for 60 days are disabled and reviewed.

### 6.6 AKR-TMS hooks

- Identity policy as admin-managed settings with the CBI minima as the floor (an administrator cannot weaken them below it without a recorded exception).
- **Sign-in anomaly rules** to the SIEM: impossible travel is not meaningful inside a bank, so use unusual hour, unusual workstation, repeated step-up failure and access from a network other than the analyst zone.
- Break-glass credential procedure and alarm.

---

## 7. Authorisation: RBAC and ABAC, segregation of duties, maker-checker, branch scoping, break-glass

### 7.1 Principles

Deny by default; roles bundle capabilities; attributes narrow them; **no inherent access by rank or position** (CBI 8, intro) so an administrator, a director or the MLRO's superior sees case data only through the same rules and the same logging as an analyst; platform administration is separate from business data; every decision, allowed or denied, is audited (the DBS Cedar practice [V]).

### 7.2 Roles

| Role | Typical holder | Can | Cannot |
|---|---|---|---|
| Analyst level 1 | AML analyst | Work assigned alerts in their branch or pool, request more information, escalate | See restricted-tier customers, suspicion-tier cases, thresholds, any export |
| Analyst level 2 | Senior analyst | Investigate, link cases, propose STR, propose alert closure with reason | File an STR, approve their own closure, change scenarios |
| MLRO and deputy | Money laundering reporting officer | Decide and file STRs, approve closures, view all suspicion-tier cases, review liaison reports | Change scenarios alone; approve a change they proposed |
| Branch liaison officer | Designated per branch (CBI ch. 11) | Branch window: see branch alerts in limited form, submit confidential reports to the MLRO | See other branches, see STR decisions, or let the branch manager see their reports |
| Compliance configuration manager (maker) | Compliance analyst | Draft scenarios, thresholds, mappings, whitelists, list settings | Approve their own change |
| Compliance approver (checker) | MLRO, deputy or compliance head | Approve or reject change requests | Author the change |
| Model-risk officer | Second-line model validation | Read models, validation reports and drift | Change production models |
| Internal auditor | Third line | Read-only, including audit logs and access history; time-boxed | Change anything; see case content unless the audit scope grants it |
| CBI or external inspector | Examiner | Read-only subset, time-boxed, named account, every action logged | Anything outside the grant |
| Platform administrator | Bank IT | Users, roles, settings, health | See business data or suspicion information |
| Infrastructure operator and DBA | Bank IT | Deploy, back up, restore, tune | Read personal data or suspicion information in clear (keys not accessible) |
| Security officer (CISO) | Bank security | Access review, security audit, approve break-glass | Open cases |
| Vendor support | AKR-TMS | Nothing standing; the `support-readonly` role (health, versions, redacted logs) inside a bank-opened, time-boxed session (13.2) | Read any customer, case, STR or export; there is no vendor break-glass |

### 7.3 Attributes

Branch scope; customer sensitivity tier (**normal**, **restricted** for staff, relatives of staff, board members, PEPs and VIPs; **suspicion** for any customer with a case at the STR-candidate stage or later, a filed STR, or an FIU, CBI or court request, 8.3); case access-control list; clearance; time window; purpose-of-access code; network origin; step-up freshness; relationship-conflict result; managed-device flag. A sketch in the DBS Cedar style (a design illustration, not tested code):

```cedar
// An analyst may open a customer only within their branch scope, and never a restricted or suspicion-tier one.
permit (principal, action == Action::"customer.read", resource)
when { principal.status == "active" && principal.roles.contains("analyst")
       && resource.branch in principal.branches
       && resource.tier == "normal" };

// Suspicion-tier case: ACL member, fresh second factor, internal network, no conflict, purpose given.
permit (principal, action == Action::"case.read_suspicion", resource)
when { principal.status == "active" && resource.acl.contains(principal)
       && context.stepup_age_seconds <= 600 && context.network == "analyst_zone"
       && context.conflict == false && context.purpose != "" };

// Nobody approves a change request they authored.
forbid (principal, action == Action::"change.approve", resource)
when { resource.maker == principal };
```

### 7.4 Segregation of duties

| One person must not hold both | Why | Enforced by |
|---|---|---|
| Maker and checker of a change request or decision | CBI 8.3.4 and 3.3.2 to 3.3.3 | Cedar `forbid` plus a database constraint |
| Author and approver of a scenario, threshold, whitelist, mapping or list override | Enforcement cases: thresholds and whitelists altered to cut volume (file 20) | Change-request workflow |
| Investigator of a case and approver of its closure | Independent review | Workflow rule |
| Staff member and investigator of their own or a relative's account | Conflict of interest | Relationship-conflict check at access time (8) |
| Platform administrator and holder of business data access | Administrators must not read data | Role design |
| DBA or operator and holder of key access | DBAs must not decrypt | OpenBao policies separate from OS and database roles |
| Developer and production operator | CBI 14.2.6: developers do not change production code or data | Deployment pipeline with a separate approver |
| Auditor and any operational role | Independence | Role exclusivity |

### 7.5 Maker-checker

One mechanism, used for every admin-managed object that governs detection or reporting (file 32 section 11.4): maker cannot approve; approval needs a fresh second factor; activation writes the new version and the audit entry in one transaction; emergency changes are allowed for a named role only, take effect at once, and require approval within 24 hours or revert automatically; a change whose main effect is to reduce alert volume needs a second approver in the compliance function and an attached backtest (file 05 section 1f). Whitelists and customer exclusions **expire** by default and need re-approval, because an open-ended list is how monitoring gets switched off (the "Verified List" case, file 20).

### 7.6 Branch scoping, the liaison window and break-glass

- **Branch scope** is an attribute on every alert and case; branch staff see only their branch and only the fields their role needs.
- **Liaison window** (CBI AML controls 2025, chapter 11; file 01 section 2d): a branch-scoped view, a confidential report form straight to the MLRO that branch management cannot see, and the monthly report by the 10th.
- **Break-glass** (modelled on the DBS break-glass screens and cockpit log): only for emergencies such as a case that nobody authorised can be opened because the MLRO and deputy are unavailable, or a stuck job. Requires a reason, two named approvers (one from compliance, one from security), is time-boxed (default four hours), notifies the CISO and the MLRO, is recorded in a dedicated log, and is reviewed within 24 hours. Break-glass never grants export.

### 7.7 Policy governance

Cedar bundles live in the database and are activated by change request; the simulator shows who can do what with a full attribute trace and is itself audited (it "reveals authorization topology", DBS M3 [V]); dry-run compares a new bundle with live traffic before enforcement; a coverage test walks the served OpenAPI document against the route table in both directions so no route answers without a decision (the DBS `authz_coverage.rs` practice [V]); quarterly access review; policy review at least yearly (CBI 8.1.8, 2.4).

### 7.8 AKR-TMS hooks

- The permission matrix as seed data and a generated access-matrix report for assessors.
- Conflict-check service and SoD constraints as tests (a build fails if a role gains both sides of a pair).
- Break-glass module and log viewer reusing the DBS pattern.

---

## 8. Tipping-off controls: who may see suspicion information

### 8.1 The rule and the stakes

Law 39 of 2015 Article 12 (Fourth) forbids disclosing to "the customer, beneficiary or any other person except competent authorities" any legal procedure taken on suspected laundering or terrorism financing; Article 39 (Second) makes it an offence for the individual, with up to three years' imprisonment (section 2.1). The CBI's 2023 sanctions guidelines extend the duty to freezes (2.3), the AML controls require an internal report to be acknowledged in writing with a reminder of the tipping-off rule (file 01 section 2d), and the CBI's classification makes suspicion information *confidential* at least (2.4). The control set below is therefore not a nicety. It is what lets a bank show that it made leaking hard, and it protects the staff.

### 8.2 Leak paths and the control for each

| # | Leak path | Control |
|---|---|---|
| 1 | A staff member looks up whether a customer, relative or friend is under investigation | **Suspicion tier** with a per-case access list; fresh step-up and purpose code; relationship-conflict check; view logging; existence masking (a refused open looks like "not found": the DBS applies the clearance ceiling inside the SQL query, so a record above clearance is never selected and cannot leak through a count or cursor, and the handler answers NotFound rather than Forbidden, `.claude/rules/auth.md` and `rust-backend.md` [V]) |
| 2 | Search, type-ahead and global lookups show a flag | Suspicion-tier cases and flags are **not indexed** in global search; they appear only to list members; counts for other roles exclude them |
| 3 | Dashboards and reports reveal who is under review | Aggregates only, with small-number suppression for non-compliance roles; branch staff see their own branch window only |
| 4 | Notifications reveal subject or type | In-app and the bank's SMTP only; text is generic ("you have a task") with no name, amount or case type; the e-mail subject never reveals content or classification (CBI confidential e-mail rule); **no WhatsApp or external push** (file 32 section 11.5) |
| 5 | A hold or block becomes visible in the CBS or at the gateway with an AML reason | AKR-TMS writes nothing to the CBS; tasks to bank operations carry a **neutral instruction and reference** ("apply compliance hold, order ref"), never the reason or the word STR; the customer-facing message is generic (file 25 section 15.7) |
| 6 | An exported report, printout or screenshot circulates | Export of suspicion-tier data needs step-up, a purpose and an approval; every export is logged; visible per-user watermark on screens and exports; printing is logged; the CBI rule "no printing or copying of confidential information without the required approvals" applies (2.4) |
| 7 | Attachments and evidence are fetched by someone outside the case | Evidence inherits the case access list; presigned URLs are short-lived and bound to the user; every fetch is logged |
| 8 | Discussion by e-mail, chat or phone | Provide in-case comments with the same access list so people do not use other channels; policy and training; confidential discussions "in a secure meeting room" (CBI) |
| 9 | DBAs, sysadmins or auditors read the content | Field-level encryption with keys they cannot reach; log redaction; the audit viewer shows metadata (who, when, what) for suspicion-tier records, content only for auditors whose scope grants it (section 10) |
| 10 | The identity of a staff reporter is exposed | Internal reports go to the MLRO through a confidential channel that branch management cannot see (CBI AML controls 2025, chapter 11; file 01 section 2d); the CBI fraud and corruption guide (2024, section 12) likewise has staff report through approved channels to an independent body and keeps the reporter's identity confidential (paraphrase of the scanned text) [V] |
| 11 | A restricted customer (staff, relatives, board, PEP, VIP) is reviewed by colleagues | **Restricted tier**: alerts route straight to a compliance-only queue; level-1 analysts never see them |
| 12 | Information received from the FIU, the CBI or a court | Registered in a restricted request register with a case-level access list (file 04 E-01 and E-07) |
| 13 | Test, demo or support copies | Synthetic data only; support bundles redacted (4.3, 13) |
| 14 | Stale access after a role change | Quarterly review plus automatic removal from case lists when a person changes role or leaves |

### 8.3 Tiers and handling

| Tier | Who it covers | CBI level (bank may raise) | Handling |
|---|---|---|---|
| **Normal** | Ordinary customers, transactions, alerts | Restricted (personal data) | Branch scope; AAL2 session; logged access |
| **Restricted customer** | Staff, relatives of staff, board members and senior managers, PEPs, VIP accounts, accounts managed by a bank officer (the typologies of the OFAC insider case) | Restricted, handled as confidential | Compliance-only queue; relationship-conflict check on every open; step-up and purpose; no branch visibility |
| **Suspicion** | Any customer with a case at "STR candidate" or later, any filed STR, any FIU, CBI or court request | **Confidential** (سري), or top secret (سري للغاية) if the bank so classifies | Per-case access list approved in writing; time-boxed grants; no remote access (internal network only); step-up each session; export by approval; no electronic transfer of content; register of every person who accessed |

The tier is an attribute set by the system when a case reaches the trigger state and by the MLRO manually; it takes effect immediately and also applies to the customer's other alerts. If the bank classifies the tier as top secret, the non-Iraqi rule applies to access grants (2.4), which also governs vendor staff (section 13).

### 8.4 Access requests: "prior written approval"

For the suspicion tier, access is **requested** (reason, case or customer, duration) and **approved** by the MLRO or deputy with a fresh second factor; the grant expires automatically (default 8 hours, extendable); the request, approval and use are kept in a register that the CISO and internal audit can read. This matches the CBI's "prior written approval" and "specific time" rules (2.4) and its register of everyone who accessed top-secret information.

### 8.5 Relationship-conflict check at access time

Before showing a customer, case or alert in a restricted or suspicion tier, the system compares the user's staff record with the customer: same family number or identifier, shared phone, e-mail, address or device, listed as a related party in the CBS, the user's own or a relative's account, same employer, or the user manages the account. A conflict **denies access** (the user sees "not found"), writes an alert to the MLRO and the CISO, and is never explained to the user. The inputs come from the canonical model (file 25 section 13: `party_relationship`, `staff`, shared contacts) and from the CBI's relative degrees (two and four degrees of kinship, file 01 section 2e).

### 8.6 Canary records

The MLRO, through maker-checker, creates a small number of **fictitious suspicion-tier customers** that exist only in AKR-TMS and are excluded from every statistic and report. Any open, search hit or export of one is a P1 alert to the CISO and the MLRO, and the actor is not told. Canaries detect snooping that no rule can, at almost no cost. They are never written to the CBS or to a regulator report.

### 8.7 What AKR-TMS cannot control

CBS flags set by the bank's own operations, what a teller says to a customer whose account is on hold, conversations outside the system, and what a person remembers. The product's contribution is neutral task wording, no AML reason in anything that reaches the CBS or the gateway, training content for the "what to say" script, and detection of the visible attempts.

### 8.8 Monthly tipping-off control report

To the MLRO and the CISO: attempts to open suspicion-tier records by people outside the list, denied conflicts, exports, break-glass uses, canary triggers, access grants and expiries, and records whose tier changed. Feeds the board's report and the CBI-approved assessor.

### 8.9 AKR-TMS hooks

- **Tier attribute and ACL model** in the `case` and `kyc` contexts; Cedar policies of section 7.3; existence-masking responses that are byte-identical to a "not found".
- **Access-request workflow** and register.
- **Conflict-check service** with an explainable result stored for the MLRO only.
- **Canary service** and its report exclusions.
- **Neutral-wording library** for tasks, notifications and holds, tested for forbidden words.
- **Test suite** that tries to infer the existence of a suspicion-tier case through search, counts, timing differences, error messages, exports, notifications and the audit viewer, and fails the build if it can.

---

## 9. Insider threat, privileged access and "who viewed which customer"

### 9.1 Why the system itself must watch its users

The evidence is Iraqi and global. OFAC's October 2025 designation of three Iraqi bankers (file 05 section 1e) shows bankers using their positions; the enforcement record shows **compliance staff** under pressure: thresholds "set based on a desire not to generate 'too many alerts'", a "Verified List" that switched monitoring off for listed customers (Rabobank N.A.), alert caps, and, in the 2026 FinCEN consent order against Canaccord Genuity, compliance employees who, when asked for evidence of review, "falsified nearly 400 documents" or "backdated versions of certain Firm policies and procedures" (file 20 sections 2.3, 2.7 and the failure-mode table). An AML platform concentrates exactly the powers such people would abuse: seeing who is investigated, changing what is detected, and recording what was reviewed. The CBI asks for the same discipline: strict selection and vetting for critical functions, strong authentication, a limited number of privileged users, timely review of their activity, no shared privileged accounts (8.4), and monitoring of database administrators' behaviour (12.4).

### 9.2 Access history: "who viewed which customer"

Every read of a customer, alert, case, transaction detail, evidence item or STR, every search that returns a restricted or suspicion-tier record, every export and every print writes an **access event**. Sketch:

```sql
CREATE TABLE IF NOT EXISTS audit.access_event (
  id            UUID NOT NULL DEFAULT gen_random_uuid(),
  occurred_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  actor_id      UUID NOT NULL,              -- the person, never a shared account
  actor_role    TEXT NOT NULL,
  on_behalf_of  UUID,                       -- delegation
  resource_type TEXT NOT NULL,              -- customer, account, alert, case, evidence, report ...
  resource_id   UUID NOT NULL,              -- pseudonymous id, never a name
  subject_party UUID,                       -- the customer the resource is about
  action        TEXT NOT NULL,              -- view, search_hit, export, print, download, unmask
  tier          TEXT NOT NULL,              -- normal, restricted, suspicion
  purpose_code  TEXT,                       -- mandatory for restricted and suspicion tiers
  reason        TEXT,                       -- free text, encrypted
  decision      TEXT NOT NULL,              -- allow, deny, conflict
  session_id    UUID NOT NULL, client_ip INET, device_id TEXT,
  correlation_id UUID NOT NULL,
  prev_hash BYTEA, entry_hash BYTEA NOT NULL,
  PRIMARY KEY (id, occurred_at)
) PARTITION BY RANGE (occurred_at);
```

It is partitioned monthly, hash-chained and anchored like the rest of the audit trail (section 10), and kept under the audit retention (unlimited in the DBS, decision D5). **Reading the access log is itself logged.** Volume is modest: two hundred analysts at five hundred views a day is about 100,000 rows a day, inside the DBS's own worst-case planning figure of about 250,000 audit rows a day (5,000 users at 50 audited reads) and its note that monthly partitions of about 7.5 million rows "are unremarkable on NVMe" [V, `docs/ARCHITECTURE.md` section 8.4]. List and search screens log one `search_hit` per restricted or suspicion-tier record shown and nothing for normal-tier rows, so the figure stays near the number of deliberate opens. The record's **access-history tab** is visible to the MLRO, the CISO and internal audit, never to analysts. A customer who asks "who has looked at my data" is answered by the bank's own process, which gives categories of staff and purposes and never names individuals or reveals any suspicion-tier access (section 19).

### 9.3 Detection rules on the platform itself

| Rule | Signal | Action |
|---|---|---|
| Mass lookup | A user opens many distinct customers outside their assigned queue in a period | Alert to the MLRO and CISO; temporary step-up on every open |
| Off-hours or off-network access to restricted tiers | Time and network origin | Deny for the suspicion tier; alert for restricted |
| Access without a work reason | Open of a customer with no assigned alert or case | Purpose code required; sampled review |
| Snooping on staff and relatives | Access to staff, relative or VIP accounts | Always alert; conflict check |
| Repeated denied or conflicted attempts | Counts per user | Alert; manager and CISO review |
| Export spikes | Number and size of exports | Block above threshold pending approval |
| Privileged action bursts | Role changes, whitelist or threshold edits, key operations | Alert; compare with approved change requests |
| Configuration edits that cut volume | Threshold raised, whitelist added, scenario retired | Volume-reduction flag, second approver, backtest (7.5) |
| Alert closure patterns | One analyst closes unusually fast or at unusual hours; closure without evidence | Second-line sampling; QA queue |
| Evidence edits | Any edit of a closed case or an attachment | Not possible: closed cases are append-only; corrections are new entries |
| DBA or operator anomalies | Direct database sessions, backup downloads, restore actions | Forwarded from pgAudit and the bastion to the SIEM (CBI 12.4) |
| Canary access | Any touch of a canary record | P1 alert (8.6) |

The recipient of these alerts is the **second line and security**, not the person's own manager, and the actor is never notified.

### 9.4 Privileged access

- **No standing administrator access to business data**: platform administrators manage users and settings; operators manage the infrastructure; neither reads personal data or suspicion information.
- **Just-in-time elevation** through the bank's PAM for infrastructure tasks, with approval, a time limit, a recorded session and automatic revocation; no shared privileged accounts (CBI 8.4).
- **DBAs** reach the database through the bastion with named accounts; keys are in OpenBao under separate policies that the DBA role does not hold; backup and restore actions are logged and need a second person.
- **OpenBao operators** hold unseal shares but no application tokens; the root token is generated only for ceremonies and revoked.
- **Developers** have no access to production code paths or data (CBI 14.2.6); releases are deployed through the signed pipeline by operators (file 32 section 16).

### 9.5 Other insider controls

- **Staff accounts are monitored through a separate path** that the staff member's manager cannot see; the CBI requires monthly movement reports on employee accounts (file 01 section 2d) and know-your-employee indicators (file 27).
- **Mandatory leave and rotation** enforced by the leave ledger (6.5).
- **Whitelist and exclusion governance**: expiry, owner, reason, approval, periodic recertification, and a report of how many customers are excluded from which scenarios.
- **Closed-case integrity**: cases are append-only after closure; any reopening is an event; hashes of evidence are recorded at closure.
- **Quality assurance**: second-line sampling of alert closures (a QA queue fed by the sampling rules), with results visible to the board report.
- **Vetting**: CBI 8.4.1 and 13.2.2.1 require strict selection and screening for staff on critical systems; the product records the vetting status as an attribute of the role grant and refuses grants without it for the suspicion tier.

### 9.6 AKR-TMS hooks

- `audit.access_event` with the logging middleware on every route that returns restricted data; an automated test fails a build if such a route lacks it (the DBS coverage-test pattern).
- **Platform UBA rules** shipped as a scenario pack running on AKR-TMS's own audit data, using the same scenario engine (file 32 section 7).
- **Access-history tab** and a monthly **privileged-activity report** for the CISO and internal audit.
- **QA sampling** configuration and report.

---

## 10. Immutable audit logs and the bank's SIEM

### 10.1 The DBS design AKR-TMS inherits

`audit_log` is append-only (UPDATE and DELETE revoked from the application role), partitioned monthly, with a per-partition hash chain (`entry_hash = H(prev_hash || canonical(entry))`); every five minutes the worker computes a Merkle root over new entries, signs it with an OpenBao transit key, stores it in `audit_anchors` and appends the anchor to an object-locked bucket with a default COMPLIANCE retention of 3650 days, so "altering or deleting history requires forging a signature" and the storage layer itself protects the anchors ("`audit/src/anchor.rs`, `common/src/objectstore.rs`" [V]). Reads of sensitive resources are always logged, and the viewer connects through a role that holds `SELECT` and nothing else (`ARCHITECTURE.md` section 8, migration 0019 [V]).

**A caution from the DBS's own audit.** `docs/plan/completion.md` records that, at the time of the audit, "`verify_chain` never checks anchor signatures or the WORM copy" [V]. AKR-TMS must treat chain verification as part of the feature: the verifier checks the chain, the Merkle roots, the signatures against every key version and the existence and retention of the locked copy, and a failure is a P1 incident with a screen that shows exactly where the chain breaks.

### 10.2 What is logged

| Class | Examples | Notes |
|---|---|---|
| Authentication | Sign-in, failures, step-up, lockout, session end, break-glass | |
| Authorisation | Every Cedar decision, allow and deny, with policy ids and an attributes hash | DBS practice |
| Data access | `audit.access_event` (9.2) | Content never stored in the log |
| Changes | Every admin-managed object: before and after, actor, reason, approver, change request id | Diffs for scenarios, thresholds, mappings, lists, roles, policies |
| Decisions | Alert closure, case decisions, STR filing, hold and release, freeze tasks, phase-2 decisions | Signed for phase 2 |
| Pipeline | Batches, reconciliation results, coverage certificates, blind periods, paused-by-bank events | File 25 |
| Security | Key operations, OpenBao audit device, token issue, certificate events | |
| Infrastructure | pgAudit on the product database, host `auditd`, firewall, bastion sessions, container and K3s events | Forwarded to the SIEM |
| CBS-side | The bank's native audit of the AKR-TMS account | The bank's own record of what was read (file 25 section 10.5) |

### 10.3 Properties

UTC timestamps from NTP-synchronised hosts (ISO 27002 8.17 [K]); correlation id on every event so an analyst action joins to its authorisation decision and its database change; **no personal data in logs** beyond pseudonymous ids (names stay out of access events and application logs); the audit trail is separate from operational logs, which are short-lived (the DBS separates `audit_log` in PostgreSQL from Loki [V]); the audit viewer is read-only by construction and **audit-the-auditor** applies: every viewer search or timeline emits an `audit.searched` event with its filters and result count, and an export carries the requesting auditor, timestamp and correlation id as a watermark, with a configured row cap [V, DBS `docs/plan/phase-16.md` section 3; module M13 Audit and Compliance in `docs/MODULES.md`].

### 10.4 Integration with the bank's SIEM and SOC

The CBI's Detect chapter (items 1.4 and 1.5) expects security logs of systems and applications to be reviewed regularly, protected and retained, and "real-time or near-real-time" monitoring, with a security operations centre as the usual means (2.4). AKR-TMS exports security events as CEF or JSON over TLS syslog and exposes the same events through a signed pull API; a use-case catalogue ships with it (9.3, failed step-ups, break-glass, policy activation, key operations, chain-verification failure, anchor lag, reconciliation failures, paused-by-bank events). The exports carry no content from the suspicion tier.

### 10.5 Retention, access by regulators and independence

Audit entries are kept for the life of the system by default (DBS decision D5; compliance's written statement was still open in the DBS), with cold partitions moved to cheaper storage and never deleted without an approved change. Inspectors and internal audit receive time-boxed read-only accounts; an export carries the requester's watermark and its own audit entry. Independent verification: internal audit can run the verifier on its own schedule and compare the anchors with the WORM bucket without the application's help. An external timestamp (an RFC 3161 authority) on the anchors is an option for banks that want a third-party time proof [K].

### 10.6 AKR-TMS hooks

- **Verifier service** that checks chains, Merkle roots, signatures per key version and the locked copy, with a status tile and a one-click run (the DBS `chain_status` and `verify_chain` services, `docs/plan/phase-16.md` section 3 [V]).
- **Event catalogue** as code (type, fields, redaction rules, severity) and an automated test that every mutating route emits its audit event.
- **SIEM export adapters** (CEF, JSON syslog) and the use-case pack.

---

## 11. Data minimisation, purpose limitation, classification, masking and pseudonymisation (PCI scope)

### 11.1 Baseline

Iraq has no comprehensive personal-data law in force (section 2.8), so the baseline is the CBI's classification and secrecy rules plus the principles that every correspondent and rating agency expects. The EU General Data Protection Regulation is used here only as a design benchmark, not as a claim of applicability: Article 5(1) lists purpose limitation (b), data minimisation ("adequate, relevant and limited to what is necessary in relation to the purposes", c), storage limitation (e) and integrity and confidentiality (f), and Article 4(5) with Recital 26 states that **pseudonymised data is still personal data** [V, EUR-Lex 32016R0679]. Recital 78 encourages producers of products that process personal data to build in data protection by design [V]. ISO/IEC 27002:2022 controls 5.34, 8.10, 8.11 and 8.12 cover the same ground for a bank's ISMS (3.2). The product therefore treats minimisation as a feature that can be tested, not as a policy paragraph.

### 11.2 Data classes (the tag set)

File 25 section 13.1 requires a classification tag on every column. The DBS already has one attribute with four levels (public, internal, confidential, secret) that drives read-audit verbosity, encryption, notification content and search visibility (`ARCHITECTURE.md` section 7 [V]); AKR-TMS keeps that mechanism and maps it to the CBI levels:

| AKR-TMS tag | Contents | Minimum CBI level | DBS level | Storage | Default display |
|---|---|---|---|---|---|
| `public` | Reference data (countries, currencies, MCC, holidays), published sanctions lists | Public | public | Plain | Shown |
| `internal` | Configuration, mapping documents, health metrics, job logs, user directory | Restricted | internal | Disk-encrypted | By role |
| `personal` | Parties, names, identifiers, contacts, relationships, staff and PEP flags, KYC profile, ratings, alerts before escalation | Restricted ("personal information of persons protected under any law", 2.4) | internal | Disk-encrypted; identifiers as keyed hash plus encrypted value | Names shown; identifiers masked outside a profile or case |
| `financial` | Accounts, balances, transactions, payment messages, counterparties, facilities, trade | Restricted (a bank may classify it confidential) | internal | Disk-encrypted | Amounts shown; account numbers masked outside a profile or case |
| `card` | Masked PAN, keyed hash or processor token, scheme, MCC, terminal and merchant ids | Restricted | internal | Keyed hash with a card-specific key | Masked PAN only |
| `detection-logic` | Scenario parameters, thresholds, whitelists, model features and weights, matching settings, tuning results | **Confidential** | confidential | Field-level encrypted for whitelist reasons and tuning notes | Role `rule-admin` only; never in support bundles |
| `secret-STR` | Escalated alerts and cases, STR drafts and filed STRs, MLRO decisions, FIU, CBI and court correspondence, freeze orders, notes, evidence, legal holds | **Confidential**; top secret where the bank so decides | confidential or secret | Field-level encrypted; per-case access list | Never masked; access controlled (section 8) |
| (not a data tag) | Keys, tokens, passwords | Top secret in practice | secret | OpenBao and Kratos only, never in product tables | Never displayed |

Rules: the tag-to-level mapping is bank-configurable but a level can only be **raised**, never lowered below the minimum above; a new column without a tag fails the migration check (11.11); derived data inherits the highest tag of its inputs (a feature computed from `secret-STR` data is `secret-STR`); and the file 25 tag list gains `detection-logic`, because the enforcement record shows these are the most tampered and the most useful to an adversary (section 1.1).

### 11.3 Minimisation, stage by stage

| Stage | Control | Specified in |
|---|---|---|
| At the source | The bank's DBA exposes only mapped columns; no full card numbers, PIN-related fields, authentication secrets, free-text customer notes unless mapped, or document images | File 25 section 10.7 |
| Mapping | The mapping document is an **allow-list**: a source column that is not named is never selected; `SELECT *` is refused; a lint inspects the generated SQL | File 25 section 14 |
| Landing | Card-number guard on free text (11.5); biometric, health and sensitive-category columns refused (11.9) | This file |
| Canonical model | Only entities and attributes in the catalogue; every attribute in the registry has an owner, a purpose and a tag; the **scenario-to-attribute dependency table** is read backwards: an attribute that no scenario, screening rule or rating factor uses is reported as "mapped but unused" and proposed for removal | File 25 sections 13.1, 13.6 |
| Features | Customer-month aggregates replace raw rows where the method allows | File 32 sections 6 and 8 |
| Display | Masks and reveal-with-purpose (11.6) | This file |
| Output | An STR carries only the fields of the Office's form; a response to the CBI or a court carries only what the request names, and is entered in the disclosure register (11.10) | File 23 |
| Retention | Clocks per record class (section 12) | This file |
| Support and vendors | Redacted, pseudonymised bundles only (section 13) | This file |
| Non-production | Synthetic data only (4.3) | This file |

### 11.4 Purpose limitation

| Purpose | Data | Used by |
|---|---|---|
| P1 Transaction monitoring and alert investigation | `personal`, `financial`, `card` | Analysts, MLRO |
| P2 KYC risk rating and periodic review | `personal`, `financial` | KYC officers, MLRO |
| P3 Sanctions and list screening | `personal`, list data | Screening analysts, phase-2 service |
| P4 Regulatory and supervisory reporting | `secret-STR`, aggregates | MLRO, compliance |
| P5 Audit and assurance | Audit trail, access events, configuration history | Internal audit, assessors, CISO |
| P6 Product operation and security | `internal`, redacted logs | Operators, the CISO |

Hard rules. (1) **No secondary use**: no marketing, sales, credit-decision or HR use of AKR-TMS data; the product has no connector, report or API for those uses, and a purpose code is required on every restricted or suspicion-tier access (9.2) and checked against the role. (2) **Information received from the Office or from abroad is purpose-locked** to AML/CFT and predicate offences (Law 39, Articles 29 and 53, 2.1) and tagged so it cannot be placed in a general report. (3) **No cross-bank use**: the deployment is single-tenant, the vendor trains no model on a bank's data and contracts say that models trained on a bank's data belong to the bank (file 32 section 19; file 29); shared learning is limited to typology content the vendor writes from public sources. (4) **Tuning and back-testing** run inside the production zone on pseudonymised working sets, and nothing is copied out. (5) **Staff-account monitoring** is a legitimate AML and insider-risk purpose that the CBI itself requires (monthly reports on employee accounts, file 01 section 2d); it must be written in the bank's staff policy and acknowledged at first login (6.5), and its results are visible only to the MLRO and the CISO.

### 11.5 Free text and card data: the guard

Narratives, remarks, payment-message fields (fields 70 and 72 of an MT 103, ISO 20022 remittance information) and any mapped note field can hold card numbers typed by a customer or a teller. The guard runs **before landing**, on every free-text column and every parsed message field, and again on logs and fixtures in CI:

1. Find digit runs of 13 to 19 digits, allowing spaces and hyphens.
2. Keep a run only if it passes the **Luhn check** and starts with an issuer prefix in the configured range table. About one random 16-digit number in ten passes the Luhn check, and Iraqi account and reference numbers are long digit strings, so the bank's own account-number and reference patterns (from its mapping) are used to **exclude** known non-card formats.
3. Replace a confirmed run with first six and last four digits and asterisks (or with a keyed token), count the event per source column, and raise a data-quality issue so the bank can fix the view.
4. Drop anything shaped like track data or a security-code field labelled as such.

This is the single exception to "landing keeps rows as read" (file 25 section 13.1). A nightly job samples text columns in the product database for card-like runs and reports to the CISO; a **PAN-free assertion test** fails the build if a card-like number appears in fixtures, logs or test output (3.4).

### 11.6 Masking by surface

| Surface | Rule |
|---|---|
| Queues, lists, dashboards, charts | Names shown; national ID, passport, account and phone masked to the last four characters; no card number beyond the masked PAN; amounts shown |
| Profile and case workspace | Full values for users on the case or with the profile permission; **reveal** of a masked identifier writes an `unmask` access event, with a purpose code for restricted and suspicion tiers |
| Reports and exports | Masked by default; an unmasked field needs an approved export template; every export is watermarked and logged; row and rate limits per role (the DBS uses a configured export cap, `audit.export_max_rows`, for the audit viewer [V, `phase-16.md`]) |
| Notifications and e-mail | No names, amounts or case types (8.2) |
| Logs, metrics, traces | Pseudonymous ids only; an allow-list of attributes; a redaction layer; a test that fails when a name or identifier shape appears |
| Error responses | RFC 9457 problem details with generic text; the value that failed validation is never echoed |
| Search results and snippets | No suspicion-tier text; highlights limited to normal-tier fields |
| Screenshots and printing | Per-user watermark; printing logged; confidential-tier printing needs approval (CBI 2.4) |
| Support bundles | Pseudonymised (11.7), size-limited, bank-reviewed before sending (13.3) |
| Development, test, demo, training | Synthetic data (4.3) |

### 11.7 Pseudonymisation and keyed hashing

- **Pseudonymous identifiers**: canonical UUIDv7 ids in the audit trail, logs and cross-system references; the audit trail never holds names.
- **Keyed hashes, not plain hashes.** National ID numbers, phone numbers and card numbers have small key spaces, so an unkeyed hash is reversed by enumeration within hours on a laptop [K]. Every join key or blind index uses HMAC-SHA-256 with a key in OpenBao, one key per purpose: search blind indexes, card tokens, export pseudonyms and support-bundle pseudonyms (5.2, 5.3).
- **Per-bundle key for vendor bundles**: each support bundle is pseudonymised with a fresh key that stays with the bank, so the vendor cannot link two bundles or re-identify anyone, and the bank can resolve a pseudonym locally when the vendor's finding refers to one.
- **Working sets for tuning and model development**: pseudonymised, in the production zone, with free text removed and dates shifted only when the method allows.
- **Aggregates leaving the system** (board reports, statistics for the CBI's self-assessment): small-number suppression with a bank-set minimum cell size (default 5, our default).
- **Pseudonymisation is not anonymisation** (11.1). Pseudonymised sets stay classified at the level of their source, are covered by the same retention and are never published.

### 11.8 PCI DSS scope: staying out by design

| Item | Allowed in AKR-TMS | How |
|---|---|---|
| Full PAN | **No** | Not selected from the source, masked at landing (11.5), never in messages or logs |
| Masked PAN (first six and last four at most) | Yes | The `card` tag; PCI DSS limits a displayed PAN to these digits [K] |
| Processor or bank token, or keyed hash | Yes, preferred: the **bank's token** | A stable surrogate AKR-TMS cannot reverse; where none exists, HMAC-SHA-256 with a card-specific OpenBao key |
| Expiry date, service code | No | No scenario needs it |
| Sensitive authentication data (full track, CVV or CVC, PIN, PIN block) | **Never**, not even encrypted | PCI DSS forbids storing it after authorisation [K]; the guard drops it |
| Cardholder name | Only through the customer's `party_name` | Not read from card tables |
| MCC, merchant and terminal ids, amounts, response code, entry mode, country | Yes | They are what the card scenarios use (file 25 section 12.6, file 27) |

PCI DSS applies to entities that store, process or transmit cardholder data and sensitive authentication data, and to systems that connect to or could affect the cardholder data environment [K; the PCI SSC documents could not be downloaded, section 3.1]. Two consequences. First, if the only card data AKR-TMS sees is a masked PAN and a token, it holds no stored PAN and is outside the stored-data requirements. Second, **never point `akr-ingestd` at a database that holds full card numbers**: point it at a view or reporting replica without them, because a system that reads from the cardholder data environment can be called "connected" and drawn into the bank's scope, which is the bank's assessor's decision to make. Where a bank wants card-level analytics that need the full PAN, those scenarios run inside its card environment under its own PCI programme, and AKR-TMS records that exclusion in the coverage matrix (file 25 section 13.6). Evidence for the assessor: data-flow diagram, the view definitions, guard statistics and the PAN-free test reports.

### 11.9 What is deliberately not collected

- **Special categories**: religion, sect, ethnicity and political opinion (GDPR Article 9(1) lists racial or ethnic origin, political opinions, religious beliefs, health and biometric data as special categories [V]). The Iraqi civil status ID, a legacy document banks still accept, records religion (file 22 section 3.2, citing the UK Home Office), and the national card, a biometric card with a chip, records **blood type** (file 01 section 7). The mapping layer has a **blocked-attribute list** that rejects these columns by default; adding one needs a written legal basis and the MLRO's approval through maker-checker, and the data may never be used as a model feature (file 29 GV-37; file 22 N-05; file 26 K-33).
- **Biometrics and images**: no fingerprint templates, face images or ID-document images; KYC documents stay in the bank's document system and AKR-TMS keeps only a vault pointer in the `kyc_document` entity (file 25 section 13.2).
- **Tribe or family name** is read only as a name component for matching and for the family-relationship graph (file 22), never as a risk factor (section 19).
- **Health data in narratives** is not parsed or extracted.

### 11.10 Disclosure register

Banking secrecy (Law 94 of 2004 Article 49, section 2.2) makes every disclosure of customer data outside the bank something the bank must be able to justify. AKR-TMS records, in the audit trail, each **transfer out of the bank**: goAML submissions (and the receipt), responses to the CBI, courts and the Office (requester, legal basis, fields released, approver), correspondent-bank answers approved by the MLRO, and support bundles (13.3). The register is a view of the audit trail, visible to the MLRO, the data-protection officer and internal audit, and it is part of the assessor pack (15).

### 11.11 AKR-TMS hooks

- **Classification registry** as code: a table of schema, table, column, tag, owner, purpose and masking function; a CI check fails any migration that adds an untagged column; the data dictionary published for assessors is generated from it (file 25 section 13.7).
- **Masking library** in `akr-common` (`mask_id`, `mask_phone`, `mask_account`, `mask_pan`) used by the API serialiser, with the `unmask` event; a test that every serialised field has a masking rule.
- **Card guard** crate with Luhn, issuer ranges and the bank's exclusion patterns; fuzz-tested; run at landing, in log redaction and in CI.
- **Pseudonymiser** service with per-purpose keys and the per-bundle key mode.
- **Blocked-attribute list** in the mapping layer, and the "mapped but unused" report.
- **Disclosure register view** and the export-approval workflow.

---

## 12. Retention and deletion: 5, 7 and 10 years, legal hold and crypto-shredding

### 12.1 The clocks that apply

| Source | Rule | Applies to |
|---|---|---|
| Law 39 of 2015, Article 11 [V] | At least **5 years** after the relationship ended, the account closed or the occasional transaction was carried out, "whichever is longer"; transaction records "attempted or executed", detailed enough to reconstruct each one; **STR copies and related documents for at least 5 years from the report date or from a ruling in a lawsuit**; records of the risk assessment. A fine of IQD 25 to 250 million for failing to keep records (Article 39, First) | The bank's records |
| CBI AML/CFT controls 2025, chapter 4 (file 01 section 2d) [V] | Keep customer and beneficial-owner records, transaction records, **unusual-transaction reports and the results of reviewing them**, STR records with copies, **records of cases the MLRO decided to close**, and training records; stored securely **with backup copies in another location** and retrievable quickly; at least 5 years (customer and transaction records from account closure, unusual-transaction reports from the report date) | The bank's records, including what AKR-TMS generates |
| OFAC final rule of 21 March 2025 (file 05 section 1e) [S] | Transaction and blocked-property records for at least **10 years**; OFAC said a conflicting foreign rule does not excuse compliance | Sanctions-related records of banks that clear US dollars |
| EU AMLR Article 77(3) (file 05 section 5e) [S] | 5 years, then delete personal data | Only banks with EU exposure |
| DBS decision D5 [V] | Audit trail retained without limit; "get compliance's statement in writing anyway" | AKR-TMS inherits the default |
| CBI cyber-resilience controls (2.4) [V] | Backups encrypted before leaving the site, retention "consistent with legal requirements", secure deletion by third parties at the end of a service, devices destroyed at end of life (confidential tier) | Backups, vendors, media |

The rules conflict (the EU says delete after five years, OFAC says keep ten). The product ships a **jurisdiction profile** per bank, with Iraq always on and OFAC and EU switchable, and the bank's policy resolves conflicts with a recorded legal basis (file 05 requirement R-24).

### 12.2 AKR-TMS is not the system of record for CBS data

Law 39's duty is the bank's. The CBS and the bank's archive hold the customer and transaction records; AKR-TMS holds a **monitoring copy** and the records it **generates** (alerts, review results, cases, decisions, STR copies, screening results, audit). Each deployment signs a short statement of this split in its record-keeping policy, because it decides which AKR-TMS tables carry a legal clock and which carry a monitoring horizon. A bank that wants AKR-TMS to be the record for a class (for example attempted payments seen only by the phase-2 service) says so, and the class is then treated as a legal record. To keep evidence independent of the monitoring copy, **opening a case freezes an evidence snapshot** (the transactions, the party profile as of that time, the list-match detail and the scenario version) into the case record, so dropping an old month of transactions never damages a case.

### 12.3 Retention classes (defaults; a bank may lengthen, and may shorten only down to the floor)

| Class | Contents | Clock starts | Floor | Default | Disposal |
|---|---|---|---|---|---|
| R1 Raw landing | Source rows as read, lineage hashes | Extraction day | None | 90 days (bank-set, 30 to 400) | Partition drop and key destruction |
| R2 Monitoring copy of transactions | `canon.txn`, events, payment messages | Booking month | 5 years (Article 11); 7 years in the Iraq profile (D2.1(c)(1)) | 10 years | Partition drop and key destruction |
| R3 Master data copy | Party, account, relationship, KYC profile, with their bi-temporal history | Relationship end (all accounts closed; a reopening resets it) | 5 years after end (Article 11); 7 years in the Iraq profile (D2.1(c)(1) names CDD files) | 7 years after end | Record-level purge of the whole history |
| R4 Features and aggregates | Windows, peer groups | Computation month | None | 3 years, or the longest scenario look-back | Partition drop |
| R5 Alerts | System alerts and their disposition reasons | Closure | 5 years from the report date (CBI chapter 4); 7 years in the Iraq profile (D2.1(c)(1); file 28 M-05) | 10 years (alerts tied to sanctions: 10 years) | Partition drop |
| R6 Cases and STRs | Cases, closure reasons, MLRO decisions, STR copies, evidence snapshots, attachments | Later of case closure and STR date | 5 years from the report date or the ruling (Article 11); 7 years in the Iraq profile (D2.1(c)(1); file 28 M-05) | 10 years | Record purge and object deletion after the lock expires |
| R7 Screening and sanctions records | Match results, clearances and whitelist decisions with reasons, blocked or rejected payments, phase-2 assessment decisions | Decision date | 7 years in the Iraq profile (5 under Article 11; D2.1(c)(1)); 10 years where OFAC applies | 10 years | Partition drop |
| R8 Rating history | Customer risk ratings, reasons, periodic-review records | Relationship end | 5 years after end (risk-assessment records); 7 years in the Iraq profile (D2.1(c)(1)) | 7 years after end | Record-level purge |
| R9 Configuration history | Scenario versions, thresholds, mappings, policies, role grants, approvals | Version retired | Must outlive every alert it explains | 10 years after retirement | Kept (small) |
| R10 Audit trail and access events | Hash-chained entries, anchors | Event time | None found | Unlimited (D5); free-text reasons follow the case class | Cold tablespace; never dropped |
| R11 Models and validation | Artifacts, training-set manifests (hashes, no data), validation reports | Model retired | None | 10 years | Object deletion after lock |
| R12 Operations | Logs, metrics, traces, support bundles, training and tuning extracts | Creation | None | 30 to 90 days; bundles 30 days; extracts deleted once the run is validated, at most 90 days (the model keeps hashes, not data) | Lifecycle |
| R13 Backups | pgBackRest, restic, OpenBao snapshots | Creation | CBI: consistent with legal requirements | About 5 weeks (the DBS scheme: four weekly fulls plus WAL; restic passes kept 48 hours then daily for 31 days [V, `docs/plan/backup-restore.md`]) | Expiry; **no long-term archive backups of personal data** |

*Alignment note (2026-10-07, from files 18 and 28):* the CBI Detailed Assessment Guide used in the banking reform asks for "high-quality data for all AML-relevant operations for 7 years", with record-keeping that covers "STRs, CDD files and audit logs" (D2.1(c)(1); file 28 section 2, file 18 VD-19). Law 39 and the CBI controls say "at least" five years, so seven years meets both texts. The Iraq profile therefore uses a **7-year floor** for R2, R3, R5, R6, R7 and R8, and the retention report shows both sources (file 28 M-05). Whether the CBI means the seven years to apply outside the assessment is still open (file 28 section 16.8, question 8).

The retention engine rejects a setting below a floor, requires a recorded board risk acceptance to go below the OFAC default, and prints the effective schedule in the assessor pack. Because Article 11 counts from the end of the relationship, class R3 and R8 are **record-level** rules on low-volume tables; high-volume time-series (R1, R2, R4, R5, R7) use partitions. This refines file 32 section 6.4, which described partition drops only.

### 12.4 Mechanisms

1. **Retention job** (a worker job, dry run first): finds expired partitions and records, shows counts and date ranges, and waits for **maker-checker approval** by the data owner (the MLRO or head of compliance) and the CISO, with a notice period (default 7 days) in which a hold can still be placed.
2. **Detach, then drop**: an approved partition is detached (still encrypted, invisible to queries) for 30 days (our default) so a mistake can be reversed, then dropped. PostgreSQL's `DETACH PARTITION ... CONCURRENTLY` avoids long locks [K].
3. **Crypto-shredding**: time-series partitions use a data key per partition-month (5.3). After the drop, the wrapped key is destroyed and the destruction is logged. Backups hold ciphertext and an older copy of the wrapped key, so **deletion becomes effective when the last backup containing the key expires** (about five weeks in the DBS scheme); the certificate states that date. Raising the transit `min_decryption_version` does not help: an older OpenBao snapshot brings the key version back, and the DBS rule says never to raise it past a version that signed evidence (5.3).
4. **Record-level purge** for R3, R6 and R8: delete by relationship or case, cascade across bi-temporal history so no partial history remains; deletion leaves remnants in free space and WAL until reuse and expiry, so these classes are *deleted*, not cryptographically erased (a per-relationship key was considered and rejected as too costly for a feature no Iraqi rule requires; revisit if one appears).
5. **Objects**: evidence objects are written with an object-lock retention equal to the **floor** (not the default), because COMPLIANCE retention cannot be shortened; at case closure the retention is extended to the class period; legal hold uses the store's hold flag; deletion happens by lifecycle after the lock expires. Anchors are extended yearly so that "unlimited" audit retention is not cut off by a 3,650-day lock (the DBS creates anchor and finals buckets with COMPLIANCE 3650d [V, `docker-compose.yml`]).
6. **No resurrection after a restore**: every run writes a signed **deletion certificate** (class, partitions and date ranges, row counts, manifest hash, key ids, holds consulted, approvers, backup-expiry date) to the audit trail and to the anchors bucket; after any restore a post-restore job replays the certificates, dropping partitions and destroying keys again.

### 12.5 Legal hold

A hold has a scope (customer, account, case or date range), a source (court, the Office, the CBI, internal investigation) and an owner, and is itself **suspicion-tier information** (the existence of a hold reveals interest). It blocks purge and disposal of everything in scope. Because a partition can hold one customer's rows beyond its expiry, the retention job **copies held rows into a hold store** under a per-hold key before it shreds the partition, verifies counts and hashes, and then proceeds; releasing a hold recomputes the clocks. Holds are listed to the MLRO, legal counsel and the CISO only, and each one has a periodic review date.

### 12.6 Requests to delete personal data

Iraq has no erasure right, and AML duties override one in any case (Law 39 Article 52, 2.1). Customers' requests go to the bank's own process, which answers without revealing any report or case (section 19). AKR-TMS offers **restriction** (a flag that stops secondary use) but never deletes inside a legal clock, and it never edits CBS-origin data: a correction made in the CBS flows in through the next load.

### 12.7 End of the contract and end of life

The bank owns the data and the models trained on it (file 32 section 19). On termination or replacement, the product provides an **export** in open formats (Parquet or CSV with a schema and the data dictionary) plus the evidence objects with manifests, and a **destruction procedure** that destroys keys, wipes or crypto-erases volumes and produces a signed certificate; the vendor holds nothing on-premises, so there is nothing to recover from it (CBI 13.1.6 requires secure deletion by third parties at the end of the service, 2.4). Media follow the bank's sanitisation procedure and the confidential-tier rule that devices are destroyed at end of life; NIST SP 800-88 is the usual reference for media sanitisation [K].

### 12.8 Evidence for assessors

A **retention report**: for each class, the clock, the floor and default, the oldest record, the next scheduled purge, the holds in force, the last certificates, and the backup-expiry horizon; plus a yearly test that restores a backup into a throwaway environment and shows that certificates replay correctly.

### 12.9 AKR-TMS hooks

- **Retention engine** (classes, jurisdiction profiles with floors, dry run, maker-checker, detach-then-drop, certificates, post-restore replay) in `akr-worker`.
- **Hold registry and hold store** with hold-aware shredding; a test that fails if a partition with an active hold is dropped.
- **Evidence snapshot** written at case opening (file 28 workflow).
- **Object-lock extension job** for evidence, anchors and models.
- **Retention report** and the export and destruction tooling.

---

## 13. Vendor access, remote support and operations

### 13.1 Principles

The CBI treats a third party as a risk to be contracted, vetted, monitored and revoked (cyber-resilience controls chapter 13, 2.4), and the 2026 risk-management controls add that an outsourced provider must make "no changes to the outsourced service without prior approval of the bank" and allow the bank's internal audit and CBI examiners to assess it (2.6). The vendor access model of AKR-TMS follows from six rules.

1. **No standing vendor access** to the bank's production environment, data or keys (zone Z7 is empty, 4.1).
2. **The bank opens every session**, for a stated reason, for a limited time, through its own PAM or jump host (CBI 8.4.2: strong authentication, strong remote-access controls, a limited number of privileged users, need-to-access, timely review of activity, no shared privileged accounts).
3. **The vendor never sees personal data or suspicion information.** The confidential tier allows no remote access and top-secret information is not to be given to non-Iraqi individuals except by a written exception (2.4); the product therefore gives the vendor role no data permission at all.
4. **Everything is recorded** on the bank's side (session recording, application audit trail, the bank's PAM log) and the vendor must make its own audit logs available on request (13.2.3.7).
5. **Updates are pulled by the bank**, verified by the bank and applied under the bank's change control (CBI 10.1.3 and 10.1.6: formal change process, user acceptance testing before implementation).
6. **Vendor staff are vetted and bound by non-disclosure terms** that survive the end of their employment (13.2.2.1 and 13.2.2.2).

### 13.2 Support tiers

| Tier | Who | What they can see | How access is granted | Limits |
|---|---|---|---|---|
| L1 | The bank's operators and platform administrators | Health, configuration, logs, metrics; no case, customer or STR content without a business role | Named accounts from the bank's directory, MFA | Roles of section 7 |
| L2 | Vendor engineer, remote | Role `support-readonly`: versions, health, reconciliation summaries, redacted logs and metrics, configuration with `detection-logic` parameters hidden | A **bank-opened, time-boxed session** through the bank's PAM; screen-share is driven by the bank's operator, or a brokered session with MFA | No data permissions; no export; no file transfer except the support-bundle tool; session recorded; reviewed afterwards |
| L3 | Vendor engineer, on site | The same role | Escorted visit under the bank's physical-access rules, on bank-supplied equipment | Same limits |
| (none) | Vendor access to suspicion-tier or personal data | Not offered | If a defect needs real data, the bank's staff reproduce it while the vendor guides, or the bank makes a pseudonymised reproduction | |

### 13.3 Support bundles

The bank runs `akr-ctl support-bundle`, which writes an archive containing: versions and image digests with the SBOM id; configuration with secrets removed and, unless the bank ticks a box, without `detection-logic` parameters; health, queue and reconciliation summaries; a metrics snapshot; **redacted logs** (an allow-list of fields, ids pseudonymised with a per-bundle key that stays with the bank, 11.7); normalised query statistics without literals; table row counts. It has a size cap and a manifest the bank's operator reads before sending, and it is signed by the bank. The bundle is itself a disclosure and is entered in the register (11.10). Bundles older than 30 days are deleted (R12).

### 13.4 Updates and patches

Release bundles are signed OCI-layout archives with an SBOM diff, release notes, test evidence and a rollback plan, imported into the bank's Harbor and verified by an admission policy (file 32 section 16.3; TS-30). The bank tests in its own environment on synthetic data, approves through its change board and applies the update with its own operators (or the vendor's under a supervised session). The CBI asks banks to define a time frame for each class of security patch and to test patches before production (9.1, 2.4); the vendor therefore publishes **patch commitments** (14.7) that let a bank meet its own time frames. There is **no call-home, no auto-update and no remote push**.

### 13.5 Contract annex: a checklist tied to the CBI items

| Clause | CBI source |
|---|---|
| The vendor applies the bank's cyber policy and the relevant law; the vendor is "legally responsible" for breaches | 13.1.4, 13.1.5 |
| Non-disclosure and **secure deletion** of the bank's information at the end of the service | 13.1.6 |
| Vetting and screening of vendor companies and staff on critical systems; non-disclosure terms that survive employment | 13.2.2.1, 13.2.2.2 |
| MFA for access to critical systems; documented grant and revoke process; revocation when staff leave or change role; regular access reviews by the vendor | 13.2.3.1, 13.2.3.4 to 13.2.3.6 |
| Vendor audit logs retained and supplied on request | 13.2.3.7 |
| Right of the bank's internal audit and of CBI examiners to assess the service; no change without prior approval; termination by the bank without financial obligation | 2026 risk-management controls (2.6) |
| Data ownership: the bank owns its data, mappings, configuration and any model trained on its data | File 32 section 19 |
| Notification of a suspected compromise of the vendor's build, signing or support systems within 24 hours (our proposal), and a joint incident runbook | CBI incident management (2.4) |
| Support staff locations, and the top-secret rule on non-Iraqi staff | 2.4 |
| Sub-contractors: none for support; software dependencies are listed in the SBOM, not treated as sub-processors | |
| Continuity of the vendor: a support-continuity plan, and **source-code and build escrow** with release conditions the bank can trigger if the vendor ceases to trade [K] | CBI outsourcing contingency plans for termination (2.6) |

### 13.6 The vendor's own security

Before the first bank: a certified ISMS (3.3); developer and build systems on managed devices with MFA; signing keys in an HSM or KMS with two-person control; a ticketing system into which the bank enters no personal data; a vulnerability-disclosure policy and a product security response team (section 14); staff screening and security training with the same awareness topics the CBI requires of banks (Protect chapter, awareness and training). The vendor holds **no CBS credentials and no bank keys**, so a compromise of the vendor cannot open a bank (file 25 section 10.2; 5.3).

### 13.7 If the vendor operates the system for the bank

A managed service on the bank's premises makes the vendor's staff privileged users: named accounts, vetting, the bank's PAM, recorded sessions, dual control for destructive actions and key operations, quarterly access review. Vendor DBAs would read names and addresses in disk-encrypted columns, so for this model the **strict mode** (5.2: identifiers and names in encrypted columns) is the default, and the suspicion tier remains unreadable to them. A vendor-operated service in a vendor data centre is a different model with its own limits (section 17).

### 13.8 The CBS vendor and other third parties

The CBS vendor and the Swift service bureau must not see AKR-TMS content. The bank's DBA creates the views and audits the AKR-TMS account (file 25 section 10); AKR-TMS never queries per customer, so the CBS vendor's logs cannot show who is under review (file 25 section 10.6). List providers receive nothing about customers: lists are pulled in and never screened remotely (file 01 section 8; file 25 section 10.4).

### 13.9 AKR-TMS hooks

- **Roles** `support-readonly` and `operator` in the Cedar bundle with tests showing neither can read a data route.
- **`akr-ctl support-bundle`** with the redaction allow-list, the per-bundle pseudonym key, manifest and signature.
- **Session-record evidence**: the product writes a "support session opened by" audit event when the bank starts a session (reason, ticket, time box) and an end event.
- **Contract annex template** in the delivery kit, with the CBI mapping of 13.5.

---

## 14. Secure development and supply chain (SBOM, signed images, dependency control)

### 14.1 Why this is the highest-leverage risk of a shipped product

One build is installed in many banks, so whoever controls the build, the signing key or a dependency controls many banks. Two public cases show the shape of the risk. In the SolarWinds incident an attacker compromised the vendor's Orion software and reached customer networks through its updates; CISA reported the activity from at least March 2020 and named a supply-chain compromise of an Orion DLL as one initial access vector [V, CISA AA20-352A]. In September 2025 a self-replicating worm ("Shai-Hulud") compromised over 500 packages on npm, scanning the infected environment for credentials and targeting GitHub personal access tokens and cloud API keys [V, CISA alert of 23 September 2025]. The CBI controls give the bank's side of the same problem: third-party risk review, vetting, and tested patches (2.4, 13). The controls below are what the vendor owes in return.

### 14.2 Framework: NIST SSDF

NIST SP 800-218 (SSDF) version 1.1 of February 2022 is the reference; a revised draft, Rev. 1 "SSDF Version 1.2", was published on 17 December 2025 with comments closed on 30 January 2026, and whether the final text has been issued was not checked [V, NIST CSRC page]. Practice numbers below are those of 1.1 [K].

| SSDF group | AKR-TMS practice |
|---|---|
| **Prepare the organisation** (PO) | This file's requirements as the security specification; named security owner; secure-development training for engineers; build and test toolchain treated as a protected asset (14.4); security checks as pipeline gates |
| **Protect the software** (PS.1 to PS.3) | Access control and branch protection on source; signed commits; verification of release integrity by signature and checksum (PS.2); each release archived with its SBOM and provenance in a locked bucket (PS.3) |
| **Produce well-secured software** (PW.1 to PW.9) | A threat model per release (the table in section 1.5 is the starting point); reuse of well-reviewed components only (PW.4, 14.6); secure coding rules (14.3); build configuration (PW.6); human review of every change and a second reviewer for security-sensitive paths (PW.7); testing (PW.8, 14.8); secure defaults, including strict mode off only by decision (PW.9) |
| **Respond to vulnerabilities** (RV.1 to RV.3) | Continuous dependency and image scanning; a disclosure channel; patch commitments (14.7); root-cause analysis recorded for each product vulnerability |

### 14.3 Coding rules specific to this stack

- **Memory and arithmetic**: Rust for all services; the workspace lint `unsafe_code = "forbid"` (the DBS setting [V, `backend/Cargo.toml`]) so that no AKR-TMS crate contains `unsafe`, which then lives only inside reviewed dependencies (the ODBC, Oracle and ONNX bindings); `unwrap_used` and `expect_used` set to warn, which becomes an error under `clippy -D warnings`, with test files opting out explicitly (`authz_coverage.rs` does [V]); `overflow-checks = true` in release builds; money in `numeric(28,6)` and `rust_decimal`, never floating point.
- **SQL**: compile-time checked SQLx queries with bind parameters only; dynamic filters built from allow-listed columns; the CBS reader's SQL is generated from catalogue metadata and the mapping document, with identifiers from the allow-list and every value bound (file 25 section 14).
- **Untrusted parsers**: the MT and ISO 20022 parsers, the T24 `XMLRECORD` parser, the mapping DSL, CEL expressions and the name-normalisation library are fuzzed (`cargo fuzz`) and have size, depth and time limits; XML parsing must not resolve external entities or DTDs [K]; the expression language runs without I/O and with a cost budget.
- **Authorisation**: every route has an entry in a route table and a Cedar action, and a test walks the router against the table in both directions and sends live requests, which is the DBS coverage pattern (`backend/api/tests/authz_coverage.rs`, three angles: router versus table, table versus router, and live traffic that must write a decision row [V]). The same test checks that every route returning restricted data writes an access event (9.6).
- **Idempotency scoped to the caller**: idempotency keys are bound to the authenticated actor and a request with no principal gets no ledger row. The DBS made this change after finding that a global key namespace let one employee pre-claim another's keys, an availability attack rather than a leak because the request hash already covered the actor (migration `0024_idempotency_keys_scoped_to_actor` [V]). Phase 2 adds byte-exact replay (file 25 section 15.3).
- **Web**: strict Content-Security-Policy with no inline scripts and no third-party origins (the air-gapped deployment has none anyway); cookies `HttpOnly` and `SameSite=Strict` with the HMAC-bound CSRF token the DBS uses [V]; `frame-ancestors 'none'`; no `dangerouslySetInnerHTML` except through DOMPurify (a dependency of the DBS frontend [V, `package.json`]); no source maps in production bundles.
- **Right-to-left text attacks**: Unicode bidirectional control characters (U+202A to U+202E and U+2066 to U+2069) in names and narratives can make a screen show a different order from the stored text ("Trojan Source", CVE-2021-42574 [K]). An AML screen is a prime target. Names and free text are stripped of these characters or displayed with them made visible, and are rendered inside isolated bidi elements.
- **Spreadsheet injection**: exported cells beginning with `=`, `+`, `-` or `@` are prefixed so they cannot execute in a spreadsheet [K].
- **Uploads and rendering**: files are scanned (ClamAV, as the DBS does for documents [V]), type-checked and stored in a quarantine bucket first; PDF generation by a headless-Chromium service (Gotenberg in the DBS) renders only internal templates, has no outbound network and is patched with the rest.
- **Errors and logs**: generic problem details; no personal data in logs (11.6).

### 14.4 Source, build and release integrity

SLSA v1.2 (approved) defines a build track with three levels (L1 provenance exists, L2 signed provenance from a hosted build platform, L3 hardened build platform that resists tampering during the build) and a source track with four (L1 version controlled, L2 history and provenance, L3 continuous technical controls, L4 two-party review) [V, slsa.dev]. The target:

| Stage | Control | SLSA target |
|---|---|---|
| Source | Private Gitea; protected default branch; **two-party review** on every merge, with code owners for authorisation, cryptography, ingestion, audit, retention and the mapping layer; signed commits; secret scanning in pre-commit and CI; no secret in any repository | Source L3 at launch, L4 for the sensitive paths |
| Build | Ephemeral runners from a hardened image, no access to production secrets, no network except to the internal registry mirror and the vendored dependency store; `cargo build --locked` with vendored crates; frontend `pnpm install --frozen-lockfile` (the DBS package manifest uses caret ranges, so only the lockfile fixes versions [V]); toolchain pinned (`rust-toolchain.toml`, as the DBS [V]); base images pinned by digest | Build L2 at launch, L3 within the first year |
| Artifacts | Static Rust binaries in minimal images, non-root user, read-only root file system, dropped capabilities, default seccomp profile, no shell where possible | |
| Provenance | An in-toto attestation per image and per bundle, signed by the build platform and stored with the artifact | |
| Release | Images, charts and the bundle signed with cosign (v3.1.3 at the time of writing, file 32); signing key in an HSM or KMS with **two-person control** and a documented rotation; public key fingerprint delivered to each bank on a separate channel (the contract annex) and pinned in the bank's Harbor and admission policy | |
| Verification at the bank | Bundle verifier (signature, checksums, SBOM present, provenance) before import; admission policy rejecting unsigned images (Kyverno or sigstore policy-controller, file 32 section 16.3) | |
| Reproducibility | A documented build recipe aiming at **bit-for-bit reproducible** binaries (pinned toolchain, vendored sources, path remapping, fixed timestamps) so an independent party, or a bank that holds the source in escrow, can rebuild and compare digests [K; reproducibility of the whole image set must be proven, not assumed] | |

### 14.5 SBOM, provenance and VEX

Each release ships a **CycloneDX 1.7 JSON** SBOM (specification 1.7.2 released 2026-09-17 [V]) merged from the Rust crates (`cargo-cyclonedx`), the frontend packages and the image contents (Syft), signed and stored beside the artifact, with an **SBOM diff** against the previous release in the release notes. The models in a release are listed as machine-learning components (CycloneDX has a machine-learning-model component type [K]); SPDX 3.0.1 is available on request. A **VEX** statement per release says which known vulnerabilities in dependencies are not exploitable in AKR-TMS and why, so a bank's scanner does not raise false alarms and the CBI-approved assessor can check the reasoning. `cargo-auditable` embeds dependency data in each binary so a binary found on a server can be audited without the repository (file 32 section 16.4).

### 14.6 Dependency governance

- **Gate**: `cargo deny` (advisories, licences, bans, sources) and `cargo audit` block a build on a known vulnerability, a yanked crate, an unmaintained crate without a recorded waiver, a disallowed licence, or a crate fetched from outside the approved registry.
- **New dependency review**: every new crate or package comes with a short record (purpose, maintainers, last release, `unsafe` and build-script use, transitive count, licence) and a second reviewer; where practical, `cargo vet` audits are imported from organisations the vendor trusts [K].
- **Minimum release age**: the front end delays adopting a freshly published package version; pnpm has settings named `minimumReleaseAge`, `trustPolicy`, `blockExoticSubdeps`, `strictDepBuilds` and `allowBuilds` for delay, provenance checks, refusal of git or tarball sub-dependencies, and control over which packages may run build scripts [V, pnpm.io settings index; semantics K]. Install scripts of dependencies are off by default and allow-listed by name.
- **Update cadence**: weekly automated update proposals, reviewed like any change; critical advisories go through the emergency path (14.7).
- **Minimal runtime**: no package manager in runtime images; no compilers; no debugging tools; the number of third-party runtime components is part of the release notes, and fewer is better (the stack decisions of file 32 favour one binary per role).
- **Stale components**: the Rust `oracle` crate is stale (file 32 section 4), so it is isolated behind the driver port and tracked in the waiver register with an owner and a review date.
- **Licence hygiene**: the third-party notices file and the legal sign-offs listed in file 32 section 19.

### 14.7 Vulnerability handling and patch commitments

The CBI requires a bank to define a time frame for each class of security patch and to test patches before production (9.1, 2.4). For a bank to meet that, the vendor commits, in the contract annex, to the following (our proposal, to be aligned with each bank's own time frames):

| Class | Advisory and mitigation | Fixed release in the bank's hands |
|---|---|---|
| Exploited in the wild, or unauthenticated remote compromise of the product | 48 hours | 7 days |
| Critical in a shipped component | 7 days | 14 days |
| High | 14 days | 30 days |
| Medium | Next release notes | 90 days |
| Low | Next release notes | Next scheduled release |

Releases follow a quarterly cadence plus security releases as needed. Supported versions are the current and previous releases (security fixes for 24 months, our proposal). A **private reporting channel**, a coordinated-disclosure policy with a 90-day default and credit to reporters, and a small product security response team handle reports; each fixed vulnerability gets a recorded root cause (SSDF RV.3) and, if it affected confidentiality, a notice to the banks.

### 14.8 Security tests in the pipeline

Unit and property tests (proptest for money arithmetic, watermark and lookback logic, retention floors); integration tests against throwaway PostgreSQL; the authorisation coverage test; fuzzers for parsers and the mapping DSL; **tipping-off inference tests** (8.9); cross-tenant tests where a hosted option exists (17); static analysis (`clippy -D warnings`, `cargo deny`, `cargo audit`, a rules-based scanner for TypeScript and SQL [K]); dynamic scanning of a staging deployment with an open-source scanner against the OpenAPI schema [K]; container and Helm scanning (Trivy, a chart linter); secret scanning; the OWASP ASVS Level 2 checklist as a release gate with evidence stored per release (3.1). Failures block the release.

### 14.9 AKR-TMS hooks

- **Pipeline definition** (Gitea Actions) with the gates above, committed in the repository and itself under two-party review.
- **Release register**: commit, builder, reviewers, approver, signing event, bundle digest, SBOM id and VEX id, kept in the audit trail and the locked bucket.
- **Dependency waiver register** with owner, reason and expiry date.
- **PSIRT runbook and advisory template**; patch-commitment table in the contract annex.
- **Reproducible-build check** run for every release candidate by a second runner.

---

## 15. Penetration testing and independent assurance

### 15.1 What the CBI asks for

| Requirement | Source |
|---|---|
| Vulnerability assessment **regularly**, automated and manual, covering common application flaws (SQL injection, cross-site scripting); a process to fix findings and **validate** the fix; **penetration tests** of network infrastructure and internet-facing systems "periodically or as needed" | Testing chapter 1.1 to 1.4 [V] |
| Security testing before go-live; vulnerability scans fixed before production and again after a major update; **independent penetration test periodically**; security review of the source code with findings fixed before production; re-assessment and re-approval after a major change; test evidence documented, classified under the bank's scheme and recording who tested, what, the results and the remediation; test data protected; **separation-of-duties controls tested** | Application security 14.4.1 to 14.4.14 [V] |
| Internal audit of information systems at least once a year with the report kept for regulators; independent, comprehensive and regular audits | Detect 2.1.4 and 2.2 [V] |
| Tests "at least annually and whenever needed, including penetration tests" | 2026 risk-management controls, IT risk [V] |
| Infrastructure and data-encryption compliance confirmed by a firm from the CBI's approved list (B7); continuity plans audited by an approved firm (B9); AML/CFT and sanctions tools verified by an approved firm (D2) | 2025 handbook (2.5) [V] |

The CBI's assessment guide and its list of approved firms were not retrieved (2.9), so the exact qualification of those firms is unknown.

### 15.2 The assurance cycle

| Activity | Who | When | Output |
|---|---|---|---|
| Release gates (14.8) | Vendor engineering | Every release | Evidence per release |
| **Independent penetration test of the product** | An external firm unrelated to development, rotated every two or three years [K] | Before the first bank delivery, yearly, and after any major release (14.4.13) | Report, fix tracker, retest letter shareable under non-disclosure |
| Source-code security review | Independent reviewer, focused on authorisation, cryptography, ingestion, tipping-off controls, audit and retention | Before first delivery and on major change (14.4.14) | Findings fixed before release |
| **Deployment test at the bank** (configuration, zones, integration, identity federation, backups, the CBS account) | Bank-appointed or vendor-appointed firm approved by the bank | Before go-live and yearly | Report in the bank's classification (14.4.4) |
| **Assumed-breach exercises** (15.4) | Independent team with the bank's SOC watching | Yearly | Detection and response findings |
| Control self-tests inside the product (15.6) | The product, continuously | Daily | Posture page and signed export |
| Internal audit of AKR-TMS | The bank | Yearly (Detect 2.1.4) | Audit report; uses the audit guide (15.5) |
| Data-integrity test report | Bank and vendor | Every 12 to 18 months, built on the reconciliation results (file 05 R-27) | Report for the CBI-approved assessor |
| Model validation | Independent validator (file 29) | Per model change | Validation report |
| DR exercise and restore tests | Bank IT | Yearly and monthly (16.7) | Documented scope, plan, results |

### 15.3 Scope of the penetration test (what is different about an AML platform)

1. **Authorisation and tipping-off**: horizontal and vertical privilege escalation across roles and branch scopes; maker-checker and segregation-of-duties bypass (self-approval, approval through a second session, replay of an approval); **inference of suspicion-tier existence** through search, counts, error messages, timing, export, notifications and the audit viewer (8.9); misuse of break-glass; conflict-check evasion.
2. **Hostile data from the CBS**: the CBS is a source of attacker-controlled text (a customer chooses a name, a payer fills a narrative). Test second-order injection (SQL, HTML, template, CSV formula), bidirectional-control characters, oversized and malformed values, malformed Swift and ISO 20022 messages, and malicious `XMLRECORD` content, through the real mapping layer.
3. **The ingestion path and the CBS account**: that the account cannot write, lock or read beyond its views (the read-only attestation, file 25 section 1.4); credential handling; behaviour when the CBS locks or drops the connection; and, with the bank DBA's written consent only, probing of the account's grants. Testers never run tests against the CBS itself beyond that.
4. **Phase-2 API**: authentication of gateways and certificate handling; **replay, signature forgery and downgrade**; idempotency abuse; denial of service that pushes the service into a fail-closed or fail-open state; probing of list contents through repeated assessments (an oracle attack), which needs rate limits and anomaly detection on the gateway client; timing differences between "clean" and "match" responses.
5. **Evidence, keys and audit**: tampering with the audit chain, anchors and evidence objects; unseal and key custody procedures; stolen-backup scenarios (can a backup be read without the key); WORM behaviour under a root credential (the conformance suite of file 32 section 12.5).
6. **Platform**: Kubernetes or compose hardening against the CIS baselines, Harbor, OpenBao, PostgreSQL, the object store, the admission policy, and the release-bundle verifier (can an unsigned or downgraded bundle be imported?).
7. **Insider paths**: a DBA with direct database access, a vendor engineer in a support session, an administrator trying to read cases, an analyst trying to export, a compliance officer trying to lower a threshold without a second approver.
8. **AI components** (section 18): prompt injection from free text, model extraction, poisoning through analyst feedback.
9. **The human interface**: right-to-left spoofing in names and amounts, look-alike characters in screening lists, session fixation, clickjacking.

### 15.4 Assumed-breach scenarios

Run once a year, against a staging copy with synthetic data and, for the detection part, in production under the bank's SOC with prior approval: (a) a stolen analyst session; (b) a malicious analyst who tries to enumerate and export; (c) a malicious DBA; (d) a compromised ingestion node; (e) a tampered release bundle; (f) a stolen backup set; (g) a compromised vendor support session. For each, the questions are: what did the attacker reach, did AKR-TMS or the SOC notice, and how long did detection take. The platform's own detection rules (9.3) and canary records (8.6) are tested in this exercise.

### 15.5 Assessor pack and internal-audit guide

The product ships, per release and per deployment, a pack for the CBI-approved firm, the bank's internal audit and the external auditor. Every item is generated from the system, not written by hand.

| Item | Where it comes from |
|---|---|
| Architecture, zone and data-flow diagrams with the PAN-free proof | Manifests and the zone table (4.5, 11.8) |
| Control catalogue: CBI control, ISO control, NIST function, feature, test, evidence location | Section 3.4 |
| Configuration baseline and drift report (CIS) | Installer pre-flight (file 32 section 16.7) |
| SBOM, VEX, provenance, signature-verification log | Section 14.5 |
| Penetration-test reports, retest letters, fix tracker | Section 15.2 |
| Role and SoD matrices, access-review reports, privileged-activity report | Sections 6, 7, 9 |
| Audit-chain verification report with anchors and the locked copy | Section 10.1 |
| Tipping-off control report and access-request register | Section 8 |
| Read-only attestation, footprint register and the bank DBA's signed load budget | File 25 sections 1.4 and 5.2 |
| Daily coverage certificates, reconciliation results, data-integrity test report | File 25 section 8 |
| Scenario coverage matrix, model inventory, validation packs | File 25 section 13.6; file 29 |
| Retention report and deletion certificates | Section 12.8 |
| Backup status, restore-test and DR-exercise reports | Section 16 |
| Key inventory, ceremony records, compromise drill results | Section 5.4 |
| Disclosure register and vendor-session register | Sections 11.10 and 13 |
| Release and change records with approvals | Sections 13.4 and 14.9 |
| Exception register (15.7) | This section |

The **internal-audit guide** turns the control catalogue into about thirty concrete tests an auditor can run alone (for example: ask for the list of everyone with access to a named suspicion-tier case and compare it to the register; run the chain verifier; request the deletion certificate for the oldest dropped partition and confirm the backup-expiry date; attempt a self-approval and observe the refusal).

### 15.6 Continuous assurance inside the product

A **security posture page** for the CISO shows, with a signed export: audit-chain and anchor status; key ages and rotation due dates; certificate expiry; release currency against the latest release and its advisories (VEX-adjusted); read-only attestation status; backup, WAL and restore-test status; OpenBao seal state; canary status; access-review status; retention status; configuration drift against the baseline; failed-login and lockout trends. The DBS already has an operations cockpit (module M15) with a backup tile fed by a heartbeat, a recorded certificate-expiry value and audit-chain status [V, `docs/MODULES.md`; `docs/plan/backup-restore.md` section 3; `docs/plan/phase-16.md`]; AKR-TMS extends the pattern.

### 15.7 Exceptions the bank must record

Some controls need a written exception in the bank's register, because the CBI text read literally conflicts with the design: (1) the **CBS service account**, an application account reading a CBS database although direct database access is for DBAs only (12.1.3; section 4.2); (2) **access by DBAs through the bastion** to ciphertext; (3) the **weekly control test payment** in phase 2 (file 25 section 15.6), a synthetic message with a known listed name sent through the real gateway to prove it is held, which touches the rule that test accounts and test data must not be used in production (14.2.9). The test identity has no account, moves no money and is never forwarded to the CBS; the bank's CISO approves it as a standing exception.

### 15.8 AKR-TMS hooks

- **Control catalogue as code** and the generated assessor pack.
- **Security posture page** and signed export.
- **Internal-audit guide** with test scripts that can run against a bank's own deployment.
- **Pre-engagement kit for testers**: rules of engagement, synthetic data set, test accounts that the installer can remove, a list of out-of-scope systems (the CBS), and a template report in the bank's classification.
- **Exception register template** (15.7, 4.5).

---

## 16. Business continuity and disaster recovery (RPO, RTO, and what happens when AKR-TMS is down)

### 16.1 What the CBI requires

A business-continuity plan and a disaster-recovery plan with a **geographically separate** site (or a near and a far site), **RTO and RPO defined per system**, annual tests with documented scope, plan and results; a backup policy with **encrypted off-site copies**, one copy on site, periodic restore tests and retention consistent with legal requirements; **notification of the CBI "as soon as possible" in case of any disaster**; incident management with severity levels and root-cause analysis (cyber-resilience controls, Respond chapter [V]; no numeric targets are mandated). The 2026 risk-management controls add a board-approved plan built from scenario analysis and a business-impact analysis, and an alternate site "far from the main site but easy to reach", in a stable area with utilities (2.6); the 2018 continuity controls refer to ISO 22301 and ISO 22317 and require tests that include the alternate site and the branches (2.7); the 2025 handbook asks for the plans to be audited by an approved firm (B9, 2.5); the AML controls require records stored with backup copies in another location and retrievable quickly (12.1). The physical-security chapter requires, for server rooms, a power generator to keep operating through an outage and an uninterruptible power supply for the servers (its items 2.4.5 and 2.4.6 [V]).

### 16.2 Obligations that run on a clock, for the bank's impact analysis

| Obligation | Clock | Depends on AKR-TMS for |
|---|---|---|
| Freeze on a confirmed sanctions match | "Immediately" (CBI sanctions guidelines 2023 [V]); "without delay" means within 24 hours of the listing decision (Regulation No. 6 of 2023 and the AML/CFT Office's 2026 guide [V], file 23 section 8); the product's default freeze clock is 8 hours, the strictest of the Office's texts (file 28 section 3.4) | Detection, the freeze task and its verification |
| Report to the Committee for Freezing Terrorists' Funds, copy to the AML/CFT Office | First report within 48 hours of the freeze, second within two months (Regulation No. 6 of 2023 [V]; the three days in the CBI's 2023 guidelines quote the repealed 2016 system; files 23 and 28) | The report clocks |
| STR to the Office | "Without delay" (Law 39 Article 12, Fifth [V]) | Case workflow and the goAML package |
| Daily screening of the customer base; list refresh | Daily; lists updated within 24 hours (12 hours under the 2025 controls; file 25 C-63) | The screening job and the list index |
| Branch liaison reports | Monthly, by the 10th (file 01 section 2d) | The liaison window |
| Phase 2: pre-release decision | Seconds | The assessment service |

The bank's BIA registers AKR-TMS as a **regulatory-critical** system: not in the payment path in phase 1, but the carrier of these duties.

### 16.3 Two kinds of data, two kinds of target

| Kind | Examples | Recoverable from elsewhere? | Consequence |
|---|---|---|---|
| **Re-derivable** | Raw landing, canonical copies, features, system-generated alerts | Yes: re-read from the CBS by watermark, within the CBS's own retention | A larger RPO is tolerable; the cost is time to rebuild and a blind period |
| **Original** | Analyst and MLRO decisions, case notes, STR drafts, evidence, whitelists and their approvals, configuration and policy versions, audit trail and anchors, keys | **No** | A small RPO is essential; these are the records the CBI and the Office will ask for |

### 16.4 Proposed targets (a bank sets its own in its BIA)

| Function | RTO | RPO | Mechanism, by hardware profile (file 32 section 16.6) |
|---|---|---|---|
| Analyst application, cases, MLRO workspace, evidence | **S**: 1 hour by restore onto a second server (the DBS target, proven in its rehearsal [V]); **M and L**: about 5 minutes by promoting a kept-in-sync replica (the DBS's second target) | **S**: at most 15 minutes (the DBS proposal; WAL is archived within a minute, files every 15 minutes [V]); **M and L**: seconds to a minute | pgBackRest with continuous WAL archiving (`archive_timeout=60`), a replica at the second site, restic or store-native copies of the object store |
| Keys (OpenBao) | 1 hour | 15 minutes by snapshot; zero with a three-node raft cluster | Raft snapshots, sealed by OpenBao's own barrier, copied off-site; unseal shares kept in sealed envelopes at separate sites (5.3) |
| Ingestion pipeline | 4 hours before the freshness SLO is breached; escalation to the MLRO at 24 hours | None for CBS-derived data; the watermark is the state | Restart, then backfill (file 25 section 7); gaps recorded as blind periods |
| Phase-2 assessment service | 15 minutes (file 25 section 15.6) | **Zero** for decisions (synchronous commit to a standby) | Two active nodes, a standby database, the gateway's **local fallback screen** |
| Audit anchors and evidence objects | With the application | 15 minutes; locked copies are independent of the database | WORM bucket and its replication |
| Release bundle, Harbor content, Helm values | 4 hours | Last release | The signed bundle held at the DR site and on offline media is itself the rebuild recipe |

### 16.5 Failure scenarios

| Scenario | Response | Notes |
|---|---|---|
| Process crash or node loss | Restart or reschedule; two nodes for phase 2 | K3s restarts; health probes |
| Disk full on the WAL archive | The DBS design sets a maximum queue size after which WAL is dropped and the gap is reported loudly, because "a database that stopped because its backups stopped is an outage; a backup that is failing loudly is an incident" [V, `pgbackrest.conf`]; AKR-TMS keeps that behaviour and pages the operator | Backup heartbeat goes stale after two hours in the DBS [V] |
| Accidental deletion or bad migration | Point-in-time recovery to just before the event; for configuration, the previous pinned version | Release rollback scripts (the DBS rehearses them [V]) |
| Corruption of the audit chain or evidence | Restore, then verify the chain against the **anchors in the locked bucket** before trusting it (10.1) | The DBS restore script compares anchor keys and the chain after a restore [V, `backup-restore.md`] |
| Loss of the site (fire, flood, conflict damage, long power failure) | Promote the DR site; re-point names; run the post-restore checks; notify the CBI | Plans name the actors and the decision rule |
| **Ransomware** | Isolate; restore into a clean environment from an **immutable or offline** copy; verify against the anchors; rotate every credential and key (5.3); restore only after the entry path is closed | Object-lock repositories, append-only credentials and a monthly offline copy [K]; the DBS backups are not immutable (no object lock on the backup repository) [K] |
| OpenBao sealed or unseal holders unreachable | Documented alternate holders; transit auto-unseal from a second server where the bank has one; the phase-2 service keeps cached keys (file 32 section 13.4) | The DBS found that a Vault restored from a snapshot reseals itself a few seconds later and made the unseal step re-check [V] |
| **The CBS is restored to an earlier point or fails over with lost commits** | AKR-TMS detects the regression, halts watermark advance and re-reconciles (file 25 section 7.5, C-64) | An AKR-TMS copy can contain rows the CBS no longer has |
| Clock failure or jump after a power event | Refuse to sign or write watermarks until NTP is healthy; installer and daily self-check | Object-lock retention uses the store's clock (file 32 section 12.5) |
| Failed upgrade | Roll back to the previous bundle; data migrations are reversible or backed by a pre-upgrade snapshot | Bank change control (13.4) |
| Loss of the vendor | Source and build escrow (13.5); the bank holds signed bundles and the runbooks; no licence server or call-home that would stop the product | |
| Staff unavailability | Named deputies for the MLRO, CISO and unseal holders; roles never held by one person | 7.4 |

### 16.6 When AKR-TMS is down

| Function | Effect | Interim control | On recovery |
|---|---|---|---|
| Transaction monitoring (phase 1) | No new alerts; the CBS and customers are unaffected | The bank's existing controls (branch liaison reports, CBS reports); the MLRO is told when data is more than the freshness SLO behind | Backfill from the watermark; alerts for the gap are produced late, **flagged with the blind period and their creation time**, never back-dated (the CBI-facing explanation) |
| Case work and STR deadlines | MLRO workspace unavailable | Read-only workspace on the **DR standby**; goAML is the Office's own system, so an STR can still be filed from the bank's records [K] | Entries made manually are keyed in with their real times |
| Daily customer screening and list refresh | A day missed | The bank's other screening tool or manual checks on high-risk parties for the gap | Catch-up run; if more than 24 hours, a recorded compliance gap and notice to the MLRO |
| Freeze tasks and the 48-hour report clock | Tasks not tracked | Manual register kept by compliance | Tasks re-created from the audit trail |
| Phase-2 pre-release decisions | Per the bank's **fail policy**: cross-border and USD fail closed into a manual four-eyes queue; low-value domestic may fail open with mandatory retrospective screening (file 25 section 15.5) | Gateway circuit breaker, local fallback screen, weekly control test payment | Blind period in the coverage statement; priority retrospective review of anything released |
| Audit trail unavailable | **No state-changing action proceeds**: an action that cannot be audited is refused | | Resume when the audit path is healthy |

### 16.7 Backup, restore and testing

- **Backups**: the DBS scheme is the starting point [V, `docs/plan/backup-restore.md`]: pgBackRest with WAL every minute, a weekly full and nightly differentials, four fulls kept, AES-256 repository encryption with the passphrase held off the server; restic every 15 minutes for buckets, the OpenBao snapshot and the secrets, kept every pass for 48 hours and then daily for 31 days; a heartbeat that counts only when every part succeeded and the newest base backup is under 26 hours old; Redis (a cache; Valkey in AKR-TMS) is not backed up. AKR-TMS adds: a second repository at the DR site; **immutable or offline copies**; no long-term archive of personal data beyond the retention classes (12.3); and backup of the wrapped-key registry under its own retention (12.4).
- **Restore tests**: a monthly automated restore into a throwaway environment that ends with audit-chain verification and a row-count comparison; a quarterly failover drill; and a **yearly full DR exercise** at the DR site with documented scope, plan and results, which the CBI requires and an approved firm may audit (B9). The DBS builds the restore, rehearses it and records the measured time before describing it to management; AKR-TMS follows the same rule: no plan is documented as working until it has been run [V].
- **Chaos drills** in the quarterly test: OpenBao sealed, archive repository full, CBS link dropped, clock skewed, node killed during a batch, bundle import interrupted.
- **Power**: UPS and generator as the CBI requires; PostgreSQL with `fsync=on` and `archive_mode=on` as the DBS verifies [V]; a power-cut test in the installation acceptance.

### 16.8 AKR-TMS hooks

- **BIA worksheet and DR runbooks** in the delivery kit, with the clocks of 16.2 pre-filled.
- **Backup heartbeat, WAL-gap and restore-test status** on the cockpit; alert when any is stale.
- **Post-restore job**: chain verification against anchors, replay of deletion certificates (12.4), key and clock checks, reconciliation re-run.
- **Fail-policy and degraded-mode tests** in CI and in the quarterly drill (file 25 section 15).
- **DR-site profile** in the installer (second repository, replica, standby read-only workspace).

---

## 17. Tenant isolation if hosted

### 17.1 Default, and what the CBI text does to a hosted model

The default is **one installation per bank, in the bank's data centre** (file 32 section 17.1), which matches banking secrecy, the read-only link to a CBS on the bank's network, and the CBI's classification rules. Those rules decide the hosting question (2.8):

| Data | CBI storage rule (2.4) | Hosted by the vendor? |
|---|---|---|
| `public`, `internal` | Unrestricted or restricted | Yes |
| `personal`, `financial`, `card` (restricted) | May be stored on "authorised external" devices, including those of cloud service providers | Possible, if the bank and the CBI agree |
| `detection-logic`, `secret-STR` (confidential or above) | **"Only on the institution's own information assets and technologies"**; no remote access; top secret may not be stored on non-governmental cloud services | **Not under the text as written** |

Because suspicion information is confidential at least, and an alert, a case and the customer data it points to cannot be separated usefully, **a vendor-hosted multi-tenant AKR-TMS is blocked by the CBI classification rules until the CBI says otherwise in writing.** Three ways forward exist, in order of preference: (a) keep the installation on the bank's own assets and let the vendor operate it under the managed-service rules of 13.7; (b) a **bank-controlled private cloud** that the bank owns or controls even if the vendor's staff run it, which the bank's CISO may be able to classify as "the institution's own assets" (a question for the CBI, not for the vendor); (c) a hosted model with **keys held by the bank** (17.2), which still stores ciphertext on non-bank assets and needs the CBI's explicit non-objection. A "split" design that hosts only the restricted copies is rejected: every alert points back to them.

What can be hosted safely is a **content service** with no bank data: sanctions and other list distribution, typology packs and release bundles that banks pull from, or import by media where air-gapped. It carries no customer information and no inbound data from banks.

### 17.2 If a hosted model is ever approved: the isolation design

File 32 section 17.2 ranks the models: cluster per bank (only model to start with), namespace per bank (later), shared database with row-level security (rejected). The security controls that go with the first two:

| Layer | Control |
|---|---|
| Network | Separate network segments per bank; no east-west traffic between tenants; default-deny policies in both directions; per-tenant ingress and egress; the vendor's administration plane separate and reachable only by bank-approved sessions (13) |
| Compute | A K3s cluster per bank; where several banks share nodes (namespace model), dedicated node pools, taints, and a sandboxed container runtime such as gVisor or Kata Containers because containers share a kernel [K] |
| Data | A PostgreSQL cluster, object-store buckets, backup repositories and an OpenBao instance **per bank**, never a shared schema, bucket prefix or secret path; no shared cache or queue |
| Keys | Per-tenant KEK and data keys; the **root of trust held by the bank** (its HSM or a bank-run key service over mutual TLS), so the bank can revoke access cryptographically ("hold your own key"); the vendor holds no unseal share |
| Identity | A separate identity realm per bank; vendor staff in a separate directory with just-in-time elevation only on bank approval |
| Observability | Per-tenant log and metric streams; redaction as in 11.6; vendor operations staff see aggregate health, never content |
| Restore | Backups are restored only into the same tenant; restore into a different tenant is impossible without the bank's key |
| Capacity | Per-tenant quotas for CPU, memory, storage and input and output; rate limits at the edge |
| Cross-bank leakage | No shared name index, no shared analytics, no model trained across banks, no cross-bank "network" feature without a lawful framework (11.4) |
| Legal | Data residency inside Iraq; board-approved outsourcing arrangement, due diligence, contract and exit plan as the 2026 CBI controls require (2.6); notification to the CBI of the arrangement; escrow (13.5) |

### 17.3 Tests

**Cross-tenant tests** run in CI and yearly by an external tester: attempts to reach another tenant at the network, database, object-store, key and identity layers; **tenant canaries**, objects in each tenant that no other tenant's credentials may read, whose access raises a priority alert; a restore of tenant A's backup into tenant B's environment (must fail); a check that logs and metrics streams carry no content. The failure of any one test blocks hosting for all.

### 17.4 Hosting is a product decision with a regulatory precondition

The recommendation for the plan: **do not offer a hosted multi-tenant model in the first release**; document the isolation design above as a roadmap item; and write to the CBI Information Technology and Payments Department with a precise question (may a vendor-operated, bank-keyed installation in an Iraqi facility count as the bank's own assets for confidential data?) before investing in it.

### 17.5 AKR-TMS hooks

- **Isolation test suite** and the tenant-canary service, maintained from the first hosted pilot.
- **Per-tenant provisioning** scripts (cluster, database, buckets, keys, realm) that create everything from one definition and tear it down with a destruction certificate (12.7).
- **CBI position request template** and **hosted-readiness checklist** in the delivery kit.
- `tenant` limited to configuration; data separation by deployment (file 32 section 17.4).

---

## 18. AI and ML security (self-hosted models only)

### 18.1 Scope and the standing rule

AKR-TMS has three kinds of learned or generative components: tree-ensemble and graph-feature scorers for alert priority, customer risk and entity resolution (file 32 section 8, file 21); learned parameters in name matching (file 22); and optional, later, **language-model aids** for case summaries and narrative drafts (file 21 section 12, file 29). The standing rule is the DBS invariant: **document and case content never leaves the bank, and external AI services are prohibited** (`ARCHITECTURE.md` section 11 [V]); inference is self-hosted. Model governance (inventory, validation, drift, human oversight, fairness) is file 29; this section is the security view. The attack taxonomy is NIST AI 100-2 E2025 (March 2025: evasion, poisoning, privacy and abuse attacks, by lifecycle stage [V, NIST CSRC]); for language models the list is the OWASP Top 10 for LLM Applications 2025 (file 29 section on standards [V]).

### 18.2 Threats and controls

| Threat | AML-specific form | Controls |
|---|---|---|
| **Evasion** | A launderer learns to stay below rule thresholds or to look like the peer group; structuring tuned to a known score cut-off | Keep thresholds, features and model files in the confidential `detection-logic` class (11.2); combine rules, models and graph signals so one boundary does not decide; sample **threshold-adjacent** activity for review; drift and "pattern vanished" monitoring; a phase-2 response returns a decision and a reason code, **never a score**, and is rate-limited per client to make extraction by repeated queries expensive |
| **Poisoning through analyst feedback** | An insider closes alerts of a chosen pattern as false positives so a retrained model learns to ignore it (the insider case of file 20 in another form) | Labels carry the analyst, the QA status and the time; retraining uses labels only after quality filters; **sampled review of what the model suppressed** (file 29 design rule 7); the person who closes alerts cannot approve a retraining (maker-checker, 7.5); closure-pattern detection (9.3); a fixed **golden set** of independently confirmed cases that every candidate model must still catch before promotion |
| **Poisoning through data** | Manipulated CBS records or hostile free text shift features | Data-quality rules and reconciliation (file 25 section 8); training sets are versioned and hashed with the window and the query that made them; no training on raw free text |
| **Model theft and logic disclosure** | Someone copies the model file to learn what is detected | Model artifacts are `detection-logic`: encrypted at rest, signed, readable only by the scoring processes, loaded into memory, never served over a network port to users; extraction attempts through explanations are limited by role (the explanation payload for `secret-STR` cases follows the case ACL) |
| **Training-data exposure** | Pseudonymised extracts leave the production zone | Training runs inside the bank's perimeter on pseudonymised sets (11.7, file 32 section 8.2); the vendor trains only on synthetic data; extracts are deleted after the run (R12); no model is trained across banks |
| **Model supply chain** | A pre-trained model or library carries malicious content | Serving accepts only **signed ONNX** artifacts that pass an operator allow-list check; no pickle, joblib or other code-bearing formats in the runtime path (training-side Python formats are converted and discarded) [K]; third-party weights (OCR, embeddings, language models) are imported through the release bundle with hash, licence and provenance recorded and preferably in a tensor-only format such as safetensors [K]; the ONNX Runtime build is pinned and vendored, and because the `ort` crate is still a release candidate (file 32 section 8.3) its version is part of the SBOM and the pen-test scope |
| **Prompt injection (indirect)** | A name, remittance text, document or adverse-media snippet contains instructions to the model | Free text is untrusted **data** in delimited blocks, never instructions; the model has **no tools and no write path**; outputs are untrusted until a person accepts them (file 29 GV-49) |
| **Sensitive information disclosure** | A summary mixes in another case, or an answer reveals suspicion-tier facts to an unauthorised user | Retrieval is **permission-aware**: the model sees only the case the user may open (Cedar decision, tier, conflict check, 8.5); the output inherits the **highest tier of its inputs** and is stored as `secret-STR`; prompts and outputs are suspicion-tier records with the same access list and logging |
| **Excessive agency** | An agentic workflow changes a list, a threshold or a case | Agents may read and propose; they may not write to cases, lists or configuration; every action is allow-listed, policy-checked, timed out, logged and can be stopped by a kill switch (file 29 GV-49; OWASP LLM06) |
| **Improper output handling** | Generated text is rendered as HTML, contains links, or reaches the customer | Output rendered as plain text; no automatic links; **no path from any model output to a customer-facing channel** (file 29, confidentiality and tipping-off) |
| **Resource abuse** | Long prompts, loops or many users exhaust the GPU or CPU | Token and time limits, per-user quotas, a separate inference pool with queueing |
| **Prompt and template integrity** | A changed template alters behaviour silently | Each prompt template and model version is an inventory item changed by maker-checker; an Arabic and English evaluation suite runs before any change (file 29 GV-01, GV-50) |

### 18.3 The inference runtime

The language-model server (llama.cpp or vLLM class) runs in its **own zone**: no egress, a read-only model volume, a non-root user, resource limits, no shell, no access to the product database (it receives only the retrieved case context from the API), and redacted logs. Scoring models run in-process in `akr-worker` and `akr-assess`, with the model hash verified at load; a model that fails verification is refused and the process falls back to rules only, with a P1 alert (file 25 section 15.5, degraded mode 1). Every output records the model id, version, prompt-template version and an input hash, so a reviewer can reproduce what was shown (file 29 model cards).

### 18.4 Lifecycle controls

1. **Training in the bank's perimeter** with a recorded run: code hash, data hash, parameters, seed, environment.
2. **Registry and signing**: artifact SHA-256 verified on load and signed with an OpenBao transit key (file 32 section 8.4).
3. **Promotion** by maker-checker with the validation report, golden-set result and fairness proxy scan (file 29 GV-37).
4. **Shadow mode** before a new model influences ranking; staged rollout by segment.
5. **Kill switch** to rules-only per model and globally, with the same two-person rule as activation.
6. **Monitoring** of drift, score distributions, override rates, alert volume and fairness (file 29 GV-18), with a security reading: a sudden drop in alerts for one pattern or one analyst's closures is a possible poisoning or evasion signal and is routed to the second line.

### 18.5 What is not done

No external AI APIs; no federated or cross-bank learning at launch; no automatic retraining without human approval; no language-model decision on alert closure, holds, freezes or reports; no model trained on suspicion text outside the bank.

### 18.6 AKR-TMS hooks

- **Model loader** that verifies hash and signature, rejects disallowed operators and falls back to rules.
- **Golden-set regression job** and **label-integrity report** (closures by analyst and pattern, QA outcomes).
- **Permission-aware retrieval layer** for any language-model feature, with tier inheritance on outputs.
- **Inference-zone manifest** (network policy, volumes, limits) and its test.
- **Adversarial test pack** in the penetration-test kit: prompt-injection corpus (Arabic and English), extraction-by-query test, poisoned-feedback simulation.

---

## 19. Privacy of data subjects within AML duties

### 19.1 The tension, and the position taken

AML law requires a bank to collect, profile, share with authorities and keep personal data, and forbids telling the person (section 2.1). Privacy principles, used here as a benchmark (11.1), ask for transparency, accuracy, fairness and limits. Iraq has no personal-data law in force and a draft only (2.8); what binds the bank is banking secrecy (Law 94 of 2004, Article 49), the CBI's classification rules and Law 39. The position of AKR-TMS: **AML duties override access and erasure, but they do not excuse inaccuracy, unfairness, over-collection or leakage**, so the product limits what it collects (11), keeps it only as long as the clocks require (12), protects its users and its data (5 to 10), and gives the bank tools for fairness and for answering people without tipping them off.

### 19.2 Transparency without tipping-off

The bank's general privacy notice says that data is processed to meet AML/CFT, sanctions and regulatory duties and may be shared with the authorities; it never mentions a case. The delivery kit includes a **notice template** in Arabic, English and Kurdish. AKR-TMS has **no customer-facing surface**: no portal and no messages to customers; the only customer-visible text is the generic `customer_message_code` that the bank's own channel displays when a payment is held (file 25 section 15.3), and staff notifications are neutral (8.2). That removes a tipping-off path and an attack surface at once.

### 19.3 Access and correction requests

| Request | Handling |
|---|---|
| Access to one's data | Answered by the bank's own process after legal review. The product offers a **subject export** whose default response set is only what the CBS already shows the customer; it **excludes** suspicion-tier data, alerts, risk ratings and detection logic, and it never confirms or denies that any report, alert or case exists. Each request and response is entered in the disclosure register (11.10) |
| Correction | Source data is corrected **in the CBS** and flows in on the next load (AKR-TMS never edits CBS-origin data). A wrong **derived** fact (a false match, a wrong relationship link, a mistaken flag) is corrected by the case officer under maker-checker, with the history kept (bi-temporal) |
| "Who has looked at my data" | The bank answers with categories of staff and purposes, never names and never suspicion-tier access (9.2) |
| Erasure | Not available inside a legal clock (12.6); restriction of secondary use is |
| Complaint about a hold or an account restriction | Handled by the bank's complaint process, which cannot reveal an AML reason; the product records the decision, its reason code and the human who made it, so the bank can review it |

### 19.4 Accuracy and fairness as privacy duties

- **Data quality**: wrong data produces wrong alerts and wrong holds; the load-time quality rules and reconciliation (file 25 section 8) are therefore also privacy controls, and quality issues are reported to the bank as findings.
- **Harm from false matches**: a false sanctions match can freeze a person's account. The CBI defines three match states (potential, confirmed, false positive) and treats a match as confirmed when there is any suspicion or doubt; the AML/CFT Office's 2026 guide calls a match true only when sufficient indicators confirm identity, so file 28 (section 3.4) keeps a per-bank doubt policy. Freezing follows a true match immediately; the Committee gets a first report within 48 hours and a second within two months (Regulation No. 6 of 2023, which replaced the three days quoted in the CBI guidelines; files 23 and 28). The product keeps a *potential match* from freezing anything, records **who judged doubt and why**, queues freezes based on doubt for second review within a day, keeps the evidence of each clearance (so the same person is not re-held for the same false match), and gives the bank a fast, documented release path.
- **Common-name burden**: very common Iraqi names produce many false positives, and the burden falls on the people who carry them. Secondary identifiers (mother's name, birth year, family number) reduce it (file 22). The fairness monitor tracks the false-positive and hold rates by **name-frequency decile**, a measure that needs no inference about anyone's religion, sect or ethnicity.
- **Protected attributes**: religion, sect and ethnicity are never collected as features, names and tribe are never rating or scoring inputs, and the proxy scan at validation checks governorate, branch, occupation and name-derived features (file 29 GV-37; file 22 N-05; file 26 K-33). Nationality and governorate may appear only as documented regulatory risk factors in transparent rules or for fairness reporting, never as the sole reason for an alert or hold (file 29 design rule 2). Outcome rates by nationality, governorate, gender, age band and customer type are reviewed monthly (file 29 design rule 5).
- **Kinship and tribe in the graph**: family and tribal links can act as proxies for ethnicity or sect. Link types that reflect kinship are context for investigators and are **excluded from automated scores** unless the validator approves them with a documented rationale (file 21 section 6d). Graph edges come from evidenced relationships (shared account, address, device, ownership, signatory, declared and documented family relationship), **not from a shared surname or tribe name**.
- **De-risking guard**: a group exit or blanket restriction can be proposed only from case management and only with an individual assessment (file 29 design rule 6).

### 19.5 Automated decisions and human review

Models prioritise and recommend; they do not decide. **No adverse action** (a hold, a reject, a freeze, an account restriction, an exit) rests solely on a score: it needs a person's decision recorded with reason codes and the evidence. In phase 2 a **score can produce at most a HOLD** for human review. A **REJECT** is returned without a person only for deterministic conditions the bank's policy has configured as prohibited (for example an exact identifier match to a designated person, or a destination the CBI has barred), with the rule id in the reasons; every other reject follows a person's decision (file 25 section 15.3). This follows the GDPR Article 22 benchmark (a right not to be subject to a decision based solely on automated processing that significantly affects the person [V, EUR-Lex]) and the human-oversight design anchors in file 29. Explanations are stored with each recommendation (file 32 section 8.4) so a reviewer, the bank's complaints team and an assessor can see why.

### 19.6 Staff as data subjects

Employee-account monitoring, the `staff` entity and the platform's own user-behaviour rules (9) process staff data. The bank's staff policy states the purposes (AML duty, insider risk, security) and the monitoring; staff acknowledge it at first login (6.5); the results are visible only to the MLRO, the CISO and internal audit; access to staff data is itself logged; and the same retention applies. Whether Iraqi labour law adds limits on employee monitoring was not researched and is a legal-review item for each bank.

### 19.7 Record of processing, impact assessment and breaches

- **Record of processing**: the delivery kit generates, from the classification registry (11.11), a record listing each data class, its purpose (11.4), source, recipients (11.10), retention (12.3) and safeguards, for the bank's compliance function to sign. The ISO/IEC 27701 style is the model [K]; the GDPR's Articles 30 and 35 (records of processing; impact assessment) are the benchmark [K].
- **Privacy impact assessment** at deployment and at each major change (a new data source, a new model, a new export), kept with the assessor pack (15.5).
- **Breaches**: the CBI requires notification "as soon as possible" for any disaster and customer information for major incidents (2.4); no Iraqi personal-data-breach statute was found. The product supports an incident record (data classes, counts, containment, notifications sent) and the bank decides what it owes the CBI, the Office and customers; the suspicion tier is handled with the tipping-off rules in any customer communication.
- **Cross-border transfers**: none by default (on-premises, no call-home); vendor bundles are pseudonymised (11.7) and are disclosures to be registered (11.10); goAML submissions go to the Office in Iraq; correspondent requests are answered by the MLRO and logged.

### 19.8 AKR-TMS hooks

- **Subject-export tool** with the default response set and the disclosure-register entry.
- **Fairness monitor** panels: false-positive and hold rate by name-frequency decile, nationality, governorate, gender, age band; the ratio-to-overall trigger of file 29.
- **Match-state workflow**: potential, confirmed (with the "doubt" justification and second review), false positive (with reusable evidence), and the release path.
- **Graph edge allow-list** and a test that no edge type is derived from a shared surname or tribe.
- **Notice templates** and the record-of-processing generator.

---

## 20. What it means for AKR-TMS

### 20.1 In one paragraph

AKR-TMS will hold the most sensitive data in an Iraqi bank, in a country where disclosing a suspicion report is a crime for the individual, where the demonstrated threat is the banker who uses their position, and where the central bank has already written a detailed cyber rulebook that an assessor will apply line by line. The product should therefore be **secure by construction and provable by evidence**: an on-premises, single-tenant system with its own tiered access model for suspicion information, an audit trail whose integrity is verified end to end, encryption and key custody the vendor cannot reach, a vendor that has no standing access and ships only signed, bank-approved updates, and an assessor pack that is generated from the system rather than written for the visit.

### 20.2 The findings that shape the design

1. **Suspicion information needs its own access model.** Law 39 Articles 12 and 39 make tipping-off a criminal offence (up to three years), the sanctions guidelines extend it to freezes, and the CBI classification makes such information confidential at least. Ordinary role-based access is not enough: the product needs per-case access lists, existence masking, a relationship-conflict check, canary records, neutral wording and an inference test that fails the build (sections 7 and 8).
2. **The insider is the Iraqi threat.** OFAC's October 2025 designation of three Iraqi bankers and the enforcement record of altered thresholds, whitelists and falsified review evidence mean the platform must watch its own users (access events, behaviour rules), put every detection-affecting change through maker-checker with expiry, and have no administrator who can see everything (sections 7 and 9).
3. **The CBI cyber-resilience controls are the rulebook, and the product can map almost every item.** Three items need written exceptions in the bank's register: the application account on the CBS, DBA access to ciphertext through the bastion, and the weekly production control payment in phase 2 (sections 2.4 and 15.7).
4. **The CBI classification blocks a hosted multi-tenant AKR-TMS.** Confidential data may be stored "only on the institution's own information assets"; top-secret data may not go to non-governmental clouds. On-premises single-tenant stays the product; a hosted model waits for a written CBI position (section 17).
5. **Stay outside PCI DSS by design.** A card guard before landing, masked PANs and bank tokens, and never reading from a database that holds full card numbers keep the product out of the cardholder-data scope (sections 11.5 and 11.8).
6. **Retention has two shapes of clock.** Time-series data (transactions, alerts, decisions) is dropped by partition and crypto-shredded; customer-linked records follow the relationship-end clock and are purged record by record; evidence snapshots decouple cases from dropped partitions; legal holds are suspicion-tier and copy held rows out before any drop; and deletion is complete only when the last backup holding the key has expired (section 12).
7. **Audit integrity must be verified, not assumed.** The DBS audit found that its chain verifier ignored anchor signatures and the locked copy. AKR-TMS makes verification of the chain, signatures and locked copy part of the feature, and uses the locked anchors to prove a restore (sections 10 and 16).
8. **The release channel is the largest single point of leverage.** SolarWinds and the 2025 npm worm show the pattern. Signed bundles, an SBOM and VEX per release, two-party review, vendored reproducible builds, key custody under two-person control, no call-home and bank-approved updates are as important as any runtime control (sections 13 and 14).
9. **The vendor never needs to see data.** No standing access, a support role with no data permissions, pseudonymised bundles with a per-bundle key that stays with the bank, and a contract annex tied to the CBI's third-party items (section 13).
10. **AI follows the same rules.** Self-hosted only; signed models; poisoning through analyst feedback treated as an insider attack; no sect, tribe or ethnicity features; humans decide every adverse action (sections 18 and 19).
11. **Two kinds of data decide recovery.** Re-derivable CBS copies can be rebuilt from the source and tolerate a larger RPO; original records (decisions, notes, evidence, configuration, audit, keys) cannot, and carry the small RPO. An action that cannot be audited is refused (section 16).

### 20.3 The security backlog by phase

| Phase | Must be built | Sections |
|---|---|---|
| **P1 launch (monitoring)** | Identity and step-up; Cedar roles, tiers, case access lists, conflict check, break-glass; maker-checker with expiring whitelists; access events and the behaviour rule pack; the hash-chained, anchored audit trail with a full verifier; field encryption, key hierarchy and drills; classification registry, masking and the card guard; the retention engine with holds and certificates; support bundles and the vendor session model; signed bundles, SBOM, VEX, admission policy; backups, restore tests and the post-restore job; the assessor pack and posture page | 5 to 12, 13 to 16 |
| **P2 pre-release API** | Per-gateway client certificates and signed, expiring decisions; rate limits and oracle detection; fail-policy and degraded-mode tests; the control-test-payment exception; zero-RPO decision storage and the local fallback screen; sealed-OpenBao survival | 16, 18.2; file 25 section 15 |
| **Later** | Strict-mode defaults and HSM integration; FIPS-validated provider on request; language-model aids with the controls of section 18; ISO/IEC 27701 and SOC 2; the hosted model if and when the CBI allows | 3.3, 5, 17, 18 |

### 20.4 Decisions for the plan

1. **ISO/IEC 27001 before the first bank**: a certified vendor ISMS and an independent penetration test are market and assessor expectations; budget the time and the cost.
2. **Write to the CBI** (Information Technology and Payments Department) with two questions: the hosting question of 17.4, and where the assessment guide and the list of approved assessor firms (handbook B7, B9, D2) can be obtained.
3. **Default retention**: 10 years for sanctions-related records and cases, 7 years elsewhere (the Iraq profile floor from the reform guide; 5 years under Law 39), with a board risk acceptance to go below the OFAC default (12.3); confirm with each bank's compliance and legal teams.
4. **Strict encryption mode** (names and identifiers encrypted) as the default only for vendor-operated installs; a documented option elsewhere (5.2, 13.7).
5. **Hardware security module**: optional, but quote it in the larger profiles because bank security teams often ask for it [K].
6. **A named product security function** (security lead, penetration-test budget, vulnerability response) from the first release.
7. **Legal opinions** on the access-list design for suspicion information, on employee monitoring, on vendor liability and on source escrow.

### 20.5 Unverified points and gaps

- The CBI **assessment guide** and the list of **approved assessor firms** (handbook B7, B9, D2) were not retrieved (2.9).
- **PCI DSS v4.0.1** and **Swift CSP** text could not be downloaded (the sites returned an access page); the PAN-display, sensitive-authentication-data and scope statements are [K] and must be checked (3.1, 11.8).
- **Banking Law Articles 38 and 49** were not retrieved; they are quoted through the handbook and file 01. The CBI **ICT governance controls of 2019** were read at chapter level only (scanned, 74 pages). The CBI **e-payment AML controls (April 2025)** and the **digital-onboarding controls (2021)** were not read in detail.
- The **OFAC rule of 21 March 2025** is cited through file 05 [S]. The final text of **NIST SP 800-218 Rev. 1** was not checked (only the initial public draft of December 2025).
- Whether the DBS **WebAuthn** ceremony works in the browser was not verified (6.1); the DBS cockpit records a certificate-expiry value rather than probing it (5.1).
- **ISO/IEC 27001, 27002, 27701 and 22301** texts are paywalled and were not read; control numbers are [K].
- **Iraqi labour law** on employee monitoring and any **Iraqi data-protection bill** text were not researched (19.6).
- Patch-commitment times, retention defaults, RTO and RPO figures, the 10-minute step-up window and similar numbers are **proposals** of this research, not rules found in a source.
- The CBI's guides page (https://cbi.iq/page/99) lists sanctions-implementation and correspondent-bank guidance PDFs that are not covered in file 01.

---

## 21. Derived requirements

IDs use the prefix SP. "Source" gives the section of this file that explains the requirement and, where there is one, the external rule or document behind it; numbers such as the 10-minute step-up window, the retention defaults, the patch times and the RTO and RPO figures are proposals of this research (section 20.5).

| ID | Requirement | Source |
|---|---|---|
| SP-01 | Rank the protected assets (suspicion information, the CBS copy, detection logic, audit trail and evidence, screening decisions, keys, availability, models) and keep a threat model per release that starts from the adversary and threat tables, with the insider as the first adversary | 1.1 to 1.5; OFAC designation of three Iraqi bankers (file 05 section 1e); file 20 |
| SP-02 | Ship a control catalogue in the repository that maps each CBI control (cyber-resilience 2024, handbook 2025, risk-management 2026), ISO 27002 control and NIST function to a feature, a test and an evidence location, and generate the assessor documents from it | 2.4 to 2.6, 3.2, 3.4, 15.5 |
| SP-03 | Obtain vendor ISO/IEC 27001:2022 certification before the first bank delivery, with an independent penetration-test report, a documented SDLC and a vulnerability-disclosure policy; plan ISO 22301 alignment and an ASVS Level 2 report within the first year | 3.1, 3.3 |
| SP-04 | Enforce the zone model Z0 to Z7 with default-deny network policies in both directions, mutual TLS between services, no inbound path from the Internet, default-deny egress with an explicit allow-list, and a test that proves what each workload can and cannot reach | 4.1, 4.5; CBI network controls 3.1.x; DBS decision D6 |
| SP-05 | Limit direct database access to named DBAs through the bank's bastion; analysts, administrators and support reach the product database only through the application; use a separate database role per binary; record the CBS service account and DBA access as approved exceptions | 4.2, 15.7; CBI 12.1.3 |
| SP-06 | Use synthetic data only in development, test, demonstration and training; allow masked extracts in a bank's UAT only with CISO approval; restore backups only into production or a labelled, destroyed-after-use restore environment | 4.3; CBI 12.1.5, 14.2.3 to 14.2.9 |
| SP-07 | Allow administration only through the bank's PAM or jump host over SSH or VPN with MFA, named accounts and recorded sessions; no vendor VPN into production | 4.4; CBI 3.1.10, 8.4, 13.2.3 |
| SP-08 | Use TLS 1.2 or later (1.3 preferred) on every link, mutual TLS between services and from gateways, PostgreSQL `verify-full`, certificates of one year or less with automated renewal, and TLS on the OpenBao listener in every profile | 5.1 |
| SP-09 | Encrypt at rest on two layers: full-volume encryption on database, object-store, OpenBao and backup volumes, and field-level envelope encryption (AES-256-GCM data keys wrapped by an OpenBao transit key, cached in process memory, never in the request path to OpenBao) for suspicion information, notes, STR text, evidence and secrets; use keyed-HMAC blind indexes, with a separate key, for exact-match lookups | 5.2; CBI 12.3.6; DBS ARCHITECTURE section 10 |
| SP-10 | Offer a per-bank strict mode that moves names and identifiers into encrypted columns with an in-memory search index, as a documented setting with its cost; make it the default for vendor-operated installs | 5.2, 13.7 |
| SP-11 | Implement the key hierarchy (Shamir 3-of-5 unseal, transit KEK, data keys per partition-month or per tier, a separate match-key HMAC key, versioned Ed25519 signing keys), with the rotation schedule of 5.3, and never raise `min_decryption_version` past a version that signed evidence | 5.3; file 32 section 13.3 |
| SP-12 | Apply the custody rules: no key material in repositories, images or production environment files; keys never shared across banks; the vendor holds no bank key; dual control for unseal; a written compromise runbook; an optional HSM for the KEK or unseal shares; destruction logged | 5.3 |
| SP-13 | Put all cryptography behind one crate trait with an OpenBao implementation, an in-memory test implementation and room for a FIPS-validated provider; disable core dumps and swap for key-holding processes; show a key inventory and run quarterly sealed-state and key-compromise drills | 5.4, 5.2 |
| SP-14 | Require MFA for every user (WebAuthn preferred, TOTP baseline); provision identities by administrator; issue recovery by administrator; hash with argon2id; make the CBI password, lockout and session minima a floor that an administrator cannot weaken without a recorded exception | 6.1, 6.2, 6.6; CBI 8.2, 8.3.1, 14.1.4 |
| SP-15 | Set assurance per action: AAL2 for work lists; a fresh step-up within 10 minutes for restricted-tier and suspicion-tier opens; a single-use step-up assertion consumed in the action's own transaction for STR filing, closure without STR, hold clearance, change approval, export, role change and backfill | 6.3 |
| SP-16 | Give every machine client its own identity: a client certificate per gateway, scoped tokens with owner, expiry and last-used time, internal-PKI identities for services, no shared service accounts | 6.4 |
| SP-17 | Block sign-in during recorded leave and require case handover, enforce the CBI rotation rule, and disable accounts unused for 60 days pending review | 6.5; file 01 section 2d |
| SP-18 | Send sign-in anomaly rules (unusual hour or workstation, repeated step-up failure, access from outside the analyst zone) to the SIEM | 6.6, 10.4 |
| SP-19 | Run deny-by-default Cedar authorisation with RBAC roles and ABAC attributes (branch scope, customer tier, case access list, clearance, time window, purpose code, network origin, step-up freshness, conflict result, managed device); give no access by rank; audit every decision | 7.1, 7.3; CBI 8 intro |
| SP-20 | Seed the role catalogue of 7.2 and generate an access-matrix report; administrators, DBAs, operators and vendor staff never read business data | 7.2, 7.8 |
| SP-21 | Enforce segregation-of-duties pairs with Cedar `forbid` rules plus database constraints, and add tests that fail the build when a role gains both sides of a pair | 7.4, 7.8; CBI 8.3.4 |
| SP-22 | Use one maker-checker mechanism for every object that governs detection or reporting: maker cannot approve, fresh second factor, atomic activation with audit entry, emergency changes that revert after 24 hours without approval, second approver and backtest for volume-reducing changes, whitelists that expire | 7.5; file 32 section 11.4; file 20 |
| SP-23 | Provide branch scoping and the liaison window of CBI chapter 11: branch-limited fields, a confidential report channel to the MLRO that branch management cannot see, and the monthly report | 7.6; file 01 section 2d |
| SP-24 | Provide break-glass with a reason, two named approvers (compliance and security), a time box of four hours, CISO and MLRO notification, a dedicated log, review within 24 hours, and no export | 7.6 |
| SP-25 | Govern policy: Cedar bundles activated by change request, an audited simulator, dry-run before enforcement, a route-table coverage test in both directions, a quarterly access review and a yearly policy review | 7.7; CBI 8.1.8; DBS `authz_coverage.rs` |
| SP-26 | Implement three data tiers (normal, restricted customer, suspicion) with a per-case access list; handle the suspicion tier as confidential: internal network only, fresh step-up each session, time-boxed grants, export by approval, no electronic transfer of content | 8.3; CBI classification (2.4); Law 39 Articles 12, 39 |
| SP-27 | Make a refused open indistinguishable from "not found"; keep suspicion-tier cases and flags out of global search, type-ahead, counts and dashboards for other roles | 8.2 |
| SP-28 | Keep a register of access requests with the MLRO's prior written approval, automatic expiry, and visibility to the CISO and internal audit | 8.4; CBI top-secret and confidential rules |
| SP-29 | Run a relationship-conflict check on every open of a restricted or suspicion-tier record; deny, alert the MLRO and the CISO, and never explain to the user | 8.5 |
| SP-30 | Maintain canary suspicion-tier records excluded from all statistics and reports; treat any open, hit or export as a priority-one alert | 8.6 |
| SP-31 | Use neutral wording for tasks to bank operations, notifications, e-mail and customer messages (no reason, name, amount or case type); no WhatsApp or external push; write no AML reason into anything that reaches the CBS or the gateway; test the wording library for forbidden words | 8.2; file 25 sections 10.6, 15.7; file 32 section 11.5 |
| SP-32 | Control export, print and screenshots of suspicion-tier data with step-up, purpose, approval, per-user watermark and logging | 8.2 |
| SP-33 | Build a test suite that tries to infer the existence of a suspicion-tier case through search, counts, timing, errors, exports, notifications and the audit viewer, and fail the build if it succeeds | 8.9, 15.3 |
| SP-34 | Produce a monthly tipping-off control report for the MLRO and the CISO | 8.8 |
| SP-35 | Write an access event for every read of a customer, alert, case, transaction detail, evidence item or STR, every search hit on a restricted or suspicion-tier record, and every export, print and unmask; require a purpose code for those tiers; hash-chain and partition the events; log reads of the access log itself | 9.2 |
| SP-36 | Ship a platform user-behaviour rule pack that runs on AKR-TMS's own audit data (mass lookup, off-hours or off-network access, no work reason, snooping on staff or relatives, repeated denials, export spikes, privileged bursts, volume-reducing configuration edits, closure patterns, DBA anomalies, canary hits) and alerts the second line and security, never the actor's manager | 9.3 |
| SP-37 | Apply privileged-access rules: no standing administrator access to business data, just-in-time elevation through the bank's PAM, no shared privileged accounts, DBAs through the bastion with named accounts, OpenBao operators without application tokens, developers without production access | 9.4; CBI 8.4, 14.2.6 |
| SP-38 | Make closed cases append-only with evidence hashed at closure; run second-line QA sampling of alert closures; record vetting status on role grants and require it for suspicion-tier grants | 9.5; CBI 8.4.1, 13.2.2.1 |
| SP-39 | Keep an append-only, hash-chained audit trail (UPDATE and DELETE revoked from the application role, monthly partitions, per-partition chain) with Merkle roots signed by an OpenBao transit key every five minutes and appended to a COMPLIANCE object-locked bucket, read through a SELECT-only role | 10.1; DBS ARCHITECTURE section 8, migration 0019 |
| SP-40 | Make the chain verifier check the chain, the Merkle roots, the signatures against every key version and the existence and retention of the locked copy; run it on a schedule and on demand; treat a failure as a priority-one incident | 10.1; DBS `completion.md` JDI-01 |
| SP-41 | Define the audit event catalogue as code, test that every mutating route emits its event, carry a correlation id, keep personal data out of events and logs, and record UTC time from synchronised clocks | 10.2, 10.3 |
| SP-42 | Apply audit-the-auditor: log every viewer search and timeline with its filters, watermark every export with the requester, time stamp and correlation id, and cap export size | 10.3; DBS phase-16 |
| SP-43 | Export security events to the bank's SIEM as CEF or JSON over TLS syslog and through a signed pull API, with a use-case catalogue and no suspicion-tier content | 10.4; CBI Detect 1.4, 1.5 |
| SP-44 | Retain the audit trail without limit by default (cold tablespace for old partitions), give inspectors time-boxed read-only accounts, and support an optional RFC 3161 time stamp on anchors; extend anchor object-lock retention yearly | 10.5, 12.4; DBS decision D5 |
| SP-45 | Tag every column with a classification (public, internal, personal, financial, card, detection-logic, secret-STR), map tags to CBI levels that can only be raised, make derived data inherit the highest input tag, fail the build on an untagged column and generate the data dictionary from the registry | 11.2, 11.11; file 25 section 13.1 |
| SP-46 | Minimise at every stage: allow-list mapping with no `SELECT *`, a "mapped but unused" report, a blocked-attribute list (religion, sect, ethnicity, blood type, biometrics, ID images) that needs a legal basis and maker-checker to lift, and STR outputs limited to the Office's form fields | 11.3, 11.9; GDPR Articles 5 and 9 as benchmark; file 29 GV-37 |
| SP-47 | Enforce purpose limitation: purposes P1 to P6, a purpose code checked against the role, no secondary use (marketing, credit, HR), purpose lock on information from the Office or abroad, no cross-bank use, and tuning on pseudonymised sets inside the production zone | 11.4; Law 39 Articles 29, 53 |
| SP-48 | Run the card guard before landing (13 to 19 digit runs, Luhn, issuer ranges, bank exclusion patterns), mask to first six and last four, drop track data and security codes, raise a data-quality issue to the bank, sample text columns nightly, and fail the build on a card-like number in fixtures or logs | 11.5, 3.4; file 25 section 13.1 |
| SP-49 | Mask by surface: masked identifiers in lists, full values in an authorised case with an `unmask` event, masked and watermarked exports, no personal data in notifications, pseudonymous ids in logs and traces, generic error details | 11.6 |
| SP-50 | Pseudonymise with HMAC-SHA-256 under an OpenBao key per purpose, a fresh key per vendor bundle that stays with the bank, small-number suppression in aggregates (default cell size 5), and keep pseudonymised data at the classification and retention of its source | 11.7; GDPR Article 4(5), Recital 26 |
| SP-51 | Stay out of PCI DSS scope by design: never ingest, store, process or transmit a full PAN or sensitive authentication data; prefer the bank's token; never point ingestion at a database that holds full card numbers; record card-analytics exclusions in the coverage matrix | 11.8; PCI DSS v4.0.1 [K]; CBI 2.4 |
| SP-52 | Keep a disclosure register of every transfer of customer data out of the bank (goAML submissions, CBI, court and Office responses, correspondent answers, support bundles) with requester, legal basis, fields released and approver | 11.10; Banking Law Article 49 |
| SP-53 | Build one retention engine with classes R1 to R13, jurisdiction profiles (Iraq always on, OFAC and EU switchable), floors it will not go below, and a recorded board risk acceptance to go below the OFAC default | 12.1, 12.3; file 05 R-24; Law 39 Article 11 |
| SP-54 | Record in each bank's record-keeping policy that the CBS and the bank archive are the system of record for customer and transaction records, and freeze an evidence snapshot into the case when it opens | 12.2 |
| SP-55 | Run retention as a worker job with a dry run, maker-checker by the data owner and the CISO, a notice period (default 7 days), detach-then-drop with a 30-day undo, and a signed deletion certificate in the audit trail and the anchors bucket | 12.4 |
| SP-56 | Crypto-shred time-series partitions by destroying their per-partition-month key, state the backup-expiry date on the certificate, and keep no long-term archive backups of personal data | 12.4, 16.7 |
| SP-57 | Purge record by record for relationship-end and case clocks (master data, rating history, cases, STRs), deleting whole bi-temporal histories | 12.3, 12.4 |
| SP-58 | Lock evidence objects at the floor, extend retention at closure and for holds through the store's hold flag, and extend anchor retention yearly | 12.4; file 32 section 12 |
| SP-59 | Replay deletion certificates after every restore so deleted data is not resurrected | 12.4, 16.8 |
| SP-60 | Keep a legal-hold registry and a hold store: copy held rows under a per-hold key before shredding, treat the hold as suspicion-tier information, block purge and disposal, and review each hold periodically | 12.5 |
| SP-61 | Offer restriction, never erasure, inside a legal clock, and never edit CBS-origin data | 12.6; Law 39 Article 52 |
| SP-62 | Provide an end-of-contract export in open formats with the data dictionary and evidence manifests, and a destruction procedure with a signed certificate | 12.7; CBI 13.1.6 |
| SP-63 | Generate a retention report for assessors (clock, floor, default, oldest record, next purge, holds, certificates, backup-expiry horizon) | 12.8 |
| SP-64 | Grant the vendor no standing access: bank-opened, time-boxed sessions through the bank's PAM, a `support-readonly` role with no data permissions, recorded sessions, and no CBS credentials or bank keys at the vendor | 13.1, 13.2; CBI 8.4.2, 13.2.3 |
| SP-65 | Provide `akr-ctl support-bundle`: allow-list redaction, per-bundle pseudonym key, manifest reviewed by the bank, signature, size cap, entry in the disclosure register, deletion after 30 days | 13.3, 11.7, 11.10 |
| SP-66 | Deliver updates only as signed bundles with release notes, SBOM diff, test evidence and rollback plan; the bank approves and applies; no call-home, no auto-update, no remote push | 13.4; CBI 10.1.3, 10.1.6; 2026 outsourcing controls |
| SP-67 | Ship a contract annex mapped to CBI 13.1 and 13.2 and the 2026 outsourcing controls: policy compliance and liability, non-disclosure and secure deletion, vetting, MFA, access reviews, audit logs and audit rights, no change without approval, breach notice within 24 hours, escrow | 13.5 |
| SP-68 | Hold the vendor to a security baseline: certified ISMS, managed devices with MFA, HSM or KMS signing keys with two-person control, a ticketing system without personal data, a product security response team | 13.6 |
| SP-69 | Follow the NIST SSDF practice groups with a threat model per release and the practices mapped in 14.2 | 14.2; NIST SP 800-218 |
| SP-70 | Apply the stack coding rules: `unsafe_code` forbidden, `unwrap` and `expect` warned and denied in CI, release overflow checks, decimal money, SQLx bind parameters only, fuzzing and limits on untrusted parsers, no XML external entities, sandboxed expressions, strict CSP, bidirectional-control stripping, spreadsheet-injection guard, quarantined and scanned uploads, generic errors | 14.3; DBS `Cargo.toml` lints |
| SP-71 | Run the route-table coverage test (router versus table, table versus router, live traffic) and extend it to access-event coverage | 14.3, 9.6; DBS `authz_coverage.rs` |
| SP-72 | Protect source and build: two-party review with code owners, signed commits, secret scanning, ephemeral isolated builders, vendored dependencies with `--locked` and `--frozen-lockfile`, pinned toolchain and base-image digests; target SLSA Source L3 (L4 on sensitive paths) and Build L2, then L3 | 14.4; SLSA v1.2 |
| SP-73 | Sign images, charts and bundles with cosign under HSM or KMS keys with two-person control; give each bank the key fingerprint on a separate channel; enforce signatures by admission policy; verify bundles before import | 14.4; file 32 sections 16.3, TS-30 |
| SP-74 | Ship a signed CycloneDX 1.7 SBOM per release covering crates, frontend packages, images and models, with an SBOM diff and a VEX statement; embed dependency data in binaries | 14.5; file 32 section 16.4 |
| SP-75 | Govern dependencies: `cargo deny` and `cargo audit` gates, a record and second reviewer for every new dependency, minimum release age and install-script allow-listing for the frontend, and a waiver register with owner and expiry | 14.6 |
| SP-76 | Publish patch commitments (exploited: 48 hours advisory, 7 days fix; critical 7 and 14; high 14 and 30; medium 90 days), a quarterly cadence, support for the current and previous release, a private reporting channel, 90-day coordinated disclosure and a recorded root cause for each product vulnerability | 14.7; CBI 9.1 |
| SP-77 | Rebuild every release candidate on a second runner and compare digests; offer source and build escrow with bank-triggerable release conditions | 14.4, 13.5 |
| SP-78 | Run the assurance cycle: independent penetration test and source review before first delivery, yearly and on major releases; a deployment test per bank before go-live and yearly; a yearly assumed-breach exercise; yearly internal audit; a data-integrity test every 12 to 18 months | 15.1, 15.2; CBI Testing 1.1 to 1.4, 14.4.x, Detect 2.1.4; file 05 R-27 |
| SP-79 | Scope tests to what is specific to an AML platform: tipping-off inference, hostile CBS data, the CBS account and ingestion path, the phase-2 API (replay, forgery, downgrade, oracle), evidence, keys and audit, the platform and bundle verifier, insider paths, AI components, right-to-left spoofing | 15.3, 15.4 |
| SP-80 | Generate an assessor pack per release and per deployment, and an internal-audit guide of about thirty runnable control tests | 15.5 |
| SP-81 | Show a security posture page with a signed export (chain status, key ages, certificate expiry, release currency, read-only attestation, backups, seal state, canaries, access review, retention, drift) | 15.6 |
| SP-82 | Keep an exception register with the CBS service account, DBA access through the bastion, and the weekly phase-2 control test payment (CBI 14.2.9), each approved by the CISO | 15.7, 4.2 |
| SP-83 | Register AKR-TMS as a regulatory-critical system in the bank's BIA with the clocks of 16.2, and ship the proposed RTO and RPO table for each hardware profile for the bank to adjust | 16.2, 16.4; CBI Respond chapter |
| SP-84 | Back up with pgBackRest (WAL within 60 seconds, weekly full, nightly differential, four fulls), restic every 15 minutes for buckets, OpenBao snapshots and secrets, an encrypted repository with the passphrase held off the server, a second repository at the DR site, an immutable or offline copy, a heartbeat and a loud WAL-gap alarm | 16.7; DBS `backup-restore.md`, `pgbackrest.conf` |
| SP-85 | Test recovery: a monthly automated restore with chain verification, a quarterly failover drill with chaos cases, and a yearly full DR exercise at the DR site with documented scope, plan and results | 16.7; CBI Respond; handbook B9 |
| SP-86 | Run a post-restore job that verifies the chain against the anchors, replays deletion certificates, checks keys and clock, and re-runs reconciliation | 16.8, 10.1, 12.4 |
| SP-87 | Define AKR-TMS-down behaviour: record blind periods, flag late alerts with their creation time and never back-date them, refuse state changes that cannot be audited, apply the phase-2 fail policy, and provide a read-only workspace on the DR standby | 16.6; file 25 sections 8.2, 15.5, 15.6 |
| SP-88 | Plan for ransomware: isolate, restore into a clean environment from an immutable or offline copy, verify against the locked anchors, rotate every credential and key before service resumes | 16.5, 5.3 |
| SP-89 | Do not offer a hosted multi-tenant model in the first release; ask the CBI in writing whether a vendor-operated, bank-keyed installation in an Iraqi facility counts as the bank's own assets for confidential data; if approved, use a cluster per bank with bank-held root keys, cross-tenant tests and tenant canaries | 17.1 to 17.4; CBI classification (2.4); file 32 section 17 |
| SP-90 | Host only content services (lists, typology packs, release bundles) outside the bank, carrying no bank data and receiving none | 17.1 |
| SP-91 | Self-host all AI: signed ONNX artifacts only, an operator allow-list, no code-bearing model formats in the runtime path, hash verified at load with fallback to rules and a priority-one alert on failure, models classed as detection-logic | 18.1 to 18.3; DBS ARCHITECTURE section 11; file 32 section 8 |
| SP-92 | Defend against poisoning and evasion: label provenance, sampled review of what a model suppressed, retraining approved by someone other than the alert closers, a golden-set regression before promotion, versioned and hashed training sets, threshold-adjacent sampling | 18.2; NIST AI 100-2 E2025; file 29 design rule 7 |
| SP-93 | Return a decision and reason codes, never a score, from the phase-2 API, with per-client rate limits and anomaly detection on repeated assessments | 18.2; file 25 section 15.3 |
| SP-94 | Constrain language-model aids: untrusted text as data, no tools or write path, permission-aware retrieval, outputs inheriting the highest input tier, no path to a customer channel, a separate inference zone without egress, prompts and outputs logged as suspicion-tier records | 18.2, 18.3; file 29 GV-49; OWASP LLM Top 10 2025 |
| SP-95 | Provide a two-person kill switch that returns any model, or all models, to rules only | 18.4; file 29 |
| SP-96 | Keep AKR-TMS free of any customer-facing surface; the only customer-visible text is the generic message code the bank's own channel shows | 19.2 |
| SP-97 | Provide a subject-export tool whose default response set excludes suspicion-tier data, alerts, ratings and detection logic, never confirms or denies any report, and writes a disclosure-register entry | 19.3 |
| SP-98 | Implement the match-state workflow: a potential match freezes nothing; a confirmed match records who judged doubt and why and is queued for second review within a day; a false positive keeps reusable evidence; a fast release path exists | 19.4; CBI sanctions guidelines; file 25 section 15.7 |
| SP-99 | Run the fairness controls: protected-attribute register and proxy scan, monitoring by name-frequency decile and the other permitted groups, a graph edge allow-list with no edge derived from a shared surname or tribe, and the de-risking guard | 19.4; file 29 GV-37; file 21 section 6d; file 22 N-05 |
| SP-100 | Rest every adverse action on a recorded human decision with reason codes: a score alone produces at most a hold; reject without a person only for bank-configured deterministic prohibitions | 19.5; file 25 section 15.3; GDPR Article 22 as benchmark |
| SP-101 | Treat staff as data subjects: policy acknowledgement at first login, restricted visibility of staff-monitoring results, logged access to them | 19.6, 6.5 |
| SP-102 | Generate a record of processing from the classification registry, require a privacy impact assessment at deployment and at major change, and provide an incident record for personal-data breaches | 19.7 |

---

## 22. Sources

Evidence marks: **[V]** read in the document or repository during this research; **[S]** secondary source; **[K]** practitioner knowledge to be confirmed. Documents in Arabic were read from the PDF text layer or, for scanned files, page by page from rendered images; translations are ours.

### 22.1 Iraqi regulation and guidance (Central Bank of Iraq)

- Anti-Money Laundering and Counter-Terrorism Financing Law No. 39 of 2015, English translation on the CBI site [V]: https://cbi.iq/static/uploads/up/file-150199913642139.pdf
- CBI cyber-resilience controls for the financial and banking sector, first edition, June 2024 (Arabic, 48 pages) [V]: https://cbi.iq/static/uploads/up/file-171939658616261.pdf
- CBI guidelines on implementing sanctions, 1 August 2023 (Arabic, scanned) [V]: https://cbi.iq/static/uploads/up/file-169554063119385.pdf
- CBI standards handbook for private commercial and Islamic banks, 2025 (Arabic) [V]: https://cbi.iq/static/uploads/up/file-17567218515495.pdf
- CBI AML/CFT controls, 2025 (chapters 4, 11 and others, through file 01) [V]: https://cbi.iq/static/uploads/up/file-173666439114818.pdf
- CBI risk-management controls, 13 July 2026, commercial banks and Islamic banks editions [V]: https://cbi.iq/static/uploads/up/file-178480215427134.pdf and https://cbi.iq/static/uploads/up/file-178480220727035.pdf
- CBI guide on combating fraud and corruption for banks operating in Iraq, 2024 (scanned) [V, paraphrased]: https://cbi.iq/static/uploads/up/file-170530434494172.pdf
- CBI ICT governance controls, 25 April 2019 (74 pages, scanned; read at chapter level only) [V]: https://cbi.iq/static/uploads/up/file-155739550647757.pdf
- CBI business-continuity controls under ISO 22301, 22 April 2018 (scanned) [V]: https://cbi.iq/static/uploads/up/file-174695792048844.pdf
- CBI guides page (lists further guides, including sanctions implementation and correspondent-banking guidance not covered in file 01): https://cbi.iq/page/99
- CBI instructions and controls for banking supervision page: https://cbi.iq/page/139
- Not retrieved: Banking Law No. 94 of 2004 (Articles 38 and 49 quoted through the handbook and file 01); the CBI assessment guide referred to by handbook B7; the list of CBI-approved assessor firms.

### 22.2 International standards, regulations and primary sources fetched in this research

- EU General Data Protection Regulation (Articles 4(5), 5(1), 9(1), 22(1); Recitals 26 and 78) as a design benchmark [V]: https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=CELEX:32016R0679
- CISA alert AA20-352A, SolarWinds Orion supply-chain compromise [V]: https://www.cisa.gov/news-events/cybersecurity-advisories/aa20-352a
- CISA alert of 23 September 2025, widespread supply-chain compromise impacting npm (Shai-Hulud) [V]: https://www.cisa.gov/news-events/alerts/2025/09/23/widespread-supply-chain-compromise-impacting-npm-ecosystem
- NIST SP 800-218, Secure Software Development Framework, version 1.1 (February 2022) [V]: https://csrc.nist.gov/pubs/sp/800/218/final
- NIST SP 800-218 Rev. 1, SSDF version 1.2, initial public draft of 17 December 2025 [V]: https://csrc.nist.gov/pubs/sp/800/218/r1/ipd
- SLSA specification v1.2 (build track L1 to L3, source track L1 to L4) [V]: https://slsa.dev/spec/v1.2/ , https://slsa.dev/spec/v1.2/build-requirements , https://slsa.dev/spec/v1.2/source-requirements
- NIST AI 100-2 E2025, Adversarial Machine Learning: taxonomy and terminology (March 2025) [V]: https://csrc.nist.gov/pubs/ai/100/2/e2025/final
- OWASP Top 10 for LLM Applications 2025 (list taken from file 29) [S]: https://genai.owasp.org/llm-top-10/
- OWASP Application Security Verification Standard 5.0.0 (released 2025-05-30) [V, GitHub]: https://github.com/OWASP/ASVS
- CycloneDX specification (1.7.2 released 2026-09-17) [V, GitHub]: https://github.com/CycloneDX/specification
- SPDX 3.0.1 (2024-12-17) [V, GitHub]: https://github.com/spdx/spdx-3-model
- pnpm settings index (names of `minimumReleaseAge`, `trustPolicy`, `blockExoticSubdeps`, `strictDepBuilds`, `allowBuilds`; semantics [K]) [V]: https://pnpm.io/settings
- ISO/IEC 27002:2022 structure (four themes, March 2022) [V, Wikipedia]: https://en.wikipedia.org/wiki/ISO/IEC_27002
- ICS Financial Systems (ISO/IEC 27001:2022 certification announcement, 2025; also file 25) [V]: https://www.icsfs.com/
- OFAC record-keeping rule of 21 March 2025 (through file 05) [S]: https://www.govinfo.gov/content/pkg/FR-2025-03-21/pdf/2025-04864.pdf
- EU Anti-Money Laundering Regulation (EU) 2024/1624, Article 77(3) (through file 05) [S]: https://eur-lex.europa.eu/eli/reg/2024/1624/oj

### 22.3 Cited from knowledge or not read (to be checked)

- PCI DSS v4.0.1 and the PCI SSC scoping guidance [K; the site returned an access page]: https://www.pcisecuritystandards.org/
- Swift Customer Security Programme and the Customer Security Controls Framework [K; the pages returned 403]: https://www.swift.com/
- ISO/IEC 27001:2022, 27002:2022, 27005, 27017, 27018, 27701 and ISO 22301, 22317 (paywalled): https://www.iso.org/
- NIST SP 800-88 Rev. 1, guidelines for media sanitisation [K]: https://csrc.nist.gov/pubs/sp/800/88/r1/final
- NIST SP 800-57, SP 800-63B, Cybersecurity Framework, AI RMF 1.0 and AI 600-1 [K or through file 29]: https://csrc.nist.gov/
- RFC 9457, Problem Details for HTTP APIs [K]: https://www.rfc-editor.org/rfc/rfc9457
- Trojan Source, CVE-2021-42574 (bidirectional-control characters) [K]: https://nvd.nist.gov/vuln/detail/CVE-2021-42574 , https://trojansource.codes/
- OWASP Web Security Testing Guide [K]: https://owasp.org/www-project-web-security-testing-guide/
- Sigstore cosign, cargo-deny, RustSec advisory database (tool versions in file 32) [V in file 32]: https://github.com/sigstore/cosign , https://github.com/EmbarkStudios/cargo-deny , https://rustsec.org/

### 22.4 Local repositories and files read (read only)

Digital Banking System (`/home/haithem/Projects/digital-banking-system`; nothing there was changed):
- `docs/ARCHITECTURE.md` (sections 7 to 11), `docs/MODULES.md` (M13, M15), `docs/API.md`, `tech-stack.md`
- `docs/plan/backup-restore.md`, `docs/plan/phase-02.md`, `docs/plan/phase-10.md`, `docs/plan/phase-16.md`, `docs/plan/completion.md`, `docs/plan/research.md`, `docs/plan/time-leave-ledger.md`
- `docker-compose.yml` (object-lock bucket creation), `deploy/prod/pgbackrest.conf`, `deploy/prod/README.md`, `deploy/prod/Caddyfile`, `deploy/prod/vault.hcl`, `deploy/compose/kratos/kratos.yml`
- `backend/Cargo.toml` (workspace lints), `backend/api/tests/authz_coverage.rs`, `backend/api/src/middleware/authz.rs`, `backend/crates/audit/src/anchor.rs`, `backend/crates/common/src/objectstore.rs`, `backend/crates/domain/src/identity/mod.rs`
- `backend/migrations/0003_idempotency_byte_exact_replay.sql`, `0019_readonly_reporting_grants.sql`, `0024_idempotency_keys_scoped_to_actor.sql`
- `frontend/package.json`, `frontend/.npmrc`, `.claude/rules/auth.md`, `.claude/rules/rust-backend.md`, `docs/plan/phase-14.md` (no existence leakage)

Companion AKR-TMS research files: 01 (Iraqi regulatory framework), 05 (US, EU, UK, GCC regimes), 20 (enforcement failures), 21 (technology approaches), 22 (Arabic name matching), 25 (core banking integration), 26 (KYC and risk rating), 27 (monitoring scenarios), 29 (governance and model risk), 32 (tech-stack mapping). Files 23 (goAML), 28 (case workflow) and 31 (analyst UX) were not yet available and are referred to by planned scope.

### 22.5 Not reached

The WebSearch budget (about 200 searches) was used up earlier in the session, so the later sections rely on direct retrieval of documents and repositories. The pages listed as unreachable in 22.3 (PCI SSC, Swift) and the missing CBI documents in 22.1 are the main gaps (section 20.5).
