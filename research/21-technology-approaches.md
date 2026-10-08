# 21 — Detection technology approaches: rules, machine learning, graphs, GenAI, data sharing, datasets and metrics

Status: done (2026-10-06)

Requirement IDs use the prefix T-xx (this file). Governance and model-risk content (inventory, tiering, validation,
drift, change management, human oversight, GenAI governance) is in file 29 (prefix GV). Published bank results from AI in
AML are in file 12 (section 8): this file cross-references them and does not repeat them. The scenario catalogue the CBI
requires as a minimum is in file 27. Arabic name matching is in file 22. Wolfsberg effective-monitoring papers are in file
04 (sections 2a, 2b, 2e); the US model-risk and FinCEN effectiveness items are in file 05 (sections 1a, 1c).

Evidence grades used where it matters: Primary (regulator publication, official PDF, dataset documentation, peer-reviewed
paper) / Secondary (reputable press, consultancy, vendor-neutral analysis) / Vendor (marketing, case study) / Unverified
(could not confirm; stated as such).

## Contents

1. What detection has to do in Iraq: the constraints that decide the technology mix
2. Rules and scenarios (thresholds, typology rules, tuning, the 80 percent problem)
3. Segmentation and peer groups
4. Supervised machine learning (alert scoring, SAR propensity) and the label problem
5. Unsupervised and semi-supervised anomaly detection
6. Graph and network analytics
7. Entity resolution
8. Customer risk scoring (static, dynamic, event-driven)
9. Alert prioritisation, triage, hibernation and suppression
10. Real time versus batch (Phase 1 post-event, Phase 2 pre-release API)
11. Explainability: reason codes, SHAP, typology-to-feature coverage
12. GenAI and agentic AI in investigations, and what regulators said in 2024-2026
13. Privacy-enhancing technologies and data sharing (federated learning, Swift, COSMIC, TMNL, UK and EU provisions)
14. Public benchmarks and datasets (IBM AMLworld, SAML-D, Elliptic, others)
15. Evaluation metrics (precision, recall, alert-to-case, STR conversion, time to detect, coverage)
16. What works with few labelled cases and a cash-heavy economy
17. What it means for AKR-TMS
18. Derived requirements (T-xx)
19. Sources

## 1. What detection has to do in Iraq: the constraints that decide the technology mix

Detection technology is chosen by what the bank must do, what data it has and who will examine the result. For Iraqi banks the
earlier files already fix most of those constraints; this table gathers them, with the technology consequence of each.

| Constraint | Evidence (file) | Consequence for the technology mix |
|---|---|---|
| The CBI's scenario catalogue is the legal **minimum**: 21 cash, 16 transfer and 49 general scenarios plus product articles; the 2024 MER found that **no interviewed institution had built any scenario of its own** and that the bank set had grown from 34 to 56 scenarios | 27 Part A; 01 section 2 (MER paras 563-582) | Rules come first and stay always-on, each mapped to its CBI article; everything else sits on top and never replaces them |
| The CBI's 2025 controls require the AML system to "use AI and machine-learning algorithms", compare **actual with declared activity**, run **real-time follow-up** and **stop** a transaction that matches a scenario until the MLRO clears it | 01 section 2d | ML must be explainable (reason codes), the profile-versus-actual comparison is a first-class feature, and Phase 2 (pre-release API) is needed for the stop control |
| Labelled outcomes are scarce: banks filed 628 money-laundering STRs in 2018-2022 (74 banks, about 1.7 per bank per year) and **451 STRs in 2024 across about 76 banks (about 6 per bank per year)**; the FIU gives only acknowledgements, not outcomes | 01 sections 2 and 5 | A bank cannot train a supervised model on its own STRs; use rules as weak labels, pooled or synthetic data, semi-supervised and unsupervised methods (sections 4, 5, 16) |
| Banks archive "large numbers of alerts ... daily against very few STRs"; first review is by a **branch liaison officer**, second review by the central AML department | 01 section 2 | The first product payoff is prioritisation and consolidation of alerts (section 9), and the workflow must model the two-tier review |
| The economy is cash-heavy and dollarised: cash about 42% of GDP (2017 estimate, about USD 73 billion); hawala, cash couriers, exchange companies, dollar smuggling, trade-based laundering, fake import invoices | 02; 01 section 1 | Cash-aware features, separate IQD and USD behaviour, expected-versus-actual profiles, exchange-company and trade segments, seasonal baselines |
| Data is read-only from each bank's CBS, with variable quality and schema; CBS changes can silently create monitoring gaps (HKMA) | 25 (CBS integration); file 29 section 5b | A critical-data-element registry, reconciliation and drift alarms are part of detection; models must tolerate missing attributes |
| On-premises deployment is the default; no cloud ML service | brief; 01 section 8 | CPU-friendly methods (gradient boosting with graph features, not GPU-bound deep graphs) and on-premises LLMs |
| Arabic-first, with Kurdish and Latin variants | 22 | Name matching and text analytics must be built for Arabic script; LLM features evaluated in Arabic |
| A CBI-approved firm verifies AML tooling; correspondents ask for validation evidence | 01 section 2e; 04 section 2c | Every model carries evidence by design (file 29) |

**The mix that follows** (developed in section 17): CBI scenario rules as the transparent base; segmentation and expected-versus-actual
profiles to set sensible thresholds; a scoring layer that **ranks the alerts the rules produce** (not a replacement for them);
unsupervised and graph detectors to find what the rules were never written for; supervised STR-propensity models only where enough
labels exist (pooled across banks or synthetic-pretrained); and GenAI strictly for assistance. This matches the hybrid that the Wolfsberg
Group recommends: "rules (well-defined risks and automatable reporting requirements), supervised models (where good investigation
labels exist) and unsupervised models (unknown and emerging typologies)" (file 04 section 2b).

## 2. Rules and scenarios

### 2a. What rules are good at, and where they fail

Rules remain the right base for three reasons: they are mandated (the CBI list), they are transparent to analysts, auditors and
supervisors, and they can be written the day a new typology appears. SAMA's counter-fraud text makes the last point explicit: "New
rules can be implemented on a timely basis to target prevention and detection of new or emerging typologies" (file 29 section 7b).
Their weakness is documented by every source read:

- **Static, wide nets and defensive habits.** The Wolfsberg Group names the habits: expanding coverage "even when the data show they
  produce almost no escalations", "no SAR left behind" tuning, and defensive filing (file 04 section 2a). The FATF 2021 report says
  most firms still run largely rule-based systems, which "has led to defensive compliance" (file 03 section 5a).
- **Alert volume without yield.** The HKMA says false positives are "a long-standing industry pain point" and that static rules "require
  constant manual intervention for adjustment to cover the latest red flags and typologies" (file 29 section 5a). The MER describes
  large daily alert archives against very few STRs in Iraq (section 1 above). Published false-positive rates are rarely given in the
  regulator texts read; the "90 to 95 percent false positives" figure often quoted by vendors could not be traced to a primary source
  in this research and is **Unverified**.
- **Little local tailoring.** In Iraq the rules are the CBI's standard set, and banks have not tuned or extended them (MER).

### 2b. Rule families the engine must support

The catalogue in file 27 lists the CBI scenarios. Technically they reduce to a small number of families, which is what the rule engine
must implement well:

| Family | Pattern | Example CBI scenario (file 27) | Data needed |
|---|---|---|---|
| Single-event threshold | One transaction above a limit, by currency and channel | Large cash deposit inconsistent with activity (cash 1) | Transactions, customer type |
| Windowed aggregate | Sum or count over a rolling or calendar window exceeds a limit | Repeated small cash deposits whose total does not fit the activity (cash 2) | Transactions, window logic |
| Structuring and splitting | Many amounts just under a threshold, across days, branches, accounts or depositors | Deposits in several stages to stay under the threshold (cash 12); repeated deposits at several branches (cash 4) | Transactions with branch, depositor, timestamp |
| Expected-versus-actual | Activity exceeds the KYC-declared income, turnover, cash limit or average | Cash withdrawal above the KYC limit (cash 16); monthly income above declared (cash 17); exceeding average movements (cash 14-15) | KYC declared profile, history |
| Rapid movement and pass-through | Funds leave soon after arriving; in-flow roughly equals out-flow | Repeated withdrawal soon after deposit (cash 7) | Transaction sequence, balances |
| Dormant and reactivated | Inactive account suddenly active | Large movements on dormant accounts (cash 6) | Account status and last-activity dates |
| Counterparty and geography | New, many-to-one, one-to-many, high-risk-jurisdiction counterparties | Equal transfers to several persons in different countries (transfers 3); repeated transfers to the same beneficiary (cash 19) | Counterparty fields, country, SWIFT data |
| Product- and channel-specific | LC, LG, credit, card, digital, FX, lockers, securities | Facility value moved abroad (cash 18); product articles 4-14 | Product tables |
| Customer-behaviour events | Observations a person records (customer takes back cash when asked for documents) | Cash 8 and the customer-behaviour list | Branch-entered events: needs a structured capture screen |
| Employee and vendor | Staff-related red flags | KYE and employee scenarios | HR and system-access data |

**Design points**

- **Declarative scenario definitions**: a scenario is data (family, parameters per segment and currency, window, exclusions, CBI article,
  typology, owner), not code. The engine compiles it to set-based SQL for batch and intraday runs and to incremental counters for the
  near-real-time path (section 10). The scenario catalogue then doubles as the coverage matrix the CBI asked for (file 27).
- **Alert evidence is frozen**: every alert stores the exact transactions, window, parameter version and data snapshot that produced it,
  so a validator can reproduce it (file 29 section 9c: functional testing) and an analyst sees the evidence without re-querying.
- **Windows have calendar semantics**: rolling versus calendar months, business days, public holidays and Iraq's pay and pilgrimage
  cycles change aggregates; the engine must support both and make the choice visible in the explanation.
- **Currency and unit handling**: IQD and USD are separate by default; conversion uses a governed rate table with a date; cash
  thresholds are set in the currency the CBI states.
- **Consolidation**: alerts for the same customer and period are grouped into one case candidate, with every contributing scenario
  listed (HKMA annex: holistic review by relationship "can be conducted by a single investigator").
- **Scenario lifecycle** as governed objects (file 29 sections 9a, 9f): draft, shadow, live, tuned, retired; the Wolfsberg-style
  retire-or-keep decision uses productivity data, but **the CBI's minimum scenarios are never retired**; only the bank's own extra
  scenarios can be.
- **Productivity per scenario**: alerts, cases, STRs, precision, time to close, share of analyst hours, overlap with other scenarios, and
  a "unique catches" count (cases found only by this scenario), because a low-volume scenario with unique catches earns its place.

**AKR-TMS hooks**

- Ship the CBI pack as the baseline scenario set with per-segment default parameters and a guided "add, test, tune your own scenario"
  workflow (a named MER gap).
- Scenario-quality dashboard: productivity, unique catches, overlap matrix, parameter history, with a recommendation list for the
  bank's extra scenarios only.
- Branch-event capture screen so that behaviour-based CBI scenarios (cash 8 and the customer-behaviour list) have data.

## 3. Segmentation and peer groups

### 3a. Why segmentation decides rule quality

A threshold only means something relative to a population. The HKMA's April 2024 review found that "appropriate customer
segmentation reduces noise and improves the quality of alerts", while inappropriate segmentation "led to less effective threshold
setting and tuning with higher volumes of false-positive alerts" and could leave "certain higher ML/TF risks ... un-monitored"
(file 29 section 5b). The Wolfsberg Group goes further: static segments (customer type, industry, inherent risk) "are outdated";
combine known attributes with **statistical clustering (k-means, Jenks)** recomputed on an ongoing basis (file 04 section 2a).

### 3b. A three-level segmentation for Iraqi banks

| Level | Basis | Examples for Iraq | Used for |
|---|---|---|---|
| Static business segment | Customer type, sector, product, channel | Salaried government employees paid through the bank, pensioners, private employees, small shops, wholesale and import traders, contractors, oil-sector suppliers, NGOs, exchange and money-transfer companies, government bodies, students, expatriate remittance recipients | Scenario applicability and default thresholds; the CBI's SDD/EDD groups |
| Behavioural cluster | Clustering of observed behaviour inside a business segment | Cash-heavy retail trader versus electronic-heavy trader; high-turnover pass-through account versus accumulator; USD-denominated versus IQD | Thresholds and peer comparison |
| Risk overlay | Customer risk rating (3 or 5 levels, CBI) | High-risk customers get "more stringent thresholds" (HKMA) | Monitoring intensity |

Inputs for the behavioural layer (computed per account or customer over 3, 6 and 12 months, in the log scale for amounts): total in and
out; transaction counts; average and median ticket; **cash share** of in and out; number of distinct counterparties and their
concentration; share of transfers abroad; **USD share**; share of third-party deposits; share of round amounts; branch spread; time-of-
month pattern (salary day concentration); balance turnover (movement divided by average balance); dormancy and reactivation.

**Methods**: k-means or Gaussian mixtures for multivariate clusters; hierarchical or HDBSCAN for irregular shapes; **Jenks natural
breaks** (one-dimensional optimal partition) for ordered metrics such as monthly turnover bands; decision-tree-based segmentation when
the bank wants rules a person can read. Choose the number of clusters with stability (re-run on different months and compare
assignments) rather than silhouette alone.

### 3c. Peer groups and "segment of one"

- **Peer comparison**: for each customer, the deviation of a metric from the segment's robust centre (median and median absolute
  deviation) and the percentile within the segment. This feeds profile-deviation rules and the unsupervised score (section 5).
- **Own-history comparison**: deviation from the customer's own trailing baseline, for customers whose behaviour is stable and who
  differ from their segment (a large trader). Large corporates and government bodies are "segments of one" and are judged on their own
  history and declared profile.
- **Segment migration as a signal**: a customer who moves between clusters (a salaried account that starts to behave like a pass-through
  trader) is itself an alert candidate and a trigger for KYC refresh (file 04 section 2q).
- **Minimum segment size**: below a minimum (for example 200 customers, a design proposal) merge the segment upwards; thresholds
  from tiny segments are noise.
- **Cash-economy seasonality**: segment baselines are calendar-aware (salary days, Ramadan and Eid, Arbaeen, harvest, school fees,
  Hajj and Umrah travel), otherwise every festival creates a flood of alerts and trains analysts to ignore them.

### 3d. Governance of segments

Segment definitions are governed objects (file 29): owner, rationale, last refresh, stability metrics, effect on alert volume and
fairness (are some governorates or nationalities over-represented in high-alert segments?). A segmentation change is a material change
(impact analysis of alert volumes by segment before activation; file 29 section 9f). Re-clustering runs on a schedule (quarterly
proposed) in shadow mode, and the bank approves the new assignment.

**AKR-TMS hooks**

- Segmentation workbench: define static segments, run behavioural clustering, inspect cluster profiles in plain language, compare
  stability across months, simulate the effect on each scenario's alert volume before activation.
- Per-segment and per-currency thresholds with percentile and robust-statistic suggestions (file 29 section 9c table).
- Segment-migration alerts and a KYC-refresh trigger when a customer's cluster changes.
- Calendar service (Iraqi and Islamic holidays, pay days, seasonal events) used by thresholds, baselines and explanations.

## 4. Supervised machine learning (alert scoring, SAR propensity) and the label problem

### 4a. Three different supervised tasks, with very different data needs

| Task | Unit and target | Labels needed | Feasible per Iraqi bank? |
|---|---|---|---|
| **Alert scoring** (secondary scoring of the alerts the rules produce) | Alert; target = escalated to case, STR filed, or closed | Every alert already has a disposition, so labels are plentiful; true positives are few | Yes, once enough dispositions exist; it ranks work and does not replace rules |
| **SAR/STR propensity** (customer or account level) | Customer in a time window; target = STR filed or exit for financial-crime reasons | STR or exit labels, which are rare | **No.** About 6 STRs per bank per year (2024, section 1) cannot train a model; needs pooled, synthetic-pretrained or weakly labelled data |
| **Transaction laundering classification** (benchmark style) | Transaction; target = part of a laundering pattern | Complete labels exist only in synthetic data | Only for pre-training and benchmarking (section 14) |

The Wolfsberg Group frames the same choice: use supervised models "where good investigation labels exist", train "on many crystallised
risks (manual cases, law-enforcement production orders), not only past automated SARs", and "balance recall and precision" because
"aiming for 100% recall ... is likely to lead to an ineffective system" (file 04 section 2a).

### 4b. What the public benchmark shows (IBM AMLworld, NeurIPS 2023; section 14)

Altman et al. trained gradient-boosted trees (LightGBM, XGBoost) on hand-built graph features (the "GFP" library) and several graph neural
networks (GIN, GIN with edge updates, PNA) on six synthetic datasets. Minority-class F1 (percent), from the paper's Table 2:

| Model | HI-Small | LI-Small | HI-Medium | LI-Medium | HI-Large | LI-Large |
|---|---|---|---|---|---|---|
| GIN | 28.7 | 7.9 | 42.3 | 3.9 | n/a | n/a |
| GIN + edge updates | 47.7 | 20.6 | 49.3 | 6.2 | n/a | n/a |
| PNA | 56.8 | 16.5 | 59.7 | 27.7 | n/a | n/a |
| GFP features + LightGBM | 62.9 | 20.8 | 59.5 | 20.9 | 48.7 | 17.1 |
| GFP features + XGBoost | 63.2 | 27.3 | 65.7 | 28.2 | 42.7 | 24.2 |

(HI = higher laundering rate, about 1 in 800-980 transactions; LI = lower, about 1 in 1,750-1,950; Table 4 of the paper.) Four lessons:

1. **Tree models with good graph features match or beat graph neural networks** ("near-par performance of PNA with GBT baselines, even
   without relying on handcrafted features"). The follow-up GFP paper reports that gradient boosting with its features "can detect
   illicit transactions with higher minority-class F1 scores than standard graph neural networks" and that CPU throughput "outperforms
   the graph neural network baselines executed on a powerful V100 GPU" (Blanuša et al., arXiv 2402.08593). For an on-premises product
   without GPUs this settles the default: **gradient boosting on engineered transaction and graph features**.
2. **A lower laundering rate roughly halves detection quality** (F1 falls from the 40-65 range to 4-28 in the LI sets). The true rate in
   real banks is unknown, because much laundering is never detected: the paper cites a study of a real Italian bank (180 million
   transactions over six months) that looked for two pattern types only, found 855 motifs, and so "around 1 laundering transaction per
   21,000" for those patterns; AMLworld's own gather-scatter and scatter-gather transactions come to a very similar 1 per 21,440.
3. **Transfer helps.** Models pre-trained on the HI datasets and applied to LI "enable higher F1 scores than those of XGBoost models
   trained on the respective LI datasets" for the medium set (PNA 36.6 versus 28.2 for XGBoost trained directly on LI-Medium), and fine-tuning for 5 epochs lifted LI-Small from
   0.0 to 27.4 (Table 3). This is direct evidence for pre-train then localise (section 16).
4. **Sharing helps.** In the paper's shared-versus-private experiment, banks "can achieve higher F1 scores by building shared machine
   learning models", and sharing the transaction graph as well "leads to even more significant improvements" (Figure 6). Section 13
   returns to how to share without moving data.

**The base-rate arithmetic every product owner should see** (simple Bayes, not from a source): at one laundering transaction per
21,000, a detector with 80 percent recall and a 1 percent false-positive rate flags about 10,000 innocent transactions per million for
38 true ones, a precision of about 0.4 percent. Reaching 10 percent precision would need a false-positive rate near 0.03 percent.
Transaction-level classification is therefore the wrong unit for production monitoring; customer-, account- and case-level units with
aggregated behaviour (and the rule alerts as candidates) give workable base rates.

### 4c. The label problem, item by item

| Problem | Why it arises in AML | Effect | Mitigation in AKR-TMS |
|---|---|---|---|
| **Scarcity** | About 6 STRs per bank per year (451 across about 76 banks in 2024; 628 from 74 banks over 2018-2022) | Per-bank supervised models are not credible | Pooled or federated learning, synthetic pre-training, weak supervision, unsupervised methods (sections 13, 16) |
| **Selection bias** | Only alerted activity is ever investigated and labelled | A model trained on dispositions learns to imitate the rules and analysts and stays blind to what the rules miss | Train alert scorers only to rank rule alerts; add labels from other routes (branch referrals, RFIs, authority requests, national-list matches, confirmed cases); sample unalerted activity for review (BTL) |
| **Noisy labels and defensive filing** | An STR means "suspected", not "proved"; some filings are defensive, and the MER reads some Iraqi banks as deliberately avoiding them | Models learn to flag what makes staff nervous, not what is criminal | **Internal re-grading** of a sample of historical STRs and closures by senior staff to separate real suspicion from defensive filing, as Wolfsberg part II recommends (file 04 section 2b); store a quality grade with each label |
| **Label delay and censoring** | Investigations and FIU action take weeks to months; the FIU gives acknowledgements only | Recent alerts look "negative" because outcomes are not in yet | Staged outcomes (alert, case, STR, FIU response, exit, freeze) with timestamps; train on matured cohorts; evaluate with lag-aware windows |
| **Feedback loops** | Alerts the model ranks low are reviewed late or never, so they generate no labels; analyst decisions then train the next model | The model reinforces its own blind spots (AI Act Article 15(4): "biased outputs influencing input for future operations") | Sample low-score alerts for forced review; use second-review labels; keep a challenger and a rules-only baseline |
| **Analyst inconsistency** | Different analysts close similar alerts differently; closures may reflect workload | Label noise; FATF's warning that errors in training data are "trained-in" (file 03 section 5a) | Controlled closure-reason vocabulary; dual review of a sample; inter-rater agreement tracked |
| **Leakage** | Features computed after the alert or from case notes | Inflated test scores that vanish in production | Point-in-time feature store; all features stamped with the data cut-off; leakage tests in validation |
| **Class imbalance** | 1 in 800 to 1 in 20,000 (section 4b) | Accuracy and ROC-AUC mislead | Report precision-recall measures; use cost-sensitive weights (AMLworld models "weighting the predictions of the minority class higher"); prefer rank-based operating points |
| **Drift and adaptive adversaries** | Criminals change behaviour when detected | Performance decays | Monitoring and scheduled retraining (file 29 section 9d) |

### 4d. A label ladder for AKR-TMS

| Level | Label | Typical volume | Reliability and bias | Use |
|---|---|---|---|---|
| L1 | Alert disposition (closed with reason, escalated) | High | Moderate; reflects analyst judgement and workload | Train alert scorers, with QA samples |
| L2 | Case outcome (STR filed, no filing, relationship exited) | Low | Higher, still noisy (defensive filing) | Pooled STR-propensity models; calibration |
| L3 | External feedback (FIU acknowledgement or grading, authority request, freeze order) | Very low | High but sparse; absent today for most STRs | Gold standard for evaluation; weights for L2 |
| L4 | Independent confirmed cases (RFI, production orders, national-list entries, convictions in the news) | Very low | High; independent of the bank's own rules | Test sets that measure what the rules miss; seeds for network labels |
| L5 | Synthetic labels (generators such as AMLworld or an Iraq-specific one) | Unlimited | Complete but not real | Pre-training, regression tests, stress tests, never the only evidence |

### 4e. Modelling choices that fit these constraints

- **Gradient-boosted trees as the default scorer** (LightGBM, XGBoost, CatBoost), which MAS names as the models banks use in AML
  (file 29 section 4b). Use **monotonic constraints** where the business logic is monotone (more cash against declared income never
  lowers risk), calibrate scores to probabilities (Platt or isotonic), and keep the feature set small and documented.
- **Transparent fallbacks for low data**: logistic regression with strong regularisation, or a generalised additive model such as the
  Explainable Boosting Machine (InterpretML, Nori et al., arXiv 1909.09223), where the contribution of each feature is exactly readable.
  Rudin's argument applies to a regulated, high-impact use: "Stop Explaining Black Box Machine Learning Models for High Stakes
  Decisions and Use Interpretable Models Instead" (arXiv 1811.10154). For Tier 1 models (file 29 section 9b), AKR-TMS should show that a
  glass-box baseline was tried and why the chosen model is better, if it is.
- **Stacking over the rules**: rule hits, counts and parameters become features, together with customer context (segment, age of
  relationship, KYC completeness, prior alerts and outcomes) and behaviour deviation scores (sections 3, 5). The model then learns which
  rule alerts matter for which customers, which is a realistic ambition with few labels.
- **Graph features rather than graph networks first** (section 6): fan-in, fan-out, cycle, gather-scatter counts and shared-attribute
  features computed in streaming fashion and passed to the tree model.
- **Operating point from capacity and cost**, not from an arbitrary 0.5: choose the threshold that maximises value for the bank's
  analyst capacity (expected cost = cost of a miss times misses + cost of a review times reviews), then confirm the miss rate with BTL
  sampling (file 29 section 9c).

### 4f. How many labels are enough? (design proposals, not regulator figures)

A common rule of thumb for stable regression coefficients is about 10-20 events per candidate variable; tree ensembles on 20-40
features need in the order of hundreds of positives to be credible, and calibration needs more. AKR-TMS proposes **gating by label
counts** per model and bank: below about 50 positives, do not train a bank-specific supervised model (use the pooled or pre-trained
starter, calibrated by recalibrating the intercept and thresholds on local data only); from about 200 positives, enable local fine-tuning
in shadow mode; from about 500, allow a local supervised challenger. These gates are starting points, to be tested on the pooled data.

### 4g. Evidence of effect from published bank results (see file 12 section 8)

File 12 collects customer-reported results (HSBC and others). One new item from the HKMA report of 22 June 2026 (customer-reported,
anonymised) is worth recording here: a global bank reported that its AI detection product **reduced false positives by 60 percent,
increased detection of suspicious activity by 2 to 4 times and shortened case investigation by about 50 percent**; a regional bank's
corporate-mule model rated 42 percent of a customer group as high risk where the rules had rated them medium, 70 percent of those were
escalated and 40 percent of the escalated customers were exited; and the report notes that the ML "improved risk detection even when
some data attributes were missing or data quality was suboptimal" (file 29 section 5c). These are results the banks chose to publish
and carry no independent audit, but they show the order of magnitude supervisors are being shown.

**AKR-TMS hooks**

- **Label registry**: every label carries source (L1-L5), grade, timestamps, analyst and reviewer, and a leakage-safe feature cut-off.
- **Disposition taxonomy**: a controlled closure-reason list, mandatory at close, designed so closures can serve as labels without
  re-reading notes.
- **QA sampling and re-grading** tools (random and risk-weighted) producing the L1/L2 quality grades Wolfsberg asks for.
- **Model ladder with label gates**: per bank, the product shows which tier of model is enabled and which gate is next.
- **Time-based backtesting harness** with lag-aware windows, precision-recall reporting, workload-at-fixed-recall curves and BTL
  integration.
- **Forced-review sample of low-score alerts** that feeds the unbiased evaluation set and the fairness monitor.

## 5. Unsupervised and semi-supervised anomaly detection

### 5a. What it is for

Rules find what someone already wrote down; labelled models find what resembles past labels. Anomaly detection looks for **behaviour
that departs from the customer's own history or from comparable customers**, with no labels. It earns its place in three situations:
(1) typologies nobody has written a rule for (the Wolfsberg Group assigns unsupervised models to "unknown and emerging typologies", file
04 section 2b); (2) segments where no rule fits (a new product, a new customer type); (3) a **cold start**, when a new bank has no
labels at all. The HKMA says AI-powered monitoring "is able to identify new typologies and detect complex networks of suspicious
customers" (file 29 section 5a).

### 5b. Method menu

| Method | Idea | Strengths | Weaknesses | Role in AKR-TMS |
|---|---|---|---|---|
| Robust deviation scores | Per feature, distance from the peer median (in median absolute deviations) and the percentile in the segment; same against the customer's own trailing baseline | Simplest; exact reason codes; works on small data | Univariate; misses combinations | **First line**; always available |
| ECOD and COPOD (Li et al.) | Estimate tail probabilities per dimension from empirical distributions and combine them | "Parameter-free and easy to interpret" (authors' words); fast; per-feature tail probabilities become reason codes | Assumes feature independence in ECOD; weaker on interactions | Good multivariate default with built-in explanation |
| Isolation Forest (Liu, Ting and Zhou, 2008; not re-fetched) | Random trees isolate points; short path means outlier | Strong general-purpose multivariate detector; scalable | Scores are not probabilities; explanation needs attribution (tree attributions such as TreeSHAP can be applied) | Second detector, Tier 2 |
| Local density or distance methods (LOF, k-nearest-neighbour) | Compare a point's density with its neighbours' | Finds local outliers inside clusters | Memory and compute heavy; sensitive to scaling | Small segments or investigations |
| Autoencoders and deep one-class models | Reconstruction error as anomaly score | Handle sequences and high dimensions | Hard to explain; need tuning and compute | Not initially |
| Time-series change detection (EWMA, CUSUM, seasonal decomposition) | Detect shifts in a customer's own series | Natural for "sudden change in behaviour" | Needs calendar-aware baselines | Per-customer drift alerts |
| Cluster-membership methods | Tiny or distant clusters are anomalous; segment migration | Links to segmentation (section 3) | Cluster choice matters | Segment-migration signal |

ECOD and COPOD come from the same group that maintains the PyOD outlier-detection library (Zhao et al., arXiv 1901.01588). The
parameter-free, per-feature design suits a product that must explain each alert to an analyst and a validator.

### 5c. What the benchmark evidence says (generic tabular data, not AML)

ADBench (Han et al., NeurIPS 2022 datasets and benchmarks track) tested 30 algorithms on 57 datasets in 98,436 experiments. Its findings
that bear on design (read from the paper's text on 2026-10-06):

- "No unsupervised algorithm statistically outperforms the rest": algorithm choice among unsupervised methods matters less than feature
  design and data; use a small ensemble of simple, explainable detectors instead of chasing one.
- "With merely 1% labeled anomalies, most semi-supervised methods can outperform the best unsupervised method": even a handful of
  confirmed cases is worth far more than a cleverer algorithm.
- "Semi-supervised methods outperform supervised methods when limited label information is available": for 1 percent labelled
  anomalies the median AUC-ROC was 75.6 percent for semi-supervised and 60.8 percent for fully supervised methods; at 5 percent, 81.0
  versus 72.7; most supervised methods needed about 10 percent labels to beat the best unsupervised one; with many labels the two
  families were comparable.

Caveat: the datasets are generic, anomalies there are not laundering, and AML has the added problem that labels are biased by past
detection (section 4c). The direction of the finding (use whatever labels exist, in a semi-supervised way, early) still carries.

### 5d. Pitfalls specific to a cash-heavy economy

- **Anomalous is not suspicious.** Pay days, Eid, Arbaeen travel, school fees, harvest sales, contract milestone payments and loan
  disbursements are large and legitimate. Baselines must be calendar-aware (section 3c) and each anomaly alert must name the benign
  explanations the analyst should rule out first.
- **Heavy tails.** Amounts follow heavy-tailed distributions; use log scales and robust scaling, or the detector flags every large trader.
- **Cash leaves little trace.** A cash deposit has a depositor but often no counterparty; behavioural features (frequency, timing, branch
  spread, depositor identity, round amounts, cash-in versus transfer-out) carry the signal.
- **Scores are not probabilities.** Convert to ranks or percentiles per segment, and alert on **capacity** (the top k per day per branch
  team, or the top share of the segment) instead of an absolute score, so the workload is predictable.
- **Stability.** A detector whose top list changes completely from week to week cannot be tuned or explained; measure rank stability.

### 5e. Evaluating detectors without labels

1. **Seeded-typology tests**: inject synthetic examples of each CBI scenario and typology (structuring, pass-through, mule rings) into
   historical data and measure recall of the seeded cases (also a regression test).
2. **Analyst acceptance rate**: share of anomaly alerts that a sample of analysts judge "worth a review", with reasons.
3. **Unique catches**: confirmed cases found by the detector and by no rule (the Wolfsberg "false-negative assessment" in reverse).
4. **Overlap with rule alerts**: high overlap is not a failure (it shows agreement) but a detector that only repeats the rules adds no value.
5. **Below-the-line review**: anomaly scores of customers who raised no alert at all, reviewed by sampling (file 29 section 9c).
6. **Stability and drift** across months.

### 5f. Reason codes for anomalies

An anomaly alert must say what is unusual in words the analyst can check: "Cash deposits this month are 14 times the segment median (99.8th
percentile); 5 branches used in 3 days; 11 new depositors; 92 percent of the amount left within 48 hours." Each clause is a feature with
a peer percentile or a baseline ratio, produced directly by the robust-deviation and ECOD layers; the multivariate detector adds "unusual
combination" with the top attributing features (file 29 section 9j).

**AKR-TMS hooks**

- A **deviation store**: per customer and period, the robust deviation and percentile of every feature versus peers and own history,
  feeding rules (expected-versus-actual), anomaly scores and reason codes from one place.
- **Capacity-based alerting** (top-k per queue and day) alongside threshold alerting.
- **Calendar-adjusted baselines** and a "benign explanations" checklist attached to each anomaly type.
- A **seeded-typology test suite** and an **anomaly graduation path**: an anomaly pattern that proves productive in shadow mode is
  written up as a documented rule (the Wolfsberg loop of feeding new patterns back, file 04 section 2a).
- **Semi-supervised upgrade path**: as soon as a bank has confirmed cases, the anomaly scores become features of the alert scorer
  (section 4e) and a semi-supervised detector is trained in shadow mode.

## 6. Graph and network analytics

### 6a. Why laundering is a graph problem

Laundering is about movement between parties. The AMLworld generator models eight standard structures: **fan-out, fan-in,
gather-scatter, scatter-gather, simple cycle, random, bipartite and stack** (Altman et al.), and the paper notes that laundering "can cross
bank and national boundaries, producing complex transaction patterns", which one institution sees only in part. The
Wolfsberg Group makes **entity resolution and graph networks** central: link customer, account, transaction and external data into a
network view and build risk profiles for customers, underlying customers and beneficial owners (file 04 section 2a). The HKMA reports
that AI with network analytics lets banks "identify suspicious accounts earlier, uncover hidden connections, and intervene before harm
escalates", especially when data is shared through public-private partnerships (file 29 section 5c). Iraq's own examples fit: the MER's
USD 21 million case was found by linking a company, its director and a corruption inquiry; the two-passports case was an
identity-link problem (file 01 section 2).

### 6b. What a single bank's replica can support

| Node | Source in the CBS replica | Edge | Notes |
|---|---|---|---|
| Customer / account | Customer and account tables | Owns, signs for, is guaranteed by | Authorised signatories, guarantors, co-borrowers |
| Counterparty (named) | Transfer and deposit fields (beneficiary, originator, depositor) | Paid to, received from, deposited for | Free-text names need entity resolution (section 7) |
| Shared attribute | Phone, address, ID document, **family-card number**, mother's name, employer, device or IP if the digital-channel database is readable | Shares | Relationship typing is required: a shared family number links relatives, not necessarily accomplices |
| Company | Company registration data, beneficial owners, directors | Controls, directs | BO threshold 20% (file 01) |
| Branch | Branch tables | Transacted at | Cluster of deposits across branches (cash 4) |
| Exchange / money-transfer company | Customer segment | Acts as intermediary | Hub nodes with very high degree |

A single bank sees **only its own side** of each payment, which is the "siloed" weakness BIS Project Aurora tested against collaborative
approaches (section 13). Within that limit, three kinds of graph analytics are workable.

### 6c. Techniques, from cheap to heavy

1. **Entity-centric network features** (cheap, explainable): in- and out-degree; number of distinct counterparties and their
   concentration; reciprocity; two-hop reach; share of counterparties that are also customers; number of customers sharing a phone,
   address, document or family number; **hops to a listed or previously reported entity** (sanctioned, PEP, STR subject, national-list
   entry); and neighbour risk (average risk score of connected parties).
2. **Pattern (motif) counts**: counts of fan-in, fan-out, gather-scatter, scatter-gather, simple and time-constrained cycles, bipartite
   and stack structures within sliding windows. Cycle enumeration explodes on dense graphs, so use time-ordered and hop-constrained
   algorithms (the AMLworld paper cites work on "simple, temporal, and hop-constrained cycles"). IBM's **Graph Feature Preprocessor** does
   exactly this in a streaming manner: it "maintains a dynamic in-memory graph, and efficiently mines subgraph patterns in the incoming
   transaction stream", and its CPU throughput "outperforms the graph neural network baselines executed on a powerful V100 GPU" (arXiv
   2402.08593). This is the approach that fits an on-premises Rust service.
3. **Community detection** (Louvain or Leiden): find groups of accounts with dense internal flows; compare each group's profile with
   legitimate communities (a company with its staff, a family) before alerting.
4. **Graph neural networks** (GIN, GraphSAGE, PNA): learn from node and edge features and structure. On the public AML benchmark they
   match but do not beat tree models with engineered graph features (section 4b), need GPUs to train at scale and are hard to explain.
   Two lines of work are worth tracking: adaptations that make GNNs "provably powerful" on directed multigraphs (port numbering, ego IDs,
   reverse message passing) improved a standard GNN's minority-class F1 "by up to 30%" (Egressy et al., AAAI 2024, arXiv 2306.11586);
   and **subgraph classification**, because "AML [is] inherently a subgraph problem" (Bellei et al., Elliptic2, arXiv 2404.19109).
   Self-supervised GNN embeddings (Inspection-L, arXiv 2203.10465: Deep Graph Infomax with a random-forest classifier on the Elliptic
   data) offer a route that uses few labels.
5. **Investigator link analysis**: an interactive subgraph view with filters by link type, time and amount, which is the human end of all
   of the above and the explanation for network-based alerts.

### 6d. Pitfalls

- **Supernodes**: utilities, government bodies, payroll sources, exchange houses and the bank's own branches connect everyone. Cap or
  down-weight edges through nodes above a degree limit, and treat known hubs as context, not suspicion.
- **Weak shared attributes**: a shared mobile number may belong to an agent, an office or a family; a shared address may be a building.
  Weight each link type by how discriminating it is (rarer identifiers weigh more) and show the weight.
- **Guilt by association and fairness**: family and tribal links can turn into proxies for ethnicity or sect. Link types that reflect
  kinship are context for investigators, are excluded from automated scores unless the validator approves them with a documented
  rationale, and are covered by the fairness monitor (file 29 section 9i).
- **Time**: a graph without time orders is misleading; use windows and directions.
- **Scale**: keep graphs in windows (rolling 30, 90, 180 days) with daily snapshots and incremental updates; store only features and
  evidence subgraphs, not the whole history, in the alert.
- **Privacy and need-to-know**: network views reveal relationships; access follows case assignment (file 30).

### 6e. Iraq-specific typologies the graph should target

Salary-domiciliation mule rings (many accounts at one employer or branch feeding one beneficiary); relatives sharing a family number who
split cash deposits below thresholds; exchange and money-transfer companies as hubs between many individuals and a few foreign
counterparties; trading companies sharing directors, addresses, phone numbers or incorporation dates (the FinCEN Iran advisory flags several
companies incorporated at about the same time that share counterparties, addresses, owners or name patterns, as summarised in file 05
section 1g);
hawala-linked pooled accounts (cash collected, consolidated, wired abroad, file 02); and company-director-state-body links in
corruption cases (file 01 case 2).

**AKR-TMS hooks**

- An **entity graph** built from the CBS replica with typed nodes and edges, relationship typing, link weights and supernode policy.
- A **streaming motif engine** (Rust, in memory, windowed) producing the eight AMLworld patterns and cycle counts as features and
  alerts, with evidence subgraphs frozen in each alert.
- **Graph features** (degree, reach, shared-attribute counts, hops to listed entities, neighbour risk) as inputs to the alert scorer
  and the customer risk score.
- **Link-analysis workspace** for analysts, with kinship links hidden by default and logged when shown.
- **Graph-model roadmap**: tree models on graph features first; GNN experiments in shadow only after the pooled-data gate (section 16).

## 7. Entity resolution

### 7a. Why it comes before everything else

Every method in this file assumes that "the customer" and "the counterparty" are single, stable entities. In an Iraqi bank they are not:
the same person can hold several customer numbers across branches and products; names are long patronymic chains with spelling variants
(file 22); Iraqi identity documents multiply (national ID, residence card, family-card number, passport, nationality certificate; file 01
section 7); counterparties in transfers and deposits are free-text names; and companies share owners and addresses. The MER's
two-passports case (two genuine Iraqi passports in one person's name used to split a transfer) is an entity-resolution failure waiting
to happen. Without resolution, profiles are fragmented (a launderer looks like ten small customers), peer groups are wrong, network
features are broken and alerts are duplicated.

### 7b. Four resolution jobs

| Job | Question | Output |
|---|---|---|
| Customer to customer (inside the bank) | Are these customer records the same person or company? | Persistent entity ID linking all customer numbers |
| Counterparty to customer | Is this beneficiary, originator or depositor already a customer, or another bank's customer we have seen? | Counterparty entity ID, link to customer if any |
| Customer to list | Is this entity on a sanctions, PEP or national list? | Screening hit (file 22, file 04 section 2d) |
| Entity to entity (relationships) | Who is related (family, ownership, control, employment, shared address)? | Typed relationship edges for the graph |

### 7c. Method

- **Probabilistic record linkage** (Fellegi and Sunter, 1969): compare field by field (name, date of birth, mother's name, ID numbers,
  phone, address, nationality), turn agreements and disagreements into weights from match and non-match probabilities, sum to a score,
  and decide link, review or no-link by two thresholds. The match and non-match probabilities can be estimated without labels by
  expectation-maximisation, which suits a bank with no ground truth. The open-source **Splink** library (UK Ministry of Justice
  analytical services) implements this model with SQL backends (DuckDB, Spark) and documents a "reproducible DuckDB benchmark: one
  billion records" (Splink documentation); a Rust service can follow the same design.
- **Blocking** (candidate generation) to avoid comparing all pairs: blocks on normalised name tokens, phonetic keys, birth year plus
  mother's first name, phone number, document number.
- **Comparators built for Arabic**: normalised Arabic-script comparison, token-set matching for patronymic chains, handling of "bin/ibn",
  tribal and family names, Kurdish variants and Latin transliteration (file 22); exact-match comparators with checksums for document
  numbers; date comparators that tolerate swapped day and month.
- **Machine-learned matchers** where labelled pairs exist (analyst confirmations), used alongside the probabilistic model.
- **Clustering** the pairwise links into entities with constraints (never merge two different national ID numbers without review).
- **Three link strengths**: *same entity* (auto-merge above the upper threshold), *possible same entity* (review queue), *related*
  (typed relationship edge). Keep "possible" and "related" out of aggregation until confirmed.

### 7d. Error costs and evaluation

A **false merge** blends two customers' behaviour (false alerts, wrong risk rating, a legitimate customer tainted by another's history); a
**false split** hides a network and understates risk. Report both. Measures: pairwise precision, recall and F1 on a labelled sample;
cluster-level measures such as B-cubed; review load at each threshold; the rate of analyst reversals; and fairness of error rates across
name families (Arab, Kurdish, Turkmen, Christian names; file 22). Sample-based audits every quarter.

### 7e. Governance

The matcher is a **Tier 1 model** (file 29 section 9b): it changes what every downstream rule and model sees. Each link decision stores the
evidence ("matched on mother's name, birth year, phone and normalised name"), who or what decided, and the threshold version; every merge can
be undone and the history of an entity is preserved. Data minimisation applies: resolution uses only the fields needed.

**AKR-TMS hooks**

- A persistent **entity service** with golden-record view, link evidence and one-click unmerge, exposed to rules, models, screening and the
  case UI through the same entity ID.
- **Review queue** for possible links with Arabic side-by-side comparison, a reason picker and the effect of the decision on alerts.
- **ER quality dashboard**: pairwise precision and recall on the audit sample, reversal rate, error rates by name family, merge counts.
- **Counterparty resolution** for free-text names in transfers and cash deposits, with learning from analyst confirmations.
- A **relationship layer** (family, ownership, control, employment, shared contact) typed and weighted for the graph, with kinship links
  governed as sensitive.

## 8. Customer risk scoring (static, dynamic, event-driven)

File 26 (KYC and customer risk rating) is the detailed treatment of CDD risk rating. This section covers only the technology: how a
customer risk score is built, kept current from monitoring data, tested and explained.

### 8a. What the CBI requires

The CBI's 2025 controls require every bank's AML system to **classify customers by risk** using "each customer's cash flows, nature of
business, geographic location, and the products and delivery channels used", at minimum **High, Medium and Low** with a five-level scale
allowed, linked to the core banking system, with **dynamic update of customer risk records** at each interaction through the customer risk
system linked to the AML system, and with periodic review especially of high-risk customers (file 01 section 2d). The Wolfsberg RBA
guidance (June 2026) calls for a risk assessment that is "dynamic and embedded in business as usual", for behaviour "judged against stated
purpose, expected activity and profile", and for "repeated or unusual amendments to key profile data without a business reason" as a risk
signal (file 04 section 2j).

### 8b. Three layers

| Layer | Inputs | Updated | Example factors |
|---|---|---|---|
| **Static (inherent) risk** | Onboarding and KYC data | At onboarding and refresh | Customer type; sector and cash intensity; ownership complexity; PEP status; nationality, residence, work and registration country; products and channels; source of funds and wealth; screening and adverse-media results; newly formed entity |
| **Dynamic (behavioural) risk** | Monitoring outputs | Daily or weekly | Actual versus declared activity; cash share and trend; counterparties' countries; alert and STR history; deviation versus peers; pass-through ratio; dormancy and reactivation; unusual profile amendments; network exposure (hops to listed entities, neighbour risk) |
| **Event triggers** | Discrete events | Immediately | PEP or list match; adverse media; sanctions-list change; STR filed; FIU or authority request; law-enforcement inquiry; change of beneficial owner or signatory; document expiry; new high-risk product |

Events do two things: re-rate the customer at once (a transparent rule such as "STR filed means High for 12 months") and open a KYC
refresh task (file 04 section 2q). The dynamic layer lets the bank see risk rising between scheduled reviews, which is what the HKMA
"dynamic risk assessment" case studies describe (section 4g; file 29 section 5c).

### 8c. Model forms

- **Transparent weighted scorecard** as the base: factors with documented points, set first by expert judgement against the bank's risk
  assessment, then calibrated against outcomes (STRs, exits, EDD findings) as data accumulates. The score is the sum, which makes the
  explanation exact.
- **Override rules** that force a level regardless of score (PEP, sanctions proximity, high-risk jurisdiction exposure, open STR).
  Overrides are listed with their reason, owner and expiry.
- **Machine-learned calibrator** (a regularised logistic model or monotone gradient boosting) added where labels permit, constrained so
  that a risk-increasing factor can never lower the score. Output maps to the bank's 3 or 5 levels through bank-set cut-offs.
- **Stability controls**: hysteresis (a customer must stay below a cut-off for a period before being downgraded), a minimum dwell time,
  and a limit on how many levels a customer can move in one cycle without human approval, so ratings do not flap and alerts do not swing.
- **Human override with maker-checker**: every manual change records reason and approver; override rates are monitored (file 29 section 9d,
  CBUAE 8.4 logic).

### 8d. Testing a risk score

| Test | Question | Method |
|---|---|---|
| Concentration of outcomes | Do STRs, exits and confirmed cases concentrate in High? | Capture rate against population share; lift curve; KS or Gini where labels allow |
| Distribution | Is the population spread sensibly? | Share of customers per level; a score that puts 95 percent in Medium is useless |
| Stability | Do ratings jump? | Migration matrix between periods; flapping count |
| Back-test at the time of the event | Would the customer have been High before the STR? | Re-score historical snapshots |
| Override review | Are overrides systematically lowering risk? | Override rate and direction by officer and branch |
| Sensitivity | Which factor weights move the distribution most? | One-factor-at-a-time analysis (file 29 section 9c) |
| Fairness | Are some groups rated High more often without justification? | Group rate ratios (file 29 section 9i) |

### 8e. Explanation and use

Each rating shows every factor, the data value, the points and the override list, so that an examiner's question "why is this customer
High?" has a one-screen answer. The rating drives **monitoring intensity** (thresholds per level, HKMA: "more stringent thresholds" for
high-risk customers), **review frequency**, **approval levels** and **resource allocation**, which is the FinCEN and Wolfsberg principle that
resources follow risk (file 05 section 1a; file 04 section 2j). Factors that touch protected characteristics follow the allowed-use matrix
(file 29 section 9i): nationality and residence are legitimate regulatory factors but are never the sole driver of a High rating.

**AKR-TMS hooks**

- **Scorecard builder** with factor library, points, cut-offs, version history, maker-checker and a what-if simulator showing the effect on
  the distribution before activation.
- **Dynamic re-rating engine** fed by monitoring outputs and the event-trigger registry, with hysteresis and dwell-time rules.
- **Override workflow** with expiry and reporting; override rate as a monitored metric.
- **Board-level distribution report** (customers by level, migration, overrides, outcomes by level) and the CBI 3- or 5-level mapping.
- **Per-customer explanation page**, linked to the KYC file, alerts and network view; interface with file 26's CDD workflow.

## 9. Alert prioritisation, triage, hibernation and suppression

### 9a. The workload problem in Iraqi banks

The MER found large numbers of alerts archived daily against very few STRs, a two-tier workflow (branch liaison officer then central AML
department) and, at some banks, too few staff to investigate alerts (file 01 section 2). The HKMA names false-positive volume as "the
primary driver" of AI adoption and notes that AI scoring "replace[s] the different levels of initial assessment manually conducted by
staff" (file 29 section 5a). The Wolfsberg Group's concern is effectiveness over volume: metrics such as alert counts "measure
quantity, not usefulness" (file 04 section 2a). The first measurable benefit of better technology is therefore **fewer wasted reviews for
the same or better detection**, achieved without breaking the CBI's rules.

### 9b. A ladder of techniques, safest first

| Step | What it does | Risk of missing suspicious activity | Controls needed |
|---|---|---|---|
| 1. **Deduplicate and consolidate** | Groups all alerts for one customer and period into one case candidate; collapses repeats of the same pattern | None (no alert is dropped) | Evidence kept per original alert |
| 2. **Rank** | Orders the queue by a priority score (rule severity, customer risk level, deviation, network exposure, prior outcomes, KYC freshness; ML scorer when labels permit) | Low: nothing is hidden, but low-ranked items may wait | Age limits so nothing waits beyond a set time; forced-review sample of low scores |
| 3. **Route** | Sends alerts to the right queue and level: branch liaison officer, central team, specialist (trade, terrorism financing), with service-level clocks by risk | Low | Skill and workload rules; escalation on breach |
| 4. **Hibernate** | After a documented not-suspicious decision, the same scenario for the same customer is silenced for a period or until behaviour exceeds the accepted envelope by a stated margin, or a trigger fires | Moderate | Hibernation registry with reason, envelope, expiry; the scenario still **runs and logs** (suppress the display, not the evaluation); audit sample; regret metric |
| 5. **Suppress by whitelist** | A recognised legitimate pattern for a customer (a school's fee receipts, a government salary payer) stops generating alerts | Moderate | Documented explanation, owner, expiry, periodic revalidation; fairness check |
| 6. **Auto-close** | Closes a low-risk alert with no human review | Highest | Policy approval, sampling QA with a stated tolerated error rate, below-the-line tests, regulatory acceptance. **Not in Phase 1** (see below) |

**Why auto-close waits.** The CBI's 2025 controls say unusual or complex transactions must be examined and "the findings recorded in
writing", and that internal reports go through an integrated system reviewed daily by the head of the AML department; STR decisions need a
written decision (file 01 section 2d). An alert that the bank's own system raises is an indicator, not yet an "unusual transaction"; but
until the CBI says automated closure of indicators is acceptable (an open question, file 29 section 10), AKR-TMS keeps a human
disposition on every alert it presents, and uses ranking, consolidation, hibernation and whitelisting to reduce the number presented.
Where the bank wants to go further, the rationale and tolerated error rate are written into its policy first.

**Hibernation design details**

- **Envelope**: the accepted behaviour is stored as numbers (for example monthly cash up to X, up to Y counterparties). The hibernation
  ends automatically when behaviour exceeds the envelope by a configured margin or a trigger event occurs (new adverse media, a list
  change, an authority request, a profile change).
- **Expiry**: every hibernation has an end date (default 90 days, configurable; the US 90-day continuing-activity convention, now only an
  optional policy after FinCEN's FAQs of October 2025, file 05 section 1b, is a useful reference point, not a requirement here).
- **Visibility**: a hibernated alert is recorded in a ledger and counted in the daily report; the head of AML sees what was suppressed and
  why; the audit workspace can sample it.
- **Regret metric**: the share of hibernated or whitelisted customers later connected to an STR, an authority request or a confirmed case;
  a high regret rate means the envelope is too generous.
- **Mandatory scenarios**: for CBI minimum scenarios the evaluation always runs; hibernation changes what is shown to analysts, not what is
  detected or logged, so coverage reporting stays complete.

### 9c. Measuring prioritisation

| Metric | Meaning |
|---|---|
| Workload at fixed recall | Share of alerts that can be moved out of the front queue while keeping at least a chosen share (for example 95 percent) of the alerts that later become STRs |
| Precision at top-k | Share of the first k alerts per analyst-day that are productive |
| Queue age and service-level compliance | Whether anything waits too long |
| Alert-to-case and case-to-STR rates | Overall yield |
| Reopen and reversal rate | How often a closed or hibernated alert is reopened |
| Regret rate | Section 9b |
| Analyst time per disposition | Efficiency, not to be used as the sole individual KPI (automation-bias control, file 29 section 9j) |

**AKR-TMS hooks**

- **Case grouping and ranking service** with the priority score decomposed into visible parts; the ML scorer plugs in as one input when its
  gate is passed (section 4f).
- **Hibernation and whitelist registry** with envelopes, expiry, owner, approver, audit sampling and regret metric.
- **Suppressed-alert ledger** and a daily "what was held back and why" report for the head of AML.
- **Routing and service-level engine** for the branch liaison and central tiers, with escalation clocks.
- **Policy switches** for continuing-activity review and auto-close, off by default, each with a documented rationale field.

## 10. Real time versus batch (Phase 1 post-event, Phase 2 pre-release)

### 10a. What the CBI asks for

The 2025 controls require "instant, real-time follow-up" of account movements against declared cash flows and approved scenarios, and
the system must **stop** a transaction matching a suspicion scenario until the MLRO verifies it; digital banks must run daily monitoring
linked to the CBS (file 01 sections 2d and 2e). The product brief splits this into Phase 1 (near-real-time post-event monitoring from the
read-only replica) and Phase 2 (an optional pre-release screening and scoring API called by the bank's payment or SWIFT gateway). The
Wolfsberg Group adds the strategic point: "turn detective controls into preventive ones where high-value patterns are known" (file 04
section 2a). The HKMA's 2026 foreword shows why speed matters: Hong Kong's faster-payment transactions index rose 229 percent in four
years and remote account opening reached 74 percent of new accounts in the fourth quarter of 2025 (file 29 section 5c).

### 10b. Four latency classes

| Class | Latency | Phase | What belongs here | Constraints |
|---|---|---|---|---|
| **L0 Pre-release** | Milliseconds to low seconds | 2 | Sanctions, PEP and national-list screening of parties; hard blocks; velocity counters; simple threshold rules; known-bad entities; pre-computed risk flags | Deterministic, explainable, cache-served, fail-safe policy; no heavy model unless validated and with fallback |
| **L1 Near-real-time** | Seconds to minutes | 1 | Running counters and windows (same-day structuring, rapid movement), new-counterparty checks, small-window motif counts, alert to the branch liaison officer the same day | Depends on how often the CBS posts to its database |
| **L2 Intraday and daily batch** | Hours | 1 | Peer deviation and expected-versus-actual, daily aggregates, segment thresholds, alert consolidation and ranking | Consistent with L1 definitions |
| **L3 Periodic** | Weekly to quarterly | 1 | Segmentation refresh, community detection, retraining, customer re-rating, KYC refresh triggers, dormancy sweeps | Run in shadow before activation |

### 10c. Read-only ingestion and what it implies

Phase 1 reads a replica without touching the CBS (file 25 covers the integration options). Design consequences:

- **Watermark-based incremental reads** (by commit time, sequence or posting date) every few minutes, plus periodic **reconciliation** of
  counts and control totals against the CBS, because late postings, reversals and back-dated corrections are normal. Keep **event time and
  load time** for every row so a late correction re-evaluates the affected windows.
- **The CBS cadence is the floor on "real time"**: where a CBS posts transactions only at end of day, Phase 1 can be no faster than that,
  and the bank should be told so. The Phase 2 API is then the only way to act before settlement.
- **Idempotent processing** with a deduplication key per transaction version, and replay from any point (needed for validation, file 29).
- **Online and offline feature consistency**: the same feature definitions run in the streaming path and the batch path, with point-in-time
  correctness for training; this is the main source of silent training-serving skew.
- **Fast state in Valkey** (the Redis-compatible fork the stack uses instead of Redis, file 32) for counters and windows, with the durable record in PostgreSQL.

### 10d. Phase 2: scoring in the payment path

- **Decision vocabulary**: release, hold for review, reject, or release with alert; the bank's policy sets which outcome each rule or score may
  trigger. ML scores are advisory (hold recommendation) until validated in shadow mode at the bank (file 29 GV-53).
- **Latency and availability targets** (design proposals): a published latency budget for list and rule checks measured in hundreds of
  milliseconds at the 99th percentile, and an availability target consistent with the gateway; both agreed per bank.
- **Fail-open or fail-closed** decided by the bank, recorded, and exercised in tests; a timeout never silently becomes approval.
- **Same logic as batch**: the L0 rules are the same scenario definitions executed on a single transaction with warm state, so the batch and
  pre-release outcomes cannot drift apart.
- **Decision log** for every call: inputs, rule and model versions, outcome, latency, and the human clearance when held.

**AKR-TMS hooks**

- Each scenario and model carries a **latency class**; the engine builds the right execution plan (counter-based for L0 and L1, set-based for
  L2, scheduled for L3) from one definition.
- **Watermark ingestion with late-data handling** and reconciliation reports; the bank is told the CBS posting cadence and what that means
  for detection delay.
- **Online feature store** with point-in-time training views and a skew monitor.
- **Phase 2 API contract** with timeouts, fail policy, decision log and a replay tool; shadow mode before any hold is enabled.

## 11. Explainability: reason codes, SHAP and typology-to-feature coverage

File 29 section 9j sets the governance standard (four audiences, human oversight). This section is the technical design.

### 11a. Three kinds of explanation

| Kind | Used for | Method |
|---|---|---|
| **Exact** | Rules and scorecards | The rule, the parameter values, the data rows that matched |
| **Attribution** | Tree models and anomaly scores | Shapley-value attributions: SHAP (Lundberg and Lee, NIPS 2017, arXiv 1705.07874) and **TreeSHAP**, which gives the first "polynomial time algorithm to compute optimal explanations based on game theory" for tree ensembles (Lundberg et al., arXiv 1905.04610); LIME (Ribeiro et al., arXiv 1602.04938) for local surrogate explanations. MAS lists SHAP and LIME as the common methods banks use (file 29 section 4b) |
| **Structural** | Network alerts | The evidence subgraph and the pattern name (fan-in, cycle, shared attribute) |

For an on-premises Rust service the practical route is tree models (fast, exact TreeSHAP) and a scorecard or additive model for Tier 1
decisions (section 4e). Rudin's argument is the rule of thumb: where the stakes are high, prefer a model that is interpretable by design
over a black box plus an after-the-fact explainer.

### 11b. The reason-code engine

A **reason code** is the unit of explanation shown to people:

- **Stable ID** (for example `CASH-STRUCT-MULTIBRANCH`), a version, bilingual text templates (Arabic and English, reviewed by compliance
  staff), the typology tag, the CBI article where one applies, and the data fields to display.
- **Selection**: the top few contributors by attribution above a minimum share, grouped into **reason families** so that five correlated
  features do not show as five reasons; **mitigating factors** (negative contributions) are shown as well, because an honest explanation
  includes what argues against suspicion.
- **Facts next to attribution**: each reason shows the actual value and a comparison ("14 times the segment median; own average 3 times
  lower than this month") so the analyst checks the evidence, not the model.
- **Confidentiality**: threshold values and feature weights are visible only to roles authorised to see detection logic; branch users see
  reason codes without thresholds (file 29 section 7b).
- **Consistency across paths**: rules, scorecards, anomaly detectors and ML models emit the same reason-code structure so the analyst
  sees one format.

### 11c. The typology-to-feature coverage inventory

Wolfsberg part II asks for explainability through **risk coverage**: "map typologies to data features in a maintained inventory"
(file 04 section 2b). The inventory is also the coverage matrix the CBI expects:

| Column | Content |
|---|---|
| Typology | For example "structuring of cash deposits" |
| Sources of the requirement | CBI article and scenario numbers; FATF or FIU typologies; national risk assessment threat |
| Rules | Scenario IDs and versions |
| Features | Feature IDs used by scorers, anomaly detectors and graph engines |
| Models | Which models use those features, with tier |
| Data sources | Registered critical data elements |
| Performance | Alerts, cases, STRs in the last 12 months; unique catches |
| Gaps | Parts of the typology no rule or feature covers; data missing |
| Review | Owner, last review, next review |

It is generated from the inventory (file 29 section 9a), not maintained by hand, and answers three questions: what risks are covered, by
what, and with what evidence it works.

### 11d. Testing explanations

- **Fidelity**: removing or perturbing the top-ranked feature changes the score in the direction and by roughly the amount claimed.
- **Stability**: near-identical alerts receive near-identical reasons; reasons do not flip with small data changes.
- **Comprehension**: analysts are tested on whether the explanation lets them find the evidence faster (a usability test with Arabic-speaking
  analysts, file 31).
- **Counterfactual hints** for internal use: what change would have prevented the alert, never shown to customers (tipping-off).
- **Language quality**: Arabic templates are reviewed by compliance staff, not only machine-translated.

### 11e. Limits that must be said out loud

Attributions describe the model, not causation; correlated features share credit unpredictably; explanations can create false confidence
and feed automation bias (file 29 section 9j). Mitigations: monotonic constraints and small feature sets, raw evidence beside every
attribution, periodic human audit of explanations, and an interface that labels attributions as "model contributions".

**AKR-TMS hooks**

- **Reason-code library** with versioning, translation review and typology and CBI-article mapping.
- **Typology-to-feature inventory generator** feeding the CBI coverage matrix and the model cards.
- **Explanation test harness** (fidelity, stability) run in validation and after each release.
- **Evidence-first alert page**: raw rows, peer comparison and network view first, reasons second, thresholds hidden from branch roles.

## 12. GenAI and agentic AI in investigations, and what regulators said in 2024-2026

File 29 section 9k holds the governance controls (logging, grounding, human review, containment). File 12 section 8 and file 13 hold the
vendor claims and published bank results. This section gives the use cases, the evidence base, the regulatory position and the evaluation
protocol.

### 12a. Use cases and their risks

| # | Use case | What the model does | Main risk | Core control | Tier (file 29) |
|---|---|---|---|---|---|
| 1 | **Case and alert summary** | Condenses customer profile, transaction pattern, prior alerts and outcomes into a short brief | Omission or invented fact | Generation only from supplied records, each statement linked to a source row, deterministic number check | 3 |
| 2 | **Document extraction** | Reads KYC documents, invoices, letters of credit, bills of lading, statements (after OCR) and returns structured fields | OCR error, forged documents, silent misreads | Field-level confidence, human confirmation of fields that drive alerts, original shown beside the extraction | 3 |
| 3 | **Adverse-media triage** (Arabic and English) | Classifies relevance, extracts allegations and dates, links to the entity | Same-name confusion, defamation, missed Arabic variants | Source link and quotation for every claim; entity-match score; human decision (file 04 section 2i) | 3 |
| 4 | **STR narrative drafting** | Drafts the goAML narrative from the case facts | Unsupported statements; tipping-off phrasing | Facts only from the case record; structured template; analyst edit and sign-off; goAML field checks (file 23) | 3 |
| 5 | **Level-1 alert review** | Gathers evidence from allowed sources and proposes a disposition with rationale | Automation bias; wrong closure | Recommendation only; evidence cited; shadow evaluation; QA sample; never auto-close | 2 |
| 6 | **Investigation assistant** | Answers questions about a case or customer in natural language | Prompt injection through free-text fields; hallucinated queries; over-privileged access | Parameterised read-only tools, row-level security, no free SQL, output treated as untrusted | 2-3 |
| 7 | **Regulatory-change and typology mapping** | Maps new advisories and circulars to scenarios and gaps | Misreading the legal text | Human compliance review; link to source; register entry (file 04 hook) | 3 |
| 8 | **Translation and transliteration support** | Arabic, Kurdish and English summaries; name transliteration suggestions | Errors in names and amounts | Names and numbers copied programmatically, never regenerated | 3 |
| 9 | **Test-data and typology scenario authoring** | Drafts typology descriptions; code generates the transactions | Unrealistic scenarios | Numbers and structure by deterministic generators; expert review | 3 |

### 12b. What the evidence supports, and what it does not

- **Industry position.** Wolfsberg part I (July 2024) called GenAI and LLMs "in their infancy" for monitoring and in need of research,
  validation and governance for bias and explainability; part II (August 2025) accepts LLMs for **surfacing the most predictive features and
  generating "investigator-ready summaries and next-best actions"** and for auto-populating case files (file 04 sections 2a and 2b).
- **Regulator position on value.** The HKMA's June 2026 report warns against stopping at productivity: AI "should not be confined to
  accelerating documentation or optimising workflows. Its real potential lies in strengthening early detection, enhancing intelligence-led
  monitoring, strengthening complex decision making", and lists "case summarisation tools" among early point solutions (file 29
  section 5c). A product that offers only summaries will be judged as an efficiency tool; the detection layers of this file are what
  supervisors say they want.
- **Measured results are customer-reported.** The bank figures in files 12 and 13 and the HKMA report come from the banks and vendors
  themselves. **No peer-reviewed or independently audited evaluation of LLM accuracy on STR narrative drafting or alert adjudication was
  found in the sources reachable for this research** (the web-search budget ended during the work; this gap statement is
  therefore limited to what was read). Every claim of time saved or accuracy must be re-measured on the bank's own cases (section 12e).

### 12c. What regulators and standard-setters said, 2024-2026

| Date | Body | Statement (short) | Where in this research |
|---|---|---|---|
| Jul 2024 and Aug 2025 | Wolfsberg Group | LLMs for investigator summaries and next-best actions; model risk should be balanced against financial-crime risk; explainability by typology-to-feature mapping | File 04 sections 2a-2b |
| 9 Sep 2024 | HKMA | Encourages AI in suspicious-activity monitoring; board-signed feasibility study and implementation plan due by end of March 2025 | File 29 section 5a |
| 5 Dec 2024 | MAS | Thematic review of AI model risk: GenAI used to assist humans, "human-in-the-loop" for decisions, input and output guardrails; AML named as an AI use | File 29 section 4b |
| Dec 2024 | US Treasury | Report on AI in financial services; cites the 2024 National Strategy: AI "has significant potential to strengthen AML/CFT compliance"; respondents asked for standards on sharing transaction data for illicit-finance detection, on privacy-enhancing technologies and on synthetic data | Section 13; Treasury report (sections on data and on policy considerations) |
| 13 Nov 2024 | FinCEN | Alert FIN-2024-Alert004 on fraud schemes using deepfake media against financial institutions (cited in the Treasury report) | Section 12d |
| Nov 2025 (consultation) | MAS | Draft AI risk management guidelines covering GenAI and AI agents; AML is a named risk; human oversight with automation-bias safeguards | File 29 section 4c |
| 21 Nov 2025 | EBA | AI Act mapping finds no significant contradiction with banking law; no new EBA guidelines needed now | File 29 section 3c |
| Sep 2025 to 2026 | FCA | No AI-specific rules; AI Live Testing, second cohort (April 2026) includes AML detection and KYC and agentic payments | File 29 section 2b |
| 17 Apr 2026 | OCC, Federal Reserve, FDIC | Revised model-risk guidance puts generative and agentic AI outside its scope | File 05 section 1c |
| 7-10 Apr 2026 | FinCEN | Programme proposal: AI use is considered in enforcement decisions; "responsible experimentation will not incur additional risk" | File 05 section 1a |
| 19 May 2026 | European Commission | Draft high-risk guidelines: AML/CFT systems are not high-risk unless tied to creditworthiness | File 29 section 3a |
| Jun 2026 | HKMA | Report on AI in fighting financial crime; second workshop on agentic AI on 23 June 2026 | File 29 section 5c |
| Jul 2026 | MAS and industry | SAFR white paper on runtime safeguards for agentic finance (not regulatory guidance) | File 29 section 4d |
| 2021 and 2022 | CBUAE and partners | AI guidelines (15 Nov 2021) and Model Management Standards (Nov 2022) cover AI and AML models | File 29 section 7a |

Not read in this research (budget and access): ECB, BaFin, DNB, BIS and FSB statements on AI, the FATF's own AI statements after 2021, and any
Swift statement on AI. The CBI's text requires AI/ML in the AML system but says nothing about generative AI.

### 12d. A note on the other side: AI-enabled crime

FinCEN's November 2024 alert on deepfake media and the Treasury report describe criminals using GenAI to defeat identity checks and run
convincing scams. Two design consequences: onboarding and KYC refresh must treat document and image authenticity as a risk
(liveness and manipulation checks are the digital-bank module's concern, file 04 section 2q and file 31), and the monitoring layers must
look for behaviour, not only identity, because a fabricated identity can pass a document check. The HKMA's face-watch-list and
mule-at-onboarding cases (section 4g) show the same shift.

### 12e. Evaluation protocol for any LLM or agent feature (design guidance)

1. **Golden set**: at least 200 real, anonymised cases per use case, with expert-written reference outputs, in Arabic and English, stratified
   by typology and difficulty; kept secret from prompt developers.
2. **Measures**: factual error rate (any statement not supported by the case record), omission rate (material facts missing), unsupported
   inference rate, tone and clarity, and for recommendations the agreement with senior reviewers and the **critical-error rate** (a
   suspicious alert recommended for closure). With zero critical errors in n reviewed alerts, the 95 percent upper bound on the error rate is
   about 3 divided by n, so 300 alerts bound it near 1 percent (file 29 section 9c "rule of three"); the bank sets the tolerance.
3. **Shadow mode** for at least a defined period: the LLM output is generated and stored but not shown, then compared with human decisions.
4. **Staged release**: assist (summary), then recommend (disposition with evidence), then pre-fill (closure text for human approval).
   Autonomous closure is excluded (section 9b and file 29 section 9k).
5. **Human QA sample** that never goes to zero (for example 10 to 20 percent of LLM-assisted dispositions re-reviewed), with results in the
   monitoring dashboard.
6. **Regression suite** rerun on every model, prompt or retrieval change (file 29 section 9f).
7. **Time-saved measurement** on the bank's own data, reported with and without the cost of review, because review effort is where claimed
   savings disappear.

### 12f. Deployment note: Arabic-capable open models on premises

Arabic-focused open-weight models exist: **Jais and Jais-chat** (13-billion-parameter Arabic-English models, released August 2023, arXiv
2308.16149) and **ALLaM** (Saudi Arabic-English model series, arXiv 2407.15390, July 2024), alongside multilingual open models. Their
quality on Iraqi dialect and on compliance writing is untested; selection, hardware sizing and licences are for file 32, and a bank-specific
evaluation (section 12e) decides. The principle from file 29: no suspicion content leaves the bank to an external model API.

**AKR-TMS hooks**

- **GenAI assistant module** with the nine use cases switchable per bank, each behind the controls of file 29 section 9k and the staged
  release of 12e.
- **Evaluation workbench** with golden sets, error taxonomy, shadow mode and QA sampling, usable by the bank's validator.
- **Detection first**: the summary and narrative features are sold as assistance; the product's investment in detection (sections 2-9)
  answers the HKMA's call for value beyond documentation.
- **Agent containment** (least privilege, allow-listed actions, runtime policy check, kill switch) in place before any agentic feature ships.

## 13. Privacy-enhancing technologies and data sharing

### 13a. Why sharing matters for detection

One bank sees only its own side of every payment. Public experiments show what is lost: in the IBM AMLworld test, banks that shared a model did better than banks that kept everything private, and banks that also shared the transaction graph did better still (section 4b); BIS Project Aurora (published 31 May 2023, a proof of concept on synthetic data combining privacy-enhancing technologies, machine learning and network analysis) concluded that "CAL [collaborative analysis and learning] approaches are more effective in detecting money laundering networks than is the current siloed approach" (BIS page text). The FATF's data-pooling stocktake (July 2021) and its July 2022 report on private-sector information sharing (file 03 section 5) say the same about direction, and the FATF's June 2026 plenary listed public-private information sharing as upcoming work (file 01 section 1).

### 13b. The privacy-enhancing technology toolbox

| Technology | What it does | Fit for AML | Main risk or cost |
|---|---|---|---|
| **Federated learning (FL)** | Each bank trains on its own data; only model updates are combined | Cross-silo model training over a common feature set (Iraqi banks on the AKR data model) | Update leakage, poisoning by a bad participant, uneven data and labels; needs secure aggregation |
| **Secure multi-party computation (MPC), private set intersection** | Parties compute a result (such as which customers they share) without revealing their lists | Finding common entities, joint statistics, shared watchlist matches | Compute and engineering cost; agreed protocol |
| **Homomorphic encryption** | Computation on encrypted data | Narrow joint calculations | Very slow for large models |
| **Trusted execution environments (confidential computing)** | Hardware-isolated computation | Central analytics on pooled data without operator access | Hardware trust, supply chain, availability |
| **Differential privacy** | Calibrated noise limits what any output reveals about one person | Sharing statistics or model updates | Accuracy loss, hardest where positives are rare (as in Iraq) |
| **Synthetic data** | Artificial data with similar structure | Testing, benchmarking, pre-training (sections 14, 16) | Fidelity gap; does not prove privacy by itself |
| **Privacy-preserving record linkage** | Match records across parties using encoded identifiers | Cross-bank entity matching | Encoding attacks; Arabic name variation makes exact encoded matching brittle (file 22) |

The UK Information Commissioner's guidance on privacy-enhancing technologies (page marked as under review after the Data (Use and Access) Act) carries case studies on differentially private noise in financial services and on synthetic data for testing vulnerable-customer detection. In the US Treasury's December 2024 report, respondents proposed homomorphic encryption and federated learning for data privacy and asked for standards on transaction-data sharing for illicit-finance detection, on acceptable privacy-enhancing technologies and on synthetic data.

### 13c. What exists, scheme by scheme

| Scheme | Type | Facts (dates and numbers) | Source |
|---|---|---|---|
| **BIS Project Aurora** | Proof of concept | Published 31 May 2023, 103-page report; synthetic data with embedded laundering patterns; tested siloed, national and cross-border monitoring with ML and network analysis; PETs tested for privacy; conclusion: collaborative analysis outperforms siloed | BIS Innovation Hub page |
| **Singapore COSMIC** | Centralised platform, legally mandated framework | Launched **1 April 2024** by MAS with DBS, OCBC, UOB, Standard Chartered, Citibank and HSBC; sharing **voluntary** in the first phase; three risk areas (misuse of legal persons, misuse of trade finance, proliferation financing); a participant may share only if the customer shows "objectively-defined indicators of suspicion" (red flags) that meet stipulated thresholds; three flows: **request** (red flags cross a threshold), **provide** (multiple flags cross a higher threshold) and **alert** (after an STR and exit, the customer goes on a watchlist); legal basis in the Financial Services and Markets (Amendment) Act 2023 and 2024 regulations; safeguards: assess before sharing, "not rely solely on COSMIC information", let customers explain, ensure accuracy. MAS and the Singapore FIU may use the information for supervision and law enforcement | MAS COSMIC page, media release 1 Apr 2024, infographic |
| **Transaction Monitoring Netherlands (TMNL)** | Bank-owned joint monitoring utility | Started **2020** by ABN AMRO, ING, Rabobank, Triodos and De Volksbank; for three years tested scenarios on human trafficking, VAT fraud and drugs; "the results have proven to be very valuable" (TMNL, 23 April 2024). On **1 July 2024** the banks announced that, with the EU AML Regulation applying from July 2027 and replacing the Dutch legal initiatives, TMNL "will wind down its existing activities and capabilities" and be redesigned; the TMNL website now says it "will scale down its current activities and restructure them ... This means that TMNL, in its current form, will cease to exist" | TMNL press releases and website |
| **UK** | Statutory safe harbour for direct disclosure | Economic Crime and Corporate Transparency Act 2023, **section 188** (in force 15 January 2024 for remaining purposes): a regulated-sector firm may disclose customer information to another when the other asked (request condition) or when the discloser has decided to take safeguarding action such as terminating the relationship, refusing a product, or restricting access (warning condition), without breach of confidence or civil liability; but nothing "requires or authorises a disclosure of information that would contravene ... the data protection legislation" (s.188(11)) | legislation.gov.uk |
| **EU** | Partnerships for information sharing | AML Regulation (EU) 2024/1624, applies from **10 July 2027**: Articles 75-76 (see below) | OJ text |
| **United States** | Voluntary safe harbour | USA PATRIOT Act section 314(b); FinCEN **fact sheet of 12 June 2026** replaces the December 2020 version: sharing may be in **real time** and through electronic platforms, may include **fraud** and other specified unlawful activities, registration through the FinCEN FI Portal; FinCEN guidance FIN-2025-G001 of 5 Sep 2025 covers cross-border sharing and SAR confidentiality | FinCEN page and fact sheet |
| **Hong Kong** | Public-private and bank-to-bank platforms | FMLIT (since 2017: police, HKMA, banks) and FINEST (2023: bank-to-bank platform initiated by the banks' association, operated by the police); the HKMA counts FMLIT-shared suspicious accounts and says shared data produced "significantly improved results" | HKMA June 2026 report |
| **Swift** | Network operator working with banks | Swift has described AI anomaly detection developed with participating banks using federated learning. **Unverified**: Swift's website blocks automated access from this location (HTTP 403), and no independent copy could be reached; the bank count, dates and results must be confirmed before use | not reached |
| **Vendors** | Commercial networks | Federated or shared-typology products (Tookitaki, Consilient, Salv Bridge) | Files 12 and 13 |

**Why TMNL was scaled back.** The primary sources give one reason: the new EU law supplies a clear legal basis for bank-to-bank and
public-private sharing but with conditions (below) that an arrangement built under earlier Dutch law has to be re-engineered to meet, so
the existing activities are wound down and a new structure is designed. Other reasons often reported in the press (privacy-regulator
concerns, cost of a five-bank utility, limited legal certainty) are **Unverified** here. The lesson that holds regardless: a sharing
scheme that is technically sound can still stop if the legal basis moves; build the legal basis first and keep the technology portable.

### 13d. The EU AMLR conditions, as a design checklist for any sharing scheme

From Article 75 of Regulation (EU) 2024/1624 (read in the OJ text): participants must notify their supervisors, who verify the scheme (with
data-protection authorities and where relevant the FIU) **before it starts**; exchange is limited to listed categories (customer and
beneficial-owner identification data; purpose and nature of the relationship, source of wealth and funds; transactions; risk factors; the
bank's risk analysis; retained records; suspicions); sharing is **strictly necessary** and **only for higher-risk customers** or those
needing extra information; the bank must **record every instance**, must **not rely solely** on shared information, must make its own
assessment, must use technical measures **including pseudonymisation**, must run a **data protection impact assessment** first, may share
suspicion information **only where the FIU agrees**, may share AI- or algorithm-generated information **only after adequate human
oversight**, and may not pass received information on except in listed cases; internal policies must define roles and the risk assessments
that justify sharing. Article 76(5) adds the human-intervention rule for decisions (file 29 section 3b).

### 13e. What this means in Iraq

- **Legal position.** Banking secrecy (Banking Law 94/2004, Article 49) must not obstruct AML/CFT controls, and Iraq has no comprehensive
  personal-data law in force (file 01 sections 3 and 8). No provision allowing bank-to-bank sharing was found in the Iraqi texts read. A
  cross-bank scheme therefore needs an explicit mandate from the CBI and the AML/CFT Office, in the way Singapore legislated COSMIC and
  the EU legislated Article 75. This research cannot assume one exists.
- **The CBI is already a hub.** All external transfers pass through the CBI's transfer-monitoring system (2023), and the AML/CFT Office
  runs the national list API (file 01 sections 2 and 6). That makes the **CBI the natural trusted aggregator** for cross-bank network
  analytics on external transfers and for a shared register of mule or exited accounts. AKR-TMS cannot decide this; it can prepare
  standard feeds and a reference design for the CBI.
- **Scale and heterogeneity.** About 76 banks (file 01), each with its own CBS. A common AKR data model (file 25) is the precondition for
  any federated training: the same feature definitions must mean the same thing everywhere.
- **Few positives.** With about 6 STRs per bank per year, differential-privacy noise on bank-level updates would swamp the signal; pooling
  through a trusted aggregator or synthetic pre-training is more realistic than noisy federated updates at bank level.

### 13f. A staged path for AKR-TMS (design proposal)

| Stage | What is shared | Mechanism | Legal need |
|---|---|---|---|
| **A. Typology and rule updates** | Nothing about customers: typology definitions, scenario packs, parameter benchmarks | AKR-TMS ships updates; banks opt in to send back anonymised aggregate statistics (alert yield per scenario) | Contract only |
| **B. Peer benchmarking** | Aggregate threshold and productivity statistics by segment | Secure aggregation of counts; minimum cohort size (for example 5 banks) | Bank consent; CBI awareness |
| **C. Shared models** | Model updates or pooled features | Cross-silo federated training of tree or scorecard models, secure aggregation, optional noise, held-out evaluation; or pooling at a trusted aggregator | CBI mandate; legal review of banking secrecy |
| **D. Shared signals** | Entity-level flags (mule accounts, exited customers, high-risk entities) | CBI- or consortium-run register; private set intersection or hashed matching; COSMIC-style thresholds and customer-protection rules | CBI and AML/CFT Office rule |
| **E. Cross-bank network analytics** | Transaction edges for external transfers | Analytics at the CBI on data it already sees, or MPC across banks | CBI mandate |

Every stage carries the Article 75-style conditions as controls: purpose limitation, necessity, higher-risk only, recording, no sole
reliance, human oversight of AI-derived signals, DPIA-equivalent assessment, pseudonymisation, and FIU consent for suspicion data.

**AKR-TMS hooks**

- A **common data model and feature library** versioned across banks (the precondition for federation) and a **model package format** that can
  be trained centrally on pooled consented data or federated.
- **Sharing-readiness module**: consent register, purpose tags, minimum-cohort rules, pseudonymisation, audit log of every outbound item.
- **Reference designs** for the CBI: a shared-signals register and a cross-bank external-transfer analytics service on the CBI's own data.
- **Federation lab** (stage C) built on synthetic and consented data with secure aggregation, plus poisoning and leakage tests; not enabled
  in production without a legal mandate.

## 14. Public benchmarks and datasets

### 14a. The datasets that exist

Real, labelled bank AML data is almost never public: transaction data is confidential, and labels are poor because "laundering often goes
undetected" (Altman et al.). The AMLworld paper cites one of the few real studies, 180 million transactions from an Italian bank that
was never released, and a Canadian study of 2,827 customer histories. What researchers can use is synthetic or cryptocurrency data.

| Dataset | Kind | Size and structure | Illicit share and labels | Access | Best use | Main caveats |
|---|---|---|---|---|---|---|
| **IBM AMLworld** (Altman et al., NeurIPS 2023 datasets track; arXiv 2306.16424) | Synthetic bank transactions from an agent-based generator | Six datasets, HI and LI by Small, Medium, Large: 5 to 180 million transactions, 515 thousand to 2.1 million accounts, 10 to 97 days; multi-bank, multi-currency, transfers, payments and credits; eight laundering patterns plus integration disguised as ordinary payments | Laundering rate 1 in 807 to 1 in 1,950; labels complete by construction | Kaggle and open generator; licence not verified here | Benchmark, pre-training, regression tests, graph-feature validation | US-style payment mix (ACH, wire, cheque) and US salary statistics; no cash economy, no Arabic names; patterns known to the generator |
| **SAML-D** (Oztas et al., IEEE ICEBE 2023) | Synthetic transaction-monitoring data | As recalled, about 9.5 million transactions, 12 features, 28 typologies (11 normal, 17 suspicious), UK and high-risk-country geography. **Unverified**: the Kaggle page blocks automated access | About 0.1 percent suspicious, as recalled | Kaggle | Typology-level tests | Unverified details; synthetic |
| **IBM AMLSim** (Suzumura and Kanezashi) | Synthetic multi-agent simulator | About 1.32 million transactions in its published sample (AMLworld Table 1); transfers only | 1 in 762 | GitHub | Older baseline | Simpler than AMLworld |
| **Elliptic** (Weber et al., 2019; arXiv 1908.02591) | Bitcoin transaction graph | 203,769 nodes, 234,355 directed edges, 166 features, 49 time steps about two weeks apart | 2 percent (4,545) illicit, 21 percent (42,019) licit, the rest unlabelled | Kaggle | Graph learning with partial labels; temporal splits | Cryptocurrency, not bank data; labels from a vendor's heuristics |
| **Elliptic++** (Elmougy and Liu, KDD 2023; arXiv 2306.06108) | Bitcoin transactions and wallet addresses | Over 822 thousand addresses with 56 features and 1.27 million temporal interactions; four graph types | As labelled | GitHub | Actor-level (address) detection | Same caveats |
| **Elliptic2** (Bellei et al., 2024; arXiv 2404.19109) | Bitcoin subgraphs | 122 thousand labelled subgraphs inside a background graph of 49 million node clusters and 196 million edges | Subgraph labels | Elliptic | Subgraph classification (laundering "shapes") | Same caveats |
| **Bank Account Fraud (BAF)** (Jesus et al., 2022; arXiv 2211.13358) | Privacy-preserving synthetic suite from a real anonymised account-opening fraud dataset | Six variants with temporal dynamics and class imbalance; each variant embeds a specific type of bias | Fraud labels | Kaggle | **Fairness and drift stress tests** | Fraud at account opening, not AML |

Other well-known fraud sets (the ULB card-fraud data, the IEEE-CIS fraud data, PaySim mobile-money simulation) are not AML and are not
used here.

### 14b. What public data can and cannot do for AKR-TMS

- **Can**: compare algorithms under fixed conditions; validate the graph-feature and tree-model code against published numbers (a
  sanity target: tree models with graph features reach minority-class F1 of about 63 percent on HI-Small, section 4b); provide pre-training
  data; supply regression and load tests at 5 to 180 million transactions; support vendor selection (ask any vendor to report on
  AMLworld and BAF).
- **Cannot**: say how any model performs in an Iraqi bank. None has IQD and USD cash behaviour, exchange companies, branch-liaison
  workflows, Arabic and Kurdish names, Iraqi salary-domiciliation patterns or the data quality of an Iraqi CBS.
- **Therefore** AKR-TMS needs (1) an **Iraq-specific synthetic generator** (section 16) validated against aggregate statistics from
  partner banks, and (2) pilot data from real banks under agreement, with the evaluation protocols of file 29 section 9c.

**AKR-TMS hooks**

- A **benchmark harness** that runs the graph-feature and tree-model code on AMLworld (all six datasets) and BAF and reports minority-class
  F1, precision-recall AUC, throughput and memory against the published numbers, on every release.
- **Vendor-evaluation kit** for banks: a standard request that any vendor reports on AMLworld and BAF with the same metrics and the
  same time-based splits.
- **Fairness and drift stress tests** built on BAF's bias and drift variants, run in validation (file 29 section 9i).
- **Dataset register** (source, licence, version, intended use, caveats) so no synthetic result is ever presented as bank performance.

## 15. Evaluation metrics

### 15a. A metric family, not one number

The Wolfsberg Group warns that metrics of "typology coverage, alert volumes, alert productivity, alert-to-SAR ratios" measure quantity,
and proposes success indicators of **priority risk coverage, expanded risk-indicator coverage, precision, recall, internal and external
feedback on SAR quality, and downstream integration**, plus **false-negative assessment** and **completeness of SAR information** (file 04
sections 2a and 2b). The HKMA's KPIs are alerts, cases warranting investigation, false-positive rates and STR rates, reported quarterly to
the oversight committee (file 29 section 5b). The table groups what AKR-TMS should compute.

| Group | Metric | Definition | Reading notes |
|---|---|---|---|
| **Detection quality** | Precision (alert yield) | Productive alerts divided by alerts, by scenario and score band | A low overall value is normal; compare across scenarios and over time |
| | Recall (capture) | Share of all suspicious customers or cases that were alerted | Needs an estimate of the unseen (15c) |
| | Minority-class F1 and precision-recall AUC | Harmonic mean of precision and recall for the suspicious class; area under the precision-recall curve | Preferred over accuracy and ROC-AUC on imbalanced data |
| | Precision and recall at top-k | In the k highest-ranked alerts per queue and day | Matches how analysts work |
| | Lift and capture curve | Share of eventual STRs found in the top x percent of the ranked list | Used for risk scores and alert scorers |
| | **Workload at fixed recall** | Alerts that can leave the front queue while keeping at least a chosen share of eventual STRs | The headline efficiency number for prioritisation |
| | Calibration | Brier score and reliability curve | Required if scores are shown as probabilities |
| **Funnel and yield** | Alert to case; case to STR; STRs per 1,000 alerts | Stage conversion rates | In Iraq the final stage is tiny (about 6 STRs per bank per year), so report the whole funnel and intermediate stages (alert, escalated, investigated, STR, FIU response) |
| | Unique catches | Cases found by this scenario or model and by none other | Justifies low-volume detectors |
| | Reopen and reversal rate | Closed alerts later reopened | Measures closure quality |
| **Timeliness** | **Time to detect** | Time from the first transaction of a pattern (identified in hindsight) to the first alert | Needs retrospective labelling of when the pattern began |
| | Time to disposition | Alert creation to decision, by queue and risk | Service-level compliance |
| | Time to STR | Detection to filing | CBI and FIU deadlines (file 23) |
| | Time to hold (Phase 2) | Alert to hold or release of a payment | Measures preventive value |
| **Coverage** | Typology coverage | Share of CBI and national-risk typologies mapped to rules or features with evidence of function | From the inventory (section 11c) |
| | Product, channel and population coverage | Share of products, channels, accounts and transaction value monitored | BCBS: the system must cover all accounts and transactions (file 04 section 1b) |
| | Data coverage | Completeness and validity of critical data elements; late-data rate | HKMA data-quality tests |
| **Outcome quality** | STR quality and usefulness | FIU grading and feedback; completeness of information; share in priority areas | File 04 section 2m; file 01 section 5 |
| | False-negative assessment | Suspicious cases that reached the bank by other routes and were missed by the system | Wolfsberg new metric |
| **Stability and fairness** | Drift and stability | PSI of inputs and scores; alert volume control charts | File 29 section 9d |
| | Group rate ratios | Alert, escalation and STR rates by customer group | File 29 section 9i |
| **Efficiency** | Cost per STR, analyst hours per productive case, backlog age | Resources against outcomes | Use as a bank-level measure, never an individual KPI |

### 15b. Choosing the operating point

Given alert capacity C and costs per missed case (c_miss) and per review (c_review), the best threshold minimises c_miss times expected
misses plus c_review times alerts reviewed, subject to alerts not exceeding C. In practice banks cannot price a miss in money; the
workable form is to fix the **accepted miss rate** in the risk-appetite statement (file 29 section 9c: Wolfsberg part II) and choose the
smallest alert load that meets it, confirmed by below-the-line sampling.

### 15c. Estimating what the system missed

Recall cannot be computed from alerts alone, because unalerted activity is never reviewed. Four complementary estimates:

1. **Below-the-line sampling** with rare-event bounds (file 29 section 9c).
2. **Seeded cases**: synthetic typology instances injected into history (section 5e); recall of seeded cases measures coverage of known patterns.
3. **Other-route cases**: branch referrals, RFIs, authority requests and STRs that did not originate from the system; the share the system
   would have alerted is a recall estimate (Wolfsberg "false-negative assessment").
4. **Capture-recapture** (a statistical technique from epidemiology, offered here as a design idea, not a regulator method): if two
   independent detection channels (for example the system and branch referrals) find n1 and n2 confirmed cases with m in common, the
   Lincoln-Petersen estimate of the total is n1 times n2 divided by m, and recall of the system is about m divided by n2. It assumes
   independent channels, which is rarely true, so it gives an order of magnitude and a prompt to investigate, and stratified or log-linear
   versions are needed for more than two channels.

### 15d. Who sees which metrics

| Audience | Pack |
|---|---|
| Analyst and team lead | Queue age, personal and team funnel (without individual throughput targets), reason-code feedback |
| MLRO and committee | Per-scenario productivity, unique catches, coverage, drift, fairness, backlog, override rates |
| Board | Effectiveness indicators (priority risk coverage, STR quality, time to detect), risk appetite status, major incidents |
| CBI and assessor | Coverage matrix, validation evidence, tuning log, KPI history |
| Correspondents | CBDDQ evidence pack (file 04 section 2c) |

**AKR-TMS hooks**

- A **metrics service** computing the table above from the inventory, case and label data, with definitions stored and versioned so a
  KPI means the same thing in every bank.
- **Funnel view** with intermediate stages and the workload-at-fixed-recall curve.
- **Recall-estimation toolkit**: BTL sampling plans, seeded-case runs, other-route reconciliation and a capture-recapture calculator with
  warnings.
- **Role-based metric packs** and exports.

## 16. What works for banks with few labelled cases and a cash-heavy economy

The constraints are in section 1: about 6 STRs per bank per year, no independent outcome feedback, heavy cash, dollarisation, uneven data.
The techniques below are ordered from least to most data-hungry, with the evidence for each.

### 16a. Typology-driven rules and expected-versus-actual profiles (no labels needed)

The CBI catalogue encodes typologies (file 27). Effectiveness is measured by productivity, not by labels. Two upgrades need no outcome data:
**per-segment, per-currency thresholds** from percentile and robust statistics (file 29 section 9c) and **expected-versus-actual profile
deviation**, which the CBI names as a system duty and which uses the bank's own KYC declarations as the "label" of normal.

### 16b. Weak supervision: rules as labelling functions

Snorkel (Ratner et al., arXiv 1711.10160) lets users "train state-of-the-art models without hand labeling any training data" by writing
labelling functions that express heuristics with unknown accuracies and correlations; a label model denoises them. In AML the labelling
functions are the CBI rules, the Wolfsberg-style indicators, screening hits, adverse media, shared-attribute links to known bad entities and
branch-staff observations. A model trained on the combined weak labels can **generalise beyond any single rule** (for example by scoring
combinations of weak signals). Caution: the model inherits the rules' blind spots, so it is an alert ranker and a feature generator, not a
substitute for confirmed cases (section 4c selection bias), and its output is validated on independent cases (L3 and L4 labels).

### 16c. Semi-supervised and positive-unlabelled learning

- **A few confirmed cases are worth more than algorithm choice**: in ADBench, with 1 percent labelled anomalies most semi-supervised methods
  beat the best unsupervised method (section 5c).
- **Graph label propagation**: risk flows from known bad entities (STR subjects, national-list entries, exited customers) to neighbours
  with decay by hops and link type (section 6c); this uses very few labels.
- **Self-supervised graph embeddings** trained without labels, then a simple classifier on the few labels (Inspection-L, arXiv 2203.10465).
- **Positive-unlabelled (PU) learning**: treat confirmed cases as positive and everything else as unlabelled (not negative). The classical
  result (Elkan and Noto, KDD 2008; not re-fetched) is that if labelled positives were a random sample of all positives, a classifier of
  labelled versus unlabelled gives probabilities proportional to the true class probability, which can be rescaled. In AML the sample is not
  random (rules decided what was investigated), so PU learning needs a model of the selection process or an unbiased audit sample (section
  4c); used carefully it avoids the error of calling all unalerted customers "clean".

### 16d. Transfer learning and pooled training

The AMLworld experiment is direct evidence: models trained on the HI data and fine-tuned for five epochs on the scarcer LI data raised
minority-class F1 on LI-Small from 0.0 to 27.4 and beat models trained only on LI for LI-Medium (section 4b). The recipe for AKR-TMS is
**pre-train on synthetic and pooled consented data, localise on the bank's data**:

1. Pre-train a starter model on synthetic Iraqi-style data plus public benchmarks (labelled "externally calibrated, temporary", file 29
   GV-05, following CBUAE 4.7.5).
2. Localise by recalibrating the intercept and thresholds on the bank's few labels, then fine-tune only when the label gates of section 4f are met.
3. Evaluate by **leave-one-bank-out** testing across pilot banks, because the product must work on a bank it has never seen.

### 16e. Synthetic data, built for Iraq

AMLworld's authors released a generator because "real data to train machine learning models to detect laundering is generally not
available"; its design (agents, accounts, banks, currencies, placement-layering-integration, eight patterns) is a template. An
**Iraq-specific generator** would model: salaried government and private employees paid through domiciled accounts, pensioners,
shopkeepers and wholesale traders with high cash share, import traders with LC and transfer flows, exchange and money-transfer companies,
NGOs, contractors paid by state bodies; IQD and USD behaviour; calendar effects; branch structure and third-party depositors; Arabic and Kurdish
name variants with realistic misspellings; CBS data-quality defects (missing fields, duplicate customers); and the CBI and FinCEN typologies
(structuring across branches, mule rings, pass-through, trade-based laundering, exchange-house layering, hawala-linked pooling). Uses:
pre-training, seeded-typology tests (section 5e), load tests, demos without real data. Controls: **validate realism against aggregate
statistics from partner banks** (distributions of cash share, ticket size, counts, branch spread) under NDA; never report synthetic-trained
performance as real performance; keep a log of generator versions; beware "teaching to the generator" (models that learn its artefacts).

### 16f. Active learning from analyst decisions

Use the analysts' scarce attention where it teaches most: ask for second review or a structured label on the alerts the model is least sure
about (uncertainty sampling) or where committees of models disagree (query by committee), plus a **random sample** to keep an unbiased
evaluation set. Embed it in the workflow ("confirm this mule pattern?") instead of a separate labelling task, and keep the controls of
section 4c (feedback loops, label quality, inter-rater agreement). The AI Act's Article 15(4) concern about continuously learning systems
applies: do not let the model's own ranking decide, unchecked, which cases ever get labelled.

### 16g. Cash-economy features and thresholds

- **Features** that carry signal when cash leaves little trace: cash share of inflows and outflows; cash velocity (cash in then out within 48
  hours); third-party depositor share and number of distinct depositors; branch dispersion of deposits; same-day cash-in and transfer-out;
  USD versus IQD cash; round-amount share; cash relative to declared turnover; cash-intensity of the customer's sector.
- **Relative thresholds**: percentiles within a segment and currency rather than absolute amounts; recomputed on a schedule and
  calendar-adjusted (section 3c).
- **Seasonality and events**: baselines by pay day, Eid, Ramadan, Arbaeen, harvest and school-fee cycles.
- **Normal is local**: in a cash society "unusual" is common, so alert on departure from the segment's own normal and from the customer's
  own history, and on structure (many small deposits across branches), not on cash itself.

### 16h. A maturity ladder with gates (design proposal)

| Stage | Capability | Gate to enter | Evidence basis |
|---|---|---|---|
| 0 | CBI rules, per-segment thresholds, expected-versus-actual, entity resolution, consolidation | Data mapped and reconciled | CBI catalogue; HKMA data tests |
| 1 | Ranking of rule alerts (weak-supervision and stacking, GBT) in shadow, then live ordering | At least 500 dispositions with QA grades; BTL baseline measured | Sections 4, 9 |
| 2 | Anomaly detectors and graph features in shadow, then as alert sources and features | Deviation store live; seeded-typology suite passing | Sections 5, 6; ADBench |
| 3 | Supervised STR-propensity with pooled or synthetic pre-training and local calibration | Label gates (section 4f); legal mandate for any pooling; leave-one-bank-out results | Section 4b-d; AMLworld |
| 4 | GenAI assistance (summaries, drafts) under the evaluation protocol | Golden sets passed; controls of file 29 section 9k | Section 12 |
| 5 | Federated or shared-signal capabilities | CBI mandate; common data model; federation lab results | Section 13 |

**What not to do**: train a supervised model on one bank's six STRs; downsample so hard that the base rate is lost; treat unalerted
customers as clean negatives; replace the CBI scenarios with a model; auto-close alerts on an ML score; report synthetic-data results as
bank performance.

**AKR-TMS hooks**

- **Weak-label factory**: labelling functions from rules, lists, adverse media and network links, with a label model and an audit of each
  function's agreement with confirmed cases.
- **Starter models and local-calibration tooling** with explicit "externally calibrated" status and a localisation plan.
- **Iraq synthetic data generator** and validation harness against partner-bank aggregates.
- **Active-learning queue** inside the case workflow with a mandatory random-sample share.
- **Cash-economy feature library** and relative-threshold engine.
- **Maturity-ladder dashboard** per bank showing the current stage and the next gate.

## 17. What it means for AKR-TMS

### 17a. The detection stack, layer by layer

| Layer | Purpose | Method | Needs | Explanation | Governance (file 29) | Phase |
|---|---|---|---|---|---|---|
| **0. Data and entities** | One reliable view of customers, accounts, counterparties | CDE registry, reconciliation, entity resolution | CBS replica | Link evidence | Tier 1 (ER); data controls | 1 |
| **1. CBI scenario rules** | Legal minimum, transparent coverage | Declarative scenarios with per-segment, per-currency parameters | Transactions, KYC profile | Exact | Track A (configuration) | 1 (L0 rules in 2) |
| **2. Profile and peer deviation** | Expected-versus-actual; segment-relative thresholds | Segmentation, robust deviation store | History | Exact numbers | Track A/B | 1 |
| **3. Alert ranking and consolidation** | Cut wasted reviews | Case grouping, priority score, GBT scorer when gated | Dispositions | Reason codes | Tier 1-2 | 1 |
| **4. Anomaly and graph detectors** | Find what rules were not written for | Robust deviation, ECOD or isolation forest, streaming motifs, graph features | Transactions, links | Deviation facts, subgraph | Tier 2 | 1 (shadow first) |
| **5. Customer risk rating** | Set monitoring intensity and reviews | Scorecard plus dynamic triggers | KYC, monitoring outputs | Factor list | Tier 1 | 1 |
| **6. Supervised STR propensity** | Learn from confirmed cases | Pooled or pre-trained tree models, local calibration | Labels (gated) | Attribution | Tier 1 | 1 late/2 |
| **7. GenAI assistance** | Speed investigation and writing | Grounded LLM with human review | Case records | Citations | Tier 2-3, Track C | 1 late |
| **8. Pre-release API** | Stop before settlement | L0 rules, lists, counters; ML advisory | Warm state | Reason codes | Tier 1 | 2 |
| **9. Shared signals** | Cross-bank view | Staged sharing (13f) | Legal mandate | Provenance | Per scheme | Later |

### 17b. Technology decisions and why

| Decision | Choice | Reason |
|---|---|---|
| Foundation of detection | CBI rules always on; everything else additive | Legal minimum; MER finding that banks have no scenarios of their own |
| Default learner | Gradient-boosted trees with monotonic constraints and calibrated outputs, on engineered transaction and graph features; glass-box baseline for Tier 1 | MAS names these models in AML; AMLworld shows trees with graph features match or beat GNNs; GFP runs on CPU faster than GNNs on a GPU; explainable by TreeSHAP |
| Graph | Features and streaming motifs first, in a Rust engine; GNNs later, in shadow | On-premises and CPU-only constraint; evidence above |
| Anomaly detection | Robust per-feature deviation plus ECOD or isolation forest, capacity-based alerting | Explainable; ADBench shows no single unsupervised winner |
| Entity resolution | Probabilistic linkage with Arabic comparators, persisted and reversible | Everything downstream depends on it; MER two-passports case |
| Alert handling | Consolidate, rank, route, hibernate; no auto-close in Phase 1 | CBI written-findings expectation; miss risk of suppression; open question for the CBI |
| Real time | Four latency classes; one definition, several execution plans; Phase 2 starts with rules and lists, ML advisory | CBI stop-until-cleared; avoid divergence between batch and real time |
| Explainability | Reason-code library, typology-to-feature inventory, evidence-first screen | Wolfsberg part II; CBI coverage matrix; MAS |
| Labels | Label ladder with quality grades and gates; weak supervision, pooled and synthetic pre-training, active learning with random sample | About 6 STRs per bank per year; ADBench; AMLworld transfer results |
| GenAI | Assist only, on premises, evaluated on golden sets, staged release | MAS, HKMA, Wolfsberg; file 29 controls; HKMA's call for value beyond documentation |
| Data sharing | Staged A-E, CBI-led for entity and transfer data; legal basis first | TMNL's experience; COSMIC and AMLR show legal design; no Iraqi basis found |
| Benchmarks | Public sets for regression and sanity; Iraq-specific generator validated on partner aggregates | Public sets lack Iraqi features |
| Metrics | One metrics service, funnel and workload-at-fixed-recall, recall-estimation toolkit | Wolfsberg new metrics; HKMA KPIs |

### 17c. What leaders should take from this file (for the plan PDF)

1. **The product starts with the CBI's own rules and makes them work better**: right thresholds for each customer group, one view of each
   customer, and fewer wasted reviews, before any "AI" claim.
2. **Machine learning is added where it is proven to help and is explained alert by alert.** Gradient-boosted scoring ranks the rule
   alerts; anomaly and network detectors look for what no rule describes; every output gives reasons in Arabic and English.
3. **Labels are the real constraint.** Iraqi banks file about six STRs each per year, so the product learns from several sources: rules as
   weak labels, pooled and synthetic data, and the analysts' own decisions, with gates that say when a model is ready.
4. **Generative AI helps staff write and summarise; it never decides.** It runs on the bank's premises and is tested on the bank's own cases.
5. **Sharing between banks is powerful and needs a legal basis.** Singapore, the EU, the UK and the US each built one; the Dutch TMNL
   utility was wound down while the law caught up. In Iraq the CBI, which already sees all external transfers, is the natural hub.
6. **Everything is measured the way supervisors now ask**: outcomes and coverage, not alert counts.

### 17d. Risks, gaps and open questions

- The CBI's position on ML-based ranking, hibernation and any automated closure is unknown; the product keeps a human disposition on every
  presented alert until the CBI says otherwise (file 29 section 10 question b).
- The CBS posting cadence caps real-time detection; some banks will see only end-of-day data (file 25).
- No independent evidence of LLM accuracy in AML tasks was found; every GenAI benefit must be measured locally.
- The Swift federated-learning programme and the SAML-D dataset details are unverified (sources blocked); the 90-95 percent false-positive
  claim is untraced; HKMA and bank results are customer-reported.
- The web-search budget ran out during the research, so sources reached by direct address only; some regulators (ECB, BaFin, DNB, FSB,
  Jordan, Qatar) were not reached.
- Public datasets are synthetic or cryptocurrency-based; the value of the Iraq generator depends on partner-bank cooperation.

## 18. Derived requirements (T-xx)

| ID | Requirement | Source |
|---|---|---|
| T-01 | Scenarios are declarative objects (family, parameters per segment and currency, window, exclusions, CBI article, typology, owner) compiled from one definition to set-based SQL, incremental counters and scheduled jobs | CBI catalogue (file 27); HKMA functional testing (file 29 5b); section 2b |
| T-02 | The CBI minimum scenarios ship as a baseline pack with default parameters per segment and are never retired by the product; bank-added scenarios are governed separately | MER paras 563-582 (file 01); Wolfsberg part I (file 04 2a) |
| T-03 | Every alert freezes its evidence: transactions, window, parameter version and data snapshot | SS1/23 3.5; HKMA functional testing |
| T-04 | Windows, thresholds and baselines are calendar-aware (rolling versus calendar, holidays, pay days, Eid, Ramadan, Arbaeen) and IQD and USD are separate by default | Section 2b and 3c |
| T-05 | Scenario productivity dashboard (alerts, cases, STRs, unique catches, overlap) with retire-or-keep advice for bank-added scenarios only | Wolfsberg part I; HKMA KPIs |
| T-06 | Structured branch-event capture so behaviour-based CBI scenarios (for example cash 8) have data | CBI catalogue (file 27) |
| T-07 | Three-level segmentation (static, behavioural cluster, risk overlay) with a workbench showing profiles, stability and the alert-volume effect before activation | HKMA April 2024; Wolfsberg part I |
| T-08 | Deviation store of robust peer and own-history deviations per customer, feature and period; "segment of one" handling for large corporates and government bodies | Sections 3c and 5b |
| T-09 | Segment-migration alerts and a KYC-refresh trigger when a customer's cluster changes | Wolfsberg digital lifecycle (file 04 2q) |
| T-10 | Minimum segment size rule and a governed calendar service used by thresholds, baselines and explanations | Section 3c |
| T-11 | Label registry with source level L1-L5, quality grade, timestamps and leakage-safe cut-off; controlled closure-reason vocabulary | Sections 4c-4d; Wolfsberg part II (file 04 2b) |
| T-12 | QA sampling and re-grading of closures and STRs with inter-rater agreement tracking | FATF 2021 "trained-in" errors (file 03 5a); Wolfsberg part II |
| T-13 | The alert scorer ranks rule alerts only; no auto-suppression in Phase 1; a forced-review sample of low scores feeds an unbiased evaluation set | Section 4c; AI Act Article 15(4) |
| T-14 | Gradient-boosted trees with monotonic constraints, calibrated scores and small documented feature sets as the default; a glass-box baseline required for Tier 1 | MAS 2024 paper (file 29 4b); AMLworld; Rudin |
| T-15 | Label gates per model and bank (about 50, 200 and 500 positives) and a model-ladder dashboard | Section 4f (design proposal) |
| T-16 | Time-based backtesting harness with lag-aware windows, precision-recall reporting and workload-at-fixed-recall curves | Sections 4 and 15 |
| T-17 | Point-in-time feature store with leakage tests and a training-serving skew monitor | Section 10c |
| T-18 | Robust deviation layer plus ECOD or isolation-forest detectors, with capacity-based (top-k) alerting alongside thresholds | ADBench; ECOD; section 5 |
| T-19 | Anomaly evaluation suite: seeded typologies, analyst acceptance rate, unique catches, rank stability | Section 5e |
| T-20 | Graduation path from a productive anomaly pattern to a documented rule | Wolfsberg part I (file 04 2a) |
| T-21 | Entity graph with typed nodes and edges, link weights and a supernode policy | Section 6 |
| T-22 | Streaming motif engine for fan-in, fan-out, gather-scatter, scatter-gather, cycles, bipartite and stack patterns with frozen evidence subgraphs | AMLworld; GFP (arXiv 2402.08593) |
| T-23 | Graph features (degree, reach, shared-attribute counts, hops to listed entities, neighbour risk) feed the scorer and the customer risk score | Section 6c |
| T-24 | Kinship links hidden by default, excluded from automated scores unless validator-approved, and logged when shown | File 29 section 9i; section 6d |
| T-25 | GNN experiments only in shadow after the pooled-data gate; tree models with graph features first | AMLworld Table 2; GFP |
| T-26 | Persistent entity-resolution service with link evidence, thresholds, three link strengths and one-click unmerge | Fellegi-Sunter; Splink; section 7 |
| T-27 | Arabic-aware comparators and blocking; never auto-merge records with different national ID numbers | File 22; section 7c |
| T-28 | Quarterly ER audit (pairwise precision and recall, B-cubed, reversal rate, error rate by name family); the matcher is Tier 1 | Section 7d-7e; file 29 |
| T-29 | Counterparty resolution for free-text names in transfers and cash deposits, learning from analyst confirmations | Sections 6b and 7b |
| T-30 | Scorecard builder with factor library, versioning, maker-checker and what-if distribution simulator | CBI 2025 controls (file 01 2d); Wolfsberg RBA 2026 |
| T-31 | Dynamic re-rating engine fed by monitoring and event triggers, with hysteresis and minimum dwell time | CBI dynamic update; HKMA DRA case (file 29 5c) |
| T-32 | Rating overrides with reason, approver and expiry; override-rate monitoring | CBUAE MMS 8.4 (file 29 7a) |
| T-33 | Risk-score tests (outcome concentration, distribution, stability, back-test at event, fairness) in the validation pack | Section 8d |
| T-34 | Per-customer rating explanation page (factors, values, points, overrides) | CBI; Wolfsberg |
| T-35 | Case grouping by customer and period with every contributing scenario listed | HKMA April 2024 annex (file 29 5a) |
| T-36 | Ranking service with a visible, decomposed priority score and maximum waiting times | Section 9b |
| T-37 | Routing and service-level engine for the branch liaison and central tiers | MER workflow (file 01 2) |
| T-38 | Hibernation and whitelist registry (envelope, expiry, owner, approver), suppressed-alert ledger, audit sampling and regret metric; evaluation of mandatory scenarios always runs | Section 9b |
| T-39 | Policy switches for continuing-activity review and auto-close, off by default, each with a rationale field; CBI position to be sought | FinCEN FAQs Oct 2025 (file 05 1b); CBI controls |
| T-40 | Latency class (L0-L3) on every scenario and model; one definition executed on the right plan | Section 10b |
| T-41 | Watermark ingestion with event-time and load-time, late-data handling, reconciliation and disclosure of the CBS posting cadence | HKMA CDE and lineage (file 29 5b); file 25 |
| T-42 | Online and offline feature consistency with a skew monitor | Section 10c |
| T-43 | Phase 2 API contract: decision vocabulary, timeouts, bank-set fail-open or fail-closed policy, decision log, replay tool, shadow mode before holds, ML advisory only until validated | CBI stop-until-cleared (file 01 2d); MAS 6.5.6; file 29 GV-53 |
| T-44 | Reason-code library with stable IDs, bilingual reviewed templates, typology and CBI mapping, reason families and mitigating factors, shared by rules, scorecards, anomaly detectors and ML | Wolfsberg part II; MAS 6.3.11 |
| T-45 | Typology-to-feature coverage inventory generated from the model inventory and usable as the CBI coverage matrix | Wolfsberg part II (file 04 2b) |
| T-46 | Explanation tests (fidelity, stability, comprehension) in validation and after each release | Section 11d |
| T-47 | Evidence-first alert page with thresholds and weights hidden from branch roles | SAMA 3.8 (file 29 7b); Wolfsberg AI principle 5 |
| T-48 | Nine GenAI use cases switchable per bank, each behind the file 29 controls and a staged release (assist, recommend, pre-fill), none autonomous | MAS 7.1; Wolfsberg part II; file 29 GV-45 |
| T-49 | GenAI evaluation workbench: golden sets of at least 200 cases per use case in Arabic and English, error taxonomy, shadow mode, QA sampling | MAS 7.1; section 12e |
| T-50 | Agent containment (least privilege, allow-listed actions, runtime policy check, kill switch) before any agentic feature ships | SAFR; OWASP LLM06 |
| T-51 | Arabic-capable open models run on premises; no suspicion content to external APIs; model choice by bank-specific evaluation | File 29 GV-52; Jais and ALLaM; file 32 |
| T-52 | Common data model and feature library versioned across banks; model package format for central or federated training | Section 13e; file 25 |
| T-53 | Sharing-readiness module: consent register, purpose tags, minimum cohort size, pseudonymisation, outbound audit log; no sharing without a legal mandate | AMLR Article 75; COSMIC; section 13d |
| T-54 | Reference designs for the CBI: shared-signals register and cross-bank external-transfer analytics on the CBI's own data | Section 13e |
| T-55 | Federation lab on synthetic and consented data with poisoning and leakage tests; production federation only with a mandate | Section 13f |
| T-56 | Public benchmarks used for regression, pre-training and sanity targets; vendors asked to report on AMLworld and BAF | Section 14b |
| T-57 | Iraq-specific synthetic generator validated against partner-bank aggregates, with a version log and seeded typologies | Section 16e |
| T-58 | Leave-one-bank-out evaluation protocol for every model that is shared across banks | Section 16d |
| T-59 | Metrics service with versioned definitions and role-based packs | Section 15; Wolfsberg part II; HKMA KPIs |
| T-60 | Funnel view with intermediate stages and the workload-at-fixed-recall curve | Section 15 |
| T-61 | Recall-estimation toolkit: BTL sampling plans, seeded runs, other-route reconciliation, capture-recapture calculator with warnings | Section 15c; file 29 section 9c |
| T-62 | Weak-label factory: labelling functions from rules, lists, media and links, a label model, and an agreement audit against confirmed cases | Snorkel; section 16b |
| T-63 | Starter models flagged "externally calibrated, temporary" with local-calibration tooling and a localisation plan | CBUAE MMS 4.7.5; file 29 GV-05 |
| T-64 | Active-learning queue inside the case workflow with a mandatory random-sample share | Section 16f; AI Act 15(4) |
| T-65 | Cash-economy feature library and a relative-threshold engine (percentiles by segment and currency, calendar-adjusted) | Sections 3c and 16g |
| T-66 | Maturity-ladder dashboard per bank showing current stage and next gate | Section 16h |

## 19. Sources

Primary and near-primary (read on 2026-10-06):

Papers and datasets
- [Altman et al., Realistic Synthetic Financial Transactions for Anti-Money Laundering Models (IBM AMLworld), arXiv 2306.16424](https://arxiv.org/abs/2306.16424)
- [Egressy et al., Provably Powerful Graph Neural Networks for Directed Multigraphs (Multi-GNN), arXiv 2306.11586](https://arxiv.org/abs/2306.11586)
- [Blanuša et al., Graph Feature Preprocessor, arXiv 2402.08593](https://arxiv.org/abs/2402.08593)
- [Weber et al., Anti-Money Laundering in Bitcoin (Elliptic), arXiv 1908.02591](https://arxiv.org/abs/1908.02591)
- [Bellei et al., The Shape of Money Laundering (Elliptic2), arXiv 2404.19109](https://arxiv.org/abs/2404.19109)
- [Elmougy and Liu, Demystifying Fraudulent Transactions and Illicit Nodes in the Bitcoin Network (Elliptic++), arXiv 2306.06108](https://arxiv.org/abs/2306.06108)
- [Weber et al., Scalable Graph Learning for Anti-Money Laundering: A First Look, arXiv 1812.00076](https://arxiv.org/abs/1812.00076)
- [Lo et al., Inspection-L: Self-Supervised GNN Node Embeddings for Money Laundering Detection in Bitcoin, arXiv 2203.10465](https://arxiv.org/abs/2203.10465)
- [IBM AMLSim repository](https://github.com/IBM/AMLSim)
- [Jesus et al., Bank Account Fraud (BAF) datasets, arXiv 2211.13358](https://arxiv.org/abs/2211.13358)
- [Han et al., ADBench: Anomaly Detection Benchmark, arXiv 2206.09426](https://arxiv.org/abs/2206.09426)
- [Li et al., ECOD, arXiv 2201.00382](https://arxiv.org/abs/2201.00382); [COPOD, arXiv 2009.09463](https://arxiv.org/abs/2009.09463); [PyOD, arXiv 1901.01588](https://arxiv.org/abs/1901.01588)
- [Lundberg and Lee, SHAP, arXiv 1705.07874](https://arxiv.org/abs/1705.07874); [Lundberg et al., TreeSHAP, arXiv 1905.04610](https://arxiv.org/abs/1905.04610); [Ribeiro et al., LIME, arXiv 1602.04938](https://arxiv.org/abs/1602.04938)
- [Nori et al., InterpretML, arXiv 1909.09223](https://arxiv.org/abs/1909.09223); [Rudin, Stop Explaining Black Box Machine Learning Models, arXiv 1811.10154](https://arxiv.org/abs/1811.10154)
- [Ratner et al., Snorkel, arXiv 1711.10160](https://arxiv.org/abs/1711.10160)
- [Jais and Jais-chat, arXiv 2308.16149](https://arxiv.org/abs/2308.16149); [ALLaM, arXiv 2407.15390](https://arxiv.org/abs/2407.15390)
- [Splink documentation (probabilistic record linkage)](https://moj-analytical-services.github.io/splink/)

Data sharing
- [BIS Innovation Hub, Project Aurora (31 May 2023)](https://www.bis.org/publ/othp66.htm)
- [MAS: COSMIC page](https://www.mas.gov.sg/regulation/anti-money-laundering/cosmic); [MAS media release, 1 April 2024](https://www.mas.gov.sg/news/media-releases/2024/mas-launches-cosmic-platform); [COSMIC infographic](https://www.mas.gov.sg/-/media/mas-media-library/regulation/anti-money-laundering/cosmic-infographics-9-may-2023.pdf)
- [TMNL website](https://www.tmnl.nl/en/); [TMNL press release, 1 July 2024](https://tmnl.nl/en/article/transaction-monitoring-netherlands-adapts-its-working-method-to-new-european-legislation/); [TMNL, 23 April 2024](https://tmnl.nl/en/article/tmnl-assesses-the-opportunities-of-new-european-anti-money-laundering-legislation/)
- [Economic Crime and Corporate Transparency Act 2023, section 188](https://www.legislation.gov.uk/ukpga/2023/56/section/188)
- [FinCEN, Information Sharing Under Section 314(b)](https://www.fincen.gov/section-314b) and [Section 314(b) Fact Sheet, 12 June 2026](https://www.fincen.gov/system/files/shared/314bfactsheet.pdf)
- [Regulation (EU) 2024/1624, AML Regulation (OJ text)](https://eur-lex.europa.eu/eli/reg/2024/1624/oj)
- [ICO guidance on privacy-enhancing technologies](https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/data-sharing/privacy-enhancing-technologies/)

Regulators on AI (all listed with URLs in file 29 section 12; those used here)
- [US Treasury, Artificial Intelligence in Financial Services (December 2024)](https://home.treasury.gov/system/files/136/Artificial-Intelligence-in-Financial-Services.pdf)
- HKMA circular of 9 September 2024 and annex; thematic review of 17 April 2024; circular and report of 22 June 2026 (URLs in file 29 section 12)
- MAS AI model risk information paper (December 2024), consultation P017-2025 and SAFR white paper (URLs in file 29 section 12)
- FCA AI approach page and AI Live Testing press release; EBA factsheet of 21 November 2025; CBUAE Model Management Standards and enabling-technology guidelines; SAMA Counter-Fraud Framework (URLs in file 29 section 12)

Files in this folder used: 01 (CBI controls, MER monitoring and STR statistics, data protection), 02 (cash economy, typologies), 03 (FATF 2021 new technologies, data pooling), 04 (Wolfsberg parts I and II, RBA 2026), 05 (FinCEN, SAR FAQs, Iran advisory, model risk), 22 (names), 27 (CBI scenario catalogue), 29 (governance).

Not reached or not verified: Swift's federated-learning announcements (swift.com blocks automated access); SAML-D dataset description (Kaggle blocked; figures recalled); Isolation Forest (Liu et al. 2008) and PU-learning (Elkan and Noto 2008) papers cited from background knowledge; FATF 2021 and 2022 data-sharing reports (fatf-gafi.org blocked; cited through file 03); JMLSG guidance; ECB, BaFin, DNB, FSB and other-regulator AI statements; independent evaluations of LLM accuracy on AML tasks; COSMIC results after launch; the "90 to 95 percent false positive" industry figure.
