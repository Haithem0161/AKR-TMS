# MemberCheck notes 02 — Products: what they do
Status: done (2026-10-11)

## Summary

- Who: The brand is MemberCheck. Its About page says it "operates under Neurocom Group, established in 1992" and has built the platform since 2008 (https://membercheck.com/about-us/ [PRIMARY]). A directory listing gives a Chatswood, NSW head office (https://zoftwarehub.com/en-ae/products/membercheck/product-details [SECONDARY]). No founder or CEO is named on the vendor pages read, so the owner's "Mohammed" is not identified. The sister brand NameScan says it is "A member of the Neurocom group" (https://namescan.io/contact [PRIMARY]).
- Modules (Q1): The products page lists nine products: PEP and sanctions screening, customer identity verification, transaction monitoring, adverse media, KYB, EDD, customer and entity risk assessment, jurisdiction risk, and a white-label partner programme (https://membercheck.com/products/ [PRIMARY]).
- One check (Q2): The vendor describes this path: set thresholds, screen, list candidate matches, score and filter, then an analyst reviews the evidence and records a decision (https://membercheck.com/pep-sanction-screening/ [PRIMARY]). Identity checks return one of six outcomes (https://membercheck.com/id-verification/ [PRIMARY]). Risk levels and the full decision list are not stated.
- Data (Q1): The vendor says it "aggregates multiple premium risk sources" and adds its own monitoring, matching and workflow layer. It does not name the sources (https://membercheck.com/compare/membercheck-vs-complyadvantage/ [PRIMARY]). It claims 1,000+ watchlist sources, refreshed every 24 hours, covering 195 countries (https://membercheck.com/pep-sanction-screening/ [PRIMARY]).
- Batch and monitoring (Q3, Q4): Batch and real-time screening use the same lists (https://membercheck.com/pep-sanction-screening/ [PRIMARY]). Customers are re-screened automatically each day, with email alerts (https://membercheck.com/faq/ [PRIMARY]; https://membercheck.com/features-sets/ [PRIMARY]). Batch file formats, size limits and schedules are not stated.
- Users and languages (Q5, Q6): Unlimited users with pre-defined roles, one account for several organisations, and multi-factor authentication on every account (https://membercheck.com/faq/ [PRIMARY]; https://membercheck.com/compliance-as-a-business/ [PRIMARY]). Interface languages are English, French and Japanese, so Arabic is not on the list (https://membercheck.com/faq/ [PRIMARY]). Single sign-on is not stated.
- Reports (Q7): Exports to PDF, Word and Excel, and a stored record of each match (https://membercheck.com/faq/ [PRIMARY]; https://membercheck.com/pep-sanction-screening/ [PRIMARY]).
- Not offered or not stated (Q8): Payment or SWIFT message screening before release is not stated. goAML filing is not stated. The monitoring product offers "support" for suspicious reports, and filing itself is not described (https://membercheck.com/transaction-monitoring/ [PRIMARY]). On-premise is not stated for direct customers. "Local deployment for data sovereignty" appears for partners only (https://membercheck.com/compliance-as-a-business/ [PRIMARY]). Offered: configurable transaction monitoring rules and user-set customer risk ratings (https://membercheck.com/product/aml-risk-assessment/ [PRIMARY]).
- Iraq (owner's question): The vendor has an Iraq page covering screening, ID checks, jurisdiction risk and ongoing monitoring for Iraq-regulated firms, but it names no Iraqi list or customer (https://membercheck.com/aml-cft-legislation-in-iraq/ [PRIMARY]). Named deployments are Australia, Germany, Oman and Indonesia (https://membercheck.com/partnership/ [PRIMARY]). [INFERENCE] By the home page's own wording, Iraq would fall under the OCE "Rest of World" sign-in (https://membercheck.com/ [INFERENCE]).
- Conflicts to resolve: founding year (2008 versus 2007), integration time (under 24 hours versus three to four weeks), deployment geography, and cloud versus local hosting (https://membercheck.com/faq/ [PRIMARY]). Details are under Contradictions and surprises.

## Findings

### Q1. Modules as the vendor names them

- The vendor says it sells nine products on one platform: eight for screening and risk, and one partner programme. Quote: "One platform, nine ways to screen, verify, and monitor risk." Source: https://membercheck.com/products/ [PRIMARY]
- The vendor says all products share one customer record. Quote: "Every product shares the same customer record." Source: https://membercheck.com/products/ [PRIMARY]
- Module 1, PEP and Sanctions Screening. The vendor describes it as screening customers, entities and counterparties. Quote: "Screen customers, entities, and counterparties against global sanctions, PEP, watchlist, and enforcement data, with configurable matching and ongoing monitoring." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Module 1 scope, as listed on the page: individuals and sole traders, entities and organisations, beneficial owners and directors, counterparties and third parties, and the existing customer base. Quote: "Beneficial owners and directors" (the scope list item). Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Module 1, named sanctions lists. The vendor says it covers the major consolidated regimes and national lists. Quote: "Screening covers the major consolidated regimes including OFAC in the United States, the United Nations Security Council list, the European Union consolidated list and HM Treasury in the United Kingdom, alongside national lists and watchlists." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Module 1, source volume and refresh. The vendor says it screens over 1,000 sources, refreshed on a 24-hour cycle, covering 195 countries. Quote: "MemberCheck screens against over 1,000 sanctions, PEP, and watchlist sources, refreshed on a 24-hour cycle, covering 195 countries." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Module 1, matching. The vendor says fuzzy matching covers spelling variants, phonetic similarity and transliterations, and that secondary identifiers narrow the hits. Quote: "Fuzzy matching handles spelling variants, phonetic similarity and alternative transliterations, and secondary identifiers narrow what remains." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Module 2, Customer Identity Verification. The vendor says it checks identity against authoritative sources and returns an identity opinion. Quote: "Verify a customer's identity against authoritative sources, block impersonation with biometric liveness, and receive a clear identity opinion that feeds the same screening record used across the business." Source: https://membercheck.com/id-verification/ [PRIMARY]
- Module 2, document types and coverage limit. The vendor says it covers passports, driving licences and national ID cards. Quote: "with document and data coverage across 60+ countries, expanding to 120+." Source: https://membercheck.com/id-verification/ [PRIMARY]
- Module 2, outcomes. The vendor says each check returns one of six outcomes. Quote: "Every check returns a confidence-based outcome — verified, not verified, review required, pending, incomplete or not requested" Source: https://membercheck.com/id-verification/ [PRIMARY]
- Module 3, Transaction Monitoring. The vendor says it monitors activity against configurable rules and scenarios and supports suspicious matter reporting. Quote: "Monitor customer transactions and activity against configurable rules and scenarios, alert on suspicious patterns, escalate for review, and support suspicious matter reporting." Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]
- Module 3, input. The vendor says transactions are fed in through the API. Quote: "Feed customer transactions and activity into MemberCheck through the API." Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]
- Module 3, architecture. The vendor says monitoring has its own platform and sign-in. Quote: "Transaction monitoring runs on its own platform, with its own credentials, separate from the MemberCheck screening app." Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]
- Platform claims on the home page: the vendor says it operates in 195 countries with an audit trail. Quote: "One platform to screen, verify, and monitor the risk in who you do business with — in 195 countries, with an audit trail regulators trust." Source: https://membercheck.com/ [PRIMARY]
- Scale claims on the home page are marketing figures with no source given: 60M scans, 13,000+ customers, under one second per screen. Quote: "Built for the scale regulated teams operate at." Source: https://membercheck.com/ [PRIMARY]

- Module 4, Adverse Media Screening. The vendor says it screens names against news and media and groups the results by risk theme. Quote: "Screen customers and entities against negative news and adverse media from thousands of sources, with risk-categorised results, reduced noise, and ongoing monitoring." Source: https://membercheck.com/adverse-media-check/ [PRIMARY]
- Module 4, source count. The vendor says "thousands" of sources and does not give a number or name them. Quote: "Screen a name against thousands of global news and media sources in seconds." Source: https://membercheck.com/adverse-media-check/ [PRIMARY]
- Module 4, output grouping. Quote: "Results are relevance-scored and grouped by risk theme such as fraud, corruption, and organised crime." Source: https://membercheck.com/adverse-media-check/ [PRIMARY]
- Module 5, Know Your Business (KYB). The vendor says it verifies entities, maps ownership, finds ultimate beneficial owners (UBOs) and screens the entity and its owners. Quote: "Verify business entities, map ownership structures, identify ultimate beneficial owners, and screen the entity and its owners against sanctions, PEP, and adverse media data." Source: https://membercheck.com/knowyourbusiness/ [PRIMARY]
- Module 5, registry coverage. The vendor says it checks company registries in 130+ countries and states, using 300+ data sources. Quote: "MemberCheck verifies entities against company-registry data across 130+ countries and states, drawing on 300+ data sources, and screens against 1,000+ sanctions, PEP, and watchlist sources." Source: https://membercheck.com/knowyourbusiness/ [PRIMARY]
- Module 5, UBO threshold. The control threshold is set by the customer. Quote: "Identify ultimate beneficial owners above your control threshold and bring each named person into the same screening and verification process used for individuals." Source: https://membercheck.com/knowyourbusiness/ [PRIMARY]
- Module 6, Enhanced Due Diligence (EDD). The vendor says it runs deeper reviews on high-risk customers and PEPs. Quote: "Run deeper investigation on high-risk customers and PEPs, covering source of funds and wealth, adverse media, ownership, a documented risk rationale, and scheduled review." Source: https://membercheck.com/enhanced-due-diligence/ [PRIMARY]
- Module 6, triggers. The vendor says the user's own risk rules start EDD automatically. Quote: "EDD is triggered automatically from your risk rules whenever a customer meets a high-risk condition, such as being a PEP, sitting in a high-risk jurisdiction or sector, or holding a complex ownership structure." Source: https://membercheck.com/enhanced-due-diligence/ [PRIMARY]
- Module 7, Customer and Entity Risk Assessment. The vendor says it scores customers and entities with factors the user sets. Quote: "Score customer and entity risk using configurable factors across geography, product, channel, and customer type, producing a risk rating that drives the level of due diligence." Source: https://membercheck.com/product/aml-risk-assessment/ [PRIMARY]
- Module 7, model. The vendor says the rating follows the customer's own methodology, not a fixed template. Quote: "so the rating reflects your own risk model rather than a fixed template." Source: https://membercheck.com/product/aml-risk-assessment/ [PRIMARY]

- Module 8, Jurisdiction Risk Assessment. The vendor says it rates countries from FATF lists, sanctions regimes and corruption indices, and applies the rating to customer and transaction risk. Quote: "Assess country and jurisdiction risk using recognised indicators such as FATF lists, sanctions regimes, and corruption indices, and apply it to customer and transaction risk." Source: https://membercheck.com/jurisdiction-risk/ [PRIMARY]
- Module 8, rating levels. The vendor says each jurisdiction gets a risk level from its underlying indicators. Quote: "Assign each jurisdiction a risk level from the underlying indicators" Source: https://membercheck.com/jurisdiction-risk/ [PRIMARY]
- Module 9, Compliance as a Business (partner programme). The vendor says partners resell the platform under their own brand. Quote: "White-label MemberCheck's risk screening platform under your own brand, and turn your compliance expertise into a new revenue stream." Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- API, across modules. The vendor says the API lets screening run inside the customer's own systems. Quote: "Embed screening directly into onboarding and core systems through the MemberCheck API, so checks run inside your product." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- API reference is not published on the public page. The vendor says it leaves out base URLs, authentication, request and response shapes, error codes and rate limits. Quote: "the endpoint-level reference is not published here — base URLs, authentication, request and response shapes, error codes and rate limits." Source: https://membercheck.com/api-integration/ [PRIMARY]

- Data origin, as the vendor describes it. The vendor says it aggregates premium third-party risk sources, then adds its own monitoring, matching and workflow layer. Quote: "MemberCheck aggregates multiple premium risk sources and screens against more than 1,000 global watchlists, then adds its own monitoring, matching and workflow layer." Source: https://membercheck.com/compare/membercheck-vs-complyadvantage/ [PRIMARY]
- The vendor does not name the premium sources on any page read. The same comparison-table wording appears on each of the four provider comparison pages read. Quote: "Aggregates multiple premium risk sources and screens 1,000+ global watchlists across 195 countries" Source: https://membercheck.com/compare/membercheck-vs-world-check/ [PRIMARY]
- [INFERENCE] The sub-processor list names two data firms for "Data analysis": Acuris Risk Intelligence Limited (England) and Kyckr Ireland Ltd (Ireland). The page does not say what either supplies. Reasoning: aggregation from named third parties fits the list, but the link is not stated. Quote: "Acuris Risk Intelligence Limited England Data analysis" Source: https://membercheck.com/sub-processors/ [INFERENCE]
- Iraq coverage, as the vendor describes it. The Iraq page lists four coverage items for Iraq: adverse media checks, customer identity verification, jurisdiction risk checks and ongoing monitoring. Quote: "Screening coverage for Iraq" Source: https://membercheck.com/aml-cft-legislation-in-iraq/ [PRIMARY]
- Iraq support claim. The vendor says it helps Iraq-regulated firms by screening against sanctions, PEP and adverse media, verifying identities and monitoring risk. The page does not name Iraqi lists or Iraqi customers. Quote: "MemberCheck helps teams meet these obligations by screening customers and entities against global sanctions, PEP, and adverse-media data, verifying identities, and monitoring risk continuously, with a clear audit trail behind every decision." Source: https://membercheck.com/aml-cft-legislation-in-iraq/ [PRIMARY]

- Whitelisting (feature). The vendor says trusted entities can be whitelisted to cut false positives. Quote: "Reduce false positives by whitelisting trusted entities, so your team can focus resources on genuine risk." Source: https://membercheck.com/features-sets/ [PRIMARY]
- Custom watchlists and blacklists (feature). The vendor says users can build their own lists. Quote: "Build targeted lists to support your own AML/CTF compliance approach." Source: https://membercheck.com/features-sets/ [PRIMARY]
- Data management (feature). The vendor says client and transaction data can be stored, accessed and deleted on request. Quote: "Store and access client and transaction data securely, with the ability to delete it when required." Source: https://membercheck.com/features-sets/ [PRIMARY]

- Transaction monitoring has its own sign-in. The vendor says it runs on a separate platform and that a screening account does not sign the user in to it. Quote: "Open the monitoring platform (opens in a new tab)" Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]
- [INFERENCE] The monitoring link points to a host named eu.fraudshield.nexiant.ai, so the monitoring product is probably sold under another brand (FraudShield) on a Nexiant domain. The page does not name the product. Reasoning: only the host name is visible, and the link leads to a registration page that was not opened. Quote: "eu.fraudshield.nexiant.ai" Source: https://membercheck.com/transaction-monitoring/ [INFERENCE]
- The Japan version of the monitoring page says it is the same product as the global page. It lists sample scopes, including "Activity inconsistent with the customer profile". Quote: "Activity inconsistent with the customer profile" Source: https://membercheck.com/japan/products/transaction-monitoring/ [PRIMARY]
- Scenarios and thresholds are set per product, customer type and channel, according to the Japan monitoring page. Quote: "Rules, thresholds and scenarios are configured per product, customer type and channel." Source: https://membercheck.com/japan/products/transaction-monitoring/ [PRIMARY]
- Related brand, NameScan. The FAQ calls NameScan a sister company that sells pay-as-you-go packages. Quote: "Small and medium-sized businesses can use our sister company NameScan and its pay-as-you-go packages, sized to their need." Source: https://membercheck.com/faq/ [PRIMARY]
- NameScan's own contact page says it is part of the Neurocom group, and it lists a pay-as-you-go service. Quote: "A member of the Neurocom group" Source: https://namescan.io/contact [PRIMARY]
- Parent group. The About page says MemberCheck operates under Neurocom Group, established in 1992, which also includes NameScan, GPayments and Creative Digital Technology. Quote: "MemberCheck operates under Neurocom Group, established in 1992, which also includes NameScan, GPayments and Creative Digital Technology." Source: https://membercheck.com/about-us/ [PRIMARY]

### Q2. Path of one check from input to decision

- The home page says the platform runs one loop: screen, review, then monitor. Quote: "Screen, review, and monitor in one loop." Source: https://membercheck.com/ [PRIMARY]
- Step 1, screen. The vendor says the user sets rules, matching thresholds and customer context before a screen runs, and that screening runs in real time. Quote: "Set risk rules, matching thresholds, and customer context, then screen against global sanctions, PEP, and watchlist data in real time." Source: https://membercheck.com/ [PRIMARY]
- Step 1, input fields. The home page schematic (labelled "Schematic, not a screenshot") shows secondary identifiers as date of birth and country. Quote: "Date of birth, country" Source: https://membercheck.com/ [PRIMARY]
- Step 2, candidate matches. The PEP page shows that every name hit against a list becomes a candidate before any scoring. Quote: "Every name that touched a list, before any scoring is applied." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Step 2, scoring and filtering. The vendor says matches below the user's threshold are removed before review. Quote: "Weak coincidental matches drop out below your threshold." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Step 3, review. Matches go to an analyst queue with evidence, an assignee and a decision history. Quote: "Evidence, assignee and a decision history a regulator can inspect." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Decision values. The only decision labels found are in the home page schematic, where one screened subject shows the decision "Escalated for review" and a status "Recorded". Quote: "Escalated for review" Source: https://membercheck.com/ [PRIMARY]
- Evidence kept per match. The vendor says each match keeps its source, score and closing decision. Quote: "Every match carries its source, its score and the decision that closed it, which is what an inspection asks for." Source: https://membercheck.com/ [PRIMARY]
- Evidence kept per cleared match. The vendor says cleared matches keep their evidence so that tuning decisions can be defended later. Quote: "Every cleared match keeps its evidence, so a tuning decision is defensible later" Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Identity check, input and checks. The vendor says document details are checked against official sources and fraud signals are assessed at the same time. Quote: "Document authenticity, facial recognition, liveness and device signals are assessed at the same time." Source: https://membercheck.com/id-verification/ [PRIMARY]
- Identity check, what comes back. The vendor says the result is a stored identity opinion, not a raw match score. Quote: "A clear outcome returns in seconds, stored with its evidence for audit, for your team to act on." Source: https://membercheck.com/id-verification/ [PRIMARY]
- Identity check, speed. The vendor says most checks complete in seconds, with time varying by country and document. Quote: "Most identity verification checks complete in seconds, fast enough to run inside a live onboarding flow." Source: https://membercheck.com/id-verification/ [PRIMARY]
- Identity check, low confidence. The vendor says a low-confidence result becomes "review required", not an automatic decline. Quote: "A check that cannot be completed, or that returns a low-confidence result, comes back as an identity opinion of review required rather than a silent decline." Source: https://membercheck.com/id-verification/ [PRIMARY]
- KYB, first step. The vendor says it confirms the business exists against company-registry data. Quote: "Confirm the business exists and its registered details against company-registry data." Source: https://membercheck.com/knowyourbusiness/ [PRIMARY]
- KYB, owners. The vendor says the entity and every owner and director are screened on one record. Quote: "Screen the entity and every owner and director against sanctions, PEP, and adverse-media data on one record." Source: https://membercheck.com/knowyourbusiness/ [PRIMARY]
- Adverse media, evidence kept. The vendor says the article, link, date and analyst decision are stored against the customer. Quote: "Source articles, links, dates, and analyst decisions are stored against each customer, giving you a defensible record of what was found and how it was assessed." Source: https://membercheck.com/adverse-media-check/ [PRIMARY]
- Risk rating, output use. The vendor says the rating sets the due diligence and monitoring level. Quote: "The rating sets the level of due diligence and ongoing monitoring each customer needs." Source: https://membercheck.com/product/aml-risk-assessment/ [PRIMARY]
- EDD, last step. The vendor says EDD records the risk rationale and decision, then schedules the periodic review. Quote: "Record the risk rationale and decision, and schedule the periodic review the relationship requires." Source: https://membercheck.com/enhanced-due-diligence/ [PRIMARY]

- The About page says "cannot determine" is a first-class outcome for a match, not a form of "no match". Quote: "a first-class outcome and not a variant of" Source: https://membercheck.com/about-us/ [PRIMARY]
- The API guide tells the integrator to give reviewers three options: match, not a match, and cannot determine. The guide does not say the platform offers all three. Quote: "Match, not a match, and cannot determine" Source: https://membercheck.com/api-integration/ [PRIMARY]
- The FAQ says users can assign "risk levels and decisions" to matched profiles. The pages read do not name the risk levels or the full decision list. Quote: "assign risk levels and decisions to matched profiles" Source: https://membercheck.com/faq/ [PRIMARY]

- [SECONDARY] A G2 listing reportedly has a demo caption for "match decisions". The page returned HTTP 403, so this comes from a search summary and the decision list itself is not shown. Quote: not retrieved. Source: https://www.g2.com/products/membercheck/reviews [SECONDARY]

### Q3. Batch screening

- The vendor says batch and real-time screening use the same lists and the same matching logic. Quote: "Screen a single customer at onboarding through the API, or run your entire book in batch. Both paths use the same lists and matching logic." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- The vendor says a batch run covers every customer in the book. Quote: "Every customer against 1,000+ sanctions, PEP and watchlist sources." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- The vendor lists batch uses as re-screening the existing book, list-change sweeps and remediation. Quote: "Re-screening the existing book, list-change sweeps, remediation" Source: https://membercheck.com/api-integration/ [PRIMARY]
- The partner page lists batch file processing as a feature and says the volume decides between ad-hoc and batch scanning. Quote: "Run ad-hoc or batch scanning depending on the volume your customers need." Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- The vendor's integration guide warns that a batch can finish only part of the book without anyone noticing, and asks for a completion check. Quote: "A completion check, not just a job that ran" Source: https://membercheck.com/api-integration/ [PRIMARY]
- Scheduling: the FAQ says ongoing screening runs "through automatic daily screening." It does not give a schedule for user-run batches. Quote: "through automatic daily screening." Source: https://membercheck.com/faq/ [PRIMARY]
- File formats, size limits and batch scheduling options are not stated on the pages checked: https://membercheck.com/compliance-as-a-business/ and https://membercheck.com/api-integration/ [PRIMARY]. See Not found.

- The back-book page describes a batch re-screen of the existing book. Quote: "The whole in-scope population against current lists and current data" Source: https://membercheck.com/use-cases/back-book-remediation/ [PRIMARY]

- [SECONDARY] A directory listing names "Batch scan enhancements" among the product's features. This shows that batch scanning exists in the product, but it gives no file format or size. Quote: "Batch scan enhancements" Source: https://zoftwarehub.com/en-ae/products/membercheck/product-details [SECONDARY]
- [SECONDARY] A G2 listing reportedly has demo captions for batch scans. The G2 page returned HTTP 403, so this comes from a search summary and no page quote is available. Quote: not retrieved. Source: https://www.g2.com/products/membercheck/reviews [SECONDARY]

### Q4. Ongoing monitoring

- The vendor says customers are re-screened automatically when sanctions or PEP lists change. Quote: "Re-screen customers automatically as sanctions and PEP lists change, so a cleared customer is reviewed again the moment a new listing affects them." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- The vendor says the watchlists refresh every 24 hours. Quote: "refreshed on a 24-hour cycle" Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- The vendor says adverse media is re-screened as new coverage appears. Quote: "Customers are re-screened as new coverage appears, so adverse media that emerges after onboarding is flagged for review rather than missed." Source: https://membercheck.com/adverse-media-check/ [PRIMARY]
- The vendor says EDD customers can be put on a shorter review cycle. Quote: "Set higher-risk customers for more frequent periodic review, so EDD is refreshed on a schedule rather than left static after onboarding." Source: https://membercheck.com/enhanced-due-diligence/ [PRIMARY]
- The vendor says the risk rating sets how much monitoring each customer gets. Quote: "The rating sets the level of due diligence and ongoing monitoring each customer needs." Source: https://membercheck.com/product/aml-risk-assessment/ [PRIMARY]
- The home page says ongoing monitoring flags new or changed signals automatically. Quote: "ongoing monitoring that flags new or changed signals automatically" Source: https://membercheck.com/ [PRIMARY]
- The vendor says transactions are assessed as they occur, once they reach the API. Quote: "assesses them against your rules and scenarios as they occur" Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]

- The FAQ says customers get notifications of profile changes through automatic daily screening. Quote: "You receive notifications of profile changes, or new information affecting your customers, through automatic daily screening." Source: https://membercheck.com/faq/ [PRIMARY]
- The partner page says automated alerts flag high-risk customers when new information enters the database. Quote: "Automated alerts flag high-risk customers as new information enters the database." Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- The FAQ says sanctions changes usually reach the database within 24 hours. Quote: "In most cases within 24 hours of the information becoming public — typically overnight." Source: https://membercheck.com/faq/ [PRIMARY]
- The FAQ says changes to officeholders (PEP updates) usually take a few days. Quote: "Updates are usually made within a few days of the information becoming public, and in some cases as early as the next day." Source: https://membercheck.com/faq/ [PRIMARY]
- The FAQ says adverse news is added overnight. Quote: "New data is generally added overnight." Source: https://membercheck.com/faq/ [PRIMARY]
- Email is the alert channel named on the features page. Dashboard or API-callback alerts are not stated on the pages checked: https://membercheck.com/features-sets/ [PRIMARY]. See Not found.

- The features page lists email notifications and says they give "timely alerts for detected risks or suspicious activity." Quote: "Get timely alerts for detected risks or suspicious activity." Source: https://membercheck.com/features-sets/ [PRIMARY]
- The features page claims real-time detection of changes to a customer's status. This conflicts with the FAQ's "automatic daily screening" wording. Quote: "Real-time detection of changes to your customers' status through continuous activity monitoring." Source: https://membercheck.com/features-sets/ [PRIMARY]
- The perpetual KYC page lists the signals it monitors against the book, including ownership changes. Quote: "Watch List changes, ownership changes, adverse media and behavioural signals monitored against the standing book." Source: https://membercheck.com/use-cases/perpetual-kyc/ [PRIMARY]
- The perpetual KYC page calls ownership change "the highest-value trigger in the set". It does not say how a change is detected (which registries or filings). Quote: "That makes ownership change detection the highest-value trigger in the set" Source: https://membercheck.com/use-cases/perpetual-kyc/ [PRIMARY]

### Q5. User and organisation features

- The FAQ says the vendor offers unlimited multi-user access, with simple and advanced permission settings and pre-defined user roles. Quote: "We offer unlimited multi-user access, with simple and advanced permission settings and pre-defined user roles." Source: https://membercheck.com/faq/ [PRIMARY]
- The partner page says each white-label client organisation has its own users, settings and records. Quote: "Each client organisation is kept separate, with its own users, settings and records" Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- The partner page says an analyst at one client cannot see or change another client's book. Quote: "Role-based access means an analyst cannot see or alter another client's book" Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- The partner page says multi-factor authentication and encrypted personal data protect every account. Quote: "Multi-factor authentication and encrypted PII protect every account on the platform." Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- The EDD page says the case record includes an approval decision. No multi-step approval workflow is described on the pages read. Quote: "Record the findings, the analyst's assessment, and the approval decision, giving you a defensible rationale for onboarding or continuing a high-risk relationship." Source: https://membercheck.com/enhanced-due-diligence/ [PRIMARY]
- Sign-in is by region. The home page says the region is the one set when the account was created. Quote: "It is the region your account was set up in — your welcome email names it." Source: https://membercheck.com/ [PRIMARY]
- The home page describes the first sign-in region as covering "Australia and regions not listed below." Quote: "Australia and regions not listed below" Source: https://membercheck.com/ [PRIMARY]
- [INFERENCE] Iraq is not named on the home page under any of its four regions. The OCE region is labelled "Australia and regions not listed below", so by the page's own wording Iraq would fall under OCE. Reasoning: this reads only the page's region labels. Quote: "Australia and regions not listed below" Source: https://membercheck.com/ [INFERENCE]
- The FAQ says a production account is set up within 24 to 48 business hours after the subscription agreement is signed. Quote: "Once the MemberCheck Client Subscription Agreement is signed, our team can have your organisation set up with a production account within 24 to 48 business hours." Source: https://membercheck.com/faq/ [PRIMARY]
- The partner page says rebranding, custom domains and a multilingual interface are part of the white-label deal. Quote: "Rebranding, custom domains, and a multilingual interface are all part of the partnership, so your customers see your brand throughout." Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- Not stated for direct customers: sub-organisations or branches, multi-step approval, and single sign-on. Checked: https://membercheck.com/features-sets/ and https://membercheck.com/compliance-as-a-business/ [PRIMARY]. See Not found.

- The features page says one account can manage several organisations. Quote: "Manage compliance for multiple subsidiaries or client organisations from one account." Source: https://membercheck.com/features-sets/ [PRIMARY]
- The features page says multiple users are included at no extra cost. Quote: "Support collaborative compliance work across your team, at no additional cost." Source: https://membercheck.com/features-sets/ [PRIMARY]
- The enterprise comparison page says the licence is built around unlimited users and organisational structures. This is a pricing claim, not a feature list. Quote: "MemberCheck's model is built around unlimited users and organisational structures" Source: https://membercheck.com/compare/enterprise-aml-platforms/ [PRIMARY]
- The partnership page lists the countries where MemberCheck is deployed. Iraq is not named. Quote: "MemberCheck is deployed in Australia, Germany, Oman and Indonesia, with expansion planned into the Americas and South Africa." Source: https://membercheck.com/partnership/ [PRIMARY]

- [SECONDARY] The TrustRadius listing names "Multiple Users" and "Multiple Organizations" as features. Quote: "Multiple Organizations" Source: https://tr-web.dev.trustradius.com/products/membercheck/details [SECONDARY]
- [SECONDARY] A directory listing names "More granular user access controls" and "Compliance officer tools for account management" among the features. It gives no detail of the roles. Quote: "More granular user access controls" Source: https://zoftwarehub.com/en-ae/products/membercheck/product-details [SECONDARY]

### Q6. Interface languages

- The FAQ says the software is currently available in English, French and Japanese. Quote: "The software is currently available in English, French and Japanese, and its documentation in English and French, with Spanish to follow." Source: https://membercheck.com/faq/ [PRIMARY]
- [INFERENCE] Arabic is not on that list, so the page set shows no Arabic interface. Reasoning: the FAQ names the available languages and Arabic is not one of them. Quote: "currently available in English, French and Japanese" Source: https://membercheck.com/faq/ [INFERENCE]
- The partner page says a multilingual interface is part of the white-label deal. It does not name the languages. Quote: "Rebranding, custom domains, and a multilingual interface are all part of the partnership" Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- Arabic names are discussed for matching, not for the interface. The PEP page says names transliterated from Arabic need fuzzy matching. Quote: "Names transliterated from Arabic, Cyrillic, Chinese or Japanese scripts rarely match character for character, which is where a name-only check fails." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]

- The Japan page says Japanese names are kept in an "Original Script Name" field next to the romanised name. The page does not say whether Arabic script gets the same treatment. Quote: "MemberCheck screens a Japanese-script name in an Original Script Name field alongside the Romanised version" Source: https://membercheck.com/japan/ [PRIMARY]
- [SECONDARY] The TrustRadius listing says the platform supports multiple languages but names none. The fetch returned a shortened version of this sentence. Quote: "MemberCheck supports multiple languages, ensuring accessibility and ease of use for teams in different regions" Source: https://tr-web.dev.trustradius.com/products/membercheck/details [SECONDARY]

### Q7. Reports for auditors and regulators

- The FAQ says reports can be created and downloaded as PDF, Word and Excel. Quote: "Yes — you can create and download reports in PDF, Word, and Excel to meet your KYC obligations and audit requirements." Source: https://membercheck.com/faq/ [PRIMARY]
- The partner page says the same three formats. Quote: "Export results as PDF, Word, or Excel to meet KYC and audit requirements." Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]
- The vendor says each screening match keeps its source list, score, identifiers, assignee and closing decision. Quote: "Each match keeps its source list, its score, the identifiers used, the assignee and the decision that closed it, so the record answers the question without reconstruction." Source: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- The vendor says adverse media evidence (article, link, date, analyst decision) is stored against each customer. Quote: "Source articles, links, dates, and analyst decisions are stored against each customer, giving you a defensible record of what was found and how it was assessed." Source: https://membercheck.com/adverse-media-check/ [PRIMARY]
- The vendor says transaction monitoring compiles the material behind an alert to support a report to the regulator. Quote: "Compile the activity, findings, and decisions behind an alert to support suspicious matter or suspicious activity reporting to your regulator." Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]
- Report templates, scheduled reports and regulator-specific formats are not stated on the pages checked: https://membercheck.com/features-sets/ and https://membercheck.com/faq/ [PRIMARY]. See Not found.

- The features page says the platform generates detailed reports. Quote: "Generate detailed reports to demonstrate regulatory compliance and surface risk management insights." Source: https://membercheck.com/features-sets/ [PRIMARY]

### Q8. What it does not do

- Customer risk rating models. Status: offered, with user-set factors and weights. The vendor says the rating follows the customer's own model and no fixed template is used. Quote: "so the rating reflects your own risk model rather than a fixed template." Source: https://membercheck.com/product/aml-risk-assessment/ [PRIMARY]
- Transaction monitoring scenarios. Status: offered as configurable rules. The page does not say whether ready-made scenario sets are supplied. Quote: "Build monitoring rules and typologies around your products, customers, and risk appetite, from thresholds and velocity to structuring patterns, and adjust them as risks change." Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]
- Payment screening before release. Status: not stated. The API guide names "a payment about to be released" as a moment to call the API, but no page says the platform reads payment messages or holds a payment. Quote: "a payment about to be released" Source: https://membercheck.com/api-integration/ [PRIMARY]
- SWIFT message screening. Status: not stated. The FAQ explains which list SWIFT's sanctioned banks appear on, and does not say that messages are screened. Quote: "SWIFT doesn't maintain its own sanctions lists; however, it adds the banks and financial institutions it has sanctioned to SWIFTRef platforms such as OFAC's list." Source: https://membercheck.com/faq/ [PRIMARY]
- Suspicious transaction report filing to goAML. Status: not stated. The vendor's wording is "support suspicious matter reporting" and reports to "your regulator". Quote: "support suspicious matter reporting" Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]
- On-premise installation. Status: not stated for direct customers. The FAQ says client data is stored on "our cloud servers". The partner page lists "Local deployment for data sovereignty" with no detail. Quote: "clients retain full ownership of the data stored on our cloud servers." Source: https://membercheck.com/faq/ [PRIMARY]
- Partner-only local deployment. The partner page lists local deployment as a white-label feature, with no detail on what it means. Quote: "Local deployment for data sovereignty" Source: https://membercheck.com/compliance-as-a-business/ [PRIMARY]

- Licensing model. Status: partly stated. The partnership page offers a "Licensing" model for cases where API use "will not do", usually for data-residency or procurement reasons. It does not say the software is installed on the customer's own servers. Quote: "Licensing is for cases where that arrangement will not do, usually because of a data-residency or procurement constraint on your side." Source: https://membercheck.com/partnership/ [PRIMARY]
- [INFERENCE] On-premise status is unclear. Reasoning: the same page says that for technology partners "we run the platform", which suggests the licensee runs the software in the licensing case. The page does not say this in terms. Quote: "we run the platform, you call it" Source: https://membercheck.com/partnership/ [INFERENCE]
- Hosting providers. The sub-processor list names Amazon Web Services and Google LLC, both for "Cloud services" and both listed as United States companies. The list does not say where client data is stored. Quote: "Amazon Web Services Inc United States Cloud services" Source: https://membercheck.com/sub-processors/ [PRIMARY]
- Data location is left to the buyer. The API guide asks "Where is the data processed and stored?" and gives no answer on the page. Status: not stated. Quote: "Where is the data processed and stored?" Source: https://membercheck.com/api-integration/ [PRIMARY]
- Iraq and suspicious transaction reports. The Iraq page lists filing suspicious transaction reports with the FIU as an obligation of regulated firms. It does not list STR filing among MemberCheck's Iraq features. Status for Iraq: not claimed. Quote: "File suspicious transaction reports with the FIU" Source: https://membercheck.com/aml-cft-legislation-in-iraq/ [PRIMARY]

- [SECONDARY] A TrustRadius listing gives the deployment type as "SaaS". The fetch returned this term only, and the full page was not read. Quote: "SaaS" Source: https://tr-web.dev.trustradius.com/products/membercheck/details [SECONDARY]
- [SECONDARY] A Capterra review, dated November 2022 in a search summary, reportedly lists, as a drawback, an inability to connect to locally hosted, offline servers (a paraphrase of the search summary; no verbatim quote was retrieved). Capterra returned HTTP 403 to WebFetch and curl, so the review was not read on the page. Quote: not retrieved. Source: https://www.capterra.com/reviews/154148/MemberCheck [SECONDARY]
- [SECONDARY] A GetApp review snippet, seen in a search result, starts with "Member check doesn't support inte" and is cut off, so the full sentence was not confirmed. Quote: "Member check doesn't support inte" Source: GetApp listing, URL not confirmed (the lead's GetApp URL is https://www.getapp.com/finance-accounting-software/a/membercheck/) [SECONDARY]
- Suspicious transaction reports in Japan. The Japan monitoring page says the product compiles support material for a report. It does not say MemberCheck files the report. Quote: "The activity, findings and decisions behind an alert are compiled to support a suspicious transaction report to your sector's competent administrative authority." Source: https://membercheck.com/japan/products/transaction-monitoring/ [PRIMARY]

## Not found

- Help centre, user guide, knowledge base, release notes, changelog. Checked the downloaded vendor pages for these words and for links to them: none found. WebSearch "MemberCheck help centre user guide release notes API documentation membercheck.com" returned only unrelated products.
- API reference (endpoints, authentication, request and response shapes, error codes, rate limits). The vendor says it is not published on the page: https://membercheck.com/api-integration/. WebSearch "\"MemberCheck\" API docs developer endpoints screening" found only third-party listings. api.membercheck.com is not linked from any downloaded page, so it was not opened.
- YouTube channel or demo videos. No YouTube link on the downloaded vendor pages. WebSearch "MemberCheck AML YouTube channel demo video screening platform" found none. The vendor offers demos and walkthroughs on request only (https://membercheck.com/contact-us/). I did not submit the form.
- Batch file formats (input and output), file size limits, batch scheduling options. Not stated on https://membercheck.com/compliance-as-a-business/, https://membercheck.com/pep-sanction-screening/ or https://membercheck.com/api-integration/. Keyword search for CSV, upload and file found no input-file statement. Secondary only: "Batch scan enhancements" (zoftwarehub, see Q3).
- Single sign-on (SSO) and SAML. No mention in any downloaded vendor page (keyword search). Only multi-factor authentication is stated (https://membercheck.com/compliance-as-a-business/).
- Branch hierarchy for direct customers, and maker-checker or multi-step approval. Not stated. Only an "approval decision" on EDD cases (https://membercheck.com/enhanced-due-diligence/).
- Names of risk levels (for example low, medium, high) and the full list of match decisions. Not stated. The home page schematic shows "Escalated for review" and "Recorded". The About page names "cannot determine".
- Payment or SWIFT message screening (for example MT or ISO 20022 messages) and any hold before release. Not stated. Keyword search for swift found only the FAQ answer about SWIFT's sanctioned-bank list (https://membercheck.com/faq/).
- goAML, or any named FIU filing system. No match in the downloaded vendor pages (keyword search). The Iraq page names the Iraqi FIU but not goAML (https://membercheck.com/aml-cft-legislation-in-iraq/).
- On-premise or self-hosted installation for direct customers. No statement. Keyword search for on-prem, on-premise, premise, self-hosted and offline found only "Local deployment for data sovereignty" (partner page). Secondary sources are in Q8.
- Data location and hosting region. Not stated. https://membercheck.com/api-integration/ asks the buyer "Where is the data processed and stored?" and gives no answer.
- Native Arabic-script name matching. Not stated. The PEP page describes transliteration and phonetic matching only (https://membercheck.com/pep-sanction-screening/). Arabic interface: not offered on the FAQ's language list (https://membercheck.com/faq/).
- Iraq-specific lists, Iraqi customers, or an Iraqi bank. No Iraqi list is named on https://membercheck.com/aml-cft-legislation-in-iraq/. The client-base summary on https://membercheck.com/case-studies/ names Australia, Africa, Europe and the Caribbean. WebSearch "MemberCheck Iraq bank customer screening Baghdad" found no Iraqi bank. https://membercheck.com/country-coverage/iraq/ returned a "Page not found" page (HTTP 403 status).
- Named customer logos. The case-studies page says "the names below are live customers", but the names do not appear in the extracted page text (they may be logo images).
- Founder, CEO or owner called Mohammed. WebSearch "MemberCheck Neurocom Group founder CEO Mohammed" found nothing relevant. The About page (https://membercheck.com/about-us/) and the other downloaded vendor pages name no person (keyword search for founder, CEO and Mohammed).
- "Prototype". No product, feature or page named Prototype in the downloaded vendor text (keyword search).
- Names of the watchlist sources. The vendor names OFAC, UN, EU and HM Treasury (https://membercheck.com/pep-sanction-screening/) and "national lists and watchlists". The 1,000+ sources are not named anywhere read.
- Adverse media news sources. Not named. The FAQ says "There's no limit on the reputable news and government sites we access" (https://membercheck.com/faq/).
- Corruption and governance indices used by jurisdiction risk. Not named (https://membercheck.com/jurisdiction-risk/).
- Exact re-screen interval per customer. The FAQ says "automatic daily screening" (https://membercheck.com/faq/) and the features page says "Real-time detection of changes" (https://membercheck.com/features-sets/). The schedule is not stated more precisely.
- Capterra, GetApp and G2 review pages. Capterra (https://www.capterra.com/reviews/154148/MemberCheck) and GetApp (https://www.getapp.com/finance-accounting-software/a/membercheck/) returned a bot challenge page ("Just a moment...") with HTTP 403 to curl, and Capterra also returned HTTP 403 to WebFetch. The 2026 Wayback snapshot of the Capterra page returned HTTP 404. G2 (https://www.g2.com/products/membercheck/reviews) returned HTTP 403 to WebFetch.
- Monitoring platform features. The sign-up page behind https://eu.fraudshield.nexiant.ai/register?src=membercheck was not opened, because it is a registration form. The vendor's public pages do not describe the platform's own features beyond the monitoring page.
- Pricing. Out of scope. The comparison pages say "Unlimited users, with no per-seat fee" (for example https://membercheck.com/compare/membercheck-vs-world-check/), but no price is published.
- What "Local deployment for data sovereignty" means. Not stated (https://membercheck.com/compliance-as-a-business/).
- Role of the sub-processors. https://membercheck.com/sub-processors/ says Kyckr, Realyou, Acuris, ASIC and Bing do "Data analysis", but gives no detail of what each one does.

## Contradictions and surprises

- Founding year. The About page gives 2008. The lead's CB Insights note (not re-read here) gives 2007, and a directory listing says "Founded in 2007/2008" [SECONDARY]. Quote: "MemberCheck was established in 2008 to help organisations manage the risk in who they do business with." Source: https://membercheck.com/about-us/ [PRIMARY]
- Integration time. Product pages say the average integration time is under 24 hours, but the FAQ says a typical integration takes three to four weeks. Quote: "the average implementation takes between three and four weeks." Source: https://membercheck.com/faq/ [PRIMARY]. Product page figure: https://membercheck.com/pep-sanction-screening/ [PRIMARY]
- Product count. The products page says nine products, and the FAQ names four main products. Quote: "across four main products" Source: https://membercheck.com/faq/ [PRIMARY]. Nine products: https://membercheck.com/products/ [PRIMARY]
- One platform or two. The home page says "One platform", but the transaction monitoring page says monitoring has its own platform and sign-in. Quote: "Transaction monitoring runs on its own platform, with its own credentials, separate from the MemberCheck screening app." Source: https://membercheck.com/transaction-monitoring/ [PRIMARY]
- Monitoring speed. The features page says real-time, but the FAQ says customers are screened daily. Quote: "through automatic daily screening." Source: https://membercheck.com/faq/ [PRIMARY]. Real-time claim: https://membercheck.com/features-sets/ [PRIMARY]
- Coverage figures measure different things. Screening covers 195 countries, identity documents 60+ countries (expanding to 120+), and company registries 130+ countries and states. The FAQ says the solutions are "supported in every country." Quote: "MemberCheck's compliance solutions are supported in every country." Source: https://membercheck.com/faq/ [PRIMARY]
- Where the product is deployed. The partnership page lists Australia, Germany, Oman and Indonesia. The case-studies page lists Australia, Africa, Europe and the Caribbean. A directory listing says "Australia, the United States, China, and other global markets" [SECONDARY]. Quote: "MemberCheck is deployed in Australia, Germany, Oman and Indonesia" Source: https://membercheck.com/partnership/ [PRIMARY]. Case studies: https://membercheck.com/case-studies/ [PRIMARY]
- Cloud or local. The FAQ says client data is stored "on our cloud servers". The partner page lists "Local deployment for data sovereignty". Quote: "clients retain full ownership of the data stored on our cloud servers." Source: https://membercheck.com/faq/ [PRIMARY]
- Own database or aggregated sources. The FAQ calls the data "our database". The comparison pages say MemberCheck "aggregates multiple premium risk sources". Quote: "Our database contains more than 1,000 sanctions lists, PEP lists, and wanted/watched lists from 195+ countries." Source: https://membercheck.com/faq/ [PRIMARY]. Aggregation claim: https://membercheck.com/compare/membercheck-vs-complyadvantage/ [PRIMARY]
- Update speed differs by list. Officeholder changes take "a few days", but sanctions changes take 24 hours in most cases. Quote: "Updates are usually made within a few days of the information becoming public" Source: https://membercheck.com/faq/ [PRIMARY]
- Figures without a visible source. The About page says unverified figures are "either marked or removed", but the home page figures (60M scans, 13,000+ customers, 75% fewer false positives) carry no source or marker. Quote: "Where a proof figure has not been independently verified it is either marked or removed." Source: https://membercheck.com/about-us/ [PRIMARY]. Figures: https://membercheck.com/ [PRIMARY]
- Iraq page list versus text. The Iraq coverage list has adverse media, ID verification, jurisdiction risk and ongoing monitoring, with no PEP or sanctions item. The text says the platform screens against sanctions and PEP. Quote: "MemberCheck screens customers and entities against global sanctions, PEP, and adverse-media data" Source: https://membercheck.com/aml-cft-legislation-in-iraq/ [PRIMARY]
- Teams in Tokyo, Sydney and Ankara. The Japan page places a Tokyo office beside teams in Sydney and Ankara, which the lead's context did not mention. Quote: "alongside the teams in Sydney and Ankara" Source: https://membercheck.com/japan/ [PRIMARY]
- Review URLs. The lead's TrustRadius URL is a development host (tr-web.dev.trustradius.com). It loaded, but a public host was not checked. Quote: not applicable (host name only). Source: https://tr-web.dev.trustradius.com/products/membercheck/details [SECONDARY]
- Sub-processor page dates. The page says "Modified on: 25.07.2023" and also "Last updated 14 September 2026". Quote: "Modified on: 25.07.2023" Source: https://membercheck.com/sub-processors/ [PRIMARY]

## Sources

Vendor pages (MemberCheck), read or used:
https://membercheck.com/ [PRIMARY]
https://membercheck.com/products/ [PRIMARY]
https://membercheck.com/pep-sanction-screening/ [PRIMARY]
https://membercheck.com/id-verification/ [PRIMARY]
https://membercheck.com/transaction-monitoring/ [PRIMARY]
https://membercheck.com/adverse-media-check/ [PRIMARY]
https://membercheck.com/knowyourbusiness/ [PRIMARY]
https://membercheck.com/enhanced-due-diligence/ [PRIMARY]
https://membercheck.com/product/aml-risk-assessment/ [PRIMARY]
https://membercheck.com/jurisdiction-risk/ [PRIMARY]
https://membercheck.com/compliance-as-a-business/ [PRIMARY]
https://membercheck.com/api-integration/ [PRIMARY]
https://membercheck.com/features-sets/ [PRIMARY]
https://membercheck.com/country-coverage/ [PRIMARY]
https://membercheck.com/faq/ [PRIMARY]
https://membercheck.com/about-us/ [PRIMARY]
https://membercheck.com/partnership/ [PRIMARY]
https://membercheck.com/sub-processors/ [PRIMARY]
https://membercheck.com/use-cases/perpetual-kyc/ [PRIMARY]
https://membercheck.com/use-cases/back-book-remediation/ [PRIMARY]
https://membercheck.com/japan/ [PRIMARY]
https://membercheck.com/japan/products/transaction-monitoring/ [PRIMARY]
https://membercheck.com/compare/ [PRIMARY]
https://membercheck.com/compare/enterprise-aml-platforms/ [PRIMARY]
https://membercheck.com/compare/membercheck-vs-complyadvantage/ [PRIMARY]
https://membercheck.com/compare/membercheck-vs-world-check/ [PRIMARY]
https://membercheck.com/compare/membercheck-vs-first-aml/ [PRIMARY] (read; wording checked)
https://membercheck.com/compare/membercheck-vs-lexisnexis/ [PRIMARY] (read; wording checked)
https://membercheck.com/aml-cft-legislation-in-iraq/ [PRIMARY]
https://membercheck.com/case-studies/ [PRIMARY]
https://membercheck.com/contact-us/ [PRIMARY] (read; a demo form only, not submitted)

Vendor pages, not usable:
https://membercheck.com/country-coverage/iraq/ [PRIMARY] ("Page not found" page, HTTP 403 status)

Sister and parent company:
https://namescan.io/contact [PRIMARY]

Secondary sources:
https://tr-web.dev.trustradius.com/products/membercheck/details [SECONDARY] (read; development host)
https://zoftwarehub.com/en-ae/products/membercheck/product-details [SECONDARY] (read)
https://www.capterra.com/reviews/154148/MemberCheck [SECONDARY] (HTTP 403; search summary only)
https://www.getapp.com/finance-accounting-software/a/membercheck/ [SECONDARY] (HTTP 403; search snippet only)
https://www.g2.com/products/membercheck/reviews [SECONDARY] (HTTP 403; search summary only)

Lead's source, not re-read:
https://www.cbinsights.com/company/membercheck/ [SECONDARY]

Tried, not usable:
https://web.archive.org/web/2026/https://www.capterra.com/reviews/154148/MemberCheck (HTTP 404)

Seen, not opened (sign-up form, not used):
https://eu.fraudshield.nexiant.ai/register?src=membercheck [PRIMARY]

Searches (WebSearch, 7 of 15 used):
"MemberCheck review offline local servers integration AML screening"
"MemberCheck Neurocom Group founder CEO Mohammed"
"MemberCheck help centre user guide release notes API documentation membercheck.com"
"Neurocom Group NameScan GPayments MemberCheck Sydney company"
"\"MemberCheck\" API docs developer endpoints screening"
"MemberCheck AML YouTube channel demo video screening platform"
"MemberCheck Iraq bank customer screening Baghdad"
