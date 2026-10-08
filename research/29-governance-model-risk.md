# 29 — Governance and model risk for AML detection systems

Status: done (2026-10-06)

Requirement IDs use the prefix GV-xx (this file). Detection techniques themselves are in file 21 (prefix T). The US
model-risk guidance of April 2026 (SR 11-7 rescinded) and the FinCEN April 2026 programme proposal are in file 05
(sections 1a, 1c) and NYDFS Part 504 is in file 05 section 1f: this file cross-references them. Wolfsberg AI/ML principles
(2022), the Wolfsberg effective-monitoring papers (2024, 2025) and the Wolfsberg audit-for-effectiveness principles (2024)
are in file 04 (sections 2a, 2b, 2e, 2m). The CBI's own system requirements are in file 01 section 2d.

Evidence grades used where it matters: Primary (regulator publication, official text) / Secondary (reputable press,
law-firm or consultancy analysis) / Unverified (could not confirm; stated as such).

## Contents

1. Why governance is the product feature, not paperwork
2. United Kingdom: PRA SS1/23 and how UK banks apply it to AML; FCA and JMLSG on AI and monitoring
3. European Union: the AI Act (is AML detection high-risk?), the digital omnibus delay, AMLR/AMLA, EBA, GDPR Article 22
4. Singapore: MAS FEAT, Veritas, the 2024 AI model risk information paper and the 2025 AI risk management guidelines
5. Hong Kong: HKMA guidance on AI, GenAI and AI in suspicious-activity monitoring
6. United States, briefly (cross-reference to file 05)
7. Gulf and Arab regulators (CBUAE, SAMA, CBJ, others) and the CBI
8. Standards that give auditors something to test against (NIST AI RMF, ISO/IEC 42001, OWASP LLM)
9. Design guidance: the AKR-TMS governance blueprint
   9a. Model and rule inventory
   9b. Tiering
   9c. Validation (conceptual soundness, data, outcomes; above- and below-the-line testing; threshold-tuning methods)
   9d. Ongoing monitoring and drift
   9e. Champion-challenger and shadow running
   9f. Change management
   9g. Documentation and model cards
   9h. Independent testing and audit; three lines of defence
   9i. Bias and fairness (nationality-based rules, name-derived proxies)
   9j. Explainability and human oversight
   9k. GenAI governance (prompt and output logging, hallucination controls, no autonomous decisions)
10. What it means for AKR-TMS
11. Derived requirements (GV-xx)
12. Sources

## 1. Why governance is the product feature, not paperwork

Three facts from the earlier files set the frame for this one.

1. **The CBI writes AI/ML into the rule, and writes a stop-and-clear control next to it.** The CBI's AML/CFT supervisory
   controls of 8 January 2025 (chapter 5, article 5; file 01 section 2d) require every bank's AML system to use "AI and
   machine-learning algorithms to analyse data and identify possibly suspicious transactions" and to stop a transaction
   that matches a suspicion scenario "until the MLRO verifies it is sound". A system that holds payments on a model's
   say-so is a system whose errors cost customers money and cost the bank complaints, so the bank needs to show it
   controls the model.
2. **The CBI has no published model-risk rule of its own** (file 05 section 1c states this; section 7 below re-checks it
   for AI and model risk specifically). The evidence of control that Iraqi banks will be asked for therefore comes from
   four other places: the CBI-approved assurance firm that verifies the AML tooling (CBI standards handbook 2025, file
   01 section 2e); the bank's internal and external auditors; US, EU and Gulf correspondents (the Wolfsberg CBDDQ asks
   about tuning, testing and validation, file 04 section 2c); and the CBI's own on-site examiners using the 2025
   controls. Foreign model-risk standards (PRA SS1/23, US SR 11-7 and its April 2026 successor, MAS, HKMA) are the
   vocabulary all four will use.
3. **Governance is where AI in AML has actually been held back.** The Wolfsberg Group says prudential-style model risk
   management "is a major barrier" and that financial-crime models should not sit in the same tier as prudential models
   (file 04 section 2b); the FATF says explainability "is the key to supervisory acceptance" (file 03 section 5a). A
   product that ships governance artefacts (inventory, tiering, validation pack, drift reports, change log, model
   cards) removes the barrier for the buyer. A product that does not will be rejected by the first validator who asks
   for them.

The product therefore needs three governance tracks (this follows file 05 section 1c): **(A) rules and thresholds** as
governed configuration; **(B) models** (supervised scores, anomaly detection, entity-resolution and name-match scoring)
under model risk management; **(C) generative AI** under human-review and logging controls. Sections 2-8 collect what the
regulators and standard-setters say; section 9 turns it into a blueprint.

## 2. United Kingdom: PRA SS1/23 and how UK banks apply it to AML; FCA and JMLSG on AI and monitoring

### 2a. PRA SS1/23 "Model risk management principles for banks" (May 2023; in force 17 May 2024)

Source: Bank of England PDF, 30 pages, read in full on 2026-10-06 (published with Policy Statement PS6/23). Primary.

**Status and scope.** "The policy comes into effect on Friday 17 May 2024" (para 1.5). Firms had to run a self-assessment
before that date and update it "at least annually thereafter", with findings and remediation plans shared with the board
(paras 1.6-1.7). The statement applies to UK-incorporated banks, building societies and PRA-designated investment firms
"with internal model approval to calculate regulatory capital requirements" (IRB credit risk, internal market-risk and
counterparty-credit models); it does not apply to firms without that permission or to third-country branches, "however,
the PRA considers that those firms may find the proposed principles useful" (para 1.2). Footnote 3: "Vendors and external
consultants may find this supervisory statement useful as it sets out the PRA's minimum expectations for firms' own MRM
frameworks." The PRA is a prudential supervisor: **the words "financial crime", "money laundering" and "AML" do not appear
anywhere in the text** (checked by search of the extracted text). The FCA, not the PRA, supervises AML in the UK. SS1/23
reaches AML models only by the route of each firm's own policy (section 2b).

**Definition of a model (principle 1.1).** "A model is a quantitative method, system, or approach that applies statistical,
economic, financial, or mathematical theories, techniques, and assumptions to process input data into output. The
definition of a model includes input data that are quantitative and / or qualitative in nature or expert judgement-based,
and output that are quantitative or qualitative." Two clauses matter for rules:

- 1.1(b): "where material deterministic quantitative methods such as decision-based rules or algorithms that are not
  classified as a model have a material bearing on business decisions and are complex in nature, firms should consider
  whether to apply the relevant aspects of the MRM framework to these methods."
- 1.1(c): the PRA "expects the implementation and use of deterministic quantitative methods not classified as models to
  be subject to sound and clearly documented management controls." Paragraph 2.5 explains why: deterministic methods are
  becoming "vastly more complex and statistically orientated".

So under the UK text a simple threshold rule is not a model, but a large, complex rule library that drives decisions
needs documented management controls and may need MRM treatment. The US April 2026 guidance goes further and excludes
deterministic rules from "model" entirely (file 05 section 1c).

**The five principles and the parts that bite on an AML detection system**

| Principle | Text (short) | Sub-principles that matter for AKR-TMS |
|---|---|---|
| 1. Model identification and risk classification | A definition of a model, an inventory and "a risk-based tiering approach" | 1.2 inventory must record purpose and use "with a comparison to its actual use", the "model operating boundaries" under which performance is acceptable, assumptions and limitations, validation findings, owners and dates, and be firm-wide to expose "direct and indirect model inter-dependencies". 1.3 tiering by **materiality** (size measures such as "number of customers to which a model applies"; qualitative purpose and importance) and **complexity** (data quality, methodology, implementation, frequency of use; for new approaches also "the use of alternative and unstructured data" and "measures of a model's interpretability, explainability, transparency, and the potential for designer or data bias"). The tiering approach is itself validated periodically (1.3(d)). |
| 2. Governance | Board sets "model risk appetite"; approves policy; names one accountable senior manager | 2.1(c): the appetite includes "thresholds for acceptable model performance and tolerance for errors". 2.1(e): the board "provides challenge to the outputs of the most material models". 2.2: one accountable Senior Management Function. 2.4: documented owner, user and developer for every model; owners are accountable that "the model's performance is monitored against the firm's board-approved risk appetite". 2.5: Internal Audit assesses the whole framework. **2.6 vendor models**: the firm must "satisfy themselves that the vendor models have been validated to the same standards as their own", "verify the relevance of vendor supplied data and their assumptions", and "validate their own use of vendor products and conduct ongoing monitoring and outcomes analysis of vendor model performance using their own outcomes". |
| 3. Development, implementation and use | Standards for design, selection, performance measurement; test data, construct, assumptions, outcomes | 3.1(b): technique "supported by published research ... or generally accepted industry practice"; compare against "alternative theories or approaches and benchmarks". 3.2: data "representative of the underlying ... customer base"; "no inappropriate bias in the data"; if not representative "the potential limitation should be taken into account in the model's tier classification"; proxies "clearly documented and subject to validation". 3.3: backward- and forward-looking tests, **sensitivity analysis** to find the "operating boundaries", comparison with **challenger models**; **dynamic models** ("able to adapt, recalibrate, or otherwise change autonomously") need testing at each material change with "parallel outcomes analysis". 3.4: model adjustments and expert judgement recorded and justified. 3.5 documentation "sufficiently detailed so that an independent third party ... would be able to understand how the model operates, to identify the key model assumptions and limitations, and to replicate any parameter estimation and model results", and for vendor models "sufficient to validate the firm's use of the model". 3.6 supporting systems under change control. |
| 4. Independent validation | "ongoing, independent, and effective challenge" | 4.2 independent review covers inputs, calculations, outputs; assesses conceptual soundness; critically analyses development evidence; additional testing. 4.3 **process verification** (inputs representative and compliant with data-quality standards; code accurate and auditable; reports complete). 4.4 **performance monitoring** against the thresholds set at development, with "benchmarking", "sensitivity testing", "analysis of overrides" and "parallel outcomes analysis", at a frequency set by tier, with reports "independently reviewed". 4.5 periodic revalidation by tier. |
| 5. Model risk mitigants | Policies for when models under-perform; review of post-model adjustments | 5.1 post-model adjustments (overlays and overrides) documented, approved, shown "with and without", independently reviewed, and a trend of recurring adjustments "indicative of flawed model design". 5.2 restrictions on model use after significant deficiencies. 5.3 exceptions and escalation: a model that "displays persistent breach of performance metrics continues to be used" is an exception that must escalate. |

Proportionality (3.3-3.4): "for firms with a smaller number of models or less complex models, maintaining a model inventory
should be less burdensome, and the criteria for classifying models into tiers can be materially simpler." Rigour of
validation, monitoring and revalidation is "commensurate with the associated model tier".

### 2b. How UK banks apply SS1/23 to AML models, and what the FCA says about AI

- **SS1/23 does not name AML**, and the PRA is not the AML supervisor (section 2a). Its reach into financial-crime models
  is therefore a choice made inside each bank's model-risk policy, which usually defines "model" broadly and puts the
  transaction monitoring, screening and customer-risk models in the same inventory as credit models.
- **UK Finance blog, 20 February 2026** ("Beyond compliance: why and how financial crime models fit within your model risk
  framework"; authors Aleksandra Bojarzyn, Ashley Lai, Rachel Olson and Alisheik Akbar Ali; the page blocks automated
  download, so the content below comes from the fetch tool's summary and from a press echo dated 24 February 2026 and is
  graded Secondary). Its four recommendations: (1) integrate financial crime models into the MRM framework (categorise
  them in the inventory, define documentation standards, assign model owners, clarify intended use); (2) "define a distinct
  financial crime model family by identifying and mapping relevant models and apply tailored validation approach"; (3)
  "avoid governance processes that slow down innovation without adding meaningful value or risk coverage and apply a
  streamlined, proportionate approach"; (4) "advanced analytical, detection or alert treatment models must be explainable
  to regulators, MLROs, investigators and relevant risk teams". It argues that "financial crime models operate in a
  fundamentally different risk landscape that is highly dynamic" compared with credit or market models, cites the
  Wolfsberg second statement (file 04 section 2b), and the press echo says firms should aim for "full alignment by end-2026"
  (the deadline's exact meaning could not be verified).
- **The "model family" idea** is the practical bridge between prudential MRM and financial-crime reality: one family
  with its own tiering criteria (coverage of customers and transactions, concentration on a single model, detection
  effectiveness, not capital impact), its own validation depth, and a fast path for threshold and rule changes. It matches
  the Wolfsberg materiality criteria (file 04 section 2b: "share of transactions covered", "concentration").
- **FCA position on AI (primary).** The FCA's AI page (first published 8 September 2025, last updated 2 October 2026)
  says: "We do not plan to introduce extra regulations for AI. Instead, we'll rely on existing frameworks, which mitigate
  many of the risks associated with AI." The approach is "principles-based and focused on outcomes". The existing
  frameworks it relies on include the Senior Managers and Certification Regime (named individual accountability) and the
  Consumer Duty (Secondary, law-firm summaries); the chief executive repeated the no-AI-rules line in December 2025
  (Secondary). Practical effect on AML: the accountable executive for financial crime (the SMF17 MLRO
  and the SMF who owns the model framework) personally answers for an AML model's failures, so the bank wants evidence.
- **FCA AI Live Testing.** Launched September 2025; the second cohort (eight firms: Aereve, Coadjute, Barclays, Experian,
  GoCardless, Lloyds Banking Group, UBS, Palindrome) was announced April 2026 and includes "anti-money laundering
  detection and Know Your Customer" use cases; testing runs to end-2026, a good-and-poor-practice report is due later in
  2026 and an evaluation report in Q1 2027 (FCA press release; dates from secondary summaries). The FCA's own explanation is
  that Live Testing is how it will communicate AI expectations "through examples rather than formal rules".
- **Enforcement context.** The 2025 FCA monitoring fines (for example Monzo and Barclays) are covered in file 20; they
  show that the FCA reads weak monitoring governance (coverage gaps, untested thresholds, backlogs) as a control failure,
  independent of any AI question.

- **JMLSG guidance** (the UK industry guidance on monitoring, Part I chapter 5): the JMLSG site returned HTTP 403 to automated
  download, so its monitoring text is **not covered** here. What was read (the FCA's AI page and the UK Finance paper) treats AI
  as something that sits inside existing systems-and-controls duties rather than under a separate rulebook.

**AKR-TMS hooks (UK)**

- A **financial-crime model family** definition built into the product: every rule pack, score, anomaly model, matcher and
  LLM prompt template is an inventory item with family, tier, owner, purpose, operating boundaries and last validation.
- **Tier criteria tailored to AML** (coverage share, concentration, decision impact such as holds, customer impact, data
  novelty, explainability) rather than capital impact; the bank's validator can override the proposed tier with a recorded
  reason.
- **Named accountability fields**: model owner, model user, developer, validator, approving committee (SS1/23 2.4, 4.1), so
  the bank's Statement of Responsibilities has a system record to point to.
- **Annual self-assessment pack** against the five principles, produced from product data (SS1/23 1.7), so a bank can
  show its board and the CBI's assurance firm a dated, versioned self-assessment even where the CBI has not asked for one.

## 3. European Union: AI Act, AMLR, EBA, GDPR Article 22

The EU rules are not binding in Iraq. They matter for three reasons: EU correspondents and European auditors will judge Iraqi
banks by them; the EU text is the most detailed statement anywhere of what "human oversight of an AML model" means in law;
and a product sold to many banks should not be built to a lower standard than the strictest large market.

### 3a. The AI Act (Regulation (EU) 2024/1689): is AML detection a high-risk system?

Sources: official text from EUR-Lex (read 2026-10-06); Commission Guidelines on prohibited practices C(2025) 5052 of 29 July
2025 (read, paragraphs 177-183 and 207-211); Commission draft Guidelines on high-risk classification of 19 May 2026 (read,
paragraphs 305-309, 348 and the point 6 examples); Regulation (EU) 2026/1744 of 8 July 2026 (the "Digital Omnibus on AI",
read). All Primary.

**Annex III point 5(b)** lists as high-risk "AI systems intended to be used to evaluate the creditworthiness of natural
persons or establish their credit score, **with the exception of AI systems used for the purpose of detecting financial
fraud**". Recital 58 gives the reason for the carve-out: "AI systems provided for by Union law for the purpose of
detecting fraud in the offering of financial services and for prudential purposes to calculate credit institutions' and
insurance undertakings' capital requirements should not be considered to be high-risk under this Regulation."
**AML/CFT detection is not listed in Annex III at all.** The Commission's draft classification guidelines (19 May
2026; consultation closed 23 July 2026; final guidelines expected by end-2026 according to secondary summaries) settle how it is read, in paragraph 309:

> "AI systems intended to be used for anti-money laundering and for countering the financing of terrorism ('AML/CFT') are
> not covered by the exception, since those activities are regulated by other EU legislation and they are not related to
> financial fraud. Such systems are nevertheless out of scope to the extent their intended use does not cover the
> assessment of creditworthiness. Nevertheless, AI systems intended to be used for AML/CFT may classify as high-risk AI
> systems under Point 5(b), if the AI system is functionally linked and simultaneously intended to be used for the
> evaluation of creditworthiness or to establish a credit-score."

So an AML transaction-monitoring or customer-risk model is **not high-risk** as long as it is not also intended to feed
credit decisions. (An early web summary said the opposite; the primary text above shows it was wrong. The fraud carve-out
does not help AML, but AML does not need it.) The draft guidelines add three other points:
- Law-enforcement use cases (Annex III point 6) do not capture a bank: "a third-party/private entity will not be considered
  to act 'on behalf of' a public authority if the private entity acts on its own behalf ... to comply with a legal
  obligation" (example: an accounting firm's AML system). Financial intelligence units doing "administrative tasks
  analysing information pursuant to Union anti-money laundering law" are also outside point 6 (paragraph 348).
- Among examples outside point 6(e): "AI systems detecting suspicious transaction patterns that indicate money laundering
  or terrorist financing, even though that allows linking such transaction to personal profiles", with a warning that
  borderline cases arise when such systems are "combined with, or enriched by, personal data, thereby enabling the
  construction of individual profiles ... If the system assesses such personal characteristics, it should be classified as
  high-risk." (This sits in the law-enforcement section, so it applies to a system built for an authority, but it shows
  where the Commission draws the line: transactions and behaviour, not personal characteristics.)
- Annex III systems are "always" high-risk when they perform profiling of natural persons (Article 6(3), last
  subparagraph), which is why the credit-scoring link in paragraph 309 matters.

**The prohibition that does bite: Article 5(1)(d).** The AI Act bans "risk assessments of natural persons in order to
assess or predict the risk of a natural person committing a criminal offence, based solely on the profiling of a natural
person or on assessing their personality traits and characteristics" (the ban does not cover systems that "support the
human assessment of the involvement of a person in a criminal activity, which is already based on objective and
verifiable facts directly linked to a criminal activity"). Recital 42 says people "should never be judged on AI-predicted
behaviour based solely on their profiling, personality traits or characteristics, such as **nationality, place of birth,
place of residence**, number of children, level of debt or type of car, without a reasonable suspicion ... based on
objective verifiable facts and without human assessment thereof", and states that the ban does not touch "risk analytics
that are not based on the profiling of individuals ... such as AI systems using risk analytics to assess the likelihood of
financial fraud by undertakings on the basis of suspicious transactions". The Commission's prohibited-practice guidelines
(29 July 2025) address banks directly (paragraphs 209-211):

> "a banking institution has an obligation under Union anti-money laundering legislation to screen and profile customers
> for money-laundering offences. If the bank uses an AI system to fulfil its obligations, that should be done based only on
> the data as specified in that law which are objective and verifiable to ensure that the persons singled out as suspect
> are reasonably likely to commit anti-money laundering offences. The predictions must also be subject to human assessment
> and verification in compliance with that legislation in order to ensure the accuracy and appropriateness of such
> assessments. Compliance with that legislation will ensure that the use of individual crime prediction AI system for
> anti-money laundering purposes fall outside the scope of the prohibition in Article 5(1)(d) AI Act."

Paragraph 210 adds that a private entity that profiles customers "for its regular business operations and safety or to
protect its financial interests (e.g., detecting financial irregularities)", without the purpose of predicting a specific
criminal offence, is outside the prohibition. Paragraph 183 says AML/CFT evaluation and classification of persons must
"comply with relevant Union legislation". **Design consequence: an AML model that scores individuals must use objective,
verifiable data tied to the legal duty (transactions, CDD data, documents), must not rest on nationality, origin or
residence alone, and every individual suspicion must pass human assessment.** That is also good practice for Iraq.

**Timeline as of 2026-10-06** (Article 113 as amended by Regulation (EU) 2026/1744):

| Date | What applies |
|---|---|
| 1 Aug 2024 | AI Act in force |
| 2 Feb 2025 | Chapters I-II: definitions, AI literacy (Article 4), prohibited practices (Article 5) |
| 2 Aug 2025 | General-purpose AI model obligations, governance, penalties |
| 2 Aug 2026 | General date of application (transparency duties of Article 50 included; providers of generative systems already on the market must mark synthetic content by 2 Dec 2026) |
| 24 Jul 2026 / 27 Jul 2026 | Digital Omnibus on AI published in the Official Journal / in force (third day after publication) |
| 2 Dec 2026 | New prohibitions added by the omnibus (Article 5(1)(ba), (bb), (1a), (1b)) |
| **2 Dec 2027** | **Annex III high-risk obligations** (was 2 Aug 2026): recital: "the delayed availability of standards, common specifications, and alternative guidance and the delayed establishment of national competent authorities" |
| 2 Aug 2028 | Annex I (product-safety) high-risk obligations |

The omnibus (political agreement 7 May 2026, Parliament vote 16 June and Council approval 29 June, dates from secondary summaries; the Regulation itself is dated 8 July 2026) also rewrote Article 4: providers and
deployers "shall take measures to support the development of AI literacy of their staff ... This obligation does not
require providers or deployers to guarantee any specific level of AI literacy of any individual."

### 3b. The AML Regulation (EU) 2024/1624 (applies from 10 July 2027): the legal text on automated decisions

Source: the OJ text already in the research folder (amlr-2024-1624), read 2026-10-06. **Article 76(5)** is the clearest
legal statement of how an AML model may be used:

> "Obliged entities may adopt decisions resulting from automated processes, including profiling ..., or from processes
> involving AI systems ..., provided that: (a) the data processed by such systems is limited to data obtained pursuant to
> Chapter III of this Regulation [customer due diligence]; (b) any decision to enter or refuse to enter into or maintain a
> business relationship with a customer or to carry out or refuse to carry out an occasional transaction for a customer, or
> to increase or decrease the extent of the customer due diligence measures ... is subject to meaningful human intervention
> to ensure the accuracy and appropriateness of such a decision; and (c) the customer may obtain an explanation of the
> decision reached by the obliged entity, and may challenge that decision, except in relation to a report as referred to in
> Article 69 [a suspicious transaction report]."

**Article 75** (partnerships for information sharing, see file 21 section 13) adds: "information generated through the use of
artificial intelligence, machine learning technologies or algorithms may only be shared where those processes were subject
to adequate human oversight"; shared information must not be relied on alone ("shall not draw conclusions or take decisions
that have an impact on the business relationship ... without having assessed that information"); and each entity must do
"their own assessment of transactions". The AMLR recitals also say obliged entities "should pay particular attention to the
principles requiring that the personal data processed ... be accurate, reliable and up-to-date" and that "under no
circumstances should AML/CFT reasons be invoked to justify commercial decisions as regards prospective or existing clients".

### 3c. EBA, AMLA and GDPR

- **EBA AI Act mapping (factsheet 21 November 2025).** The EBA mapped the AI Act against banking and payments law with a
  focus on creditworthiness and credit scoring (the only banking use case that is high-risk). Findings: "No significant
  contradictions have been found between the AI Act and EU banking and payment legislation"; the AI Act "is complementary";
  "The EBA has not identified any immediate need to introduce any new or review existing EBA Guidelines." In 2026-2027 it
  will promote a common supervisory approach and feed the AI Office. It does not discuss AML models. Nothing in it changes
  the AML analysis above.
- **AMLA** (the new EU AML Authority) and its mandates (RTS under the AMLR package) are covered in file 05 section 2 (the
  lead's file; not yet written when this file was drafted). No AMLA instrument specific to AI in monitoring was found.
- **GDPR Article 22** (automated individual decisions) is acknowledged by AMLR recital and Article 76(5), which allow such
  processes with the three safeguards above. Iraq has no equivalent general data-protection statute (file 01 section 8 and
  file 30), so these safeguards are a contract-level design standard here, not a legal duty.

**AKR-TMS hooks (EU)**

- **Purpose binding.** Every score and feature carries a purpose tag "AML/CFT"; an export or API call that would feed a
  credit decision or marketing is blocked unless an approved exception exists. This keeps AML models outside Annex III 5(b)
  (para 309 "functionally linked") and matches Wolfsberg principle 1 (file 04 section 2e).
- **No solely automated adverse decision.** The product can raise alerts, rank, recommend and (Phase 2) request a hold, but
  account refusal, closure, restriction or an STR is a recorded human decision with the analyst's reasons, which is
  AMLR Article 76(5)(b) and the CBI's "until the MLRO verifies it is sound" (file 01 section 2d) in one design.
- **Objective-and-verifiable feature rule.** A feature registry lists each model input with its source table, legal basis
  and a flag "objective and verifiable" (transaction facts, CDD facts, documents); inputs that describe personal
  characteristics (nationality, place of birth, residence, name-derived attributes) are either excluded from model features
  or allowed only as documented risk factors set by the bank's risk assessment (section 9i).
- **Explanation on request.** For every decision-support output the system can render a plain-language explanation
  (reason codes, section 9j) suitable for the customer-facing staff, the regulator, and, where a bank's law allows, the
  customer (Article 76(5)(c)), with the STR carve-out (no explanation for suspicion reports; tipping-off rules).
- **Information-sharing guard.** Any AI-generated information leaving the bank (to another bank, to a consortium or to the
  FIU) is marked as AI-assisted and goes only after recorded human review (AMLR Article 75(g)).

## 4. Singapore: MAS FEAT, Veritas, the December 2024 AI model risk paper and the 2025-2026 AI risk management guidelines

MAS is the regulator that has said the most, in the most operational detail, about AI model risk in banks, and its text names
AML detection explicitly. It is not binding on Iraqi banks, but it is the best available checklist.

### 4a. FEAT (2018) and the Veritas Initiative

FEAT (Fairness, Ethics, Accountability, Transparency) was co-created by MAS and industry in late 2018 "to strengthen internal
governance around the application of AI and the management and use of data" (MAS Veritas page, Primary). Veritas turned it
into methods: Phase 1 (concluded 6 January 2021: credit scoring and customer marketing; fairness assessment methodology and
open-source code), Phase 2 (4 February 2022: five white papers on fairness, ethics, accountability and transparency
assessment, Toolkit v1; use cases included **fraud detection**), Phase 3 (26 June 2023: Veritas Toolkit 2.0 piloted at seven
institutions, open source at github.com/veritas-toolkit). Veritas gives AKR-TMS ready-made fairness metrics and assessment
templates that can be adapted to alert-rate and outcome analysis by customer group (section 9i).

### 4b. MAS Information Paper "Artificial Intelligence Model Risk Management: Observations from a Thematic Review" (5 December 2024)

Source: MAS PDF, 50 pages, read on 2026-10-06 (Primary). Based on a mid-2024 thematic review of selected banks; MAS says the good
practices "should generally apply to other financial institutions". Points that matter here:

- **AML is named as a use case.** Banks "commonly use" decision-tree models "such as XGBoost, LightGBM and CatBoost" in AML
  systems "to detect suspicious transactions" (para 2.1). MAS lists a regulatory risk: "poor performance of AI used to
  support AML efforts could lead to non-compliance with regulations" (para 2.2). Tree models "are also explainable as the
  relative importance of different features to model predictions can be extracted" (footnote 4).
- **Inventory** (5.2): complete AI inventories that "capture the approved scope of use" (purpose, use case, application,
  system, conditions) and "provide a central view of AI usage".
- **Risk materiality** (5.3.1) by three dimensions: (a) impact on the bank, customers or other stakeholders (financial,
  operational, regulatory, reputational); (b) complexity of the model or novelty of the use case; (c) **reliance on AI**,
  "which takes into account the autonomy granted to the AI, or the involvement of humans in the loop as risk mitigants".
  Materiality is reviewed over time (5.3.2).
- **Development standards** (section 6.3): data management, model selection, robustness and stability, **explainability**,
  **fairness**, reproducibility and auditability. Explainability: global and local methods "to identify the key features ...
  and their relative importance; assess whether these features were intuitive from a business and/or user perspective; and
  provide additional justification for retaining features which were not intuitive"; SHAP (global and local) and LIME
  (local) are named as common methods; the required depth depends on materiality and whether outputs must be explained to
  customers (credit decisioning is the most exacting). Fairness: define protected attributes ("gender, race or age"),
  test whether they are used, define at-risk groups, measure disadvantage with fairness metrics (Veritas toolkit), justify any
  use. Documentation (6.3.16-17) includes AI model cards.
- **Validation** (6.4): independent validation for higher-materiality AI before deployment, peer review for lower; "even for AI
  of lower risk materiality, the involvement of either an independent validator or peer reviewer allowed for some degree of
  challenge".
- **Pre-deployment** (6.5.1): beyond validation, "forward testing" (limited production data or users) and "live edge case
  testing", automated CI/CD pipelines, and checks that human oversight, backup models and contingencies are in place.
- **Monitoring** (6.5.3-6.5.5): metrics tracked "against predefined thresholds, usually determined at the development and
  validation stages"; **tiered thresholds** ("additional early warning thresholds" and different thresholds that decide when
  retraining or full redevelopment is needed); incident tracking from discovery to resolution.
- **Contingency** (6.5.6): fallback to alternative systems or manual processes; "kill switches" for mission-critical AI.
- **Change management** (6.5.9-6.5.10): define what is a significant change (new architecture, training technique) versus
  retraining on newer data; version control of "data and other artifacts such as hyperparameters and the trained model
  parameters or weights"; **"dynamic AI"** (automatic updating) is allowed for fast-drifting uses such as fraud only with
  "justifications for enabling automatic updating", a clear definition "of what can be updated automatically, for example,
  restricting changes to the retraining of AI model with more recent datasets, but not allowing for changes to AI model
  architectures or hyperparameters", and enhanced data-quality and monitoring controls.
- **Generative AI** (7.1): banks use it "to assist or augment humans", not in customer-facing roles "without a
  human-in-the-loop"; "banks continue to require human oversight or have a human-in-the-loop when using Generative AI to aid
  in decision-making" (7.1.9); detailed development and validation guidelines per task archetype (summarisation, information
  extraction, question answering, conversational agents); input and output guardrails (filters); evaluation harder for text
  than for yes/no predictions, so human feedback and expert-written reference summaries are used.
- **Third-party AI** (7.2): compensatory testing, contingency planning, and contracts with "performance guarantees, data
  protection, the right to audit, and notification when AI is introduced (or not incorporating AI without the bank's
  agreement)"; providers should notify version changes.

### 4c. MAS "Guidelines on Artificial Intelligence Risk Management": consultation P017-2025 (13 November 2025 to 31 January 2026)

Source: MAS consultation paper PDF, read on 2026-10-06 (Primary). The draft covers "Generative AI, AI agents and any newer AI
technologies" and applies to every FI in Singapore proportionately. Key paragraphs:

- Para 1.8(d) lists **financial crime risk** as a named AI risk: "when used to support anti-money laundering efforts, the
  greater uncertainty and unexpected behaviour of AI could mean that suspicious transactions go undetected."
- 4.1-4.4: lifecycle controls, proportionate to risk materiality; 4.5 data management; **4.6-4.7 transparency and explainability
  "according to its assessed risk materiality"**, with "reliance on AI for the final decision (i.e., the degree of AI
  autonomy)" as a key factor; 4.8-4.9 fairness: "define what it considers 'fair' outcomes", define protected attributes,
  measure systematic disadvantage, document mitigation.
- **4.10 Human oversight**: "The need for and degree of human oversight should take into account the objective of using AI, and
  be proportionate to the risk materiality of AI used. It should also take into account **automation bias and decision
  fatigue** as the use of AI increases in speed and scale." Considerations: clear roles; competent personnel "with the
  necessary authority and ability to intervene"; design "from the outset to enable and facilitate appropriate human
  oversight"; and documented review of human oversight decisions "including incidents as well as near misses".
- 4.18-4.21 pre-deployment review by parties not involved in development; 4.23 monitoring with "tiered thresholds (e.g., early
  warning threshold levels)", checks for "data drifts ... concept drifts ... and overall model drifts", monitoring of
  "reasoning processes, actions taken, tools used" for agents, "kill switches or override mechanisms" for high-risk
  materiality, strictly controlled and logged access to "AI models or systems, training data, pipelines, and configuration
  files"; 4.24 periodic aggregate-risk review and independent re-validation of high-materiality AI; 4.25 change control
  (material versus minor change; "human-in-the-loop" change control; version control of code, data, parameters); 4.26
  retirement and decommissioning.
- **Implementation: a transition period of 12 months after the Guidelines are issued** (para 4.7). **Status: not final as of
  2026-10-06.** MAS said in 2026 that the guidelines "will be finalised soon"; no response-to-feedback paper was found.

### 4d. MindForge handbooks (2025-2026) and the SAFR white paper (July 2026)

MAS's Project MindForge (industry consortium) produced an AI Risk Management Executive Handbook (17 considerations in four
sections: scope and oversight, AI risk management, AI lifecycle management, enablers), an **Operationalisation Handbook**
(published at the Singapore FinTech Festival, November 2025; January 2026 edition) and an AI Risk Management Toolkit
(March 2026). The Operationalisation Handbook addresses agentic AI separately: least privilege for agent tool and data
access, certification of reusable components, accountability divided at design time, kill switches and timeouts, and
traceability through searchable logging (as summarised in the SAFR paper). **SAFR, "Safeguards for Agentic Finance at
Runtime" (White paper v1.0, July 2026, hosted by MAS, developed with industry; "does not constitute regulatory guidance")**
proposes a runtime governance layer that "sits between the agent and the systems it acts on, evaluating proposed actions
before execution": for each agent and action type, define what it is authorised to do, assess each proposed action at runtime
before execution, and keep records for review and remediation. It names "compliance triage" among agentic applications.

**AKR-TMS hooks (MAS)**

- Inventory and **materiality rating with three dimensions** (impact, complexity, reliance), where reliance is computed from the
  product's own autonomy setting: advisory-only, human-confirmed hold, or auto-hold (Phase 2). Higher autonomy raises the tier.
- **Tiered monitoring thresholds** for every model: green, early-warning (amber), action (red: retrain, challenge, or revert to
  the champion/rules-only), with an incident log from discovery to resolution and a **kill switch** per model that reverts the
  affected queue to rules-only scoring without losing alerts.
- **Dynamic-model policy**: automatic retraining allowed only on newer data with frozen architecture and hyperparameters, behind
  a validation gate; any other change is a material change with revalidation (MAS 6.5.10).
- **Forward testing (shadow mode) and live edge-case testing** as release stages before a model or rule goes live.
- **Human oversight design** against automation bias and decision fatigue: random re-review of a sample of closed alerts, second
  review of auto-prioritised bottom-of-queue items, analyst override logging, and near-miss logging (4.10).
- **Third-party AI clauses** for any external model or LLM used: right to audit, version-change notification, no silent addition
  of AI, compensatory testing (MAS 7.2) built into AKR-TMS's own vendor-management checklist for its model suppliers.
- **Agent guardrails now, even if agents come later**: least privilege tool access, a runtime policy check before any agent
  action, searchable logs of reasoning steps, tools used and actions (SAFR; MAS 4.23).

## 5. Hong Kong: HKMA on AI in suspicious-activity monitoring (2024-2026)

The HKMA is the only supervisor found that has **asked banks to adopt AI in transaction monitoring** and then published what
governance it expects around it. All items below were read in full from HKMA PDFs on 2026-10-06 (Primary), except where noted.

### 5a. Circular of 9 September 2024 "Use of Artificial Intelligence for Monitoring of Suspicious Activities" (ref B10/1C)

- The HKMA observes that AI-powered systems "take into account a broad range of contextual information focusing not only on
  individual transactions, but also the active risk profile and past transaction patterns of customers" and "have proved to
  be more effective and efficient than conventional rules-based transaction monitoring systems".
- Authorised institutions "with significant operations in Hong Kong should give due consideration to adopting artificial
  intelligence in their ML/TF monitoring systems". They were to run a **feasibility study** against their current and near-future
  ML/TF risk profile and formulate an **implementation plan**; both "should be signed off at the board level and submitted to
  the HKMA by the end of March 2025". Support: a dedicated HKMA team with an external consultant through the Fintech Supervisory
  Sandbox and Chatroom; an industry experience-sharing forum in November 2024.
- The Annex lists benefits (Primary): (1) **wider data coverage**, because rules look only inside "prescribed periods (e.g. daily/
  weekly/ monthly)" while AI uses history and context, including own data (profile, transactions, previous alerts and their
  outcomes, STRs filed) and external data (group, law enforcement, sharing platforms FMLIT and FINEST, watchlists); (2)
  **enhanced detection**, with "a higher STR conversion rate without compromising system effectiveness"; (3) **more timely
  follow-up**: false-positive volume was the "primary driver", and ML scores "replace the different levels of initial
  assessment manually conducted by staff", with alerts consolidated "into a holistic review on a customer-relationship basis,
  which can be conducted by a single investigator"; (4) **sustainability**: static rules need "constant manual intervention",
  while "retraining an artificial intelligence empowered model can be much faster", and typologies from law enforcement,
  public-private partnerships and previous STRs are fed back into the model.

### 5b. Thematic review "Insights for Design, Implementation and Optimisation of Transaction Monitoring Systems" (17 April 2024, 18 pages)

The HKMA reviewed the end-to-end life cycle of banks' TM systems. It is the most practical regulator text found on tuning and
testing, and its findings map one-to-one onto product features:

- **Critical Data Elements (CDE).** Every reviewed bank could document the key data fields feeding the system (customer
  attributes, transaction details, counterparty information) and the path source system, ETL, warehouse, data-quality check,
  reconciliation, alert generation. "There is a direct correlation between maturity and quality of data and the ability to
  leverage the opportunities presented by advanced technologies, such as machine learning."
- **Data quality and lineage testing.** Tests of completeness and validity of CDEs, and lineage tests that data "had flowed
  correctly from the source systems to the TM systems"; most banks reconciled daily or per periodic review. Warning: "a few AIs
  showed weaknesses in accounting for certain critical data **during changes made to core banking systems**. This could lead to
  prolonged and significant gaps in monitoring, resulting in legal and regulatory risks." (Directly relevant to a product that
  reads a CBS replica: file 25.)
- **Segmentation.** "Appropriate customer segmentation reduces noise and improves the quality of alerts"; inappropriate
  segmentation produced "less effective threshold setting and tuning with higher volumes of false-positive alerts" and could leave
  "certain higher ML/TF risks ... un-monitored". Steps: define criteria (transaction frequency, size and type, geography,
  occupation, business nature), analyse with clustering and regression, subdivide by customer risk rating (high-risk customers get
  "more stringent thresholds"), review regularly.
- **Threshold setting and tuning.** "While there is no standardised method", banks commonly used: thresholds "based on a certain
  number of standard deviations away from the mean of historical transaction data; or percentile rank among historical
  transaction amounts"; **clustering analysis**; **above-the-line and below-the-line testing** "which involves adjusting current
  thresholds and parameters to arrive at an optimal configuration"; and **sensitivity analysis** of how changes in thresholds,
  rule configurations or risk-scoring models affect performance. Banks "should be aware of the strengths and limitations of
  different approaches" and document the analysis behind each threshold.
- **Functional testing.** The HKMA used a technology tool that "replicated AIs' detection scenarios according to their TM system
  functional specifications, and then compared the outputs (e.g. alerts generated) between simulated and deployed scenarios".
  This finds "data-mapping logic, incorrectly configured scenario thresholds, and transaction codes issues which can be difficult to
  identify through management dashboards". "The majority of TM detection scenarios deployed by AIs were found to be functioning as
  intended."
- **Periodic review.** Reviews every **12 to 24 months** depending on size and complexity, plus **trigger-event reviews** (surge
  in alert volumes, new products, emerging typologies); KPIs (alert counts, cases warranting investigation, false-positive rates,
  STR rates) reported "to the oversight committee quarterly"; use of independent teams for validation gives "a fresh
  perspective and minimizes bias".

### 5c. HKMA circular and report of 22 June 2026, "Supporting AI Adoption in Fighting Financial Crime" (23 pages)

The newest HKMA statement (circular ref B10/1C, 22 June 2026) follows the November 2025 workshop and covers dynamic risk
assessment, facial watch lists and corporate mule detection. It announces a **second workshop on agentic AI (23 June 2026)**
and says banks' implementation plans "should be kept under review". Governance language that a product must be able to evidence:

- "Where A.I. systems influence risk assessments or decision pathways, the responsibility remains firmly with senior management
  and Boards. Technology does not replace governance. Institutions must be able to explain how models operate, how outcomes are
  validated, and how risks such as bias, drift or over-reliance are managed. Supervisory tolerance for experimentation exists
  within well-defined guardrails — not outside them."
- "While more than half of financial institutions report A.I. deployment within risk functions, **fewer than one third operate
  fully integrated model governance frameworks across lines of defence**." Institutions should: establish formal cross-line AI
  governance committees; "define Board approved A.I. risk appetite statements"; "implement real-time monitoring for model drift,
  bias and explainability"; "conduct annual scenario testing aligned to enterprise test frameworks".
- Programmes must show "improving detection while reducing unnecessary alerts; accelerating investigations without
  compromising judgement; making regulatory reporting clearer and more useful; prioritising genuinely higher-risk activity".
- "The future state is not the elimination of rules, but their integration within layered intelligence models. Supervisors
  expect explainable, well governed A.I. that demonstrably improves effectiveness rather than merely efficiency."
- The report charts adoption of AI use cases in TM workflows (risk detection, alert triaging, investigation) by 2025 on a scale
  of up to about 25 percent of institutions (chart read from the PDF; exact shares not extractable).

Not read: the HKMA's 2019 high-level principles on AI and its 2024 consumer-protection guidance on generative AI (links not
reachable in this session; the web-search budget ran out). Treat any statement about them as Unverified.

**AKR-TMS hooks (HKMA)**

- **Independent functional-test harness.** AKR-TMS keeps scenario specifications as machine-readable functional specs and can
  re-implement each scenario independently (a second code path or SQL) over the same replica data, then diff alerts "simulated
  versus deployed". This is the HKMA functional test and also the cheapest answer to the CBI-approved assurance firm.
- **CDE registry, lineage and CBS-change guard.** Every field a scenario or model reads is a registered critical data element
  with a daily reconciliation and a schema-drift alarm on the CBS replica; a CBS upgrade or table change raises a coverage-gap
  warning and an automatic test run (HKMA's "prolonged and significant gaps").
- **Threshold-tuning workbench** offering the four HKMA methods (standard deviation, percentile, clustering, above/below-the-line
  and sensitivity), with the analysis stored against the threshold change request.
- **Review calendar**: 12-24 month periodic reviews and trigger-event reviews (alert surge, new product, new typology, CBS
  change), with a quarterly KPI pack for the oversight committee (alerts, productive cases, false-positive rate, STR rate).
- **Board pack generator**: a board-level AI adoption plan (feasibility, risk appetite statement, controls, rollout stages and
  success measures), mirroring what the HKMA asked banks to sign off, adapted to the CBI's expectations.
- **Governance committee support**: cross-line committee (compliance, risk, IT, audit, business) with dashboards for drift, bias
  and explainability, and annual scenario testing results.

## 6. United States, briefly (details in file 05)

File 05 already covers the US position; the points that matter for governance design are repeated here only as a table of how
each source maps to the three governance tracks of section 1.

| US source (file 05) | Track A: rules | Track B: models | Track C: GenAI |
|---|---|---|---|
| OCC/Fed/FDIC Revised Guidance on Model Risk Management, 17 April 2026 (section 1c) | Deterministic rules are not models: govern as configuration | Still a core component: validation, "effective challenge"; vendor models stay in scope even when not fully verifiable | Out of scope ("novel and rapidly evolving") |
| FinCEN AML/CFT programme NPRM, April 2026 (section 1a) | Design versus implementation findings; risk assessment drives resources | FinCEN will weigh "effective use of artificial intelligence, federated learning, and other advanced monitoring tools"; "responsible experimentation will not incur additional risk of AML/CFT enforcement" | Not addressed |
| NYDFS Part 504 (section 1f) | Documented scenarios, assumptions, thresholds; pre- and post-implementation testing; change management; annual certification | "Model validation" named for the filtering program | Not addressed |
| FFIEC manual (section 1d) | Management "should periodically review and test the filtering criteria and thresholds"; independent validation of "programming methodology and effectiveness" | Same | Not addressed |

Practical reading for an Iraqi bank: US correspondents will accept lighter, risk-based model governance than before April 2026, but
they will still ask who validated the monitoring system, when, and what changed. A product that produces the evidence on demand
satisfies both the old and the new US standard.

**AKR-TMS hooks (US)**

- **Track separation in the product**: rules and thresholds as governed configuration (not "models"), models under validation and
  monitoring, GenAI under review and logging, so the same evidence satisfies the US 2026 guidance, a correspondent's older SR 11-7-style
  questions and the CBI's assessor.
- **Effectiveness evidence pack** for FinCEN-style expectations: outputs (STRs that led to action, true positives by typology, time to
  detect), design-versus-implementation findings, and the log of governed experiments (file 05 section 1a hooks).
- **Annual certification pack** modelled on NYDFS Part 504 (file 05 section 1f), reused for correspondents.

## 7. Gulf and Arab regulators, and the CBI

### 7a. United Arab Emirates (strongest regional instruments; both Primary, read on 2026-10-06)

**CBUAE Model Management Standards (MMS), version date November 2022, 56 pages, with a companion Model Management Guidance.**
The MMS is binding ("must" is mandatory, "should" strongly recommended), applies to all licensed banks including Islamic banks,
and "represent[s] the minimum requirements to be met within the UAE" (1.1.2). It applies to "all types of models employed by
institutions to support decision-making" (2.4.1). **Table 1 of the scope names "AML: Fraud alert and other models" and "Artificial
Intelligence"** among the covered model types, so UAE banks must treat AML detection models as governed models. Relevant provisions:

| Ref | Requirement (quoted or closely paraphrased) |
|---|---|
| 3.1.3 | "Model Risk must be incorporated in institutions' risk framework alongside other key risks", with an "appetite for model uncertainty" |
| 3.1.5, 4.4.5 | Models "must be grouped according to their associated Model Risk"; minimum two groups, Tier 1 and Tier 2; grouping reviewed by the CBUAE |
| 3.1.6 | A **Model Oversight Committee** "accountable for all significant modelling decisions", accountable to senior management and the Board |
| 3.1.7-3.1.8 | The CRO is responsible institution-wide; the Board "bears the responsibility of all modelling decisions with material implications" and must define the appetite for model risk; the Board must ensure staff with "robust technical expertise" |
| 3.1.9 | Internal audit assesses the framework but "must not be involved in the validation of specific models" |
| 3.1.10, 4.7.1 | "Institutions must remain the owners of their models at all times, under all circumstances", even when a third party supports any step |
| 4.4.1-4.4.4 | Inventory of all production models including third-party models, with historical models kept for roll-back; unique ID, and "a model with a new calibration must carry a different identification number"; dates, owners, previous validations; consultants "must not be considered as responsible for any step" |
| 4.7.4 | "If a methodology acquired from a third party is not fully understood by the institution, then it must not be considered fit for purpose"; later validation must be by a party "independent from the original provider" |
| 4.7.5 | **Ready-made models calibrated on external data are acceptable where the bank cannot collect enough internal data, and only temporarily where it can**: "immediately after the model implementation, institutions must take the necessary actions to (i) collect historical internal data ... and (ii) collect future internal data in order to develop a model internally" |
| 2.2.7 | "Lack of data will not be an acceptable reason for material models to fall short of these requirements. Instead, institutions must implement temporary solutions to mitigate Model Risk" |
| 4.8.1-4.8.2 | Enough internal technical staff to manage models "independently of third parties"; "If an institution does not have the required internal skills to manage complex models, these models should be simplified or replaced" |
| 8.4 | Overrides are permitted "but within limits"; limits and controls on "frequency and magnitude"; "Models whose input and/or outputs that are frequently and materially overridden must not be considered fit for purpose and must be recalibrated or replaced"; overrides documented, justified, approved and reported to the committee and Board |
| 8.4.4 | "If a model has been approved and is deemed suitable for production, its outputs must not be ignored" |
| 9.4 | Metrics and limits capturing "accuracy and stability" and "model usage to assess whether the model is used as intended" |
| 10.5 | Validation and monitoring frequencies by tier (for example retail scorecards: Tier 1 monitoring 3 months, validation 1 year; Tier 2 6 months and 3 years); "For models other than those mentioned in table 2 ... institutions must establish a schedule ... coherent with their nature and their associated Model Risk"; dates tracked in the inventory and reported to the committee at least quarterly |
| 10.6-10.7 | Validation report content (reference, materiality, findings with severity, KPI values against limits, qualitative and quantitative review, model risk rating, conclusion); remediation tracked, reported quarterly to the committee and yearly to the Board |
| 2.2.2 | Banks self-assess against the MMS and submit a remediation plan to the CBUAE within six months of the effective date |

**CBUAE, SCA, DFSA and FSRA "Guidelines for Financial Institutions adopting Enabling Technologies" (issued 15 November 2021).**
Principles 2.25-2.30 for big data analytics and AI: an "approved and documented governance framework"; the governing body and
senior management "remain accountable for the outcomes and decisions of their ... AI Applications including those Applications
that make decisions on behalf of the Institutions"; models "reliable, transparent, and explainable, commensurate with the
materiality"; a framework "to review the reliability, fairness, accuracy and relevance of the algorithms, models and Data used prior
to deployment ... and on a periodic basis after deployment". Guidance 3.98-3.111 adds: avoid "black-box" models (3.98(c));
"promptly suspend material ... Applications" on malfunction (3.101); "rigorous, independent validation and testing" including a check
"to identify any unintuitive or false causal relationships" (3.102); a nine-item documentation list (3.103: data sources, data
quality checks, design rationale, methodology, results, metrics, usage, validation frequency, assumptions and limitations); triggers
for retraining, calibration, redevelopment or discontinuation (3.106(b)); review of outcomes "for fairness or unintentional bias (e.g.
through monitoring and analysis of false positive and/or false negative rates)" (3.106(c)); fallback to "human intervention or the use
of conventional processes" (3.106(d)); versioning of data, algorithm and variables (3.108); "a human-in-the-loop mechanism" where
needed to detect bias, "the possibility of manual intervention to mitigate or reverse irresponsible and erroneous decisions", and
that models be "explainable" (3.109); and a definition of what "fair" means for the model (3.110).

### 7b. Saudi Arabia

SAMA's **Counter-Fraud Framework** (No. 000044021528, dated 11 October 2022; Primary, read from the SAMA Rulebook) is a fraud rather
than an AML instrument but it states the regional supervisor's expectations for detection technology. Section 3.8 requires that, for
a vendor or in-house system: "The rationale for scenarios developed and thresholds applied is documented"; rules "can be customised";
"New rules can be implemented on a timely basis to target ... new or emerging typologies"; "Awareness of rules to prevent and detect
potential internal fraud is limited to a restricted, documented set of roles which does not include employees or third parties
responsible for the operation of processes and controls being monitored (e.g., branch/customer facing staff ...)"; configuration
changes follow IT-governance change management; and **"Where Machine Learning or Artificial Intelligence are used the system should
not be 'black box' and should be capable of being audited (e.g., the organisation should have the capability to test what the
algorithms are designed to do and whether they are correctly implemented)"**. Section 5.1(f) says higher inherent risk calls for
"additional detection controls (e.g., real time monitoring, additional data sources or Machine Learning models) or more stringent
detection threshold criteria"; section 5.2 requires 24/7 operation, complete, timely and accurate data with "data mapping from source
system to the detection system ... validated". Searches of the SAMA Rulebook for "artificial intelligence", "model risk", "machine
learning" and "algorithm" (2026-10-06) returned no general AI or model-risk standard (only cyber-threat-intelligence, Basel internal
model capital rules and the counter-fraud pages); SAMA material outside the Rulebook (for example national AI principles from SDAIA)
was not read.

### 7c. Others not reached

Central Bank of Jordan, Qatar Central Bank (an AI guideline was reported in 2024; not verified), Central Bank of Bahrain, Central Bank of
Egypt and Central Bank of Oman: their sites could not be searched or read in this session (the web-search budget ran out and direct
fetches failed or returned no relevant page). **Unverified**: do not cite them.

### 7d. Iraq: what the CBI has and has not said

- **AI/ML is required in AML systems** (2025 controls, ch. 5 art. 5; file 01 section 2d) with a stop-until-cleared control: this is a
  functional requirement, not a governance one.
- **AML tooling is assessed by a CBI-approved specialist firm** and the **three lines of defence** are assessed the same way (CBI
  standards handbook 2025, D2 and D4; file 01 section 2e).
- **Closest thing to a model-validation standard: Article 16 of the CBI's risk-management controls (July 2026, commercial banks)**,
  clause و, for systems that measure or monitor *market* risk: they must be "documented", use "reliable" data, have "realistic"
  assumptions, be "validated before use through testing and analysing their results", be validated by staff "sufficiently qualified
  and independent of those who developed the models or who bear the risks", and be "reviewed periodically, at least quarterly and
  whenever needed" (translated from the Arabic text of the controls; clause 5 ends on page 40 and clause 6 sits on page 41). It is scoped to market risk and says nothing about AI
  or AML. It shows the CBI's vocabulary for model governance: documentation, data reliability, validation before use, independence,
  quarterly review. A bank's auditors can reasonably extend it by analogy to AML models, and AKR-TMS should meet it by default.
- **No CBI instrument on AI governance or model risk was found.** The Arabic CBI PDFs are partly scanned, so a keyword search of the text
  layer (done on 2026-10-06 for the Arabic words for AI, machine learning, algorithm and model risk) is not conclusive; file 01's reading of
  the rendered pages is the better evidence and it also found none. Treat "none exists" as probable, not proven.

**AKR-TMS hooks (Gulf and Iraq)**

- **Build to the CBUAE MMS as the regional high-water mark**: unique IDs per model and per calibration (4.4.3), archive and roll back
  (4.4.2), two-tier minimum, Model Oversight Committee workflow, quarterly reporting of monitoring and validation dates, yearly Board
  report, validation report template with the ten fields of 10.6.1. A bank in Baghdad that meets the MMS will meet any plausible CBI
  requirement and any correspondent's.
- **Externally calibrated model policy** (4.7.5): the AKR-TMS pre-trained models are labelled "externally calibrated, temporary"; each
  bank's deployment starts a data-collection and local-retraining plan from day one, with a target date after which the bank's own
  model replaces or recalibrates the starter model. This is the product answer to scarce labels (file 21 section 16).
- **Override limits and fitness test** (8.4): every analyst override of a model score or alert priority is logged with reason; the
  override rate by model and segment is a monitored metric with a limit; breach flags the model "not fit for purpose" for review.
- **Rule-secrecy roles** (SAMA 3.8(d)(5)): threshold values are visible only to a documented set of roles (compliance, audit, model
  owner); branch staff see reason codes only.
- **Black-box ban** (SAMA 3.8(d)(8); CBUAE 3.98(c)): no model goes live without a documented test of "what the algorithms are designed to
  do and whether they are correctly implemented", provided as a one-click validation pack.
- **Suspension switch** (CBUAE 3.101): the bank's model owner can suspend any model at once, with automatic fallback to the rules-only path.

## 8. Standards that give auditors something to test against

| Standard | What it offers | Use in AKR-TMS |
|---|---|---|
| NIST AI Risk Management Framework 1.0 (January 2023) | Four functions (Govern, Map, Measure, Manage); referenced by MAS in its explainability footnote | Structure for the bank's AI risk register and for the model card's "measure" section |
| NIST AI 600-1, Generative AI Profile (July 2024) | Twelve GenAI risks, including "Confabulation: The production of confidently stated but erroneous or false content (known colloquially as 'hallucinations' or 'fabrications')", Data Privacy, Information Security, Human-AI Configuration, Information Integrity, Value Chain and Component Integration | Risk list for the LLM features (section 9k) |
| ISO/IEC 42001:2023 | "specifies requirements for establishing, implementing, maintaining, and continually improving an Artificial Intelligence Management System (AIMS)" (ISO page) | A bank can certify its AI management system; AKR-TMS maps its controls to the clauses to speed that audit |
| OWASP Top 10 for LLM Applications 2025 | LLM01 Prompt Injection; LLM02 Sensitive Information Disclosure; LLM03 Supply Chain; LLM04 Data and Model Poisoning; LLM05 Improper Output Handling; **LLM06 Excessive Agency**; LLM07 System Prompt Leakage; LLM08 Vector and Embedding Weaknesses; LLM09 Misinformation; LLM10 Unbounded Consumption | Security test list for the GenAI components (file 30 for the security programme) |
| EU AI Act Articles 12, 14, 15 (high-risk requirements; not binding on AML systems) | Art. 12: "automatic recording of events (logs) over the lifetime of the system"; Art. 14(4): overseers can understand capacities and limits, "remain aware of the possible tendency of automatically relying or over-relying on the output ... (automation bias)", interpret output, "decide ... not to use the ... system or to otherwise disregard, override or reverse the output", and intervene through "a 'stop' button"; Art. 15(4): systems that keep learning must address "the risk of possibly biased outputs influencing input for future operations (feedback loops)" | Design anchors for logging, human oversight and feedback-loop control, even though AML detection is not high-risk |

**AKR-TMS hooks (standards)**

- A **control crosswalk**: each AKR-TMS governance control is tagged to PRA SS1/23, CBUAE MMS, MAS, NIST AI RMF, ISO 42001 clauses so that one set of evidence serves a CBI assessor, a correspondent's auditor and a bank's own ISO programme.
- The **feedback-loop control** from AI Act Article 15(4): analyst decisions used to retrain a model are filtered (second review, closure-reason quality, exclusion of decisions on alerts the model itself suppressed) so the model does not learn its own bias (file 21 section 4).

### 8b. Wolfsberg Principles for Using AI and ML in Financial Crime Compliance (December 2022), mapped to the blueprint

File 04 section 2e reads the paper in full. Its five principles map onto the blueprint of section 9 as follows, so one control set
answers both the industry's own standard and the regulators' texts above.

| Wolfsberg principle (2022) | What it asks | Where the blueprint answers it |
|---|---|---|
| 1. Legitimate purpose | Guard against misuse and bias; data collected for financial-crime compliance is not reused for other purposes without review | Purpose binding (GV-41); protected-attribute register and proxy scan (9i) |
| 2. Proportionate use | Weigh benefit against risk; severity of the crime risk against the model's margin for error; validate use and configuration regularly | Tiering grid and accepted miss rate (9b, 9c); periodic review calendar (9d) |
| 3. Design and technical expertise | Understand the technology and its limits; results "adequately explained or proven given the data inputs"; ongoing testing and re-configuration | Validation pack, explainability tests, shadow mode (9c, 9e, 9j); internal skills (CBUAE 4.8) |
| 4. Accountability and oversight | The bank is responsible whether the AI is built in-house or bought; staff trained; technical teams can be challenged | Named roles, independent validator, committee, vendor-transparency rule (9h; CBUAE 4.7.1, 4.7.4) |
| 5. Openness and transparency | Be open within legal limits, but without exposing detection logic or breaching STR confidentiality | Role-based visibility of thresholds (SAMA 3.8(d)(5)); customer explanation with STR exclusion (9j) |

## 9. Design guidance: the AKR-TMS governance blueprint

The blueprint below is design guidance, not a regulatory finding. Each element cites the source it follows; where a number or
method is an engineering choice rather than a regulator's, it says so. The principle throughout is the one the Wolfsberg Group and
the HKMA both state: **proportionate governance that produces evidence automatically, so that governance speeds adoption instead of
blocking it** (file 04 section 2b; section 5c above).

### 9a. Model and rule inventory

One inventory holds every governed object in a bank's AKR-TMS deployment. The inventory is a system table, not a spreadsheet; the
product refuses to activate an object that has no inventory record (SS1/23 1.2; CBUAE 4.4; MAS 5.2).

| Object type | Examples | Governance track |
|---|---|---|
| Rule / scenario | CBI minimum scenarios (file 27), bank-added scenarios, structuring and rapid-movement rules | A (configuration) |
| Parameter set | Segment thresholds, risk-factor weights, country and product risk tables | A |
| Statistical or ML model | Alert-scoring model, anomaly detector, customer risk score, network features | B |
| Matcher | Name-screening scorer, entity-resolution matcher (file 22) | B |
| Reference list | UN, OFAC, EU, Iraqi national list, internal lists | A (data) |
| LLM component | Summary prompt, narrative drafter, adverse-media triage, each with model and version | C |

**Fields** (merging SS1/23 1.2(c), CBUAE 4.4.3-4.4.4, MAS 5.2): unique ID (new ID for every new calibration, as CBUAE requires);
name; type; purpose and approved scope of use; typology and CBI article it serves; owner, user, developer, validator, approving
committee (named people, SS1/23 2.4); tier; status (design, shadow, live, suspended, retired); version; inputs as registered critical
data elements (HKMA CDE); operating boundaries (population and date range the model was built on); assumptions and limitations;
performance limits (green, amber, red); externally calibrated flag with a local-recalibration due date (CBUAE 4.7.5); dates and results
of last monitoring, last validation and next due; override rate; open findings; links to documentation, test evidence and the change
log; archived predecessors with roll-back (CBUAE 4.4.2). The inventory is exportable for the CBI assessment, correspondents and audit.

### 9b. Tiering

CBUAE requires at least two tiers (4.4.5); SS1/23 rates **materiality and complexity** (1.3); MAS rates **impact, complexity and
reliance** (5.3.1). AKR-TMS proposes one scoring grid that combines them, fully visible to the bank and overridable by its validator
with a recorded reason (SS1/23 1.3(e)):

| Dimension | Question | Raises the tier when |
|---|---|---|
| Decision impact | What happens when the object is wrong? | It can hold or stop a payment (CBI stop-until-cleared; Phase 2 API), restrict or close an account, or decide an STR |
| Coverage | Share of customers, accounts and transaction value it touches (Wolfsberg "share of transactions covered") | It covers a large share, or is the only detector for a typology (Wolfsberg "concentration") |
| Autonomy / reliance (MAS) | Does a human confirm before effect? | It acts or ranks without human confirmation of each effect |
| Complexity (SS1/23 1.3(c)) | Interpretability, number of inputs, use of unstructured data, dynamic updating | Black-box or self-updating, or uses text and graph features |
| Data novelty and quality | Is training data representative, labelled, local? | Few labels, proxies, synthetic data, externally calibrated (SS1/23 3.2(c); CBUAE 4.7.5) |
| Regulatory exposure | Is it a CBI-mandated control? | It implements a CBI minimum scenario or a screening duty |
| Customer and fairness exposure | Could errors disadvantage a group? | It uses nationality, geography, age, gender, or name-derived inputs |

Proposed outcome: **Tier 1** (high: alert-scoring models that auto-suppress or auto-hold, customer risk score that sets monitoring
intensity, entity-resolution that merges identities, any model with Phase 2 pre-release effect), **Tier 2** (medium: anomaly
detectors that only prioritise, name-match scorers with human review of every hit), **Tier 3** (low: LLM drafting aids with mandatory
human edit, reporting helpers) and **rules as configuration** with their own lighter lifecycle (9c, 9f). The banks' validator may
collapse Tier 3 into Tier 2 to meet the CBUAE two-group minimum.

**Review cadence anchors** (design proposal anchored on published figures): CBUAE retail scorecards, Tier 1: monitoring every 3
months and validation every year; Tier 2: 6 months and 3 years. HKMA TM reviews: every 12 to 24 months plus trigger events. AKR-TMS
default: Tier 1 monitoring monthly and revalidation yearly; Tier 2 monitoring quarterly and revalidation every two years; rules
reviewed every 12 months and on triggers; each bank can tighten but not loosen below the CBUAE/HKMA floor without a documented
justification.

### 9c. Validation

A validation pack has seven parts. Rules get parts 2, 3, 5 and 6 (they are not "models" under the US 2026 guidance, but they carry
the same data and implementation risks, SS1/23 1.1(c)); models get all seven.

1. **Conceptual soundness and design review** (SS1/23 4.2; CBUAE 10.3): purpose, typology link, technique justified "by published
   research ... or generally accepted industry practice" (SS1/23 3.1(b)), assumptions, why each feature, the "unintuitive or false causal
   relationships" check (CBUAE enabling-technology guidance 3.102).
2. **Data validation** (SS1/23 3.2, 4.3(a); HKMA 04): critical data elements, completeness and validity, **lineage from the CBS replica
   to the alert**, representativeness of training data, treatment of proxies and synthetic data, label quality (second-review rate,
   closure reasons).
3. **Implementation and process verification** (SS1/23 4.3; HKMA functional testing): an independent re-implementation of each
   rule from its functional specification, run on the same data, with alert-level diff "simulated versus deployed"; replay tests with
   seeded transactions that must alert (a regression suite per CBI scenario); code review of scoring and feature pipelines.
4. **Outcomes analysis and back-testing** (SS1/23 4.4; CBUAE 10.4): alert-to-case and case-to-STR conversion, precision and recall
   with confidence intervals on labelled holdout data (out-of-time, not only out-of-sample, MAS footnote 52), performance by segment,
   by branch and by customer group, time to detect where timestamps exist.
5. **Sensitivity analysis** (SS1/23 3.3(b)(ii); HKMA): how alert volume and productivity move when each threshold or weight moves; the
   "operating boundaries" where performance holds.
6. **Above-the-line and below-the-line testing** (HKMA 06; Wolfsberg file 04 section 2a). Definitions used in the product (industry
   practice; HKMA gives only the one-line description "adjusting current thresholds and parameters to arrive at an optimal
   configuration"): *above-the-line* (ATL) looks at alerts that fired, asks what would be lost if the threshold were raised, and
   measures the productivity of the alerts nearest the line; *below-the-line* (BTL) samples activity that did not fire, just below the
   line and further down, and has experienced analysts review it to find suspicious activity the rule misses. Wolfsberg warns that
   "simply lowering thresholds by 10% is not meaningful false-negative testing": a credible BTL test also uses cases that arrived by
   other routes (branch referrals, RFIs, authority requests, STRs from outside the system) to see what the system missed.
7. **Benchmarking and challenger comparison** (SS1/23 3.3(b)(iii)): against a simpler baseline (rules only, logistic regression) and,
   for Tier 1, one alternative model class. A complex model that cannot beat a transparent baseline on the bank's data should not go
   live, which also answers CBUAE 4.8.2 ("simplified or replaced").

**Statistical methods for threshold setting and tuning** (the HKMA list, with practical notes):

| Method | How it works | Caution for Iraq's cash-heavy, dollarised customers |
|---|---|---|
| Standard deviations from the mean (HKMA) | Alert when a value exceeds mean plus k standard deviations of the segment's history | Cash and trade amounts are heavy-tailed; mean and standard deviation are inflated by a few large customers, so k-sigma either misses or floods |
| Percentile rank (HKMA) | Alert above, for example, the 99th percentile of the segment's historical monthly values | Robust to heavy tails; needs enough customers per segment and a separate threshold per currency (IQD, USD) |
| Robust statistics (design choice) | Median and median absolute deviation, or log-transformed values, per segment | Works where a few huge accounts dominate |
| Clustering analysis (HKMA; Wolfsberg: k-means, Jenks) | Group customers or transactions by behaviour and set thresholds per cluster; flag small outlying clusters | Re-cluster on a schedule; clusters shift with seasons (Ramadan, Arbaeen, harvest, salary days) |
| Sensitivity analysis (HKMA) | Move one threshold at a time and chart alerts, productive alerts, coverage | Use to find the knee of the curve, not a single "optimal" point |
| ATL/BTL testing (HKMA) | Section 6 above | Keep the CBI minimum scenarios at their mandated meaning; tune only the bank's own extra scenarios and parameters the CBI allows |
| Rare-event sampling bounds (design choice, textbook "rule of three") | If a BTL sample of n reviewed items contains no productive case, the 95 percent upper bound on the productive rate is about 3/n; n = 300 gives about 1 percent | Gives analysts a defensible sample size when productive cases are rare |

Wolfsberg also states that "aiming for 100% recall ... is likely to lead to an ineffective system" (file 04 section 2a): the validation
pack therefore records the **accepted miss rate** (risk appetite for missed activity, Wolfsberg part II) alongside the metrics.

**Validate once, then validate the delta** (Wolfsberg part II, file 04 section 2b; SS1/23 2.6(c) for group subsidiaries): the AKR-TMS
model family gets one full validation per version; each bank gets a delta validation covering its data mapping, calibration, local
thresholds and feedback-loop settings. A bank may still commission a fuller review.

### 9d. Ongoing monitoring and drift

Monitoring runs on the same objects as the inventory and reports three kinds of drift (MAS Annex A and consultation 4.23):
**data drift** (inputs change: new product, CBS change, currency regime), **concept drift** (the relationship between inputs and
suspicious outcomes changes: new typology, new fraud method), **model drift** (scores change without data change).

| Signal | Metric (design proposal) | Why it matters here |
|---|---|---|
| Input distribution | Population Stability Index (PSI) and Characteristic Stability Index per feature, with the industry convention of about 0.10 as a watch level and 0.25 as an action level (a credit-scoring convention, not a regulatory number) | CBS upgrades, new branches, new product codes, dollar-ban effects (file 01 section 4) |
| Score distribution | PSI on the score; share of alerts per score band | A model that quietly scores everything lower is a silent miss |
| Alert volume and mix | Alerts per segment per week against control limits | The HKMA surge-in-alerts trigger; backlog risk |
| Productivity | Precision by band (alert to case, case to STR), with lag-aware windows | Labels arrive weeks or months after alerts; use escalation as an early proxy and refresh the true STR outcome later |
| Overrides | Analyst override rate by model, segment, analyst (CBUAE 8.4) | Frequent, material overrides mean the model is not fit for purpose |
| Fairness | Alert, escalation and STR rates by customer group (9i) | Detects drift into unequal treatment |
| Data quality | Completeness, validity, reconciliation and lateness per critical data element (HKMA 04) | Missing data is the commonest cause of silent coverage loss |
| LLM quality (Track C) | Reviewer edit rate, factual-error rate in sampled outputs, refusal rate | Detects prompt or model-version regressions |

**Thresholds are tiered** (MAS 6.5.4; consultation 4.23): green; amber early warning (investigate, increase sampling); red (retrain,
recalibrate, restrict use, or fall back to rules-only: SS1/23 5.2, 5.3 and the kill switch of MAS 6.5.6). Every breach opens an incident
with an owner and a due date; the monitoring report is independently reviewed (SS1/23 4.4(e)) and goes to the Model Oversight
Committee at least quarterly (CBUAE 10.5.2).

### 9e. Champion-challenger and shadow running

- Every Tier 1 and Tier 2 model change passes through **shadow mode**: the new version scores live data without creating alerts;
  outputs are compared with the champion on the same cases and with analyst outcomes (this is MAS "forward testing", the FATF 2021
  "pilot/shadow mode", and SS1/23 3.3(b)(iii) challenger comparison in one mechanism).
- **No mandatory parallel run of the old system** (Wolfsberg part II calls the classic parallel run "an artefact of a less advanced
  testing environment"): promotion is decided against pre-agreed success criteria (priority-risk coverage, precision, recall,
  stability, explainability), not against re-detecting every historic alert ("no SAR left behind" is the failure mode Wolfsberg names).
- **Promotion record**: criteria, results, approver, date, rollback point. The champion stays available for roll-back (CBUAE 4.4.2).
- A permanent **challenger** runs for Tier 1 models so that drift or underperformance of the champion can be seen against a live
  alternative.

### 9f. Change management

- **Three classes of change.** *Minor*: parameter inside an approved band, list refresh, retraining on newer data with frozen
  architecture and hyperparameters (MAS 6.5.10). *Material*: new rule, new threshold outside the band, new feature, new model
  architecture, new training method, new prompt template or LLM version, new data source or CBS mapping. *Emergency*: sanctions-list or
  regulator-ordered change, applied now and reviewed within a fixed number of days.
- **Maker-checker with evidence** (NYDFS Part 504 "defined, managed, controlled, reported, and audited", file 05 section 1f): each change
  request shows the reason, an **impact analysis** (alert volume and coverage before and after, sensitivity run, BTL result for
  threshold relaxations) and, for any change whose main effect is to reduce alert volume, a **flag** that routes it to the MLRO and the
  committee (DFS's concern about changes made "for purposes of minimizing generated alerts").
- **Version control of everything**: code, parameters, thresholds, training data snapshot, feature definitions, prompts, model weights
  (MAS 6.5.9: "data and other artifacts such as hyperparameters and the trained model parameters or weights"), reproducible rebuild from
  snapshot.
- **Release trains**: product-level releases (core model family) are validated once; bank-level configuration changes follow the bank's
  own change board. The bank can pin a version and defer an upgrade within a supported window; AKR-TMS notifies every model or
  data-feed change in advance (MAS 7.2 vendor-notification clause).
- **Rollback** within one change window and **kill switch** per model (9d).
- **Change log as audit evidence**: who proposed, who approved, what changed, effect measured afterwards (post-change review at 30 and
  90 days).

### 9g. Documentation and model cards

A **model card** (Tier 1-3) and a lighter **rule card** are generated from the inventory and test results, not written by hand. The
model card follows SS1/23 3.5(b), CBUAE enabling-technology 3.103 and MAS 6.3.16:

1. Identity and purpose: ID, version, owner, intended use, out-of-scope uses, approved population and period.
2. Data: sources and CDEs, training window, label definition and label quality, synthetic or external data share, exclusions, known gaps.
3. Method: algorithm, features with typology mapping, why chosen, comparison with baselines and challengers.
4. Performance: metrics with confidence intervals, by segment and by group; thresholds used; operating point and the accepted miss rate.
5. Explainability: global importance, local reason-code logic, stability checks.
6. Fairness: protected attributes considered, tests, results, mitigations.
7. Limitations and operating boundaries; failure modes; fallback.
8. Monitoring plan: metrics, thresholds (green, amber, red), owners.
9. Validation history and open findings; change log.
10. Contacts and approval record.

The rule card holds: purpose, typology, CBI article (for minimum scenarios), parameters and the rationale for each value, tuning history
with test results, data inputs, owner, last review. The documentation standard is the SS1/23 test: "an independent third party with the
relevant expertise would be able to understand how the model operates ... and to replicate any parameter estimation and model results."

### 9h. Independent testing, audit and the three lines of defence

| Line | Who in an Iraqi bank (typical) | Role for AKR-TMS objects | Source anchor |
|---|---|---|---|
| First line | Branch and operations staff; KYC and alert analysts; branch liaison officers (CBI 2025 controls, file 01) | Use outputs, record decisions and overrides, report anomalies and near misses, take refresher training | SS1/23 2.4(d); MAS 4.10 |
| Model owner | Head of AML/compliance (the MLRO function) for AML models | Accountable that performance is monitored against appetite, tier is right, inventory is current | SS1/23 2.4(c); CBUAE 4.5 |
| Developer | AKR-TMS (vendor) and/or the bank's data team | Design, build, document, supply the monitoring pack | SS1/23 2.4(e); 3.3(a) |
| Second line | Risk and compliance oversight; model oversight committee | Frameworks, tiering, monitoring review, committee decisions, Board reporting | CBUAE 3.1.6; Wolfsberg MRM/assurance split |
| Independent validation | Bank's model validation unit, an external firm, or the CBI-approved assurance firm; must be independent of developer and owner | Validation pack review, revalidation by tier | SS1/23 4.1(d); CBUAE 3.1.12, 4.7.4; CBI handbook D2 |
| Third line | Internal audit | Audits the framework and its operation; "must not be involved in the validation of specific models" (CBUAE 3.1.9) | SS1/23 2.5; Wolfsberg 2024 audit principles (file 04 section 2m) |

Product features that make independence real rather than nominal:

- **Role separation enforced in the system**: the person who proposes a change cannot approve it; a developer account cannot sign a
  validation; an auditor account is read-only with full history (file 04 section 2m hook).
- **Validator workspace**: the validator receives the pack, the data snapshot, the code and a harness to re-run the functional tests
  and re-compute the outcome metrics on the bank's own data (SS1/23 2.6(b)(iii): "validate their own use of vendor products ... using
  their own outcomes"). AKR-TMS ships full documentation so that a vendor model is "fully understood" (CBUAE 4.7.4).
- **Two supervisors' tests, one answer**: the same pack answers the CBI-approved firm (CBI handbook D2 and D4), a correspondent's
  CBDDQ section on testing (file 04 section 2c) and NYDFS-style certification evidence (file 05 section 1f).
- **Findings tracker** with severity, owner, due date, repeat-finding flag (CBUAE 10.7.5: "Particular attention should be given to
  repeated findings"); quarterly to the committee, yearly to the Board.
- **Annual governance calendar**: self-assessment (SS1/23 1.7), validation and monitoring dates (CBUAE 10.5.2), scenario testing (HKMA
  2026), Board AI risk appetite review, policy review.

### 9i. Bias and fairness

**Why AML models need a fairness programme.** AML is the one banking decision where a protected characteristic can look like a
legitimate risk factor. The CBI's own controls make monitoring intensity depend on "nationality and links abroad" and geography (file
01 section 2d), and FATF high-risk-jurisdiction lists are mandatory inputs. The EU AI Act and the Commission's guidance draw the
line precisely: **a person must not be judged "on AI-predicted behaviour based solely on their profiling, personality traits or
characteristics, such as nationality, place of birth, place of residence ... without a reasonable suspicion ... based on objective
verifiable facts and without human assessment"** (Recital 42; Commission guidelines C(2025) 5052, paras 209-210, section 3a). The
UAE guidance says institutions must "ensure that such individuals or groups are not systematically disadvantaged unless the decisions
suggested by the models have a clearly documented justification" (CBUAE 3.111). MAS prescribes the method: list protected attributes,
test whether they are used, define at-risk groups, measure disadvantage, justify any use (MAS 6.3.15; draft guideline 4.8-4.9).

**Design rules**

1. **Protected-attribute register** (bank-configurable; default for Iraq): nationality, place of birth, governorate or region
   (including Kurdistan Region), gender, age band, religion, ethnicity and sect (these last two are never collected as features).
2. **Allowed-use matrix**: for each attribute and each model, one of: *prohibited as a feature*; *allowed only as a documented
   regulatory risk factor in a transparent rule* (for example FATF-listed country of the counterparty, CBI-listed jurisdictions);
   *allowed for monitoring only* (fairness reporting). Nationality and residence are never the sole reason for an alert or a hold;
   a nationality-based rule must also require a transactional or behavioural condition.
3. **Name-derived proxies are prohibited as model features.** Arabic and Kurdish names can reveal tribe, sect or ethnicity (file 22).
   Name data is used for screening and entity resolution only, never as an input to risk scores or alert priority. This is a design
   inference from the Iraqi context, not a sourced finding; it should be confirmed with the bank's compliance and legal teams.
4. **Proxy scan at validation**: test each feature for association with the protected attributes (for example governorate, branch or
   occupation acting as a proxy) and record the result in the model card.
5. **Outcome fairness metrics, monitored monthly** by group (nationality, governorate, gender, age band, customer type): alert rate,
   escalation rate, STR rate, false-positive rate, hold rate, time-to-clear, account-restriction rate. Report the ratio of each
   group's rate to the overall rate; an internal trigger of "outside 0.8 to 1.25" is proposed (adapted from the US "four-fifths"
   convention, not an AML rule) and opens a review that must conclude with a documented justification or a change. Small groups use
   confidence intervals to avoid false alarms.
6. **De-risking guard**: a customer-group exit or blanket restriction can only be proposed from the case-management side with an
   individual assessment, mirroring AMLR recital: "Under no circumstances should AML/CFT reasons be invoked to justify commercial
   decisions as regards prospective or existing clients" (file 05 section 2 when written; text in the AMLR 2024/1624 recitals).
7. **Feedback-loop control**: analyst decisions feed retraining only after quality filters; alerts the model suppressed are sampled
   and reviewed so the model cannot only learn from what it already flagged (AI Act 15(4); file 03 "trained-in" errors).

### 9j. Explainability for analysts, supervisors and customers, and human oversight

**Four audiences, four outputs**

| Audience | Needs | Output |
|---|---|---|
| Analyst | Why this alert, what to check first | Reason codes in Arabic and English (top contributing factors in plain language), the rule or feature behind each, the typology, the data rows and graph neighbours that triggered it (Wolfsberg "every alert must say why it fired") |
| MLRO and committee | Whether the system covers the risks and behaves as designed | Typology-to-feature coverage inventory, KPI pack, drift and fairness dashboards (Wolfsberg part II explainability lenses) |
| Supervisor, auditor, validator | How the model works and whether it was tested | Model card, validation pack, reproducible re-run |
| Customer-facing staff (where law or policy allows) | A safe explanation of a decision that affects the customer | A limited explanation that never reveals suspicion or the STR (EU AMLR 76(5)(c) exempts STRs; tipping-off rules in file 03) |

**Method choices.** Tree-based models are the practical default for scoring because per-feature contributions can be extracted
(MAS footnote 4); SHAP gives global and local attributions and LIME local ones (MAS footnote 55). Each reason code must map to a named
feature, a typology and a CBI article where one applies. **Fidelity and stability tests** belong in validation: remove or perturb the
top-ranked feature and confirm the score moves as claimed (fidelity); check that near-identical alerts get near-identical reasons
(stability); and check that reasons make business sense ("assess whether these features were intuitive", MAS 6.3.11). Reason codes for
rules are exact; for models they are attributions, and the interface says so.

**Human oversight, the design standard** (EU AI Act Article 14(4), MAS 4.10, AMLR 76(5)(b), CBI stop-until-cleared):

- Overseers can understand the model's capacities and limits (model card is one click from every alert).
- **Automation bias is designed against** (Article 14(4)(b); MAS "automation bias and decision fatigue"): analysts must open the
  evidence before an approve/close button activates for Tier 1-scored alerts; a random sample of low-score closed alerts gets second
  review; time-on-alert and closure-rate outliers are monitored; no analyst or team is measured on alerts closed per hour alone.
- **Override and disregard** are always possible, logged with a reason, and analysed (CBUAE 8.4; Article 14(4)(d)).
- **No solely automated adverse decision** (AMLR 76(5)(b)): the system may rank, recommend and request a hold; STR filing, account
  restriction and closure require a named human decision.
- **Stop button**: kill switch and fallback to rules-only (MAS 6.5.6; Article 14(4)(e); CBUAE 3.101).
- **Documentation of oversight**: overrides, near misses and incidents are recorded and reviewed (MAS 4.10(d)).

### 9k. Generative AI governance (Track C)

Regulatory anchors: MAS (human-in-the-loop for GenAI that aids decisions, guardrails, task-archetype validation, third-party
controls; section 4b); the US 2026 guidance leaves GenAI outside model risk management (file 05 section 1c) so bank policy must fill
the gap; NIST AI 600-1 (twelve GenAI risks); OWASP LLM Top 10 2025; AMLR 75(g) (AI-generated information shared only after adequate
human oversight); HKMA June 2026 (agentic AI workshop; governance guardrails); SAFR (runtime authority checks for agents).

**Permitted uses (Phase 1):** case summaries; draft STR narratives; Arabic-English translation of narrative fields; extraction of
entities and facts from documents and adverse media; next-best-action suggestions; query assistance. **Not permitted:** deciding
whether an alert is suspicious; closing alerts; filing or submitting an STR; contacting customers; changing rules, thresholds or
models; writing to the CBS (which AKR-TMS can never do anyway). "L1 alert review" by an LLM is allowed only as a *recommendation with
cited evidence* that a human accepts or rejects, in Tier 3 form until validated, and its recommendations must never lower the human's
review depth.

| Control | Specification |
|---|---|
| **Prompt and output logging** | Every call records user, case ID, timestamp, model name and exact version, prompt template ID and version, the retrieved context (record IDs, not copies where possible), the output, and the human's edits and final decision. Retention follows STR records (file 01: at least 5 years). Logs are tamper-evident and searchable (MindForge/SAFR "traceability through searchable logging"). |
| **Grounding and citation** | Outputs are generated only from records of the case and the bank's own data passed in the prompt; each factual statement carries a link to a source record; a post-check rejects numbers, dates, names or account references that do not appear in the supplied records (NIST "confabulation"). Unsupported claims are marked or removed. |
| **Hallucination controls** | Structured extraction before narrative (fields first, prose second); temperature at the low end; a deterministic verifier that re-reads every cited figure; refusal when evidence is insufficient; separate evaluation sets in Arabic (including Iraqi dialect terms, transliterated names) and English with expert-written reference summaries and factual-error counting (MAS 7.1: evaluation is harder for text and needs human feedback). |
| **Human review and attribution** | Every draft is stamped "AI-generated draft"; the analyst must edit or accept before it can be saved to the case or exported; the final text records who is responsible; edit-rate and error-rate are monitored (section 9d). No AI text goes to the FIU, a customer or another bank without human sign-off (AMLR 75(g)). |
| **Confidentiality and tipping-off** | LLM outputs are internal; the system blocks any path from an LLM output to a customer-facing channel; suspicion content is never sent to external model APIs. Default deployment runs the model on the bank's premises (file 30). |
| **Prompt-injection and data-poisoning defence** (OWASP LLM01, LLM04, LLM05) | Free-text fields in payments and KYC records (remittance purpose, beneficiary names, document text) are untrusted input; they are passed as data in delimited blocks, never as instructions; outputs are treated as untrusted before display or use; no tool access from the summariser. |
| **Excessive agency** (OWASP LLM06; SAFR) | Any agentic workflow runs with least privilege, an explicit allow-list of actions, a runtime policy check before each action, timeouts, a kill switch, and a full action log. Agents may read the case and propose; they may not write to cases, lists or configuration without a human approving the specific action. |
| **Versioning and change** | Model version pinned per bank; prompt templates under change control; any model or prompt change runs the evaluation suite and shadow mode before promotion (MAS 7.1, 6.5.10). Vendor model updates are not accepted silently (MAS 7.2 clause). |
| **Tiering** | Drafting aids are Tier 3 with mandatory human edit; an LLM component that influences alert priority or recommends closure is Tier 2 at least. |
| **Third-party model risk** | A model-supplier checklist: provenance, licence, training-data statement, security testing results, right to audit, notification of versions, exit plan; open-weight models run on-premises are preferred (file 32). |

## 10. What it means for AKR-TMS

1. **Governance is a product module, not a document set.** The blueprint of section 9 needs its own data model: an inventory and
   evidence store (objects, versions, tiers, owners, tests, findings, incidents, change requests, approvals), an immutable event log,
   a monitoring service and a workflow engine for the oversight committee and the change board. These sit beside detection in the
   same PostgreSQL-based platform (file 32 maps the stack), are read-only for auditors, and export to PDF and structured files for
   the CBI-approved firm and for correspondents.
2. **Three tracks, three weights.** Rules and parameter sets (Track A) get configuration governance with functional testing, ATL/BTL
   and change control. Models (Track B) get full model-risk treatment, tiered. GenAI (Track C) gets human-review, logging and
   containment controls. This is the structure the US 2026 guidance, the UK Finance paper and the MAS papers converge on, and it keeps
   effort where the risk is (Wolfsberg: financial-crime models should not be forced into prudential tiers).
3. **Meet the strictest regional text by default, and say so.** The CBUAE Model Management Standards (November 2022) cover AML models
   explicitly and are the closest binding standard in the region; PRA SS1/23 and MAS are the international references; the CBI has
   no AI or model-risk rule but expects a CBI-approved firm to verify AML tooling and the three lines of defence. A product that
   satisfies the CBUAE text will satisfy a CBI assessor, a correspondent's auditor and a Gulf parent bank. Treat this as a selling
   point: "assessor-ready out of the box".
4. **Transparency is the vendor's obligation, because ownership stays with the bank.** CBUAE 4.7: a methodology "not fully
   understood" is "not ... fit for purpose"; SS1/23 2.6: the bank validates its own use of a vendor model with its own outcomes; MAS
   7.2: audit rights and change notification. AKR-TMS therefore ships documentation, feature lists, training-data descriptions and a
   bank-run validation harness, runs on the bank's premises, and never ships a model the bank's validator cannot re-run.
5. **Scarce labels turn into a governance rule, not an excuse.** CBUAE 2.2.7 and 4.7.5 let a bank start on an externally calibrated
   model but require it to collect local data immediately and replace or recalibrate. AKR-TMS starter models are labelled that way and
   the product tracks the local-recalibration plan (file 21 section 16 on how to do it with few labels).
6. **Fairness has to be designed for Iraq, not copied from the EU.** The CBI itself lists nationality and links abroad as monitoring
   inputs; Iraqi names can encode tribe, sect and ethnicity. The product keeps those out of model features, allows nationality and
   geography only as documented, transparent risk factors that never act alone, and measures outcomes by group every month.
7. **Human oversight is a design pattern.** CBI stop-until-cleared, EU AMLR 76(5)(b), MAS 4.10 and AI Act Article 14 all point the
   same way: the system proposes, a named human decides, the human can disregard and stop, and the interface fights automation bias.
8. **Phase 2 raises the stakes.** A pre-release scoring API that can hold payments moves every model it uses to Tier 1: it needs
   latency and availability targets, a recorded fail-open or fail-closed policy, a tested fallback to rules, and a decision log for
   each hold. Do not enable holds from an ML score alone until the model has run in shadow mode and passed a BTL review at the bank.
9. **Open questions for the CBI and the first banks.** (a) Does the CBI expect model-validation evidence for AI/ML under the 2025
   controls, and what will the CBI-approved assurance firms test? (b) Does the CBI regard AML tuning as a change that needs the
   AML/CFT Office's consent (the 2025 controls say banks may "develop new scenarios under the supervision of, and in coordination
   with, the AML/CFT Office", file 27)? (c) Will banks accept an LLM running on premises, and on what hardware (file 30 and file 32)?
   (d) Who in an Iraqi bank can act as an independent validator, and can AKR-TMS or a partner offer validation-as-a-service without
   breaching independence?

## 11. Derived requirements (GV-xx)

| ID | Requirement | Source |
|---|---|---|
| GV-01 | Every rule, parameter set, model, matcher, reference list and LLM component has an inventory record (fields in 9a) before it can be activated; activation without a record is blocked | PRA SS1/23 1.2; CBUAE MMS 4.4; MAS 2024 paper 5.2 |
| GV-02 | A new calibration or material variation gets a new unique ID; retired versions are archived and can be rolled back to | CBUAE MMS 4.4.2-4.4.3 |
| GV-03 | Tiering grid (decision impact, coverage, autonomy, complexity, data novelty, regulatory exposure, fairness exposure) with at least two tiers, validator override with recorded reason, and periodic review of the grid itself | SS1/23 1.3; CBUAE 4.4.5; MAS 5.3 |
| GV-04 | Every object has a named owner, user, developer, validator and approving committee | SS1/23 2.4; CBUAE 4.5 |
| GV-05 | Starter models are flagged "externally calibrated, temporary"; each bank deployment carries a local data-collection and recalibration plan with a due date | CBUAE MMS 4.7.5, 2.2.7 |
| GV-06 | Model Oversight Committee workflow: quarterly auto-built pack (monitoring and validation dates, findings, overrides, incidents), logged decisions, yearly Board report | CBUAE MMS 3.1.6, 10.5.2, 10.7.5; SS1/23 2.1(d) |
| GV-07 | Board-approved AI and model risk appetite statement template covering accepted miss rate, performance limits, fairness limits and autonomy limits | SS1/23 2.1(c); HKMA June 2026; Wolfsberg part II |
| GV-08 | Annual self-assessment pack against PRA SS1/23, CBUAE MMS and MAS guidelines, with a control crosswalk to NIST AI RMF and ISO/IEC 42001 | SS1/23 1.7; CBUAE 2.2.2; NIST; ISO 42001 |
| GV-09 | Validation pack with seven parts (conceptual soundness, data, implementation, outcomes, sensitivity, ATL/BTL, benchmark) and the CBUAE report fields | SS1/23 4.2-4.5; CBUAE 10.3, 10.4, 10.6.1 |
| GV-10 | Independent re-implementation of each rule from its functional specification with alert-level diff against production | HKMA April 2024 section 07; SS1/23 4.3 |
| GV-11 | Regression suite of seeded cases per CBI scenario, run on every release and every configuration change | HKMA 07; NYDFS Part 504 (file 05 1f) |
| GV-12 | Threshold-tuning workbench offering standard-deviation, percentile, robust, clustering, sensitivity and ATL/BTL analyses, stored against each change request | HKMA April 2024 section 06 |
| GV-13 | BTL sampling-plan generator (stratified, with rare-event upper bounds) that includes cases from non-system routes (branch referrals, RFIs, authority requests) | Wolfsberg part I (file 04 2a); HKMA 06 |
| GV-14 | Each model is benchmarked against a transparent baseline on the bank's data; no go-live if it does not beat the baseline without documented reason | SS1/23 3.3(b)(iii); CBUAE 4.8.2 |
| GV-15 | Validate the model family once per version, then run a per-bank delta validation (data mapping, calibration, thresholds, feedback settings) | Wolfsberg part II (file 04 2b); SS1/23 2.6(c) |
| GV-16 | Evaluation uses out-of-time splits and label-lag-aware windows; metrics include confidence intervals and by-segment results | MAS 2024 paper fn 52; SS1/23 3.3 |
| GV-17 | Critical-data-element registry with daily reconciliation, lineage checks, schema-drift alarm on the CBS replica, and an automatic test run after any CBS change | HKMA April 2024 sections 03-04; NYDFS Part 504 |
| GV-18 | Monitoring of data drift, score drift, alert volume, productivity, overrides, fairness, data quality and LLM quality with green, amber, red thresholds set at validation | MAS 6.5.3-6.5.4; SS1/23 4.4 |
| GV-19 | Incident log from discovery to resolution; a red breach forces a recorded decision (retrain, recalibrate, restrict use, or fall back to rules-only) | MAS 6.5.5; SS1/23 5.2-5.3 |
| GV-20 | Per-model kill switch and rules-only fallback, executable by the model owner at once and tested quarterly | MAS 6.5.6; CBUAE enabling-tech 3.101; AI Act 14(4)(e) |
| GV-21 | Analyst override logging with reason, and override-rate limits per model and segment; a breach marks the model for fitness review | CBUAE MMS 8.4; SS1/23 4.4(c) |
| GV-22 | Review calendar: rules every 12 months; Tier 1 monitoring monthly and validation yearly; Tier 2 quarterly and two-yearly; trigger reviews (alert surge, new product, new typology, CBS change) | HKMA April 2024 section 08; CBUAE 10.5 |
| GV-23 | Quarterly KPI pack (alerts, productive cases, false-positive rate, STR rate, coverage, fairness) to the oversight committee | HKMA April 2024 section 08 |
| GV-24 | Shadow mode for every new or changed Tier 1-2 model and rule, comparison report, success criteria agreed in advance, and no mandatory parallel run | MAS 6.5.1 (forward testing); Wolfsberg part II; FATF 2021 (file 03) |
| GV-25 | A permanent challenger runs against every Tier 1 champion | SS1/23 3.3(b)(iii); Wolfsberg part I |
| GV-26 | Three change classes (minor, material, emergency), maker-checker, impact analysis, a flag for changes whose main effect is fewer alerts, and post-change reviews at 30 and 90 days | NYDFS Part 504 (file 05 1f); MAS 6.5.9; SS1/23 3.6 |
| GV-27 | Version control of code, parameters, thresholds, data snapshots, feature definitions, prompts and weights, with reproducible rebuild and rollback in one change window | MAS 6.5.9; CBUAE enabling-tech 3.108 |
| GV-28 | Dynamic-model policy: automatic retraining only on newer data, architecture and hyperparameters frozen, behind a validation gate; all other changes are material | MAS 6.5.10 |
| GV-29 | Banks can pin a version; AKR-TMS gives advance notice of every model, data-feed or prompt change | MAS 7.2; SS1/23 2.6 |
| GV-30 | Auto-generated model cards (ten sections) and rule cards, meeting the "independent third party can replicate" test | SS1/23 3.5; CBUAE enabling-tech 3.103; MAS 6.3.16 |
| GV-31 | Plain-language explanation of every rule and threshold generated from its configuration | FFIEC (file 05 1d); CBUAE MMS 3.1.13 |
| GV-32 | Role separation enforced in software (proposer is not approver; developer cannot sign validation; auditor read-only with full history) | CBUAE MMS 3.1.9, 3.1.12; SS1/23 4.1(d) |
| GV-33 | Validator workspace with data snapshot, code access and a harness to re-run functional tests and outcome metrics on the bank's own data | SS1/23 2.6(b)(iii); CBUAE 4.7.4 |
| GV-34 | Findings tracker with severity, owner, due date and repeat-finding flag; quarterly to committee and yearly to Board | CBUAE MMS 10.7.5 |
| GV-35 | One evidence pack serves the CBI-approved firm, correspondent questionnaires and annual certification | CBI handbook D2, D4 (file 01 2e); Wolfsberg CBDDQ (file 04 2c); NYDFS 504 |
| GV-36 | Protected-attribute register and allowed-use matrix per model; nationality, residence and geography are never the sole trigger of an alert, hold or score | AI Act recital 42 and Commission guidelines paras 209-210; CBUAE enabling-tech 3.111; MAS 6.3.15 |
| GV-37 | Name-derived and sect, tribe or ethnicity proxy features are prohibited in models; a proxy scan is part of validation | Design inference (file 22); SS1/23 3.2(b) |
| GV-38 | Monthly outcome fairness metrics by customer group with an internal review trigger and a required justification or change | MAS 6.3.15; CBUAE enabling-tech 3.106(c) |
| GV-39 | Retraining labels pass quality filters, and a sample of model-suppressed alerts is reviewed so the model does not learn only from what it flagged | AI Act 15(4); FATF 2021 (file 03) |
| GV-40 | Purpose binding: scores and features tagged AML/CFT-only; exports to credit, marketing or other uses blocked unless an exception is approved | Wolfsberg principle 1 (file 04 2e); Commission draft guidelines para 309 |
| GV-41 | Reason codes in Arabic and English for every alert, each tied to a feature, typology and CBI article; global and local attribution for models; fidelity and stability tests in validation | Wolfsberg part II; MAS 6.3.10-6.3.11 |
| GV-42 | Automation-bias controls: evidence opened before closure of Tier 1-scored alerts; random second review of closed low-score alerts; throughput never the sole analyst KPI | AI Act 14(4)(b); MAS draft 4.10 |
| GV-43 | No solely automated adverse decision: STR filing, account restriction and closure require a named human decision; the system may rank, recommend and request a hold | EU AMLR 76(5)(b); CBI 2025 controls (file 01 2d) |
| GV-44 | Customer-facing explanation template that never reveals suspicion or an STR | EU AMLR 76(5)(c); tipping-off rules (file 03) |
| GV-45 | Technically enforced GenAI permitted-use list (summaries, drafts, translation, extraction, suggestions) and forbidden-use list (deciding, closing, filing, contacting customers, changing configuration) | MAS 7.1.9; SAFR; file 05 1c |
| GV-46 | Prompt and output logging: user, case, time, model and version, template and version, retrieved record IDs, output, human edits and final decision; tamper-evident; retained at least as long as STR records | SAFR/MindForge; MAS draft 4.23(d); CBI record rules (file 01) |
| GV-47 | Grounding and verification: every factual statement cites a source record and a deterministic post-check rejects unsupported figures, dates, names and account references | NIST AI 600-1 (confabulation); MAS 7.1 |
| GV-48 | AI-generated drafts require analyst edit or acceptance before save or export; nothing AI-generated leaves the bank without human sign-off | EU AMLR 75(g); MAS 7.1.9 |
| GV-49 | Prompt-injection and excessive-agency controls: untrusted free-text handled as data, least-privilege tools, allow-listed actions, runtime policy check, timeouts, kill switch, full action log | OWASP LLM01, LLM05, LLM06; SAFR; MAS draft 4.23 |
| GV-50 | LLM evaluation suite in Arabic and English with expert reference summaries and factual-error counts, run before any model or prompt change | MAS 7.1; NIST AI 600-1 |
| GV-51 | Third-party model supplier checklist (provenance, licence, training-data statement, security tests, audit right, version notice, exit plan); open-weight models on premises preferred | MAS 7.2; SS1/23 2.6; CBUAE 4.7 |
| GV-52 | GenAI components run on the bank's premises by default; no suspicion content to external model APIs | MAS 7.1; file 30 |
| GV-53 | Phase 2 pre-release API: Tier 1 by default, latency and availability targets, a recorded fail-open or fail-closed policy, tested rules fallback, decision log for every hold, and no ML-only holds before shadow mode and a BTL review at the bank | CBI stop-until-cleared (file 01 2d); MAS 6.5.6; SS1/23 5.2 |
| GV-54 | Rule-secrecy roles: threshold values visible only to a documented set of roles; branch staff see reason codes only | SAMA Counter-Fraud 3.8(d)(5); Wolfsberg principle 5 |

## 12. Sources

Primary regulator and standard-setter texts (read in full or in the cited parts on 2026-10-06):

- [PRA SS1/23 Model risk management principles for banks (May 2023), PDF](https://www.bankofengland.co.uk/-/media/boe/files/prudential-regulation/supervisory-statement/2023/ss123.pdf) and [landing page](https://www.bankofengland.co.uk/prudential-regulation/publication/2023/may/model-risk-management-principles-for-banks-ss)
- [FCA: our approach to AI (first published 8 Sep 2025, updated 2 Oct 2026)](https://www.fca.org.uk/firms/innovation/ai-approach)
- [FCA press release: second cohort for AI Live Testing (April 2026)](https://www.fca.org.uk/news/press-releases/fca-announces-second-cohort-ai-live-testing)
- [Regulation (EU) 2024/1689, the AI Act, OJ text](https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=OJ:L_202401689)
- [Regulation (EU) 2026/1744, Digital Omnibus on AI (OJ 24 July 2026)](https://eur-lex.europa.eu/legal-content/EN/TXT/HTML/?uri=OJ:L_202601744)
- [Commission draft guidelines on high-risk classification under Article 6 (19 May 2026), library page](https://digital-strategy.ec.europa.eu/en/library/draft-commission-guidelines-classification-high-risk-ai-systems) and [annex PDF](https://ec.europa.eu/newsroom/dae/redirection/document/128561)
- [Commission Guidelines on prohibited AI practices, C(2025) 5052 (29 July 2025), library page](https://digital-strategy.ec.europa.eu/en/library/commission-publishes-guidelines-prohibited-artificial-intelligence-ai-practices-defined-ai-act) and [PDF](https://ec.europa.eu/newsroom/dae/redirection/document/112367)
- [Regulation (EU) 2024/1624, the AML Regulation (OJ text; local copy amlr-2024-1624 in the research folder)](https://eur-lex.europa.eu/eli/reg/2024/1624/oj)
- [EBA factsheet: AI Act implications for the EU banking and payments sector (21 Nov 2025)](https://www.eba.europa.eu/sites/default/files/2025-11/d8b999ce-a1d9-4964-9606-971bbc2aaf89/AI%20Act%20implications%20for%20the%20EU%20banking%20sector.pdf)
- [MAS Veritas Initiative page (FEAT, phases, toolkit)](https://www.mas.gov.sg/schemes-and-initiatives/veritas)
- [MAS Information Paper: AI Model Risk Management, thematic review (December 2024), PDF](https://www.mas.gov.sg/-/media/mas-media-library/publications/monographs-or-information-paper/imd/2024/information-paper-on-ai-risk-management-final.pdf)
- [MAS Consultation Paper P017-2025: Guidelines on AI Risk Management (November 2025), PDF](https://www.mas.gov.sg/-/media/mas-media-library/publications/consultations/bd/2025/final_consultation_paper_on_guidelines_on_ai_risk_management_forrelease.pdf) and [consultation page](https://www.mas.gov.sg/publications/consultations/2025/consultation-paper-on-guidelines-on-artificial-intelligence-risk-management)
- [MAS / industry white paper: Safeguards for Agentic Finance at Runtime, SAFR v1.0 (July 2026)](https://www.mas.gov.sg/-/media/mas-media-library/development/fintech/ai-safr/safr.pdf)
- [HKMA circular 9 Sep 2024: Use of AI for Monitoring of Suspicious Activities](https://brdr.hkma.gov.hk/eng/doc-ldg/docId/getPdf/20241122-3-EN/20241122-3-EN.pdf) and [its Annex on benefits](https://brdr.hkma.gov.hk/eng/doc-ldg/docId/getPdf/20241122-4-EN/20241122-4-EN.pdf)
- [HKMA thematic review, Insights for Design, Implementation and Optimisation of TM Systems (17 Apr 2024)](https://brdr.hkma.gov.hk/eng/doc-ldg/docId/getPdf/20240417-2-EN/20240417-2-EN.pdf)
- [HKMA circular 22 Jun 2026: Supporting Adoption of AI in Fighting Financial Crime](https://brdr.hkma.gov.hk/eng/doc-ldg/docId/getPdf/20260622-1-EN/20260622-1-EN.pdf) and [report](https://brdr.hkma.gov.hk/eng/doc-ldg/docId/getPdf/20260622-2-EN/20260622-2-EN.pdf)
- [CBUAE Model Management Standards (version date November 2022), rulebook](https://rulebook.centralbank.ae/en/rulebook/model-management-standards) and [PDF](https://rulebook.centralbank.ae/sites/default/files/en_net_file_store/CBUAE_EN_4881_VER1.pdf)
- [CBUAE/SCA/DFSA/FSRA Guidelines for Financial Institutions adopting Enabling Technologies (15 Nov 2021)](https://rulebook.centralbank.ae/en/rulebook/guidelines-financial-institutions-adopting-enabling-technologies)
- SAMA Counter-Fraud Framework (No. 000044021528, 11 Oct 2022): [3.8 Counter-Fraud Technology](https://rulebook.sama.gov.sa/en/38-counter-fraud-technology), [5.1 Fraud Detection Standards](https://rulebook.sama.gov.sa/en/51-fraud-detection-standards), [5.2 Fraud Detection Systems](https://rulebook.sama.gov.sa/en/52-fraud-detection-systems)
- [NIST AI 600-1, Generative AI Profile (July 2024)](https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.600-1.pdf)
- [ISO/IEC 42001:2023 AI management system (ISO page)](https://www.iso.org/standard/42001)
- [OWASP Top 10 for LLM Applications 2025](https://genai.owasp.org/llm-top-10/)
- CBI: [AML/CFT/CPF supervisory controls, 8 Jan 2025](https://cbi.iq/static/uploads/up/file-173666439114818.pdf); CBI standards handbook 2025 and risk-management controls of 13 July 2026 as listed in file 01 (local copies cbi-handbook-2025 and cbi-risk-commercial-2026).
- Files in this folder: 01 (CBI), 03 (FATF new technologies 2021), 04 (Wolfsberg parts I and II, AI/ML 2022, audit 2024), 05 (US model risk, FinCEN, NYDFS 504).

Secondary sources (graded Secondary; used only for items marked so in the text):

- [UK Finance blog, 20 Feb 2026: why and how financial crime models fit within your model risk framework](https://www.ukfinance.org.uk/news-and-insight/blog/beyond-compliance-why-and-how-financial-crime-models-fit-within-your-model) (page blocks automated download; content via fetch-tool summary)
- [Press echo of the UK Finance blog, 24 Feb 2026](https://letsdatascience.com/news/uk-finance-urges-integrating-financial-crime-models-0dbd5ec3)
- [Resultsense summary of FCA second cohort, 22 Apr 2026](https://www.resultsense.com/news/2026-04-22-fca-second-cohort-ai-live-testing/)
- [DLA Piper summary of the Commission's draft high-risk guidelines (June 2026)](https://www.dlapiper.com/en/insights/publications/2026/06/eu-commission-draft-guidelines-on-classification-of-high-risk-ai-systems-key-points)

Not reached or not verified: Central Bank of Jordan, Qatar Central Bank, Central Bank of Bahrain, Central Bank of Egypt and Oman instruments on AI or model risk; SDAIA national AI principles; HKMA 2019 high-level principles on AI and 2024 generative-AI consumer-protection guidance; NIST AI RMF 1.0 text (cited from background knowledge); final text of the MAS AI risk management guidelines (not issued as of 2026-10-06 as far as could be found); the FCA's own page on the second cohort was read through its press-release title only (details from secondary summaries).
