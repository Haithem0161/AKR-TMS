# 31. Analyst user experience: roles, evidence, Arabic-first design, screens and rules

Status: done (2026-10-07)

Prefix for derived requirements: UX (UX-01, UX-02, ...). Written by the research agent for files 28 and 31. Cross-references use "file NN" for the other files in this folder; file 28 owns the case workflow and states, file 22 the name matching and its explanation, file 32 the stack (section 18 the frontend), file 30 the access and tipping-off controls. The approved visual identity is `plan/identity.md`; the eight approved mockups are `plan/images/final/01-command-centre.png` to `08-data-connection.png` with prompts in `plan/images/prompts/`. Anything not verified is marked "NOT VERIFIED". Evidence grades: A = primary source (standard, regulator text, official documentation, peer-reviewed paper, measurement made for this file) read in full or in the relevant part; B = primary source read through a secondary summary; C = vendor or practitioner statement; D = inference by the author.

## Contents

1. Scope, method and the starting point (identity, mockups, DBS conventions)
2. Who the users are and what they do all day
3. Evidence on analyst productivity, alert fatigue and what interface patterns do about it
4. Arabic-first right-to-left design: bidirectional text, digits, dates, names, typography, sorting and search
5. Accessibility (WCAG 2.2), contrast of the approved palette, and keyboard use
6. Bilingual help, training and onboarding inside the product
7. Explanation design: why an alert fired, why a screening hit scored as it did, what the AI suggested
8. Dashboards, reports and exports for the MLRO, the board and the assessor
9. The user experience of the vendor products in files 08-16
10. Screen inventory for AKR-TMS
11. Interaction rules
12. What the eight mockups need
13. What it means for AKR-TMS
14. Derived requirements (UX-xx)
15. Sources

(Sections are appended as they are researched; each topic section ends with "AKR-TMS hooks".)

## 1. Scope, method and the starting point

**Question.** What should the people who use AKR-TMS every day see and do, in Arabic first and English second, so that the system shortens the time between an alert and a defensible decision without hiding evidence, and so that a CBI-approved assessor, an MLRO and a branch liaison officer can each use it correctly on their first day?

**Method.** (1) Read the approved identity, the eight mockups (viewed one by one) and their prompts. (2) Read the frontend conventions of the sibling Digital Banking System (DBS): `.claude/rules/frontend.md`, `docs/DESIGN-SYSTEM.md`, `DESIGN.md`, `PRODUCT.md`, and the `frontend/src/i18n/` helpers. (3) Measured, on 2026-10-07: digit usage in 16 Iraqi regulator texts; the Unicode CLDR defaults for Iraqi Arabic and Sorani Kurdish; the glyph coverage and digit widths of the fonts vendored in `plan/assets/fonts/`; the contrast ratios of the identity palette (section 5). (4) Read peer-reviewed and standards sources reachable without search (arXiv, W3C, publishers' open pages). (5) Reused files 01, 13, 20, 21, 22, 28, 29, 30 and 32 and the vendor findings of files 08-16.

**The approved visual identity** (`plan/identity.md`, proposal of 2026-10-06): light theme on a warm clay background (`#F5F0E6`), white data cards, a lapis navigation rail (`#0F2147`) with a gold active marker (`#C8A24B`, never a status colour), Ishtar Blue (`#1E4FA0`) for primary actions, IBM Plex Sans Arabic with IBM Plex Sans (one family designed to sit together), IBM Plex Mono for identifiers, amounts and account numbers, 8 px radius, thin sand borders, "dense but quiet", "every alert shows its reason codes", "Arabic-first: layouts are designed right-to-left and mirrored for English; the mockups in the plan show the English version because the plan is in English".

**The approved mockups** (English, left-to-right, 1536 x 1024, generated with the common head and tail prompts; Arabic names composited into empty boxes later):

| No. | Screen | What it shows |
|---|---|---|
| 01 | Command centre (MLRO) | Five KPI tiles; alert backlog by age and workstream; regulatory deadlines; risk distribution; top scenarios with productive rate; correspondent evidence pack |
| 02 | Case workspace | Summary strip; "Why this case was raised" chips and CBI scenario rows; draft AI summary with "DRAFT · REQUIRES REVIEW"; money in and out against declared turnover; counterparties; decision panel; activity log |
| 03 | Screening review | Side-by-side customer and list entry with Arabic names in boxes and transliteration; score badge; per-part name comparison; decision tiles; pair-memory checkbox; screening history |
| 04 | Customer risk profile | Risk gauge and history; score drivers; EDD checklist; ownership tree; evidence vault list |
| 05 | Scenario studio | Parameters; plain-language description; version test chart; change request approval; version history |
| 06 | Network view | Ego graph with mule ring, senders and exchange companies; cluster summary; links found; next steps |
| 07 | goAML report builder | Six-step stepper; reason and indicators; narrative with the draft badge; validation panel; timeline |
| 08 | Data connection | Read-only status; replica delay; reconciliation table; data-quality issues; feed volume |

The navigation rail in all eight has nine entries: Command centre, Alerts, Cases, Customers, Screening, Scenarios, Reports, Data, Governance. The mockups were made to explain the product to bank leadership, not to specify it: they omit the work queue, the branch window, the RFI desk, QA, the MLRO decision desk, the assessor workspace and administration. Section 10 supplies those, and section 12 lists exactly what the eight need to change.

**The DBS conventions that bind AKR-TMS** (file 32 section 18.1; read from the DBS sources on 2026-10-07):

- React 19, TanStack Router (file routes), Query, Table 9, Tailwind v4 with the Vite plugin and no config file, Orval-generated API hooks, `react-aria-components` over a vendored Untitled UI set, i18next, Zod with react-hook-form, `@xyflow/react`, Motion, Fontsource-vendored fonts with "no runtime font requests".
- `dir` and `lang` are set on `<html>` from the locale; logical properties only (`ms-*`, `me-*`, `ps-*`, `pe-*`, `text-start`); "a PR containing `ml-` or `mr-` fails review"; directional icons flip through one attribute; Western digits (0-9) and `YYYY-MM-DD HH:MM` in both languages; no `toLocaleString`, no `<input type="date">`; bidi isolates are wrapped into the value (`ltr()` for numeric runs, `auto()` for names; U+2066, U+2068, U+2069); `fold()` for Arabic search; text sorts through `Intl.Collator`, never by code point; "sort the number, render the sentence"; never print a machine id at a reader; a control that cannot work is not rendered (the API returns affordances); a refused request never looks like an empty list; status never by colour alone; WCAG AA in both themes; "exceptions are loud, health is silent".
- Local time (the wall clock of Baghdad), not UTC, is what a person reads; the API stores UTC.

**What this means in practice.** AKR-TMS reuses these conventions and the `i18n` helpers by copy (file 32: the DBS repository is read-only and not a dependency), applies its own identity on top of the same component discipline, and extends them where Iraqi data demands more (Kurdish letters, the mother's name, Hijri display on request, transliteration beside the original script). The places where the approved mockups and these conventions disagree are listed in section 12.

## 2. Who the users are and what they do all day

### 2.1 The roles

Roles follow file 30 section 7.2 (the permission model) and file 28 (the workflow). Head counts are **planning assumptions for a mid-sized private bank with 30 to 60 branches** (NOT VERIFIED for any Iraqi bank; the discovery checklist must collect them). The CBI facts are from file 01.

| Role | Typical count | How often and on what | What they do in AKR-TMS | Must never see or do | The interface must make easy |
|---|---|---|---|---|---|
| **Branch liaison officer** (موظف الارتباط) | One per branch plus a named alternate (CBI ch. 11); works "full-time on compliance and AML only", but only a slice of that day is spent in the AML system | Daily, short sessions on a branch PC (desktop browser; the weak-link case must work) | Opens the branch window; reviews branch-first alerts: adds the explanation, attaches or confirms documents, answers branch RFIs; submits to central; files a confidential report to the MLRO; signs the monthly report | Other branches; thresholds; any outcome of a case (neutral statuses only); suspicion-tier cases; STR existence; anything visible to the branch manager | Finish a review in a few minutes; know what is asked and by when; trust that nothing they type reaches branch management |
| **Level-1 analyst** | 8 to 20 | All day, desktop, one or two monitors | Works the central queue: reads the case, compares with the declared profile, opens evidence, records a structured disposition, requests information, escalates | Restricted and suspicion tiers; thresholds and weights; export | Clear the straightforward case in minutes with the evidence in front of them; never lose context between alerts of one customer |
| **Level-2 analyst or investigator** | 3 to 8 | All day, desktop | Investigates: transaction history, network, documents, RFIs, narrative; recommends to the MLRO | Approving own work; filing | A single workspace for a complex case; the graph and the timeline; drafting help that cites its sources |
| **Screening specialist** | 1 to 4 | Daily, desktop | Compares customer and list entries, decides true, potential or incorrect match; raises freeze tasks | Closing a freeze-list incorrect match alone | Arabic names side by side with the evidence of the match; one-click reasons; never re-reviewing the same pair |
| **MLRO and deputy** (مسؤول الإبلاغ) | 2 | Several times a day, desktop; occasionally outside the office, on a laptop; good English required by the CBI | Decides (report, shelve, more evidence); signs reports; approves closures above delegated authority; reviews liaison reports; reads the command centre; answers the board and the CBI | Nothing by rank is hidden from them, but every access is logged | A short decision queue with the case's reasons in one view; sign-off with step-up; a command centre that shows what is late |
| **QA reviewer** | 1 to 3 (or internal audit when the bank is small) | A few hours a week, desktop | Reviews sampled closures against the rubric; records findings; calibrates | Reviewing their own closures; export | A sample list that is fair and fast; the case as the decider saw it, including which panels they opened |
| **KYC reviewer** | 3 to 10 | Daily | Periodic and event reviews, EDD checklists (file 26) | Monitoring cases of the same party unless on the list | A review queue with due dates; the risk drivers on one page |
| **RFI desk officer** | 1 to 3 | Daily | Incoming correspondent RFIs; authority requests; outgoing requests (file 28 section 9) | Releasing without a second signature | Pre-filled answers; a clock; a tipping-off check |
| **Operations officer** (executes holds and freezes) | Several | When a task arrives; works mainly in the CBS | Sees a neutral instruction and reference, performs it in the CBS, enters the CBS reference | The reason, the case, the words STR or suspicion | A task that reads like any other operations instruction |
| **Compliance configuration manager and approver** | 1 to 3 | Weekly | Scenario, threshold, routing, delegated-authority and template changes through change requests (mockup 05) | Approving own change | Plain-language descriptions, tests on history, a clear diff |
| **Data steward** | 1 to 2 | Daily at first, then weekly | Connection health, reconciliation, mapping, data-quality issues (mockup 08, file 25) | Business case content | A red/amber/green that means something and a path to the fix |
| **Compliance head and board committee readers** | 1 plus 3 to 7 | Monthly | Board pack, effectiveness, backlog, findings | Case content (unless on the access list) | One page that answers "are we in control" with evidence |
| **Internal auditor** (third line) | 1 to 3 | Quarterly, time-boxed | Read-only: access history, audit chain, QA, configuration changes | Changing anything | Questions answerable without asking IT |
| **CBI-approved external assessor and CBI inspector** | A team, for weeks | Time-boxed named accounts (file 30) | Draw a random sample of alerts; read end-to-end traces; read configuration, permissions, retention, chain verification | Anything outside the grant; export beyond the pack | A workspace that mirrors the D2 test script (file 28 section 12.6) |
| **Platform administrator, security officer (CISO)** | 1 to 3 | Occasionally | Users, roles, health, access reviews; the CISO reads access history and alerts about misuse | Business data and suspicion information | Offboarding in one click; an access review that takes minutes |

The roles with the largest effect on the design are the liaison officer (the simplest and most numerous user, whose mistakes create tipping-off risk), the level-1 analyst (volume), and the MLRO (accountability). The assessor is the only user whose first impression decides whether the bank passes the D2 test.

### 2.2 A working day, as design scenarios

These are **scenarios for design and for usability tests**, not measurements; the times are assumptions to be replaced by pilot data (file 28 section 16.7).

**Liaison officer, branch of a mid-sized bank.** 08:30 opens the branch window: three items from last night's run, one a request for documents about a customer with large cash deposits, two alerts on a trader's transfers. For each: a one-screen summary (what was seen, against what the customer declared, reason codes without thresholds), a box for what the branch knows, a check for documents on file, "submit". She finds the invoices in the paper file, ticks "documents on file", writes two lines, submits. At 11:00 a teller tells her a customer insisted on splitting a deposit after being told about the reporting limit: she opens the observation form (the CBI behavioural indicators), ticks the indicator, adds notes, and sends it to the MLRO's desk; she receives the written acknowledgment with the tipping-off reminder at once. Around the 5th of the month the system has already drafted her monthly report; she checks the list, signs, and sends it before the 10th. Total time in the system: well under an hour.

**Level-1 analyst.** 08:00 opens the queue, sorted by priority class then score; the P0 list matches are at the top and belong to the screening specialist, so she starts with P1. She opens a case for a company: the summary strip (rating, branch, declared turnover), "why this case was raised" with three reason codes, and the timeline of money in and out against the declared turnover; she opens the counterparties and the earlier alerts, sees the branch input ("invoices on file"), checks that the invoices' totals match, records `EXPLAINED_CORROBORATED` with the document reference, and proposes closure; because the class needs a second person, it goes to L2. She works fourteen such cases before lunch; the keyboard shortcuts (next item, open evidence, record disposition) and the saved view for her team's classes are where the time goes. Mid-afternoon a case shows 12 transfers from unrelated individuals to a phone shop; she escalates to L2 with two sentences.

**Level-2 investigator.** Opens the escalated case; the graph shows the account sharing a phone number with eight others and sending 92 percent of funds to four exchange companies within 24 hours; she asks the assistant for a draft summary (it cites 14 transactions), edits it, adds the graph snapshot to evidence, requests documents through the liaison officer (a neutral RFI), and recommends "STR candidate" to the MLRO with the CBI checklist filled. The case moves to the suspicion tier; her colleagues no longer see it in search.

**MLRO.** 09:00 the command centre shows 1,284 open alerts, 212 older than 30 days, an overdue liaison report from one branch, a CTR batch due at 23:59 and a correspondent RFI due in two days. She opens the decision queue: three cases, each with the recommendation, the checklist, the reasons and the evidence she can open. She decides one STR (step-up, reason codes, sign-off) and one shelving with reasons; asks for more evidence on the third. She signs the liaison reports for the month and answers the RFI desk's draft to a US correspondent. At month end the board pack is generated from system facts.

**Assessor.** Arrives with a named, time-boxed read-only account. Draws 25 random alerts from the register; for each opens the end-to-end trace (alert snapshot, grouping, tasks, decisions, RFIs, report, QA record, audit events); runs the audit-chain verifier; opens the access matrix and the delegated-authority matrix; exports the pack she needs. Her questions mirror the CBI guide: rule logic, alert generation, communication and reporting paths, review and improvement mechanism, thresholds, permissions, audit-trail integrity, data traceability (file 28 section 2.5).

### 2.3 Constraints that shape the interface for all of them

- **Language.** Arabic is the working language and English the second; the MLRO and deputy must have good English (CBI), analysts may not. Arabic terms are written by compliance staff, not machine-translated (file 32 section 18.3). Kurdish (Sorani) appears in **data** (names, documents) from day one; a Kurdish interface is a later option (CLDR supports `ckb-IQ`, section 4).
- **Hardware and links.** The hardware of Iraqi bank workstations and the quality of the links between branches and head office are NOT VERIFIED. Design for a 1366 x 768 minimum, test at 1280 and 1920 widths, do not depend on a second monitor, and keep the branch window light enough to work on a slow link (a budget is set in section 11).
- **Sessions and step-up.** MFA for everyone, a 15-minute idle time-out and an 8-hour absolute lifetime by default, a fresh step-up for suspicion-tier opens and a single-use step-up for signing-grade actions (file 30 sections 6.2, 6.3). The interface must make these cheap: a visible countdown before time-out, no loss of drafts on time-out, and a step-up dialog that does not lose the screen state.
- **Confidentiality.** Every role works under need-to-know; the interface never reveals the existence of what a user cannot open (refusal looks like "not found", file 30 section 8.2).

### AKR-TMS hooks (section 2)

- A **role and scenario catalogue** (this table) kept with the design system and used to write usability tests with real compliance staff before each release.
- **Role-specific navigation**: the rail shows only what the role may open (never a control that cannot work).
- **Branch window** as a separate, light route bundle with its own performance budget.

## 3. Evidence on analyst productivity, alert fatigue and what interface patterns do about it

### 3.1 What was found

No published, independent measurement of **minutes per AML alert** or of AML investigators' time distribution was found in the sources reachable for this research (arXiv searches on 2026-10-07 returned no such study; the papers read on AML modelling, arXiv 2201.04207 and 2112.07508, state none). The evidence that exists is from neighbouring fields and from a handful of studies with real analysts. It is summarised here with its limits, because the interface rules of sections 7, 10 and 11 rest on it.

| Study (grade A unless noted) | Field and design | Finding that matters here | Limit |
|---|---|---|---|
| Alahmadi, Axon, Martinovic, "99% False Positives", USENIX Security 2022 | Security operations centres; survey of 20 practitioners and a qualitative study of 21 | Practitioners "confirmed the high FP rates of the tools used, requiring manual validation"; the study examines alarm quality and validity | Qualitative, security domain |
| Ancker and colleagues, BMC Medical Informatics and Decision Making 17:36 (2017) | 112 primary-care clinicians; retrospective cohort on electronic health record alerts, 2010 to 2013 | One quarter of drug alerts and one third of clinical reminders were **repeats** for the same patient within the year; "likelihood of reminder acceptance dropped by 30% for each additional reminder received per encounter, and by 10% for each five percentage point increase in proportion of repeated reminders"; **no effect of workload per se** and no desensitisation over time; "Reducing within-patient repeats may be a promising target" | Clinical alerts, not AML |
| Uetz and colleagues, arXiv 2609.02465 (2026) | Security operations; eight alert datasets | Risk-based prioritisation reached AUROC 0.92 on average against 0.72 for ranking by alert severity | Security domain; method, not AML data |
| Jesus, Belém, Balayan, Bento, Saleiro, Bizarro, Gama, FAccT 2021 | **Three professional fraud analysts**; 1,300 transactions in the model's review band, 50 percent fraud prevalence; stages Data Only, Data + model score, Data + score + explanation (LIME, SHAP, TreeInterpreter) | Data only: accuracy **62.0 percent**, 59.5 s per decision. Score added: accuracy **51.0 percent**, **44.6 s** (about 25 percent faster, 11 points less accurate). Explanations: accuracy 56.5 to 59.7 percent, 43.0 to 50.3 s; "showing Data Only results in the highest decision accuracy and the slowest decision time"; LIME was the least preferred | Three analysts, fraud not AML, review band |
| Buçinca, Malaya, Gajos, "To Trust or to Think", CSCW 2021 | N = 199; simulated AI, 75 percent accurate; three cognitive forcing designs: **on demand** (suggestion hidden until requested), **update** (decide first, then see the AI), **wait** (30-second delay) | Forcing functions "significantly reduced overreliance compared to the simple explainable AI approaches"; when the AI was wrong, correct decisions rose (for example 9 percent against 3 percent overall); but "people assigned the least favorable subjective ratings to the designs that reduced the overreliance the most" | Non-expert participants on a food task |
| Poursabzi-Sangdeh and colleagues, CHI 2021 | N = 3,800 pre-registered experiments varying model transparency and number of features | A clear, few-feature model let people simulate predictions but did not make them follow it more, and "showing participants a clear model meant that they were less able to detect and correct for the model's sizable mistakes, seemingly due to information overload" | Lay participants, a housing-price task |
| Bansal and colleagues, CHI 2021 | Three datasets, mixed-method | Explanations "increased the chance that humans will accept the AI's recommendation, regardless of its correctness" | Lay participants |
| Esteves, Costa, Fallon, Bizarro, arXiv 2605.10522 (2026) | Feedzai authors; **three** money-laundering risk consultants in 45-minute shadowed interviews on grouped transaction networks | Grouping reduces clutter (38 nodes to 24, then to 19 in one example) but "node reduction alone is not a perfect solution as it removes nuance"; "information for grouped accounts must be easily accessible" through summaries, widgets or an expandable network; no single grouping fits all schemes; grouping targets graphs of 15 or more accounts | Three participants, static mock-ups |
| HKMA, regulator-compiled (file 12 section 8.4) | Hong Kong banks | Bank C: "80% of alerts scored as high-risk lead to an STR"; Bank B: 7 percent of alerts auto-closed with sample checks, "daily workload for an investigator is reduced by 1 to 3 hours" | Bank-reported, anonymised |
| Vendor-reported time claims (grade C; file 13) | Lucinity 3 hours to 30 minutes; Flagright 4 minutes against a 38-minute manual baseline; Oracle "over 80%" reduction; Silent Eight up to 80 percent of monitoring cases without manual review | Directional only; baselines and definitions are not given | Vendor |

**What the studies agree on.** (1) Fatigue comes from **repetition and low-information alerts**, not from workload as such (Ancker), so merging repeats is a first-order intervention. (2) A model score or an explanation is not a free gain: it speeds the decision and can lower accuracy and raise acceptance of wrong suggestions (Jesus, Bansal, Buçinca); more features and more transparency can overload (Poursabzi-Sangdeh). (3) The raw data remain the strongest basis for the decision (Jesus: Data Only had the highest accuracy). (4) Designs that make the human think first work better on accuracy and are liked less (Buçinca), so they must be used where the stakes justify them (P0 and P1), not everywhere. (5) Network views need grouping **and** one-click access to the detail underneath (Esteves).

### 3.2 Where the time goes: a model to measure, not to believe

Unknown: the split of an analyst's minutes between finding data, reading it, deciding and writing. Hypothesis for the pilot (file 28 section 16.7): most time is spent **reading evidence and writing the rationale**, with a smaller but avoidable share on navigation (returning to a list, opening a second system, searching for the same customer again) and on re-reading the same customer across alerts. The product therefore records, per case, time in each panel, panel switches, back navigations, notes written, and the number of earlier alerts the analyst had to open; these numbers decide which patterns below earn their place.

**What keyboard work can and cannot buy.** With the keystroke-level model's standard operator times (keystroke K = 0.20 s, pointing P = 1.1 s, homing H = 0.4 s, mouse button press B = 0.1 s, mental preparation M = 1.35 s; Card, Moran and Newell, as tabulated in the Wikipedia article "Keystroke-level model", grade B), a disposition by mouse (open the reason list, pick, press submit, return to the queue: four pointing moves each with a click, about 5.2 s) against the keyboard (home once, five keystrokes, about 1.4 s) saves roughly 3.8 s a case; at 60 to 100 cases a day that is 4 to 6 minutes a day. The mental steps (M) do not shrink. The conclusion is blunt: keyboard shortcuts are cheap and should exist, but the big levers are **fewer repeats, no lost context, evidence at hand, and not re-doing the same decision**; shortcuts save seconds, not the hour a day that the HKMA's Bank B reported from a different mechanism.

### 3.3 Interface patterns and the evidence behind each

| Pattern | What it is in AKR-TMS | Evidence and precedent | Rule |
|---|---|---|---|
| **One case per customer, with repeats merged** | Alerts of one party and family arrive as one case with the alerts listed inside (file 28 section 3.3) | Ancker (repeats lower acceptance); HKMA "holistic review on a customer-relationship basis"; ING account-level monitoring failure | The queue counts cases; the case shows how many alerts were merged and lets the analyst open any |
| **Single-screen case workspace** | Summary strip, why-raised, timeline, counterparties and decision panel on one page with panels that expand in place (mockup 02) | Precedent: NICE ActOne entity view, SAS Visual Investigator workspaces, Marble's case page with investigation panel, alerts table and side panels (documentation read 2026-10-07). No controlled AML study found | Opening evidence never leaves the case; the decision panel stays visible while scrolling (sticky) |
| **Entity timeline** | One time axis for alerts, transactions in windows, observation logs, RFIs and answers, authority requests, rating changes, decisions, tasks, QA events | Marble's "Investigation Panel" ("everything that happened on the case ... filter by the event types"); NICE ActOne point-in-time history; SAS timelines and maps | Filter by event type; every row links to its source; times are local, `YYYY-MM-DD HH:MM` |
| **Evidence first, reasons second** | The alert page leads with raw rows, peer comparison and the network; reason codes and score parts follow (file 21 section 11) | Jesus et al. (data only was the most accurate); Bansal; Poursabzi-Sangdeh | Maximum three headline reasons, each beside the fact it rests on; mitigating factors shown too; thresholds hidden from branch roles |
| **Cognitive forcing for high-stakes classes** | Evidence panels must be opened before decision buttons enable for P0 and P1; optional assess-first mode before any AI suggestion appears | Buçinca et al. | Off for P3; on for P0 and P1; measure acceptance and override rates |
| **Network view with grouping and drill-down** | Ego graph by default; automatic grouping above about 15 accounts with a one-click expansion and a summary of each group; a table alternative | Esteves et al.; mockup 06 | Never group away the subject or the account that triggered the alert; keep a node and link table for accessibility |
| **Queue-run mode** | "Next item" without returning to the list; a one-key jump to the oldest unassigned case | Marble: "one-click shortcut to the oldest unassigned case, allowing rapid triage without interruption" | The run keeps the filter and the sort; leaving the case never discards a draft |
| **Keyboard shortcuts** | List navigation, open evidence, set a reason, submit, next; a cheat sheet on `?` | KLM arithmetic above; WCAG 2.1.4 | Use **physical key codes** so that Arabic-layout typists get the same shortcuts (below); every shortcut can be remapped or turned off |
| **Bulk actions with safeguards** | Assign, tag, snooze, request information, mark duplicate; no bulk closure for P0 or P1; a count, a preview and one reason code per batch | IBM Carbon: "Once an item from the table is selected, the batch action bar appears at the top of the table"; Cloudscape: "friction should match severity"; Coinbase and Block show what bulk shortcuts cost | Batch size cap (default 25); each item is logged; a batch closure of alerts is allowed only inside an already decided case |
| **Saved views and "My inbox"** | Per-user and shared views; filters and sort in the URL; role defaults | Cloudscape: "Use preferences to allow users to view and change their display configuration"; filter persistence for "easy sharing of a table view"; Marble: "My inbox" consolidates all cases assigned to you; DBS reports: "The period and every filter live in the address" | A view is a named filter set; sharing a view never shares data the recipient cannot open |
| **Density and sticky columns** | Compact, standard and relaxed row heights; sticky first column and header | Carbon: row heights from 24 px to 64 px in five sizes; Cloudscape: sticky headers "optional, but recommended" | Compact is the default for analysts; relaxed for the branch window |
| **Response-time budgets** | Interactions under 0.1 s feel instant; under 1 s keep the flow of thought; progress indicators beyond about 10 s | Nielsen, "Response Times: The 3 Important Limits", quoted | Queue and case navigation within 1 s on the LAN; long jobs (pack generation) show progress and continue in the background |
| **Draft safety** | Autosave of unsent drafts; restore after time-out or step-up; one active editor per case with visible presence for list members | DBS: unsaved-changes guard on long forms; file 30 session rules | A time-out never loses text; presence is shown only to members of the case's access list (it would reveal interest) |

**Arabic keyboards and shortcuts (a concrete trap).** `KeyboardEvent.key` returns the character produced by the active layout; `KeyboardEvent.code` "represents a physical key on the keyboard (as opposed to the character generated by pressing the key) ... a value that isn't altered by keyboard layout" (MDN, grade A). A handler that tests `key === "j"` does nothing for an analyst whose layout is Arabic. AKR-TMS therefore binds letter shortcuts to `code` (for example `KeyJ`), labels them with the Latin keycap, and accepts both digit sets in every number field and in search (the DBS `westernDigits()` and `fold()` helpers; the DBS source comments record that "an Arabic keyboard types ٢٠٢٦ by default" and that number boxes which stripped such digits made "the clerk watch their typing vanish", findings QOL-3e, F182 and F249 of its own testing).

### AKR-TMS hooks (section 3)

- **Instrumentation** of the analyst workspace (time in panel, switches, back navigations, notes, earlier alerts opened) as pilot metrics, without per-person league tables.
- **Pattern library**: merged-repeat case rows, case workspace layout, timeline, evidence-first alert page, drill-down network, queue-run mode, shortcut layer on physical keys with remapping, bulk-action bar with safeguards, saved views, density settings, draft safety.
- **Cognitive-forcing switches** per priority class and an assess-first mode, with override and acceptance rates in the AI monitoring panel (file 28 section 14.3).

## 4. Arabic-first right-to-left design

All measurements below were made on 2026-10-07 (grade A as measurements): Unicode CLDR data through Node 22.23.1's ICU; the Unicode Bidirectional Algorithm through two independent implementations (the Rust `unicode-bidi` crate and the Python `python-bidi` package, which gave identical results); glyph tables of the fonts in `plan/assets/fonts/` through fontTools; digit counts in 16 Iraqi regulator and Office texts.

### 4.1 Defaults, and where Iraq's data disagree with the standards

| Question | What the sources say | Decision for AKR-TMS |
|---|---|---|
| Interface direction and language | Arabic is the working language, English second (file 32 section 18.3); Iraqi Arabic is `ar-IQ`; Sorani Kurdish is `ckb-IQ` and also RTL | `dir` and `lang` on `<html>` from the user's language; Arabic default; Kurdish UI is a later option; Kurdish **data** are supported from day one |
| Digits | **CLDR**: `ar-IQ` default numbering system is `arab` (Arabic-Indic digits, decimal separator U+066B, group separator U+066C); generic `ar` is `latn`. **W3C ALREQ 6.1.2**: Arabic-Indic numerals are used in "Eastern Arabic-speaking countries; e.g. Egypt, Saudi Arabia, Iraq". **Iraqi regulator texts** (CBI controls 2025, Detailed Assessment Guide, Phase 1 guide, standards handbook, reform circular, risk-management controls 2026, cyber-resilience, sanctions guidelines, the Office's STR and UTR guides, Law 39): **16,197 Western digit characters against 944 Arabic-Indic** (5 percent); 15 of the 16 documents use Western digits only or overwhelmingly; the Office's CDD rules use Arabic-Indic digits only. **Fonts**: in IBM Plex Sans every digit is 600/1000 em wide (tabular by default); in IBM Plex Sans Arabic the Arabic-Indic digits are proportional (widths 263 to 630) | **Western digits (0-9) everywhere in data** (amounts, identifiers, dates, counts, tables), as in the DBS; a per-bank and per-user setting may show Arabic-Indic digits in **prose only**; numbers in tables never switch, because proportional digits break column alignment; **every input and search box accepts Western, Arabic-Indic and Persian digits** and normalises them (the DBS `westernDigits()` and `fold()`), showing the understood value |
| Dates | CLDR `ar-IQ` calendar list starts with Gregorian, then Islamic variants; the week starts on Saturday, weekend Friday and Saturday | Gregorian `YYYY-MM-DD HH:MM` in both languages (the DBS convention), local time of Baghdad; no `toLocaleString`, no native date inputs |
| Month names, when words are wanted | `ar-IQ` uses the Mesopotamian names: كانون الثاني، شباط، آذار، نيسان، أيار، حزيران، تموز، آب، أيلول، تشرين الأول، تشرين الثاني، كانون الأول; generic `ar-EG` and `ar-SA` use يناير، فبراير ... أكتوبر ...; `ckb-IQ` has its own (for example تشرینی یەکەم for October) | Reports and the board pack use the `ar-IQ` names by default and the bank can switch; tables use the numeric form |
| Hijri dates | The same instant is a different Hijri date by calendar variant: 7 October 2026 (Baghdad) is **24 Rabi' II 1448** in `islamic-civil`, **25** in `islamic-tbla`, **26** in `islamic` and `islamic-umalqura` | Hijri is **display and entry only**, labelled with the variant, never used for a clock, a deadline or a comparison; documents dated in Hijri keep the text as entered (field `date_hijri`, file 24) beside the Gregorian value; seasonal markers (Ramadan, Eid, Muharram, Arbaeen) are drawn on timelines so a seasonal peak is not read as an anomaly (file 24) |
| Currency | `ar-IQ` formats IQD as "‏٦١٢٬٤٠٠٬٠٠٠ د.ع.‏" and with Western digits as "‏612,400,000 د.ع.‏"; the name is "دينار عراقي"; USD is "US$" | Show the **ISO code** (IQD, USD) as a left-to-right isolate next to the figure, as in the mockups ("IQD 612.4 million"); decimals follow the currency table, not guesswork; large values in KPI tiles may use "مليون" and "مليار" with correct plural forms |
| Plurals | Arabic has **six** plural categories in CLDR: zero, one, two, few (3 to 10), many (11 to 99), other (100 to 102, then the cycle repeats: 103 is few, 111 is many); English and Sorani have two | Every counted phrase ("3 alerts", "11 alerts") goes through ICU plural rules with six Arabic forms; no string is built by concatenation (the DBS rule: hold keys, translate at render) |
| Week and business days | Weekend Friday and Saturday (CLDR); business days Sunday to Thursday (file 28 section 5.6) | Calendars per bank (file 28) |

### 4.2 Bidirectional text: what goes wrong, measured

The Unicode Bidirectional Algorithm lays out each run by its own direction and orders runs by the paragraph direction. Digits after Arabic letters are resolved as Arabic numbers (rule W2), so separators and spaces between number groups become neutrals that take the paragraph's right-to-left direction. Results for four common values inside an Arabic sentence:

| Value (logical order) | Without isolation, as displayed | With the whole value in a left-to-right isolate |
|---|---|---|
| Phone "+964 770 123 4567" | The groups are laid out right to left and the plus sign ends up trailing: "4567 123 770 964+" | "+964 770 123 4567" |
| Date and time "2026-10-07 14:32" | The time is shown **before** the date and the date's parts are reversed: "14:32 07-10-2026" | "2026-10-07 14:32" |
| Range "13,000,000 - 30,000,000" | The ends swap: "30,000,000 - 13,000,000" (the first value sits on the right, where an RTL reader starts) | Order unchanged |
| A Latin name inside an Arabic sentence | Placed correctly | Keeps its own direction even with adjacent punctuation (first-strong isolate) |

The DBS reached the same conclusion from its own browser tests: "a date-time is TWO left-to-right runs separated by a space ... the reader sees `15:00 2026-08-27`", and moved the fix **into the value** with U+2066 and U+2069 (`ltr()`) for numeric expressions and U+2068 (`auto()`) for names, because "a rule everybody must remember is a rule that is sometimes not remembered" (`bidi.ts`, `datetime.ts`). The W3C's guidance is the same in principle: wrap every opposite-direction phrase in markup, use `dir="auto"` or `<bdi>` for text of unknown direction, and use the Unicode isolate characters only where markup is unavailable (W3C, "Inline markup and bidirectional text in HTML"). AKR-TMS follows both: markup in JSX, isolates inside strings that are interpolated into translated sentences.

**Rules.**
1. A numeric expression (phone, date and time, range, amount with currency code, IBAN with spaces, reference number) is isolated **as a whole**, never by its parts; ranges render in the same order everywhere (left-to-right), so one value never shows two orders on one screen (DBS DR-67). Whether Arabic-speaking analysts prefer ranges in reading order is NOT VERIFIED and belongs in the first usability test.
2. A name is wrapped with a first-strong isolate so that an Arabic name stays right-to-left in an English line and a Latin one stays left-to-right in an Arabic line.
3. An isolate is a **character**: copy buttons copy the raw value without isolates; stored values never contain them; searches and equality checks strip them (the DBS "Isolate at the VIEW").
4. Incoming text from the CBS, payments and documents is **scanned for bidirectional control characters** (U+202A to U+202E, U+2066 to U+2069, U+200E, U+200F, U+061C) and zero-width joiners; they are removed for matching and displayed as a data-quality flag, because hidden controls can reorder a name to disguise it (file 30 SP-70, "bidirectional-control stripping"; file 22 section 13.11).
5. Every span of a different language carries `lang` (`ar`, `ckb`, `en`) so that fonts, hyphenation and screen readers choose correctly (WCAG 3.1.2).
6. Directional icons (back, next, expand, send) flip with one attribute; symmetric icons and media controls do not (DBS `data-rtl-flip`); progress bars and sliders mirror; **the geometry of a graph does not mirror** (file 32 section 18.2); chart time axes follow the DBS rule "time runs right to left in Arabic: positions are mirrored, never the drawing" (to be confirmed in the RTL spike of file 32 section 18.4).

### 4.3 Names: how an Iraqi name is shown

Facts from file 22: an Iraqi name is an **ordered chain** (given name, father, grandfather, optionally great-grandfather, then tribe, nisba or laqab); the national card prints the mother's name and the maternal grandfather's name; the Arabic original is the only form that is stable across documents; Latin spellings are generated downstream and differ between passport, card machine-readable zone and the bank's own typing; kunya, titles (Sayyid, Hajji, Dr) and Kurdish letters appear in real data.

| Element | Display rule | Reason |
|---|---|---|
| The name | Arabic original first and larger, in the chain order, right-to-left, `lang="ar"` or `ckb`; the Latin transliteration beneath in a smaller size with a label "transliteration" or its source (card, passport, typed, generated) | The Arabic is the record; a Latin-only display of an Iraqi customer invites mismatch |
| Chain parts | In a comparison, parts are aligned one under another with role labels (given, father, grandfather, tribe or family) | File 22 section 13.7; mockup 03 already shows the table |
| Mother's name | Always shown beside the birth year in identification blocks (customer, list entry, screening comparison) | It is on 99.2 percent of national-list rows and is the strongest tie-breaker (file 22 section 3.3) |
| Truncation | **Never truncate a name in a table.** Wrap to a second line; where a column is narrow, truncate at the end with a popover of the full chain; never split a compound name into "first name" and "last name" | The DBS "Whole-Name Rule": "a first name split off an Arabic compound name is wrong" |
| Titles and kunya | Shown as separate small tokens or in a popover; never merged into the name used for matching | File 22 sections 2.3, 2.4 |
| Aliases and former names | Listed under the name with class (strong, weak, kunya, former) | File 22 |
| Kurdish letters | Rendered in the original letters (پ چ ژ گ ڤ ڕ ڵ ۆ ێ ە) with the standardised Arabic spelling beside it when the bank holds both | File 22 sections 4.2 to 4.4 |
| Entity names | Legal-form words (شركة، مؤسسة، المحدودة، Co., Ltd) set in a lighter weight so the distinguishing words read first | File 22 section 13.11 |
| Quality flags | Truncated, mixed-script, meaningless or default values, legacy ZWNJ shown as a small data-quality mark with a tooltip | File 22 section 13.3; Kurdish typography facts |

**Font coverage, measured.** The vendored IBM Plex Sans Arabic subset (681 mapped characters) contains **all ten Kurdish letters** and the Persian kaf and yeh, Arabic-Indic and extended digits, harakat and tatweel. The vendored **Latin subsets of IBM Plex Sans and Mono do not contain ş, ğ and İ** (Latin Extended-A), nor the scholarly transliteration marks (ā, ī, ū, ḥ, ṣ, ḍ, ṭ, ẓ, ʿ, ʾ). Kurmanji and Turkmen names in Latin letters and any transliteration scheme that uses those marks would fall back to a system font with different metrics. **Action: ship the `latin-ext` subsets** (Fontsource provides them) and test the fallback.

### 4.4 Typography for dense Arabic data

- **Faces**: IBM Plex Sans Arabic with IBM Plex Sans for Latin and all digits, IBM Plex Mono for identifiers (identity). One `font-family` stack lets the browser fall through per code point, as in the DBS: "every digit renders in Inter ... so a column of reference numbers stays aligned on an Arabic screen"; the same holds with Plex.
- **Leading**: Arabic marks above and below the line make the inked height exceed the Latin box; raise line-height for Arabic only (the DBS raises it under `[dir=rtl]`; W3C ALREQ 7.4: "Arabic ascenders and descenders extend much further than those of the Latin script, and care must be taken to correctly align text in the different scripts when they appear together"). Starting values for tables: 14 px text with 22 to 24 px leading in Arabic (20 px in Latin); verify against marked text (shadda, kasra).
- **No letter-spacing on Arabic, ever** (the DBS rule, enforced in its bridge CSS; ALREQ 7.3 distinguishes the stretching of cursive scripts from letter-spacing); uppercase and tracking only under `lang="en"`.
- **No italics or bold-for-emphasis tricks on Arabic**: use weight and colour; italics are for Latin only (IBM Plex Sans Arabic has no italic in the vendored set).
- **Size**: Arabic glyphs look smaller than Latin at the same font size; check mixed lines (a name in both scripts) for baseline and x-height alignment and adjust per context rather than globally.
- **Density**: three row heights (compact, standard, relaxed) as in IBM Carbon (24 to 64 px in five sizes); compact is the analyst default; relaxed for the branch window.
- **Numbers in tables**: tabular Western digits in IBM Plex Sans, right-aligned in LTR and **left-aligned at the logical end** in RTL (end-aligned), with the currency code in a separate narrow column.

### 4.5 Sorting and search in Arabic

**Sorting.** Code-point sorting scatters names that a reader sees as adjacent: measured on 32 invented Iraqi and Kurdish names, code-point order puts آمنة first, then أحمد, إبراهيم, ابراهيم, احمد, and sends every Kurdish name (پ چ ڕ ژ ڤ ک گ) to the **end** of the list, after يوسف. `Intl.Collator('ar')` orders آمنة, ابراهيم, إبراهيم, أحمد, احمد, then باسم, بشار, **پەیمان**, تميم ... **چنار** after جاسم, **ڤیان** after فاطمة and **گۆران** after كريم/کریم, so each Kurdish letter sits next to its base letter. With `sensitivity: "base"` the alef forms and the diacritics compare equal, but ة versus ه and Arabic ك versus Persian ک do **not**, and `numeric: true` sorts "case 9" before "case 10". Rules: text sorts through the collator (the DBS rule), never by code point; sort the number, render the sentence; a Sorani collation (`ckb`) is available for Kurdish-language lists.

**Search and filters.** Client filtering uses `fold()` (alef forms, ya, ta marbuta, harakat, tatweel, both digit sets) only on lists already in memory; server search folds both sides in SQL with a parity test (file 32 section 6.6). AKR-TMS **extends the fold** with the Kurdish and Persian letters, ZWNJ removal and the repair of legacy Heh plus ZWNJ (file 22 sections 4.3, 6.4), and searches Latin transliterations through the consonant key (file 22 section 7.4). The search box detects the kind of input: an **IBAN** (Iraqi IBANs are 23 characters, `IQkk bbbb sssc cccc cccc ccc`: four letters of bank code, three digits of branch and twelve of account, printed in groups of four with the mod-97 check, per the IBAN registry as tabulated by Wikipedia, grade B), a national or residence card number, a phone number (digits normalised, plus sign and spaces ignored), a case or alert number, or a name; results are grouped by type; matched text is highlighted in the **original** string by mapping offsets back from the folded text (file 32 section 18.2). Suspicion-tier cases never appear in global search for non-members (file 30 SP-27).

### 4.6 Exports and print

PDF through HTML templates and the Gotenberg image that carries the Arabic fonts (pure-Rust PDF libraries do not shape Arabic, file 32 section 18.2); XLSX with the sheet view set to right-to-left for Arabic reports; CSV in UTF-8 with a byte-order mark so that spreadsheet software reads Arabic, ISO dates, Western digits, and **formula-injection guards** on cells that begin with `=`, `+`, `-` or `@` (file 30 SP-70); every export carries the requester's watermark and an audit entry; Hijri columns, if requested, carry the variant name.

### AKR-TMS hooks (section 4)

- **i18n kit** copied from the DBS (`direction`, `bidi`, `datetime`, `fold`, `AriaLocale`) with additions: Kurdish and Persian folds, bidi-control scanning, six-form Arabic plurals, a digit-preference setting, a Hijri formatter with variant labels and a seasonal calendar.
- **Name component** (original and transliteration, chain alignment, mother's name, quality flags) shared by customer, screening, case and report screens.
- **Font package**: Plex Sans Arabic, Plex Sans and Mono with `latin-ext` subsets, verified by a glyph-coverage test for every letter listed in file 22.
- **Collation and fold parity tests** between browser and server, run in CI.

## 5. Accessibility (WCAG 2.2), contrast of the approved palette, and keyboard use

### 5.1 Target

WCAG 2.2 level AA in both themes, as the DBS commits (`PRODUCT.md`: "WCAG AA text contrast in both themes; status always paired with a word; touch targets sized for thumbs; full keyboard and screen-reader support"). WCAG 2.2 became a W3C Recommendation on **5 October 2023**, adds nine success criteria and removes 4.1.1 Parsing (W3C, "What's New in WCAG 2.2", read 2026-10-07). No Iraqi accessibility rule for bank staff software was found (NOT VERIFIED); the target is justified by staff with disabilities, by procurement (foreign-owned and Gulf-parent banks ask for it) and by the fact that an accessible interface is also the one a keyboard-only analyst can drive fast.

### 5.2 The criteria that bite hardest in this product

| Criterion | What it requires here | Consequence |
|---|---|---|
| 1.4.1 Use of colour; 1.4.11 Non-text contrast | Status and chart series not by colour alone; component boundaries and graphics need 3:1 | Every status chip carries a word; stacked-bar legends get direct labels or patterns; a thin separating gap between segments (section 5.3) |
| 1.4.3 Contrast (minimum) | 4.5:1 for normal text | Measured table below: four tokens fail |
| 1.4.10 Reflow; 1.4.4 Resize text | Usable at 400 percent zoom | Tables scroll horizontally inside their own container (two-dimensional data are excepted); the page never scrolls sideways (DBS rule) |
| 1.3.2 Meaningful sequence; 2.4.3 Focus order | DOM order is the reading order in Arabic | No CSS `order` tricks to place the rail or the decision panel; mirror with logical properties only |
| 2.1.1 Keyboard; **2.1.4 Character key shortcuts (A)** | Everything works from the keyboard; a shortcut made only of letters, digits or punctuation must be "turned off", "remapped to include one or more non-printable keys", or "only active when that component has focus", because "single-key shortcuts are disastrous for speech users" (W3C Understanding) | A settings switch to turn shortcuts off and to remap them with Ctrl or Alt; shortcuts bound to physical keys (section 3.3) |
| **2.2.1 Timing adjustable (A)** | A time limit set by the content must be turn-off-able, adjustable or extendable; the Understanding document says that if a limit is for security it "would be considered to have been set by the content", gives the example "After a period of inactivity the web page asks if the user needs more time" and "at least 20 seconds to extend the time limit with a simple action"; "time-based / time-limited two-factor authentication tokens" may be essential | The 15-minute idle time-out of the CBI-minimum session policy shows a warning (default 60 seconds) with an "extend" action; drafts survive time-out (file 30 section 6.2) |
| 2.2.2 Pause, stop, hide | Auto-updating content that lasts more than five seconds can be paused | A "pause live updates" control on queues and dashboards fed by server-sent events; the list never moves under the cursor |
| 2.4.11 Focus not obscured (AA, new) | A focused item is at least partly visible | Sticky headers, the sticky decision panel and the step-up dialog must not cover the focused row (use scroll-padding) |
| 2.5.7 Dragging (AA, new) | A pointer alternative to every drag | Graph node dragging and any kanban-style reordering have buttons or keyboard moves |
| 2.5.8 Target size minimum (AA, new) | Targets at least 24 by 24 CSS pixels or enough spacing | Compact table rows keep 24 px hit areas on icon buttons |
| 3.1.2 Language of parts | Mark changes of language | `lang` on Arabic, Kurdish and English spans (section 4.2) |
| 3.2.6 Consistent help (A, new) | Help is in the same place on every page | One help entry in the header, same position in both directions (section 6) |
| 3.3.7 Redundant entry (A, new) | Do not ask for the same information twice in a session | RFI, report and decision forms are pre-filled from the case; the analyst never retypes a customer's data |
| **3.3.8 Accessible authentication, minimum (AA, new)** | "A cognitive function test ... is not required for any step in an authentication process unless ... Alternative, Mechanism, Object Recognition, Personal Content"; the Understanding document says "A service that requires manual transcription of a verification code is not compliant" and that copy and paste, password managers and hardware keys are acceptable | The step-up dialog must allow paste and autofill of the one-time code (`autocomplete="one-time-code"`) and offer WebAuthn first; never block paste in a password or code field |
| 4.1.2 Name, role, value | Custom widgets expose roles | Use React Aria components; the graph and the timeline get ARIA roles and keyboard models or a table alternative |

### 5.3 Contrast of the approved identity, measured

WCAG contrast ratios computed from `plan/identity.md` on 2026-10-07; "Clay" is the page background `#F5F0E6`, "Paper" the white card.

| Pair | Ratio | Result | Use |
|---|---|---|---|
| Ink `#18202F` on Paper / Clay | 16.3 / 14.4 | Pass | Body text |
| Stone `#5B6474` on Paper / Clay | 6.0 / 5.3 | Pass | Secondary text |
| Ishtar Blue `#1E4FA0` on Paper / Clay | 7.8 / 6.9 | Pass | Links, primary buttons (white on blue 7.8) |
| White on Lapis Night `#0F2147` | 15.8 | Pass | Rail labels |
| Babylon Gold `#C8A24B` on Lapis Night | 6.6 | Pass | Active marker and logo on the rail |
| **Babylon Gold on Paper / Clay** | **2.4 / 2.1** | **Fail** | Never text, border or icon on light surfaces (the identity already keeps it to the logo and the rail marker) |
| **Glaze Teal `#1F8A8A` on Paper** | **4.15** | **Fail for text**, pass for graphics (3:1) | Teal text must use `#177272` (5.7 on Paper, 5.0 on Clay) |
| **Risk medium `#B26A00` on Paper / Clay** | **4.24 / 3.73** | **Fail for text** | Pass only as large text or graphics; and **on its own amber tint (`#FDEBD7`) 3.64** |
| Risk medium fill `#F2A33A` on Paper / Clay | 2.08 / 1.84 | Fail as a graphic boundary | Fill only; labels in Ink on the fill (7.8) |
| **Risk low `#2F7D4F` on Clay** | **4.44** | **Just fails for text** (5.04 on Paper) | A slightly darker green for text on Clay (`#2D794C`: 4.68; on its green tint 4.59) |
| Risk high `#B3261E` on Paper / Clay / its tint `#FBE1E1` | 6.5 / 5.8 / 5.3 | Pass | |
| **Sand line `#DDD6C9` on Paper / Clay** | **1.44 / 1.27** | **Fail for the border of an input or any control** | Fine for decorative card edges; **input, select and text-area borders need at least 3:1** against the adjacent colours (`#7F8695`: 3.65 on Paper, 3.22 on Clay) |
| Adjacent chart segments: blue / teal 1.9; teal / amber 2.0; amber / red 3.1 | | Below 3:1 between neighbours | Separate segments with a white gap and label values; do not rely on hue |

*Decision (2026-10-07, after this file was written):* the owner locked the identity together with these screen tokens; `plan/identity.md` now lists them (teal text `#177272`, amber text `#9C5D00`, green text `#2D794C`, control border `#7F8695`).

**What the mockups do.** The text colours *sampled from the mockup images* (the near-extreme pixels of 20 labels, pills and notes) all measure 4.6 or better (sampling the most extreme pixels slightly overstates the contrast of thin, anti-aliased strokes, so treat this as an upper bound), for example the amber pill text `#AF4E0A` on `#FDEBD7` at 4.61, the "Due in 4 days" pill `#832902` on `#FED69B` at 6.7, and the red pill text `#AA060A` on `#FBE1E1` at 6.2. The image generator picked darker colours than the **tokens** define. The tokens are therefore the risk: a developer who uses `Risk medium` for amber text will ship 3.6 to 4.2. **Fix in the design system**: define explicit "on-tint" text tokens (amber text `#9C5D00` or darker `#8A4B00` on tints, green text `#2D794C`, teal text `#177272`), forbid gold and the amber fill as text or boundary, and define a control-border token at 3:1 or better.

**Small text.** The mockups use 11 to 12 px uppercase labels (for example "DATA COVERAGE", "NAME PART") in Stone. They pass contrast but are small; in Arabic there is no uppercase, so such labels are written in sentence case with weight 600 at 12 px or larger.

### 5.4 Tables, lists, charts and live regions

- **Data tables** use semantic tables or the React Aria table (`role="grid"` only where cells are interactive); header cells with scope; sort state announced; the row count and row index exposed when the list is virtualised (`aria-rowcount`, `aria-rowindex`) so a screen-reader user hears "row 14 of 1,284"; keyset pagination keeps the browser's focus stable.
- **Charts** carry a short sentence of what they show and a data table (the DBS "the trend carries a table for screen readers"), no colour-only legends, a single-hue sequential scale for quantity (DBS), and direct labels on stacked bars (mockup 01 already prints values).
- **The network graph** has a node-and-link table and a text summary ("9 accounts, 214 senders, 96 percent to two exchange companies") and keyboard traversal.
- **Live updates** (server-sent events) are announced politely and in aggregate ("3 new items in your queue"), never row by row; the queue does not reorder under the cursor; a "pause live updates" switch exists (2.2.2).
- **Dialogs** (step-up, confirmation) trap focus, return it on close, and keep the underlying page state; the step-up dialog is the shared component of the DBS design system.
- **Reduced motion** switches off transitions (DBS rule); there is almost no motion in the product by design ("exceptions are loud, health is silent").

### 5.5 Testing

Automated checks (axe-core or equivalent) on every route in both languages and both themes in CI; a manual keyboard-only walkthrough of the six core tasks (open a case, record a disposition, request information, escalate, sign a report, answer an RFI) per release; zoom to 400 percent in Arabic; screen-reader passes with at least NVDA and VoiceOver. **Arabic screen-reader behaviour with this interface has not been tested** (NOT VERIFIED), including pronunciation of mixed Arabic and Latin names and the announcement of Western digits in Arabic speech; it is a named item in the first usability round with Iraqi staff, and an accessibility conformance statement is issued per release.

### AKR-TMS hooks (section 5)

- **Design tokens** with measured contrast: on-tint text tokens, control-border token, forbidden pairs, enforced by a CI contrast check over the token file.
- **Accessibility test pack** (axe on all routes, keyboard scripts, zoom checks) and the conformance statement generator.
- **Session warning** component (extend action), **live-update pause** control, **shortcut settings** (off, remap), **step-up dialog** with paste and WebAuthn first.

## 6. Bilingual help, training and onboarding inside the product

### 6.1 What the Iraqi rules ask for

- **Annual training for all staff, including AML staff**, planned with the AML/CFT Office and the CBI; training records kept (programme, trainees, departments, content, duration, provider) (CBI controls 2025, ch. 2 art. 4 and ch. 4; file 01 section 2d).
- **Front-line staff**: an annual training plan approved by senior management, **competency tests**, and training records for the CBI (CBI circular of 26 August 2026, file 01 section 2c).
- **The Office's UTR guide (first edition 2026), section 9**: financial institutions must raise staff awareness of unusual-transaction reporting through periodic training programmes **supervised by the reporting managers**, including "practical explanation of possible scenarios".
- **CBI Detailed Assessment Guide D2.1(b)(4)**: investigations "conducted and/or supervised by qualified and appropriately experienced staff" (file 28 section 2.5).
- **MLRO and deputy**: certification (ACAMS, CGSS or CFCS) within a year of appointment (file 01 section 2d).

So the product holds a **training register** and links it to access (file 28 M-50): no queue access until the recorded training and the supervisor's sign-off exist.

### 6.2 What the evidence says about teaching an interface

Nielsen Norman Group concludes that onboarding tutorials "interrupt users who are attempting to do something else at that moment, they don't tend to be memorable, and they also don't result in better task performance", and recommends **pull revelations**: "help content triggered by some signal that the user would benefit from that information at that moment", easy to dismiss and to recall, with progressive disclosure (NN/g, "Onboarding Tutorials vs. Contextual Help", read 2026-10-07). Two of Nielsen's heuristics apply directly: "Minimize the user's memory load by making elements, actions, and options visible" (recognition rather than recall) and "It's best if the system doesn't need any additional explanation. However, it may be necessary to provide documentation to help users understand how to complete their tasks." WCAG 3.2.6 adds that help must sit in the same place on every page. AKR-TMS therefore has **no forced product tour**; help is in context, searchable, and bilingual.

### 6.3 The help system

| Element | Design |
|---|---|
| **Contextual help** | A "?" on each panel and each reason code opens a short popover: what this is, why it matters, what to do next, and a link to the bank's procedure or the CBI article it comes from; dismissible; never modal |
| **Terminology** | A glossary of Arabic and English terms owned by compliance staff, not machine-translated (file 32 section 18.3); file 06 section 12 holds about 45 terms as a seed; terms in the interface link to their definition on hover and focus |
| **Reason-code explanations** | Each reason code (file 21 section 11b) carries a plain-language explanation, the typology and the CBI article where one applies, in both languages, reviewed by compliance staff |
| **Procedures in context** | The bank attaches its own procedure text to a workflow step or a template (for example the branch RFI step shows the branch procedure); versioned with the policy pack (file 28 section 15.4); later compiled from the bank's SOPs (file 13 VB-42) |
| **Help centre** | Searchable, Arabic and English, **bundled with the release and available offline** (no external requests); every article has an owner and a review date |
| **Keyboard cheat sheet** | Opens on `?`; lists the shortcuts and how to remap or turn them off (section 5.2) |
| **Empty states and errors** | Every list has a designed empty state with one sentence and one action; an error says what happened, what the user can do, and shows the correlation ID (DBS rule); a refusal never looks like an empty list |
| **What changed** | Bilingual release notes in the product; **policy-pack changes** (routing, clocks, delegated authority, templates) notify the affected roles with the effective date and a plain summary of the difference |
| **No chatbot for policy advice** | The case assistant (file 28 section 14) answers questions about a case from its records; it does not answer regulatory questions; help content is static, versioned and reviewed |
| **One place** | The help entry is in the same header position on every screen, in both directions (WCAG 3.2.6) |

### 6.4 Training inside the product

- **Role-based learning paths** of short tasks: liaison officer (branch window, observation form, the monthly report, what never to say to a customer), level 1, level 2, screening specialist, MLRO and deputy, QA reviewer, KYC reviewer, RFI desk, assessor (read-only tour).
- **A training workspace** with **synthetic data only** and a permanent "TRAINING" mark; it cannot touch live data, its outputs are excluded from every statistic and report, and it uses the same screens, so skills transfer (file 30 SP-06: synthetic data only in test, demonstration and training).
- **Practice cases** with a known answer built from the typology library (file 27) and the golden set (file 28 section 10.6): the trainee decides, sees the expert path, and the result is recorded.
- **The tipping-off module**: what must not be said or written, the neutral wording library, examples of leaks, a short test (file 30 section 8.7: "what to say" training is where the product's control ends and the bank's begins).
- **Competency tests and records**: scores per topic, date, provider and duration written to the training register that the CBI may ask for; failures keep queue access closed; refresher cycle annual (file 28 M-50).
- **Training coverage reports**: by role and branch (for example liaison officers trained in the last 12 months), exportable for the CBI and the assessor.
- **Language**: Arabic first with English; Kurdish (Sorani) content is a later option for banks in the Kurdistan Region.

### 6.5 Usability testing with Iraqi staff

Nielsen's rule of thumb is five users per round, which finds about 85 percent of problems, with several small rounds rather than one large one, and three to four users per group when there are two distinct groups (NN/g, "Why You Only Need to Test with 5 Users", read 2026-10-07; the 85 percent is a model result, not a measurement for this product). AKR-TMS has at least five distinct groups (liaison officers, analysts, MLRO, QA, assessor), so the plan is small iterative rounds of three to five participants **per group**, with Arabic-speaking compliance staff, on the real screens in Arabic first: tasks from section 2.2, measures of success, time, errors and subjective ease, and a specific list of **mixed-script checks** (a mixed Arabic and Latin name, a phone number, a date, a range, a Kurdish name, Eastern and Western digit input). The first round happens on mockup-grade screens before the case workspace is built; the second on the first working vertical slice (file 28 section 16.7).

### AKR-TMS hooks (section 6)

- **Help service**: contextual popovers, glossary, offline help centre, release-note and policy-change notices, all bilingual and versioned.
- **Training register** and **training workspace** (synthetic, marked, excluded from reports) linked to queue access.
- **Usability test kit**: task scripts per role, a mixed-script checklist, and a measurement sheet.

## 7. Explanation design: why an alert fired, why a hit scored as it did, what the AI suggested

### 7.1 Principles drawn from the evidence and the rules

1. **Evidence before explanation.** Raw rows, peer comparison and the network come first; reasons second (Jesus et al.: data only gave the highest accuracy; explanations helped speed more than accuracy; file 21 section 11 "evidence-first alert page").
2. **Few reasons, each next to its fact.** At most **three headline reasons**, each with the actual value and a comparison ("14 times the segment median", file 21 section 11b), because more transparency and more features overloaded people in Poursabzi-Sangdeh et al. and explanations raised acceptance of wrong suggestions in Bansal et al.
3. **Show what argues against suspicion.** Mitigating factors (documents on file, a corroborated explanation, a known seasonal pattern) are part of an honest explanation (file 21 section 11b) and protect against both over- and under-reporting.
4. **Say what kind of explanation it is.** Rule logic is **exact**; model attributions are labelled "model contributions" because they describe the model, not causation (file 21 sections 11a, 11e).
5. **Progressive disclosure by role.** Branch users see reason codes without thresholds; analysts see parts and values; validators see the rule or model card; thresholds and weights are visible only to roles authorised to see detection logic (file 21 section 11b; file 29 section 9j).
6. **Same format everywhere.** Rules, scorecards, anomaly detectors, models and graph findings emit one reason-code structure (file 21 section 11b), so the analyst learns one shape.
7. **Stable and reproducible.** The explanation is stored with the alert (rule and model versions, list version, matcher version, folds applied) so it can be re-rendered years later (file 22 section 13.11; file 28 section 12).
8. **Never explain to the customer.** Counterfactual hints ("would have passed if ...") are internal only (file 21 section 11d; file 29 section 9j).

### 7.2 Four layers

| Layer | Where | Content | Size |
|---|---|---|---|
| **L0 queue row** | Work queue | Priority class badge (a word), customer name (Arabic first), the top reason in a few words, age, due, merged-alert count | One line |
| **L1 case header** | Top of the case workspace (mockup 02 "Why this case was raised") | Up to three reason chips with the fact in each ("Turnover 15x declared"), then the alert rows with scenario code, description and date | Fits above the fold |
| **L2 alert detail** | Expands in place | Score parts (typology, deviation and size, customer risk, counterparty, network, history), the numbers and rows behind each, peer comparison, what argues against, data-quality flags | One panel |
| **L3 model or rule card** | One click for validators, analysts on request | Plain-language rule generated from its configuration, parameters, version, typology and CBI article, test results, fidelity and stability notes (files 21, 29) | Separate page |

### 7.3 Monitoring alerts (case workspace)

- The headline chips of mockup 02 are the right form. Each chip is a button that scrolls and highlights the evidence it rests on (the transaction window, the counterparty rows).
- The alert rows show the CBI scenario code in a monospace chip (as in the mockup), its plain-language description and the date; the **plain-language description is generated from the configuration** so that the text and the rule cannot drift (mockup 05; file 27).
- Beside the money-in-and-out chart the **declared turnover** is drawn as a dashed line (mockup 02) and a **seasonal band** is drawn when the period includes Ramadan, Eid, Muharram or Arbaeen (file 24), so a seasonal peak is not read as an anomaly.
- A small "what argues against" list sits next to the reasons (documents on file, branch input marked "corroborated" or "not corroborated", earlier alerts closed as explained with their dates), and **data-quality flags** appear where a reason depends on a degraded field (file 20 L-44).
- The decision panel carries the **seven CBI escalation criteria** as a checklist (file 28 section 6.3) and the **UTR versus STR** decision aid (file 28 section 6.4).

### 7.4 Screening hits: how a score is explained

File 22 section 13.7 fixes the content (aligned name parts, folds applied, identifiers compared, frequency context, pair memory, "would change if"). The screen shows it in this order:

1. **A verdict sentence in words**, derived from the evidence and the CBI vocabulary: for example "Strong name match; birth year differs by 2 years; mother's name not compared" with the **score** secondary. A bare number such as the mockup's "91 match score" invites false precision; the product shows the score with the **evidence level** (strong, moderate, weak; the bits are on hover for specialists) and the threshold that applied to this list and context ("above 0.78 with identifiers", file 22 section 13.6). The ring colour must not carry the meaning alone (amber on white is 2.1:1).
2. **Both records side by side** (customer left, list entry right in LTR; mirrored in RTL), Arabic original first with transliteration beneath (section 4.3), and the **identifiers that decide**: birth date with its precision, **mother's name and maternal grandfather**, document numbers, nationality, place of birth, residence, family number, each marked agree, differ, or not comparable (the Office's guide lists the comparison data for a potential match as the four-part name, alias, two-part mother's name, birth date and place, nationality, residence, ID number, nature of activity and address).
3. **The per-part alignment table** (given, father, grandfather, tribe or family) in the **Arabic** original with the Latin form as a second line; the result for each part in words (same name, spelling variant; same name, very common; differs) plus the weight (high, medium, low), as the mockup does; the folds applied are listed ("hamza on the first letter; article stripped"); unmatched tokens are visible.
4. **Frequency context in plain terms**: "given name, father and grandfather are shared by 0.3% of your customers (about 520 people); the tribe name by 1.1%" (file 22 section 13.7), the sentence that tells the analyst why a "strong" match on a common name is weak evidence.
5. **List-entry facts**: the list and its **version and date**, the listing basis, aliases with their class (strong, weak, kunya), and a link to the source entry.
6. **Pair memory**: whether this pair has an earlier decision, by whom, when, and what changed since; the memory is shown as an explanation, not a silent suppression.
7. **For a cross-script alert** the screen says plainly "Arabic against Latin alias, graded match (not exact)" and shows the consonant key and the key distance (file 22 section 13.7).

**Decision wording follows the Iraqi texts, and there are two.** The AML/CFT Office's Guidance on the Prompt Implementation of Targeted Financial Sanctions (uploaded 10 September 2026, art. 12; Arabic read on 2026-10-07; Terrorist Funds Freezing Regulation No. 6 of 2023) names three results: **التطابق الحقيقي** (true match: "sufficient indicators confirm" that the customer, a related person or the beneficial owner is the listed person), **التطابق المحتمل** (potential match: "similarity in name or data", to be reviewed, compared and documented, with nothing disclosed to the customer) and **التطابق غير الصحيح** (incorrect match: no link after verification; "the reason for excluding the match is recorded"). The CBI's 2023 sanctions guidelines (p. 21) use **تطابق محتمل**, **تطابق مؤكد** (confirmed, also "when there is any suspicion or doubt") and **نتيجة إيجابية خاطئة** (false positive: "it is confirmed that the match is not compatible"). The two differ on doubt (the CBI treats any doubt as confirmed; the Office sends a doubtful case to further review) and on the Committee report clock (three days against 48 hours; file 28 section 3.4). The recommendation: the **Office's terms are the labels** (True match, Potential match, Incorrect match), the CBI's terms appear as aliases in help text and exports, and a per-bank **doubt policy** sets what an unresolved potential match becomes at the verification deadline. Each label carries its **consequence as a second line**: "Potential match: review continues, nothing is frozen, verification due in 8 hours"; "True match: creates a freeze task with an 8-hour clock and a 48-hour report to the Committee"; "Incorrect match: records the reason for exclusion and writes pair memory, expires in 12 months". A coded reason is required with free text.

The mockup's tiles ("Possible match: escalate", "True match: freeze and report", "Not the same person") are already close to the Office's wording; the change is "Possible" to "Potential", "Not the same person" to "Incorrect match", the consequence lines, and one inconsistency: **"Remember this decision for this pair" appears only on Incorrect match**, unticked, showing its expiry; the mockup shows it ticked while "Possible match" is selected, but pair memory records a final not-the-same-person decision and is invalidated when either record changes materially (file 22 section 13.8).

### 7.5 The customer risk score (mockup 04)

Score drivers are a sorted bar list with the points and the reason beside each (the mockup does this well); add the **date the factor was last refreshed**, the **share of missing inputs** (file 20 L-10, L-44: "unknown is not low"), and a one-click **history** of the factor changes behind a rating change. The semicircular gauge is decorative; the number, the level in words and the 12-month trend carry the meaning and should stay.

### 7.6 AI summaries, drafts and suggestions

| Element | Rule |
|---|---|
| **Label** | "DRAFT · REQUIRES REVIEW" stays on every AI-generated block until a human edits or accepts it; after editing it reads "DRAFT BY ASSISTANT · EDITED BY ANALYST" (as in mockup 07) and the saved text records the responsible person (file 29 section 9k) |
| **Sentence-level sources** | Every sentence has a source marker; clicking or hovering highlights the rows it rests on; a sentence the verifier cannot ground is shown struck through or removed (file 21 section 12a); the single "view sources" link of mockup 02 is not enough |
| **What it did not consider** | One line listing data that were unavailable (for example "no documents on file", "branch input missing") |
| **Recommendations** | Shown after the required evidence has been opened (P0 and P1) and optionally after the analyst's own first assessment; they list the checklist items and source rows they rest on; no confidence percentage |
| **Acting on it** | Accept, edit, or dismiss with a one-click reason; never "accept all"; override and acceptance are logged per task (file 28 section 14.3) |
| **Feedback** | A "useful or not" control per output feeds the AI monitoring panel; open-source Marble's AI review has the same control and lets the analyst "add the review as a comment in the investigation timeline" (documentation read 2026-10-07) |
| **Provenance** | Model and exact version, prompt template version, retrieved record IDs one click away (file 28 section 14.2) |
| **Language** | Output language follows the user; names, numbers and dates are copied, never regenerated (file 21 section 12a use case 8) |

### 7.7 Not overwhelming the analyst

Budgets: three headline reasons; five score parts; one verdict sentence per hit; no panel that needs scrolling inside a panel; no explanation longer than the evidence it explains. Each budget is a test: a usability task measures **time to first relevant evidence opened**, **correct disposition on seeded cases** and **self-reported usefulness**, with and without the explanation, in Arabic with Iraqi analysts. Fidelity and stability tests of the explanations themselves belong to validation (file 21 section 11d).

### AKR-TMS hooks (section 7)

- **Explanation component set**: reason chips with drill-down, mitigating-factor list, data-quality flags, layered disclosure by role, rule and model cards, verdict-sentence generator for screening hits, frequency-context sentence, pair-memory panel.
- **Decision vocabulary**: the CBI's three match states in Arabic and English, with consequence lines.
- **AI block component**: label states, sentence-level sources, unsupported-sentence handling, feedback control, provenance link.
- **Explanation usability protocol** (time to first evidence, seeded accuracy, usefulness) run per release.

## 8. Dashboards, reports and exports for the MLRO, the board and the assessor

### 8.1 What the rules ask for

- **To the board**: "detailed periodic reports to the board via the audit committee on unusual and suspicious transactions and actions taken, with copies kept for CBI inspection" (CBI controls 2025, ch. 3, MLRO duty k).
- **D2.1(a)(2) of the CBI Detailed Assessment Guide**: structured reporting to senior management and the board with "AML/CFT risks, compliance gaps, escalation records and mitigation", including key risk indicators and "trends in SAR/STR reports and the results of regulatory inspections or audits".
- **The monthly regulatory reports** the AML system must produce (CBI ch. 15; file 01 section 2d): account movements and balances for customer **and employee** accounts, the transfers register, and correspondent-account activity.
- **The liaison officers' monthly reports** by the 10th, reviewed and signed by the MLRO (CBI ch. 11).
- **The CBI's PEP effectiveness indicators** (a to g, circular of 26 August 2026; file 01 section 2c) as a ready report.
- **Four evidence packs** promised in the plan, one per audience: CBI and the Office; correspondent banks; the independent validator; the board and audit committee (plan section "Governance and evidence packs").

### 8.2 Dashboard principles

1. **Attention first, then the period, then the detail**: the DBS report rule "what needs attention NOW, then the period against the one before, then the detail; every count opens the list behind it, and nought is plain text" (DBS `DESIGN-SYSTEM.md` section 8). Mockup 01 follows it: KPI tiles, then backlog by age, then deadlines.
2. **Every number is a door and has a definition.** A tile opens the filtered list that makes it up; hovering shows the definition and formula from the metric catalogue (file 28 section 11.5), the data-as-of time (the mockup prints "data as of 06 Oct 2026, 14:32", which is right) and the caveat.
3. **Exceptions are loud, health is silent** (DBS): badges only on what needs attention; a healthy tile is plain.
4. **No staff league tables** and no metric that rewards closing speed alone (file 29 section 9j).
5. **No claim without proof.** "Data coverage 100%" is a link to the reconciliation ladder and the coverage certificate (file 25), not a decoration; "List freshness 38 min" opens the list-load log; a tile that cannot be proven is not shown.
6. **Charts**: one hue for quantity (DBS), direct labels, a data table for screen readers, no colour-only legends (section 5.4); chart libraries are chosen after the RTL spike (file 32 section 18.4); the DBS shows that small in-house SVG charts can carry a whole report suite.
7. **Print and PDF**: A4, light, landscape for packs, with print column counts (DBS reports) and Arabic shaping through the Gotenberg image.

### 8.3 The MLRO command centre (mockup 01 and what to add)

Present in the mockup: data coverage, list freshness, open alerts with the count older than 30 days, STRs filed in 30 days with "2 awaiting MLRO sign-off", overdue KYC reviews; alert backlog by age and workstream; regulatory deadlines; customer risk distribution; top scenarios with the productive rate; the correspondent evidence pack.

Add, in this order of urgency:

| Block | Why |
|---|---|
| **Decisions waiting for me** (cases at `mlro_decision`, closures above delegated authority, report sign-offs) with age | It is the MLRO's own queue; today it is hidden behind "Cases" |
| **Overdue and at-risk clocks** (decision-to-filing, RFIs, hold and freeze tasks, Committee notifications, liaison monthly reports) | These are the dated control failures regulators punish (file 28 section 11.1) |
| **Arrivals versus throughput** and **capacity ratio** with the forecast | Backlog is a flow problem (file 28 section 11.2) |
| **Verification failures** (a freeze with no verified block, a post-order debit) | The failure mode to design out (file 28 section 7) |
| **QA status** (sample drawn, critical findings, QA backlog age) | D2 assesses QA |
| **Data and coverage incidents** (blind periods, quarantine, reconciliation breaches) | Metro Bank (file 20 section 6.3) |
| **Tipping-off control report summary** (denied opens, conflicts, exports, break-glass) | Monthly report to the MLRO and the CISO (file 30 section 8.8) |

The "Top scenarios this month" table should show alerts, **cases**, UTRs and STRs per scenario, not only alerts and a "productive" percentage (productive must be defined: it is not "led to an STR" alone, file 28 section 11.5).

### 8.4 The board and audit-committee view

One page, generated from system facts, with any manual edit shown with author and time (file 20 L-56; file 28 M-56). Proposed content:

1. **Control status**: coverage of monitored transactions by count and value; reconciliation status; blind periods in the period.
2. **Flow**: alerts generated, grouped into cases, decided; ageing; arrivals against throughput; time to decision and to filing (median, 90th percentile, maximum).
3. **Reports**: UTRs, STRs and TFRs filed, quality grades received from the Office, rework, late-detection share.
4. **Quality**: QA failure and critical rates with intervals; reopen rate; shelved-decision sample result.
5. **Resources**: staffing against forecast workload; training coverage; liaison reporting compliance.
6. **Screening**: list freshness, known-answer test results, freeze tasks and their verification, Committee notifications.
7. **People and access**: summary of the tipping-off controls (attempts, conflicts, exports), privileged access.
8. **Governance**: changes to scenarios, thresholds, lists and policy packs with approvals (volume-reducing changes flagged), open findings from the CBI, auditors, validators and correspondents with age and recurrence.
9. **Correspondents**: RFIs received and answered, time to answer, open requests.
10. **PEP indicators** (a to g) and EDD backlog.

The board view is a **PDF and a read-only page**; it contains no case-level content and no names, so it can be circulated within the bank without tipping-off risk.

### 8.5 The assessor's workspace, mapped to D2

The assessor needs the product organised like the CBI's test script. A "D2 evidence matrix" page lists each clause with the screen, report or export that answers it, so the external firm can work through it in order:

| D2.1 clause (element and item) | Product evidence |
|---|---|
| (a)(2) Programme monitoring and reporting | Board pack, escalation records, KRI trends |
| (a)(3) Programme organisation | Role model, MLRO reporting line, staffing evidence |
| (b)(1) Customer risk classification | Rating method, factor refresh dates, rating history with reasons (file 26) |
| (b)(4) AML investigation | The state machine, delegated-authority matrix, end-to-end case traces for a random sample, timeliness by priority class (file 28) |
| (b)(5) Transaction monitoring | Scenario register with versions and coverage matrix, above- and below-the-line tests, calibration history, QA results |
| (b)(6) Sanctions screening | List freshness log, known-answer harness results, resolution timelines, freeze verification |
| (c)(1) Data: 7 years, accurate, complete, traceable | Retention report; reconciliation and coverage certificates; lineage from CBS column to scenario input (files 25, 30) |
| (c)(2) Systems and the assessment of enablers | Configuration export, threshold register, access matrix and role-grant history, audit-chain verifier, integration status, backup and restore tests, retention certificates |

(Numbering follows the CBI Detailed Assessment Guide 2025, standard D2.1: element (a) governance and organisation, (b) procedures, (c) enablers; file 28 section 2.5.)

The page offers a **random sample draw** from the alert register, the end-to-end trace, the verifier, and the pack download, all through a named, time-boxed, read-only account whose every action is logged (file 28 section 12.6; file 30 section 7.2).

### 8.6 Reports and exports

| Report | Audience | Format | Notes |
|---|---|---|---|
| Monthly account movements (customer and staff), transfers register, correspondent-account activity | CBI, internal | PDF, XLSX | Built in, scheduled; the staff-account part is restricted (file 30 section 9.5) |
| Liaison monthly report | MLRO, inspectors | PDF | Auto-drafted by the 10th |
| Board pack | Board, audit committee | PDF | Section 8.4 |
| Correspondent evidence pack | Correspondent banks | PDF in English and Arabic | Lists, freshness, tests, coverage, backlog, RFI times (file 05 section 5) |
| Validator pack | Independent validator | ZIP of PDF, JSON | Inventory, tests, change history (file 29) |
| Assessor pack | CBI-approved firm | ZIP plus workspace | Section 8.5 |
| Evidence pack for a case | Courts, the Office | PDF/A, JSON, CSV, manifest | File 28 section 12.6 |

All packs: watermark with the requester, an audit entry, a disclosure-register entry for external recipients, generation from system facts, and Arabic and English renderings.

### AKR-TMS hooks (section 8)

- **Dashboards**: command centre additions, board view, D2 evidence matrix, each tile linked to its list and definition.
- **Report builder** for the built-in regulatory reports with scheduling and the watermark and disclosure-register behaviour.
- **Metric catalogue UI** (definition, formula, caveat, data-as-of) shared by all dashboards.

## 9. The user experience of the vendor products in files 08-16

### 9.1 How much can be said

Independent usability evidence on AML case tools is thin. Review sites that carry user comments (G2, Gartner Peer Insights, TrustRadius, Capterra) were blocked or returned no cached copy on 2026-10-07; what was reachable is PeerSpot summaries, one Software Advice page, vendor documentation and vendor-chosen quotes (grade C). Statements below are therefore **what is documented or reported**, not a usability ranking. The strongest single finding is negative and complete: **no incumbent documents an Arabic interface, right-to-left layout or Hijri dates** (file 07 section 8; 51 NICE pages searched without a hit, file 08; none for SAS, SymphonyAI, Oracle's UI or the AI-native vendors, files 10, 11, 12; WorkFusion's agent supports English and Spanish only, file 13).

### 9.2 What is described, and what reviewers say

| Product (file) | UI pattern documented or shown | Praised | Criticised | Grade |
|---|---|---|---|---|
| **NICE Actimize ActOne** (08) | Dynamic workflows by alert type and severity; queues and routing by risk; entity-centric view with point-in-time history; graph exploration; alert auto-sampling, analyst scoring and surveys (QA); investigator-performance dashboards; "Productivity Studio"; alert-source-agnostic intake | "It's a very good case management system" with strong audit trails (AVP compliance, 10,001+ employees, 2023); "The case management tool is user friendly" (VP compliance, 2020); rules designer "very user-friendly" for business users (application analyst, fraud product) | "Too complex for its own good" (director, 2021); an AML analyst reports that searching for customer transactions "sometimes results in errors" and "performance issues ... especially when many users are online"; a fraud strategist wants "reporting features and dashboards" improved with Tableau-like structures; "interface speed" listed as an improvement area | C (PeerSpot, 18 reviews, rating 4.1) |
| **Oracle FCCM, Investigation Hub, Nexus** (09) | Queues with attributes; case merging; RFIs with participants and template variables; dossier generation; graph with rule-based styling; workflow and case designers; goAML STR application with maker-checker, validation errors tab with in-place correction and report preview; AI Investigator with an agent decision-tree UI and a chatbot inside the hub | Documented goAML workflow (the only incumbent with one) | "The UI's performance is very slow, and the processing time is very high when handling a huge amount of data" (cloud service review); "Oracle's performance isn't good, and it's very slow" (project lead on three products) | C (PeerSpot, one review of the cloud service) |
| **SAS Visual Investigator** (10) | Alerts organised by domain, strategy and queue; configurable workflows with routing and escalation; investigative workspaces with **timelines, maps and network views**; audit log; **mobile app** for field staff | Forrester Q3 2026: highest current-offering score, top marks in 9 of 11 criteria including case management and queue definitions and alert routing | PeerSpot has no reviews; Arabic and RTL not verified | A for the analyst ranking (report not read), none for user comments |
| **SymphonyAI SRI Investigation** (11) | Subject-centric single view across AML, KYC, sanctions and fraud; copilot; Narrative Agent | Forrester Q2 2025: 5/5 for case management and queues; customer-level roll-up (20 percent fewer alerts at Metro Bank, vendor-reported) | PeerSpot reviewer on the Detica heritage: "not that great in terms of the UI" | C |
| **Nasdaq Verafin** (11) | In-platform alert, case and e-filing | A bank case study: "more user-friendly, provides very distinct alerts and allows us to do e-filing within the platform" | None found; SaaS in North America | C (vendor case study) |
| **LexisNexis Bridger Insight XG** (14) | Screening search and alert screens | Software Advice: documentation, support, value (4.3 of 5, 13 reviews) | "Reporting needs a lot of work"; "Not really user friendly and too technical vocabulary"; "Continually having to tick features when accessing search" | C (13 reviews) |
| **Thomson Reuters World-Check One** (15) | Cloud screening and case-management platform; batch, single-name and ongoing screening; "user interface in multiple languages" | Named by the CBI; Arabic search; interface languages are a vendor statement | Arabic UI not verified | C |
| **Lucinity Case Manager and Luci** (13) | Case-first design; Customer 360; copilot panel available wherever the analyst works; configuration of automations | Customer quote: investigations "easily traceable" | One-customer, undefined "3 hours to 30 minutes" claim | C |
| **Hummingbird** (13) | 360 profiles, no-code automations, agent steps visible with accept, reject or override at each step | The step-visible autonomy dial is the clearest published human-oversight UI | Few published results | C |
| **Flagright** (13) | Workflow builder; rule simulation and shadow rules; 7-stage filing with side-by-side version comparison and locked finals | Filing lifecycle and version comparison | Claims without method | C |
| **Napier Continuum** (13) | Plain-language explanation per alert and per task; sandbox before go-live; agents with chosen autonomy | Sandbox-first change control | UK-centred references | C |
| **Hawk, Feedzai, Quantexa** (12) | Per-alert explanations, two scores per case with human-readable text (Hawk); whitebox explanations (Feedzai); the network as the explanation (Quantexa) | Explanation as a first-class output | No Arabic UI evidence | C |
| **Marble** (open source; documentation read 2026-10-07) | Inboxes by team or level and a personal "My inbox" across them; **"Next Unassigned Case" one-click** to the oldest unassigned case; case statuses pending, in progress, closed; snoozing a case to a date; an **Investigation Panel** of all events with filters by type, notes and documents; an alerts table; tags; escalation to a designated inbox; closing with an outcome (false positive, valuable alert, risk confirmed); **AI review** per case with a "useful or not" vote and "add as comment"; screening hits reviewed **one by one** with similarity percentage, "no match" or "confirmed match" plus comment, other hits marked "skipped" once one is confirmed; pair whitelist | A complete, documented, self-hostable reference for a modern case UI | Whitelist entries "are not limited in time" (AKR-TMS expires pair memory, file 22 section 13.8); screening review only inside a case; AI features SaaS-only | C (vendor documentation) |

### 9.3 What AKR-TMS takes from this

| Lesson | From | Product consequence |
|---|---|---|
| Case management with audit trails is what reviewers praise in the market leader | NICE | The audit trail is visible in the case (the activity log and the timeline), not only in an admin screen |
| **Search and transaction lookup are where the pain is** | NICE (search errors), Bridger (search options re-ticked) | A search that detects the kind of input, remembers options, and is fast under concurrency (section 4.5) |
| **Complexity and slowness drive criticism** | NICE ("too complex"), Oracle ("very slow") | Role-based navigation that shows only what a role needs; performance budgets tested with concurrent users; a light branch window |
| **Reporting and dashboards are repeatedly called weak** | NICE, Bridger | Reports and dashboards are first-class screens with definitions and drill-down (section 8) |
| **Technical vocabulary** | Bridger | Arabic terms written by compliance staff; plain-language reason codes; a glossary (section 6.3) |
| Business users can edit rules when the designer is friendly | NICE fraud designer, Flagright, Napier | The scenario studio (mockup 05) with plain-language descriptions, tests on history and approval |
| **In-platform filing** is valued | Verafin, Oracle, Flagright | The report builder inside the case (mockup 07) with validation and sign-off |
| Visible autonomy and step-by-step reasoning | Hummingbird, Napier | The AI block states what it did and did not consider (section 7.6) |
| Quick triage between cases | Marble | Queue-run mode and a one-key "next" (section 3.3) |
| A whitelist that never expires is a hidden suppression | Marble's screening whitelist | Pair memory with fingerprints and expiry (file 22) |
| Nobody serves Arabic, right-to-left, Hijri | file 07 | Arabic-first design is the largest user-experience differentiator and a testable claim (section 4) |

### AKR-TMS hooks (section 9)

- **Competitive usability benchmark**: the six tasks of section 5.5 timed on a vendor demo where one is obtainable, to replace the thin review evidence with a measurement.
- **Performance budgets**: queue and case navigation within 1 s with 50 concurrent analysts on the reference hardware (file 32), measured in CI against a synthetic data set.

## 10. Screen inventory for AKR-TMS

"Mockup" gives the approved mockup number where one exists (the mockups show the English left-to-right layout; every screen is designed right-to-left first and mirrored). "New" means no mockup exists. Phase refers to the release plan of file 28 section 16.7 and the plan's roadmap: P1 is the first release. Roles are the abbreviations L (liaison officer), A1 and A2 (level-1 and level-2 analyst), SS (screening specialist), M (MLRO and deputy), QA, KYC, RFI (RFI desk), OPS (operations officer), CFG (compliance configuration manager and approver), DS (data steward), CH (compliance head), BD (board committee readers), IA (internal auditor), AS (CBI-approved assessor or CBI inspector), ADM (platform administrator), SEC (CISO).

### 10.1 Cross-cutting screens

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| X1 | Sign-in and step-up dialog | All | Authenticate; re-authenticate for signing-grade actions | Password and second factor, WebAuthn first, paste allowed in code fields, a calm failure message with the correlation ID, lockout text, session countdown and "extend" action, state preserved across step-up | New | P1 |
| X2 | Global search and command palette | A1, A2, SS, M, KYC, RFI, QA, CH | Find a customer, account, IBAN, card, phone, case, alert or report; jump to a screen | One box in the header; detects input kind; Arabic fold and digit normalisation; results grouped by type; suspicion-tier cases hidden from non-members; keyboard shortcut; recent items | Header box in all mockups | P1 |
| X3 | Notifications and task inbox | All | One place for "waiting on you" | Generic text with no name, amount or case type; counts; paused live updates control; digest option | Bell icon in all mockups | P1 |
| X4 | Profile and preferences | All | Language, digit style in prose, density, shortcuts (remap or off), saved views, notification digest | Arabic and English toggle; no preference weakens a security policy | New | P1 |
| X5 | Help centre, glossary, "what changed" | All | In-context and searchable help | Offline, bilingual, owner and review date per article; policy-change notices | New | P1 |
| X6 | "Not found" and "no access" screens | All | A refusal that does not leak existence | Byte-identical "not found" for suspicion-tier refusals; a distinct permission-denied page only where existence is not secret | New | P1 |

### 10.2 Work: queues, cases, investigation

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| W1 | My work | A1, A2, SS, RFI, QA, KYC | Start the day: what is mine, what is due | Next item (queue-run), due today, overdue, items awaiting information, saved views | New | P1 |
| W2 | Alert and case queue | A1, A2, M, CH | Triage and pick work | Priority class badge as a word, customer (Arabic first), top reason, age, due, merged-alert count, owner; filters (class, scenario, branch, age, tier, owner); saved views; bulk action bar with safeguards; density; keyboard navigation; paused live updates | "Alerts" and "Cases" rail items, no screen | P1 |
| W3 | Case workspace | A1, A2, M | Investigate and decide | Summary strip; why raised (chips and alert rows); entity timeline; money in and out against declared turnover with seasonal band; counterparties; evidence panels (transactions, documents, RFIs and answers, earlier cases, screening, adverse media); network link; AI block; decision panel with escalation checklist and UTR/STR aid; activity log; sticky decision panel | 02 | P1 |
| W4 | Party page (customer 360) | A1, A2, M, KYC | See everything about a party across families | Profile (name chain, mother's name, identifiers), accounts, rating and drivers, open and closed cases and watches, linked parties, timeline, documents | 04 is the risk-profile part | P1 |
| W5 | Transaction explorer | A2, M | Explore beyond the alert window | Windows and filters, counterparty grouping, export disabled for suspicion tier, add-to-evidence | Part of 02 | P1 |
| W6 | Network view and network case | A2, M | Work a cluster | Ego graph with grouping and drill-down, cluster summary, links found, node and link table, "open one case for the cluster", "add graph to evidence" | 06 | P1 |
| W7 | Decision dialogs | A1, A2, M | Record a decision | Outcome, coded reasons, free text with generic-text check, checklist, evidence considered, step-up, neutral wording | Decision card in 02 | P1 |
| W8 | RFI composer and RFI desk | A1, A2, RFI, M | Ask and answer | Six kinds, templates, deadline and clock, pre-filled Wolfsberg answers, four-eyes release, forbidden-word scan, answers as claims | New | P1 (branch RFIs), P2 (correspondents) |
| W9 | Operations tasks and verification | M, A2, OPS | Instruct and verify holds, freezes, restrictions | Neutral instruction and reference; clock; verification level V0 to V3; failure incident; for OPS a minimal inbox that shows nothing else | New | P1 |
| W10 | Watches and hibernations | A2, M, CH | See what is held back and why | Ledger with envelope, expiry, owner, regret rate | New | P1 |
| W11 | Internal reports and authority requests | M | Register and answer | Acknowledgment times, decision notices, request register with deadlines | New | P1 |
| W12 | Seeded cases and training workspace | A1, A2, QA | Practise and calibrate | Synthetic cases, expert path, results to the training register | New | P2 |

### 10.3 Screening

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| S1 | Screening review | SS, A2, M | Decide a potential match | Records side by side, Arabic first with transliteration, identifiers that decide (mother's name, birth date and precision, documents), per-part alignment, verdict sentence, frequency context, list entry facts, pair memory, CBI three states with consequences, history | 03 | P1 |
| S2 | Screening queue | SS | Work hits by priority | Lists, P0 flags, clocks, bulk "incorrect match" only where the pair evidence is identical, and never on freeze lists | New | P1 |
| S3 | List status and known-answer tests | SS, CFG, M, AS | Prove lists are fresh and tests pass | List versions, load times, record counts and checksums, daily harness results in Arabic, Kurdish and Latin | New | P1 |
| S4 | Pair-memory and exemption register | SS, M, CH, AS | Govern suppressions | Entries with fingerprints, expiry, approver, volume passed; entries now matching a current designation (must be zero) | New | P1 |
| S5 | Freeze board | M, SS, OPS | Track freezes to verification and notification | Task, verification result, post-order debit alarm, freeze clock, 48-hour Committee report and two-month update | New | P1 |

### 10.4 Customers, KYC, ownership

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| K1 | Customer risk profile | KYC, A2, M | Understand and review a rating | Rating, drivers with refresh dates and missing-input share, history, EDD checklist, ownership, evidence vault | 04 | P1 |
| K2 | KYC review queue and workspace | KYC | Periodic and event reviews | Due dates, overdue by age, questionnaire designer outputs, exit-candidate queue | New | P1 |
| K3 | Ownership and control; evidence vault viewer | KYC, A2 | See beneficial ownership and documents | Tree with percentages, 20 percent test, documents with lock status, hash, language tag | Part of 04 | P1 |
| K4 | PEP register and relatives | KYC, M | Maintain PEP status | Positions, relatives to second degree, status persistence, CBI KPI report | New | P1 |

### 10.5 Detection configuration

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| D1 | Scenario library and coverage matrix | CFG, M, AS | See what is monitored | The CBI catalogue mapped to scenarios, versions, owners, last tuned, data-readiness grade, coverage gaps | New | P1 |
| D2 | Scenario studio | CFG, M | Change a scenario safely | Parameters by segment, plain-language description, test on history, below-the-line sample, change request and approval, version history | 05 | P1 |
| D3 | Policy studio | CFG, M | Edit the policy pack | Routing table, clocks, calendars, delegated-authority matrix, reason codes, templates, switches; dry run on history; change request | New | P1 |
| D4 | Model registry and AI governance | CFG, M, IA, AS | Govern models and assistants | Inventory, tiers, cards, validation, drift, evaluation, autonomy level, kill switch | New | P2 |

### 10.6 Reporting

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| R1 | Report builder (UTR, STR, TFR) | A2, M | Build, validate, sign and file | Six-step stepper, reason and indicators, narrative with draft badge, validation panel with linked fixes, timeline, preview, goAML reference and acknowledgement | 07 | P1 |
| R2 | Report list and FIU feedback register | M, A2 | Track filings | Status, acknowledgement, quality grade, rework tasks, requests for more information | New | P1 |
| R3 | Regulatory and periodic reports | M, CH, DS | Produce scheduled reports | CTR batches, monthly movements, transfers register, correspondent activity, scheduling, watermark | New | P1 |
| R4 | Board pack and evidence pack builder | M, CH | Generate packs per audience | Period, audience, contents, edits shown, verifier | New | P1 |
| R5 | Liaison report review | M | Review and sign monthly liaison reports | Per branch status, overdue, drill into items, sign with step-up | New | P1 |

### 10.7 Data

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| T1 | Data connection | DS, M, ADM | See that the link is read-only and healthy | Read-only check, replica delay, reconciliation, data-quality issues, feed volume, bank kill switch | 08 | P1 |
| T2 | Reconciliation and coverage ledger | DS, M, AS | Prove completeness | Per table and per type counts and sums, quarantine, coverage certificates, dark units | New | P1 |
| T3 | Mapping workbench | DS | Map the CBS | Code maps, drift alerts, version comparison | New | P1 |
| T4 | Data-quality and quarantine queue | DS | Fix data issues | Issues by field and source with age and owner | Part of 08 | P1 |

### 10.8 Governance and assurance

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| G1 | Command centre | M, CH | Know what is late | Mockup 01 plus decisions waiting, clocks, arrivals against throughput, verification failures, QA, data incidents, tipping-off summary | 01 | P1 |
| G2 | Capacity workbench | M, CH | Plan staffing | Workload by class, staffing inputs, projection, sensitivity | New | P2 |
| G3 | QA workspace | QA, M, CH | Review samples | Sample draw and register, rubric, findings, calibration, bulk reopen | New | P1 |
| G4 | Registers | M, CH, IA, AS | Decisions, exclusions, findings, warnings | Decision register, exclusion register with expiry, findings with recurrence, warnings register | New | P1 |
| G5 | Access and insider controls | SEC, M, IA | Review access | Access history per record, access-request approvals, conflict denials, break-glass log, privileged activity, canary alerts | New | P1 |
| G6 | Audit viewer and chain verifier | IA, SEC, AS | Prove integrity | Chain status, anchors, signatures, locked copy, audit-the-auditor | New | P1 |
| G7 | Retention and legal holds | M, CH, SEC | Govern disposal | Retention report, dry run, holds, deletion certificates | New | P1 |
| G8 | Assessor workspace | AS | Run the D2 test | D2 evidence matrix, random sample, end-to-end trace, verifier, packs | New | P1 |
| G9 | Training register | CH, M | Evidence competence | Programmes, participants, tests, coverage by role and branch | New | P1 |

### 10.9 Branch window (liaison officer)

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| B1 | Branch home | L | See items for this branch | Items with neutral statuses, due times, the monthly report status, a prominent "report a concern" button | New | P1 |
| B2 | Branch review | L | Add what the branch knows | One-screen summary, reason codes without thresholds, branch input form (explanation as a claim, documents on file yes or no, visit result), submit | New | P1 |
| B3 | Observation form | L, front-line staff | Record a CBI behavioural indicator | The eight indicator groups as ticks, notes, send to the MLRO, written acknowledgment on screen | New | P1 |
| B4 | Confidential report | L | Report straight to the MLRO | Short form, neutral confirmation, tipping-off reminder | New | P1 |
| B5 | Monthly liaison report | L | Check and sign by the 10th | Auto-drafted list with neutral statuses, sign with step-up | New | P1 |
| B6 | Branch transactions and documents check | L | Examine the branch's transactions, check documents attached | Branch-scoped transactions, document-completeness flags, screening results | New | P1 |

### 10.10 Administration

| ID | Screen | Roles | Purpose | Key elements | Mockup | Phase |
|---|---|---|---|---|---|---|
| A1 | Users, roles and leave | ADM, SEC | Provision, offboard, review | Role grants with vetting status, one-click offboarding, leave ledger, quarterly access review | New | P1 |
| A2 | Settings | ADM, SEC | Identity, session, notification and mail settings | CBI minima as floors; changes through change request | New | P1 |
| A3 | Platform health | ADM | Jobs, backups, certificates, seal state | Status tiles and drill-down, no business data | New | P1 |
| A4 | Release and updates | ADM, SEC | Apply signed bundles | Bundle verification, SBOM diff, release notes, rollback | New | P1 |

**Count:** 6 cross-cutting, 12 work, 5 screening, 4 KYC, 4 detection, 5 reporting, 4 data, 9 governance, 6 branch, 4 administration: **59 screens**, of which 8 have mockups.

### 10.11 Navigation by role

The rail shows only what a role may open (never a control that cannot work):

| Role | Rail |
|---|---|
| Liaison officer | Branch window only (home, items, observation form, report to the MLRO, monthly report, documents check) |
| Level-1 and level-2 analyst | My work, Alerts and cases, Customers, Screening (if skilled), Reports (read), Help |
| Screening specialist | My work, Screening, Customers, Cases (linked), Help |
| MLRO and deputy | Command centre, Decisions, Alerts and cases, Screening, Customers, Reports, Governance, Help |
| QA reviewer | QA workspace, sampled cases, Help |
| Compliance configuration manager | Scenarios, Policy studio, Governance (change requests), Help |
| Data steward | Data, Help |
| Compliance head and board readers | Command centre, Reports, Governance (read) |
| Internal auditor and assessor | Assessor workspace, Governance (read), Reports (read) |
| Platform administrator and CISO | Administration, access and insider controls (CISO), platform health |
| Operations officer | A single "operations tasks" inbox |

The nine rail entries of the mockups map to: Command centre = G1; Alerts and Cases = W2 and W3 (separate entries today; the queue should be one entry with "Alerts" and "Cases" as saved views); Customers = W4, K1 to K4; Screening = S1 to S5; Scenarios = D1 to D4; Reports = R1 to R5; Data = T1 to T4; Governance = G2 to G9 and the registers. **Missing from the mockups' rail: My work, Branch window, RFI desk, QA, Assessor workspace, Training, Administration.**

### AKR-TMS hooks (section 10)

- **Screen catalogue** as a machine-readable list (ID, roles, route, affordance rules) that feeds route generation, the role-aware rail, the access-matrix report and the usability test plan.
- **Route-to-policy coverage test** so no screen answers without a decision (file 30 SP-25, SP-71).

## 11. Interaction rules

Rules are numbered so that design reviews and tests can cite them. "Why" names the evidence or the earlier rule. Budgets are targets to be tested, not measurements.

### 11.1 Orientation and navigation

| ID | Rule | Why |
|---|---|---|
| IR-01 | The rail shows only what the role may open; no control is rendered that cannot work; the API returns affordances | DBS frontend rules; section 10.11 |
| IR-02 | Every screen shows its **data-as-of** time and, where relevant, the list and configuration versions | Mockups 01 and 08; file 20 FM-29 |
| IR-03 | Every count is a door: clicking opens the filtered list behind it; zero is plain text | DBS reports rule |
| IR-04 | Filters, sort, view and period live in the address, so a view can be bookmarked or shared; sharing a view never shares data the recipient cannot open | DBS reports; Cloudscape filter persistence |
| IR-05 | Breadcrumbs and back always return to the same place in the same queue position | Queue-run mode |
| IR-06 | The help entry has one place in the header in both directions | WCAG 3.2.6 |

### 11.2 Lists and queues

| ID | Rule | Why |
|---|---|---|
| IR-10 | Queue rows show: priority class as a word, customer (Arabic first), top reason, age, due, merged-alert count, owner; no more | Section 7.2 L0 |
| IR-11 | Sorting is by priority class, then score, then age; the sort is explained in a tooltip; text sorts through the collator | File 28 section 4; section 4.5 |
| IR-12 | Rows are never reordered under the cursor by live updates; new items are announced as a count and appear on request; a "pause live updates" control exists | WCAG 2.2.2; section 5.4 |
| IR-13 | Three densities; compact is the analyst default; sticky first column and header | Carbon; Cloudscape |
| IR-14 | Pagination is by keyset; virtualised lists expose row counts to assistive technology | Section 5.4; file 32 section 18.2 |
| IR-15 | Saved views are per user and shareable; role defaults ship; "My inbox" collects everything assigned to the user | Marble; Cloudscape |
| IR-16 | **Queue-run mode**: "Next" opens the next item in the current view; a key jumps to the oldest unassigned item of the user's pool; leaving never discards a draft | Marble; section 3.3 |

### 11.3 Case work

| ID | Rule | Why |
|---|---|---|
| IR-20 | A case opens on one screen: summary strip, why raised, timeline and evidence panels expand in place; the decision panel stays visible while scrolling | Section 3.3 |
| IR-21 | The header shows: priority class, clocks (age and the next due), tier (a lock icon with a word for restricted and suspicion tier), number of merged alerts, current stage | Mockup 02 gap |
| IR-22 | Evidence the class requires must be opened before decision actions enable (P0 and P1 by default); the panel shows which were opened | File 28 M-21 |
| IR-23 | The assistant's output is a labelled draft with sentence-level sources; saving it requires an edit or an explicit accept | Section 7.6 |
| IR-24 | Notes are posted, not edited; "amend" creates a new note that references the old; drafts autosave and are not evidence | File 28 section 12.2 |
| IR-25 | A second person on the case sees a visible presence indicator only if on the case's access list | File 30 (presence reveals interest) |
| IR-26 | Concurrent edits use optimistic version checks; a conflict shows both versions and asks, never overwrites | DBS `If-Match` rule |
| IR-27 | "Add to evidence" captures structured data (window, filters, graph definition) as well as an image | File 28 section 12.3 |

### 11.4 Decisions

| ID | Rule | Why |
|---|---|---|
| IR-30 | Decisions and actions are separate controls: **actions** (request information, escalate, create operations task, start watch) do not end the case; **decisions** (propose closure, recommend UTR or STR, MLRO decision) do | Mockup 02 mixes them |
| IR-31 | Every decision needs coded reasons plus text; a generic-text check blocks phrases such as "no suspicion" on their own; capacity-driven wording is refused | File 28 section 6.2 |
| IR-32 | The CBI escalation checklist (seven criteria) sits beside the decision and is pre-filled where computable | File 28 section 6.3 |
| IR-33 | Signing-grade actions (file a report, close without a report at MLRO level, clear a hold, approve a change, export) require a single-use step-up; the dialog keeps screen state and allows paste and WebAuthn | File 30 section 6.3; WCAG 3.3.8 |
| IR-34 | Four-eyes is explained at the point of action in terms of the case's class ("this closure needs approval by a level-2 analyst"), not as a blanket line | File 28 section 5.4 |
| IR-35 | Destructive or irreversible actions show consequences and the target in bold; friction matches severity, no extra friction for low-severity actions | Cloudscape |
| IR-36 | A screening decision uses the Office's three states (True, Potential, Incorrect match) as labels with the consequence beneath; pair memory is offered only on **Incorrect match** and shows its expiry and invalidation rule | Section 7.4 |

### 11.5 Bulk actions

| ID | Rule | Why |
|---|---|---|
| IR-40 | Selecting rows reveals a batch action bar with the count; selection is cleared by sort, filter or page change, as in Cloudscape's note that "selection affects only visible rows" | Carbon; Cloudscape |
| IR-41 | Bulk actions are limited to **assign, tag, snooze, request information, mark duplicate**; **no bulk closure** of P0 or P1 items; bulk closure of alerts is allowed only inside an already decided case (inherited outcome) | Coinbase and Block (file 20) |
| IR-42 | A batch shows a preview list of what will change, requires one reason code, is capped (default 25 items) and writes one event per item | File 28 M-18 (an event per item), M-40 (bulk reopen) |
| IR-43 | Reversible bulk actions offer undo for a short window; irreversible ones do not appear in the bar | Error prevention |

### 11.6 Forms, input and language

| ID | Rule | Why |
|---|---|---|
| IR-50 | Every number box and every search box accepts Western, Arabic-Indic and Persian digits and shows the understood value | DBS `westernDigits()` |
| IR-51 | No native date or time inputs; typed dates accept `20261007`, `2026-10-7` and `7/10/2026` only after an explicit order is shown; unreadable values stay in the field with the fault displayed | DBS DR-67 |
| IR-52 | Hold i18n **keys** in state, not translated strings; counted phrases use six-form Arabic plural rules | DBS; section 4.1 |
| IR-53 | Validation errors are translated by rule and shown under the field with an icon and text, never toast-only | DBS |
| IR-54 | Never print a machine ID at a reader; every enum and step has a bilingual label from the API | DBS DR-63 |
| IR-55 | Copy buttons copy raw values without bidi isolates; paste into search strips control characters | Section 4.2 |
| IR-56 | Free text is scanned for card numbers and secrets before saving (the sensitive-text guard) and for forbidden words where the text can leave the compliance zone | File 30 section 11.5; file 28 section 9.3 |

### 11.7 Time, clocks and notifications

| ID | Rule | Why |
|---|---|---|
| IR-60 | Dates are `YYYY-MM-DD HH:MM` in local time with isolates; a clock shows its kind (age, stage, decision-to-filing, RFI, Committee) and its target, never a bare "late" | File 28 section 5.6 |
| IR-61 | "Awaiting information" pauses the handling clock only; the elapsed age keeps running and is shown | File 28 section 5.6 |
| IR-62 | The idle time-out shows a countdown warning (default 60 s) with an "extend" action; drafts survive | WCAG 2.2.1; file 30 |
| IR-63 | Notifications are generic ("you have a task"), in-app and the bank's mail only, never name, amount or case type; no sound; a digest option | File 30 section 8.2 |
| IR-64 | The badge counts items that need action, not items that exist | "Exceptions are loud, health is silent" |

### 11.8 Errors, empty states and trust

| ID | Rule | Why |
|---|---|---|
| IR-70 | A refused request never looks like an empty list; suspicion-tier refusals look exactly like "not found" | DBS; file 30 SP-27 |
| IR-71 | Every error says what happened and what to do and shows the correlation ID that joins it to the audit row | DBS |
| IR-72 | Every list has a designed empty state with one sentence and one action | DBS |
| IR-73 | A long job (pack generation, backtest) shows progress and continues in the background; beyond about 10 s a percent-done indicator appears | Nielsen limits |
| IR-74 | When data are degraded or a source is dark, the screen says so beside the figure it affects | File 20 FM-01, FM-02 |

### 11.9 Performance and weight (budgets)

| ID | Budget | Why |
|---|---|---|
| IR-80 | Queue and case navigation first content within 1 s on the bank LAN at 50 concurrent analysts; evidence panels within 300 ms from cache or a skeleton; type-ahead search p95 within 300 ms; interactions that must feel instant respond within 100 ms | Nielsen's 0.1 s, 1 s and 10 s limits |
| IR-81 | The branch window loads in about 300 KB of compressed script and is usable on a slow link (target: first content within 5 s at 2 Mbps and 300 ms latency); no graph or chart library in its bundle | Section 2.3; link quality NOT VERIFIED |
| IR-82 | Graphs render 100 nodes within 2 s; above about 15 accounts grouping is on by default with drill-down | Esteves et al. |
| IR-83 | Fonts, libraries and icons are bundled; no runtime fetch (egress rule) | File 32; file 30 |

### 11.10 Keyboard

| ID | Rule | Why |
|---|---|---|
| IR-90 | Everything is reachable by keyboard in a logical order that is the Arabic reading order; focus is always visible and never hidden by sticky elements | WCAG 2.1.1, 2.4.3, 2.4.11 |
| IR-91 | Letter and digit shortcuts bind to **physical key codes** and are labelled with the Latin keycap; they can be turned off and remapped with Ctrl or Alt | MDN `KeyboardEvent.code`; WCAG 2.1.4 |
| IR-92 | A cheat sheet opens on `?`; shortcuts never trigger inside text fields | Section 6.3 |
| IR-93 | Dragging has a keyboard and button alternative; targets are at least 24 by 24 CSS pixels | WCAG 2.5.7, 2.5.8 |

### AKR-TMS hooks (section 11)

- **Interaction-rule checklist** attached to every pull request that adds a screen (IR-01 to IR-93), with automated checks where possible (logical properties only, no native date input, no `toLocaleString`, contrast tokens, shortcut layer on key codes).
- **Budget tests** in CI against a synthetic data set and 50 concurrent sessions.

## 12. What the eight mockups need

The mockups are good explanatory pictures and are not wrong as such; they are English left-to-right images for bank leadership. Where they would mislead a build, or where this research finds a conflict with a rule, the change is listed. "Keep" items are things the mockups already get right and the build should preserve.

### 12.1 Changes that apply to all eight

| # | Change | Why |
|---|---|---|
| 1 | **Mirror for Arabic**: the rail on the right, columns and icons mirrored, charts' time axes per section 4.2, numbers and identifiers isolated, Arabic type set in IBM Plex Sans Arabic with raised leading; the English mockups remain the English version | Identity: "designed right-to-left and mirrored for English" |
| 2 | **Role-aware rail** instead of one fixed list of nine; add My work, Branch window (separate bundle), RFI desk, QA, Assessor workspace, Training, Administration; merge "Alerts" and "Cases" into one queue entry with saved views | Sections 2.1 and 10.11 |
| 3 | **Tier badge**: a lock icon and a word ("Restricted case", "Restricted customer") in the header and the browser title of any restricted or suspicion-tier record; **no** such badge visible to users outside the list | File 30 section 8.3 |
| 4 | **Contrast tokens** (section 5.3): on-tint text colours for amber and green, teal text `#177272`, control borders at 3:1, no gold or amber fill as text or boundary; keep the sampled darker pill colours the mockups already use | Measured contrast |
| 5 | **Dates**: the product writes `YYYY-MM-DD HH:MM` in tables and clocks (the DBS convention); the mockups' "06 Oct 2026, 14:32" is acceptable in prose and in the plan | One convention per screen |
| 6 | **Uppercase micro-labels** (11 to 12 px) are an English-only style; in Arabic use sentence case, weight 600, 12 px or more | Section 5.3 |
| 7 | **Names as text, not images**: the Arabic names in boxes are copyable, searchable text with `lang`, original first and transliteration second | Section 4.3 |
| 8 | Header: add a language toggle, the help entry in a fixed place, and the session countdown when it is near; keep search and the bell; the user chip shows the role | WCAG 3.2.6; section 5.2 |
| 9 | Show **focus states** and keyboard hints in the design system (not in the pictures) | WCAG 2.4.7, 2.4.11 |

### 12.2 Mockup by mockup

| Mockup | Keep | Change | Add |
|---|---|---|---|
| **01 Command centre** | KPI tiles with a sub-line that explains the figure; "212 older than 30 days" beside the count; backlog by age with values inside the segments; regulatory deadlines with colour and a word; data-as-of stamp; correspondent evidence pack with a generate button | Open alerts: show **cases as well as alerts** ("1,284 alerts in N cases") and compute ageing on elapsed age; stacked-bar segments separated by a gap and labelled directly (blue, teal, amber, red are 1.9 to 3.1:1 against each other); "Top scenarios": define "productive" and add cases, UTRs and STRs; "STRs filed" tile adds UTRs; every tile opens its list; "Branch liaison monthly reports 10 Oct" shows how many branches have submitted | **Decisions waiting for me**; overdue and at-risk clocks; arrivals against throughput and capacity ratio; verification failures; QA status; data and coverage incidents; tipping-off summary (section 8.3) |
| **02 Case workspace** | Summary strip; "Why this case was raised" with chips and CBI scenario rows; dashed declared-turnover line; counterparties table; draft badge on the summary; "A second reviewer must approve any closure" as an honest control note | **Decision card**: separate **actions** (Request information from branch, escalate to level 2, create operations task, start watch) from **decisions** (propose closure, recommend UTR, recommend STR; for the MLRO: STR, UTR, TFR, shelve with reasons, more evidence); add coded reasons and the seven-criterion checklist; replace the blanket four-eyes line by the class-specific one; **summary**: sentence-level sources and a "not considered" line instead of one "view sources" link; header strip adds priority class, clocks and tier; "Activity" becomes a filterable **timeline**; "Due in 4 days" names the clock | Evidence panels (documents, RFIs and answers, earlier cases, screening, adverse media); merged-alert count; seasonal band on the chart; sticky decision panel; evidence-opened indicator |
| **03 Screening review** | Side-by-side layout; Arabic name boxes with transliteration; per-part table with result words and weights; the history panel; the freeze-task note under the decision | Replace "91 match score" ring (amber on white 2.1:1) by a **verdict sentence** plus score and evidence level and the applied threshold; identifier grid must include **mother's name and maternal grandfather**, document numbers, place of birth, family number, each marked agree, differ or not comparable (birth year "differs by 2 years" is shown, mother's name is absent); the per-part table shows the **Arabic** parts first and lists folds and unmatched tokens; decision tiles keep "True match" and change "Possible match" to **"Potential match"** and "Not the same person" to **"Incorrect match"** (the Office's terms, section 7.4), each with a consequence line (freeze clock and 48-hour Committee report; verification due; reason for exclusion recorded); **"Remember this decision for this pair"** appears only on Incorrect match, unticked by default, showing expiry; add a four-eyes notice for freeze lists and a coded reason | Frequency context sentence; list-entry aliases and listing basis; link to the source entry; pair-memory history |
| **04 Customer risk profile** | Score drivers as sorted bars with points and reasons; EDD checklist with dates; ownership tree with the 20 percent note; vault with lock icon plus the word "Locked" and the immutability note | Gauge is decorative: keep the number, the level word and the 12-month trend; mark the colour dots in the checklist with words (they have icons and text, which is right) | Missing-input share and factor refresh dates; rating change reasons; open cases and watches strip |
| **05 Scenario studio** | Parameters with old value struck and new shown; plain-language description "written automatically"; test chart with values; change request panel with approver; version history | The change cuts alerts from 412 to 298 and STR-or-EDD from 31 to 30: that is a **volume-reducing change**. The mockup already shows the second-line review ("Passed") and a below-the-line sample ("1 missed of 200 reviewed"), which is right; what is missing is the **reason code** (risk-driven or capacity-driven), and the example reason "Too many alerts on normal multi-branch use" reads as capacity-driven, which the rules treat as a flagged change (file 28 M-55; file 20 L-18), so the example should be reworded to a risk reason ("licensed cash-in-transit companies are expected multi-branch depositors") or carry the capacity flag; input borders to 3:1 | Segment tabs' counts; effective date; the regime of changes that bypass a second approver (none) |
| **06 Network view** | Ego graph with ring, senders, exchange companies; cluster summary; links found in words; "Open one case for the cluster" and "Add graph to evidence" with the snapshot note; legend | Replace "Acct 1" labels by masked account number and Arabic customer name (toggle); add grouping control and one-click expansion for the 214 senders; the red tint needs an outline or label (it has one) | Node and link table alternative; keyboard traversal; text summary |
| **07 Report builder** | Stepper with validation; indicators as chips; narrative with the "DRAFT BY ASSISTANT · EDITED BY ANALYST" badge; validation list with a "Fix" link; timeline | Add a **report type selector** (UTR, STR, TFR) and the fields for **origin of suspicion** and **earlier reports**; the stepper adds step 7 **Filed and acknowledged** (goAML reference, time, acknowledgement); "Download goAML XML" stays and "Record filing reference" is added; narrative: sentence-level sources and a tipping-off phrase check; the "STR quality check" mirrors the Office's grading criteria (indicator present, profile mismatch quantified, documents attached, actions recorded) | Clock "decision to filing"; amendments and version comparison; FIU feedback strip after filing |
| **08 Data connection** | Read-only tile with the check time; replica delay; reconciliation table with "Match" words and check icons; data-quality list with severity and counts; field-mapping coverage | Severity dots must carry words (red, amber, grey); the feed chart needs axis units, a table alternative and a sentence | Bank kill-switch state; quarantine count and age; dark units; coverage certificate link |

### 12.3 What should not change

The calm, dense, white-card-on-clay look; the single primary button per region; reasons beside numbers; the "draft, requires review" discipline; the four-eyes and freeze-task notes; the immutability note on the vault; the "snapshot kept with the case" note. These are the product's credibility cues and match the rules of file 28.

### AKR-TMS hooks (section 12)

- **Mockup change list as design tickets** (UX-55 to UX-63), each with an acceptance check against section 12.2; the revised mockups for the plan PDF keep the English left-to-right form and add one Arabic right-to-left mirror of mockups 02 and 03 to show the Arabic-first design.
- **A design-system token file** carrying the measured contrast results (section 5.3) so that the mockups and the build use the same colours.

## 13. What it means for AKR-TMS

1. **The users who decide success are the liaison officer, the MLRO and the assessor, not only the analyst.** The liaison officer is the simplest and most numerous user and the one whose screen can cause tipping-off; the MLRO carries accountability; the assessor decides whether the bank passes D2 on the strength of a first session. The design starts from those three (sections 2, 8.5, 10.9).
2. **The evidence says: merge repeats, show data first, do not oversell scores.** Alert fatigue tracks repetition and low-information alerts, not workload (Ancker); a model score speeds decisions and can lower accuracy, and explanations raise acceptance of wrong suggestions (Jesus, Bansal); more detail can overload (Poursabzi-Sangdeh); making the human decide first reduces overreliance and is liked less (Buçinca). Hence one case per customer, an evidence-first page, three headline reasons, and cognitive forcing only for P0 and P1 (section 3).
3. **Keyboard shortcuts are cheap but small.** By the keystroke-level model they save seconds per case; the large levers are fewer repeats, no lost context and not redoing decisions. They must still exist, bind to physical keys so Arabic-layout typists get them, and be switchable (WCAG 2.1.4).
4. **Arabic-first is a technical programme, not a translation.** Measured hazards: an Arabic sentence reorders phone numbers, dates and ranges unless the whole expression is isolated; digits are a three-way decision (CLDR and W3C say Eastern for Iraq, Iraqi regulators write Western in 15 of 16 documents, Arabic-Indic digits in the vendored font are proportional and break columns); Hijri dates differ by up to two days across calendar variants; code-point sorting scatters alef forms and sends every Kurdish letter to the end; Arabic has six plural forms; Iraqi month names differ from the Gulf ones. The DBS i18n kit solves most of this and is reused, with Kurdish folds, bidi-control scanning, plurals and a Hijri formatter added (section 4).
5. **Names are the interface's hardest data.** Arabic original first, transliteration second with its source, an aligned chain with role labels, the mother's name beside the birth year, no truncation in tables, no "first name" fragments (section 4.3). One measured gap: the vendored Latin font subsets lack ş, ğ, İ and the transliteration marks; ship the `latin-ext` subsets. The Arabic subset already covers all ten Kurdish letters.
6. **The approved palette needs five new tokens before build.** Gold and the amber fill must never carry text or a boundary on light surfaces (2.4:1 and 2.1:1); amber text needs `#9C5D00` or darker (the token gives 3.7 to 4.2); green text on Clay needs `#2D794C` (4.44 fails); teal text must be `#177272`; input borders need a 3:1 token (`#7F8695`). The mockup images already use darker text colours than the tokens, so the risk is in the tokens, not the pictures (section 5.3).
7. **Two accessibility rules touch security flows directly.** The step-up dialog must allow paste of one-time codes and offer WebAuthn first (3.3.8); the 15-minute idle time-out needs a warning with an "extend" action and drafts that survive (2.2.1) (section 5.2).
8. **Help is in context, training is on synthetic cases, and both are bilingual.** NN/g finds onboarding tutorials ineffective and recommends help pulled at the moment of need; the CBI requires annual training, competency tests and records; so the product has contextual help, a training workspace with synthetic data and a register tied to queue access (section 6).
9. **The explanation rules follow the Iraqi texts, which use two vocabularies.** The Office (September 2026) writes true, potential and incorrect match (التطابق الحقيقي، التطابق المحتمل، التطابق غير الصحيح); the CBI's 2023 guidelines write potential, confirmed and false positive (تطابق محتمل، تطابق مؤكد، نتيجة إيجابية خاطئة) with a different doubt rule and a three-day report clock that the 48-hour rule of Regulation 6/2023 supersedes. The screen uses the Office's terms with the consequence beneath, keeps the CBI's as aliases, shows the mother's name and a frequency sentence, and offers pair memory only on an incorrect match (section 7).
10. **Vendors are thin on user-experience evidence and silent on Arabic.** No incumbent documents an Arabic interface, right-to-left layout or Hijri dates; reviewers praise case management and audit trails and criticise complexity, slowness, search errors, weak reporting and technical vocabulary. AKR-TMS's differentiators are Arabic-first design, plain vocabulary, fast search, first-class reports and in-case filing (section 9).
11. **There are 59 screens and 8 mockups.** The missing ones include the work queue, branch window, RFI desk, QA, assessor workspace and administration; the rail must be role-aware; the case decision card must separate actions from decisions (section 10, 12).

**Limits and open items.** (a) No independent measurement of minutes per AML alert exists in the sources reached. (b) Bank hardware, screen sizes and branch link quality are NOT VERIFIED. (c) Arabic screen-reader behaviour with this interface is NOT VERIFIED. (d) Whether Iraqi analysts prefer numeric ranges in reading order or left-to-right is NOT VERIFIED. (e) Chart direction in right-to-left (mirrored time axis) is NOT VERIFIED against a third-party standard; the RTL spike decides. (f) Review-site evidence for vendor interfaces is limited to PeerSpot summaries, one Software Advice page and vendor material; G2, Gartner Peer Insights and TrustRadius were not reached. (g) The Esteves study has three participants and the Jesus study three analysts; both are small. (h) The digit measurement uses text layers of 16 documents; the Office's CDD rules use Arabic-Indic digits, so practice is not uniform.

## 14. Derived requirements (UX-xx)

Priority: M = must have at go-live of the first release; S = should have within the first year; C = could have later.

| ID | Requirement | Source | Pri |
|---|---|---|---|
| UX-01 | Keep a role and scenario catalogue (section 2) with the usability tests derived from it; the rail and screens are role-aware | Section 2; DBS "never render a control that cannot work" | M |
| UX-02 | The branch window is a separate, light route bundle (about 300 KB compressed script, usable on a slow link), with neutral statuses and no outcomes | CBI ch. 11; file 28 section 13.2; IR-81 | M |
| UX-03 | Run small iterative usability rounds (three to five participants per distinct group) with Iraqi compliance staff, in Arabic first, with a mixed-script checklist, before and during the build | NN/g five-users guidance; section 6.5 | M |
| UX-04 | Instrument the analyst workspace (time in panel, panel switches, back navigations, notes, earlier alerts opened) for pilot measurement; no per-person league tables | Section 3.2; file 29 section 9j | M |
| UX-05 | Show merged alerts as one case row with the merged count and let the analyst open any underlying alert | Ancker et al. 2017; HKMA; file 28 section 3.3 | M |
| UX-06 | Case workspace on one page with in-place evidence panels and a sticky decision panel | Section 3.3; mockup 02 | M |
| UX-07 | Entity timeline for alerts, windows, observations, RFIs, authority requests, rating changes, decisions, tasks and QA events, filterable by type, with seasonal (Hijri) bands | Marble investigation panel; file 24; section 3.3 | M |
| UX-08 | Evidence-first alert page: raw rows, peer comparison and network before reasons; at most three headline reasons each beside its fact; mitigating factors; thresholds hidden from branch roles | Jesus et al.; Poursabzi-Sangdeh et al.; file 21 section 11 | M |
| UX-09 | Cognitive-forcing switches per priority class (evidence opened before decision buttons; optional assess-first before AI suggestions), with acceptance and override rates monitored | Buçinca et al.; file 29 section 9j; file 28 M-21, M-72 | M |
| UX-10 | Network view with automatic grouping above about 15 accounts, one-click expansion, group summaries, never grouping away the subject, and a node and link table | Esteves et al. 2026; mockup 06; file 32 section 18.2 | M |
| UX-11 | Queue-run mode and a one-key jump to the oldest unassigned item in the user's pool; leaving never discards a draft | Marble; section 3.3 | S |
| UX-12 | Shortcut layer on physical key codes, labelled with the Latin keycap, never active in text fields, remappable with Ctrl or Alt and switchable off; cheat sheet on `?` | MDN KeyboardEvent.code; WCAG 2.1.4 | M |
| UX-13 | Bulk actions limited to assign, tag, snooze, request information and mark duplicate; no bulk closure of P0 or P1; preview, one reason, a cap (default 25), an event per item | Carbon; Cloudscape; Coinbase and Block (file 20) | M |
| UX-14 | Saved views per user and shared; filters, sort and period in the address; role defaults; "My inbox" | Cloudscape; Marble; DBS reports | M |
| UX-15 | Three row densities, sticky header and first column; compact default for analysts | Carbon; Cloudscape | S |
| UX-16 | Response budgets (queue and case first content within 1 s at 50 concurrent analysts; type-ahead p95 300 ms; progress beyond about 10 s) tested in CI | Nielsen's limits (NN/g) | M |
| UX-17 | Draft safety: autosave, restore after time-out or step-up, one active editor with presence visible only to the case's access list | File 28 M-59; file 30 | M |
| UX-18 | Western digits in all data and tables in both languages; a per-bank and per-user setting may show Arabic-Indic digits in prose only; every input and search accepts Western, Arabic-Indic and Persian digits and shows the understood value | Measured: 16,197 against 944 digits in Iraqi regulator texts; CLDR; W3C ALREQ; Plex digit widths; DBS | M |
| UX-19 | Isolate whole numeric expressions (phone, date and time, range, amount with code, IBAN) with LRI and PDI inside the value, and names with a first-strong isolate; use `dir` and `bdi` in markup; copy buttons copy raw values; searches strip controls | Measured bidi results; W3C inline-bidi article; DBS `bidi.ts` | M |
| UX-20 | Scan incoming text for bidirectional controls and zero-width characters, remove them for matching and flag them as data quality | File 30 SP-70; file 22 section 13.11 | M |
| UX-21 | Dates as `YYYY-MM-DD HH:MM` in local time with isolates; Iraqi month names (`ar-IQ`) in report prose with a switch; no native date inputs and no `toLocaleString` | DBS DR-67, DR-70; CLDR | M |
| UX-22 | Hijri is display and entry only, labelled with the calendar variant, never used for clocks or comparisons; documents dated in Hijri keep the original text beside the Gregorian value; religious-season markers on timelines | Measured: variants differ by up to two days; file 24 | S |
| UX-23 | Show amounts with the ISO code as an isolate, decimals per the currency table, and "مليون" and "مليار" forms with correct plurals in KPI tiles | CLDR `ar-IQ` currency output; mockups | M |
| UX-24 | All counted phrases use ICU plural rules with six Arabic forms; hold i18n keys in state; no concatenated sentences | CLDR plural categories (measured); DBS | M |
| UX-25 | A shared name component: Arabic original first, transliteration with source label, aligned chain with role labels, mother's name beside birth year, no truncation in tables, no first and last name splitting | File 22 sections 2, 3, 13.7; DBS Whole-Name Rule | M |
| UX-26 | Ship IBM Plex Sans Arabic, Plex Sans and Plex Mono with the `latin-ext` subsets; a glyph-coverage test for every letter in file 22 (Kurdish, Turkish, transliteration marks) | Measured font coverage | M |
| UX-27 | Arabic typography rules: raised leading under RTL, no letter-spacing, no italics, size alignment checked on mixed lines, tabular Western digits end-aligned in tables with the currency code in its own column | DBS design system; W3C ALREQ 7.3, 7.4 | M |
| UX-28 | Text sorts through `Intl.Collator` (Sorani collation optional), sort the number and render the sentence, with a parity test between browser and server | Measured collation results; DBS | M |
| UX-29 | Search detects the kind of input (IBAN, card number, phone, case number, name), applies the extended fold (Kurdish, Persian, ZWNJ repair), highlights matches in the original text, validates Iraqi IBANs (23 characters, mod-97), and excludes suspicion-tier cases for non-members | File 22; file 32 section 18.2; IBAN registry; file 30 SP-27 | M |
| UX-30 | Mirroring rules: directional icons flip by one attribute, media controls and graph geometry do not, progress mirrors, chart time axes follow the result of the RTL spike | DBS; file 32 section 18.4 | M |
| UX-31 | Exports: PDF through the Arabic-capable Gotenberg image; XLSX with a right-to-left sheet view; CSV in UTF-8 with a byte-order mark, ISO dates, Western digits and formula-injection guards; watermark, audit entry and disclosure-register entry | File 32 section 18.2; file 30 SP-70, SP-52 | M |
| UX-32 | Target WCAG 2.2 AA in both themes; axe checks on every route, language and theme in CI; a conformance statement per release | W3C WCAG 2.2 (5 October 2023) | M |
| UX-33 | Define on-tint text tokens (amber `#9C5D00` or darker, green `#2D794C`, teal `#177272`), a control-border token at 3:1 or better (`#7F8695`), forbid gold and the amber fill as text or boundary, and run a contrast check over the token file in CI | Measured contrast (section 5.3) | M |
| UX-34 | Status never by colour alone; stacked bars separated by a gap and labelled directly; legends use words or patterns | WCAG 1.4.1, 1.4.11; mockup 01 | M |
| UX-35 | Idle time-out shows a countdown warning (default 60 s) with an extend action; drafts survive | WCAG 2.2.1 Understanding; file 30 section 6.2 | M |
| UX-36 | The step-up dialog allows paste and autofill of one-time codes (`autocomplete="one-time-code"`), offers WebAuthn first, and preserves screen state | WCAG 3.3.8 Understanding; file 30 section 6.3 | M |
| UX-37 | Live updates are announced in aggregate, a pause control exists, and lists never reorder under the cursor | WCAG 2.2.2 | M |
| UX-38 | Tables expose row counts and indexes when virtualised; graphs and charts have table alternatives and text summaries; sticky elements never obscure focus; targets are at least 24 by 24 CSS pixels; drags have alternatives | WCAG 2.4.11, 2.5.7, 2.5.8, 4.1.2 | M |
| UX-39 | Accessibility testing: keyboard-only walkthroughs of six tasks, 400 percent zoom in Arabic, screen-reader passes (NVDA, VoiceOver) including mixed-script names; Arabic screen-reader behaviour is a named test item | Section 5.5 | S |
| UX-40 | Contextual pull help, a bilingual glossary owned by compliance staff, reason-code explanations, procedures in context, an offline help centre, "what changed" notices, one fixed help location, no policy chatbot | NN/g onboarding article; WCAG 3.2.6; file 32 section 18.3 | M |
| UX-41 | A training workspace with synthetic data and a permanent TRAINING mark, role-based paths, practice cases, a tipping-off module, competency tests, a training register with coverage reports, linked to queue access | CBI ch. 2 art. 4, ch. 4, circular 26 Aug 2026; Office UTR guide s. 9; file 30 SP-06; file 28 M-50 | M |
| UX-42 | Explanation layers L0 to L3; three headline reasons each beside its fact; mitigating factors; model attributions labelled "model contributions"; disclosure by role; the explanation stored with the alert | File 21 section 11; file 29 section 9j; section 7 | M |
| UX-43 | The screening hit screen follows the order of section 7.4: verdict sentence with evidence level and applied threshold, records side by side, deciding identifiers (mother's name and maternal grandfather, birth precision, documents), Arabic-first per-part alignment with folds and unmatched tokens, frequency sentence, list-entry facts, pair memory, cross-script note | File 22 sections 13.6, 13.7; CBI sanctions guidelines | M |
| UX-44 | Match states in the Office's words in Arabic and English (التطابق الحقيقي، التطابق المحتمل، التطابق غير الصحيح) with the CBI's terms as aliases and consequence lines (freeze clock, 48-hour Committee report); a per-bank doubt policy; pair memory offered only on an incorrect match, unticked, with expiry shown; coded reason required | Office TFS guide art. 12 (Sept 2026); Regulation 6/2023; CBI sanctions guidelines 2023 p. 21; file 23 section 3.4; file 22 section 13.8 | M |
| UX-45 | AI block: draft label states, sentence-level sources, removal or marking of unsupported sentences, a "not considered" line, recommendations only after evidence is opened, no accept-all, a "useful or not" control, provenance one click away, names and numbers copied | File 29 section 9k; file 28 sections 14.2, 14.3; Marble AI review | M |
| UX-46 | An explanation usability protocol (time to first relevant evidence, seeded-case accuracy, usefulness, with and without) run per release in Arabic with Iraqi analysts | File 21 section 11d; section 7.7 | S |
| UX-47 | Dashboards follow attention, then period, then detail; every count opens its list; every figure has a definition, a formula and a data-as-of time; no tile that cannot be proven (coverage links to reconciliation) | DBS reports rule; file 20; section 8.2 | M |
| UX-48 | Command-centre additions: decisions waiting, overdue and at-risk clocks, arrivals against throughput and capacity ratio, verification failures, QA status, data incidents, tipping-off summary; the scenario table shows cases, UTRs and STRs with a defined "productive" | Section 8.3; file 28 | M |
| UX-49 | A one-page board view from system facts (ten blocks of section 8.4) as PDF and read-only page, with no case-level content and manual edits shown | CBI ch. 3 MLRO duty (k); D2.1(a)(2); file 20 L-56 | M |
| UX-50 | A D2 evidence matrix page and assessor workspace with random sample draw, end-to-end trace, chain verifier and pack download under a named, time-boxed, read-only account | CBI Detailed Assessment Guide D2.1 assessment process; file 28 section 12.6 | M |
| UX-51 | Built-in regulatory and periodic reports with scheduling, watermark and Arabic and English renderings | CBI ch. 15 and ch. 11; section 8.6 | M |
| UX-52 | A competitive usability benchmark on a vendor demo where obtainable, and performance tests under concurrent users, to replace thin review evidence | Section 9 | C |
| UX-53 | Keep the screen catalogue of section 10 machine-readable (ID, roles, route, affordance rules) and drive the role-aware rail, the access matrix and a route-to-policy coverage test from it | File 30 SP-25, SP-71 | M |
| UX-54 | Attach the interaction-rule checklist IR-01 to IR-93 to every pull request that adds a screen, with automated checks for logical properties, native date inputs, `toLocaleString`, contrast tokens and the shortcut layer | Section 11; DBS rules | M |
| UX-55 | Apply the changes common to all eight mockups (mirroring, role-aware rail, tier badge, tokens, product date convention, English-only micro-labels, names as text, header controls, focus states) | Section 12.1 | M |
| UX-56 | Mockup 01: cases beside alerts, ageing on elapsed age, labelled stacked bars, defined "productive", UTRs in the STR tile, clickable tiles, plus the added blocks | Section 12.2 | M |
| UX-57 | Mockup 02: separate actions from decisions; coded reasons and the seven-criterion checklist; class-specific four-eyes line; sentence-level sources; header with class, clocks and tier; filterable timeline; evidence panels | Section 12.2; IR-30 | M |
| UX-58 | Mockup 03: verdict sentence instead of a bare score ring; mother's name and other deciding identifiers; Arabic-first per-part table; the Office's states ("Potential", "Incorrect match") with consequences; pair memory only on incorrect match, unticked | Section 12.2; UX-43, UX-44 | M |
| UX-59 | Mockup 04: missing-input share and refresh dates, rating-change reasons, open cases strip | Section 12.2; file 20 L-10, L-44 | S |
| UX-60 | Mockup 05: a reason code (risk-driven or capacity-driven) on volume-reducing changes and a risk-based example reason | Section 12.2; file 28 M-55; file 20 L-18 | M |
| UX-61 | Mockup 06: masked account numbers and Arabic names, grouping control, table alternative, keyboard traversal | Section 12.2; UX-10 | M |
| UX-62 | Mockup 07: report type selector (UTR, STR, TFR), origin of suspicion and earlier-report fields, a seventh "Filed and acknowledged" step, record-filing-reference action, sentence-level sources, tipping-off phrase check, decision-to-filing clock | Section 12.2; file 28 sections 2.2, 2.3, 6.6 | M |
| UX-63 | Mockup 08: words with severity dots, chart units and table alternative, kill-switch state, quarantine and dark units | Section 12.2; file 25 | S |
| UX-64 | Separate **actions** (do not end the case) from **decisions** (end or propose to end the case) throughout the case UI | Section 11.4 IR-30; mockup 02 | M |
| UX-65 | Decision forms require coded reasons and refuse generic or capacity-driven text, with the generic-text check shown to the user | File 28 section 6.2; IR-31 | M |
| UX-66 | Branch window flows: items with neutral statuses, branch input form (explanation as a claim, documents on file, visit result), monthly report auto-draft and signature | CBI ch. 11; file 28 sections 4.3, 13.2 | M |
| UX-67 | An observation form listing the CBI behavioural-indicator groups as ticks with notes, routed to the MLRO, with the written acknowledgment shown at once | CBI circular 26 Aug 2026; CBI ch. 2 art. 3 | M |
| UX-68 | RFI desk UI: six kinds, template library, pre-filled Wolfsberg answers, clock, four-eyes release, forbidden-word scan | File 28 section 9; file 04 section 2p | M |
| UX-69 | A minimal operations-task inbox that shows only the neutral instruction and reference | File 28 section 7.3; file 30 SP-31 | M |
| UX-70 | Capacity workbench and QA workspace screens as specified in file 28 sections 10 and 11 | File 28 | S |
| UX-71 | Vendor lessons applied: fast, kind-aware search; role-based simplicity; first-class reporting; plain Arabic vocabulary; in-case filing | Section 9.3 | M |

**Count:** 71 requirements (UX-01 to UX-71): 62 must have, 8 should have, 1 could have. Cross-referenced and not repeated: file 22 N-xx (matching and display), file 28 M-xx (workflow), file 30 SP-xx (access, tipping-off, step-up), file 32 TS-xx (frontend).

## 15. Sources

All read on 2026-10-07 unless marked. "Local" means a file on this machine; "measured" means the number was produced for this file on that date.

**Project material (local)**
- Visual identity: `/home/haithem/Projects/AKR-TMS/plan/identity.md`; outline: `plan/outline.md`; plan text: `plan/plan.md` (investigation, reporting, governance sections)
- Approved mockups viewed one by one: `plan/images/final/01-command-centre.png` to `08-data-connection.png`; prompts: `plan/images/prompts/*.txt`
- Vendored fonts: `plan/assets/fonts/` (IBM Plex Sans Arabic subset, Plex Sans Latin subsets, Plex Mono)
- Digital Banking System (read only): `.claude/rules/frontend.md`, `docs/DESIGN-SYSTEM.md`, `DESIGN.md`, `PRODUCT.md`, `docs/DSL.md`, `frontend/src/i18n/{direction,bidi,datetime,fold}.ts`
- Research files 01, 04, 05, 06, 07, 08-16, 18, 20, 21, 22, 24, 26, 28, 29, 30, 32 (cross-references in the text)

**Accessibility, internationalisation and interface standards**
- W3C, What's New in WCAG 2.2 (nine new criteria; Recommendation of 5 October 2023): https://www.w3.org/WAI/standards-guidelines/wcag/new-in-22/
- W3C Understanding 3.3.8 Accessible Authentication (Minimum): https://www.w3.org/WAI/WCAG22/Understanding/accessible-authentication-minimum.html
- W3C Understanding 2.1.4 Character Key Shortcuts: https://www.w3.org/WAI/WCAG22/Understanding/character-key-shortcuts.html
- W3C Understanding 2.2.1 Timing Adjustable: https://www.w3.org/WAI/WCAG22/Understanding/timing-adjustable.html
- W3C Internationalization, Inline markup and bidirectional text in HTML: https://www.w3.org/International/articles/inline-bidi-markup/
- W3C Arabic Layout Requirements (sections 4.2.8, 6.1.2, 7.3, 7.4): https://www.w3.org/TR/alreq/
- MDN, `KeyboardEvent.code`: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
- Wikipedia, International Bank Account Number (Iraq row; mod-97 check): https://en.wikipedia.org/wiki/International_Bank_Account_Number
- Wikipedia, Keystroke-level model (operator times): https://en.wikipedia.org/wiki/Keystroke-level_model

**Usability and design-system guidance**
- Nielsen Norman Group, Response Times: The 3 Important Limits: https://www.nngroup.com/articles/response-times-3-important-limits/
- Nielsen Norman Group, Onboarding Tutorials vs. Contextual Help: https://www.nngroup.com/articles/onboarding-tutorials/
- Nielsen Norman Group, Why You Only Need to Test with 5 Users: https://www.nngroup.com/articles/why-you-only-need-to-test-with-5-users/
- Nielsen Norman Group, 10 Usability Heuristics: https://www.nngroup.com/articles/ten-usability-heuristics/
- AWS Cloudscape, Delete with additional confirmation: https://cloudscape.design/patterns/resource-management/delete/delete-with-additional-confirmation/ ; Table view: https://cloudscape.design/patterns/resource-management/view/table-view/
- IBM Carbon, Data table usage: https://carbondesignsystem.com/components/data-table/usage/

**Studies with analysts and on alert fatigue**
- Jesus, Belém, Balayan, Bento, Saleiro, Bizarro, Gama, "How can I choose an explainer? An Application-grounded Evaluation of Post-hoc Explanations", FAccT 2021: https://arxiv.org/abs/2101.08758 (results table read from the PDF)
- Buçinca, Malaya, Gajos, "To Trust or to Think: Cognitive Forcing Functions Can Reduce Overreliance on AI in AI-assisted Decision-making", CSCW 2021: https://arxiv.org/abs/2102.09692 (intervention definitions and Table 2 read from the PDF)
- Poursabzi-Sangdeh, Goldstein, Hofman, Wortman Vaughan, Wallach, "Manipulating and Measuring Model Interpretability", CHI 2021: https://arxiv.org/abs/1802.07810
- Bansal, Wu, Zhou, Fok, Nushi, Kamar, Ribeiro, Weld, "Does the Whole Exceed its Parts? The Effect of AI Explanations on Complementary Team Performance", CHI 2021: https://arxiv.org/abs/2006.14779
- Esteves, Costa, Fallon, Bizarro, "The Balance between Nuance and Clarity: Decluttering Tabular Sequential Graphs to Counter Money Laundering" (2026): https://arxiv.org/abs/2605.10522
- Uetz, Bönninghausen, Hackländer-Jansen, Henze, "Can Risk-Based Alerting Mitigate Cybersecurity Alert Fatigue?" (2026): https://arxiv.org/abs/2609.02465
- Alahmadi, Axon, Martinovic, "99% False Positives: A Qualitative Study of SOC Analysts' Perspectives on Security Alarms", USENIX Security 2022: https://www.usenix.org/conference/usenixsecurity22/presentation/alahmadi
- Ancker, Edwards, Nosal, Hauser, Mauer, Kaushal, "Effects of workload, work complexity, and repeated alerts on alert fatigue in a clinical decision support system", BMC Medical Informatics and Decision Making 17:36 (2017): https://doi.org/10.1186/s12911-017-0430-8 (abstract via Europe PMC, PMC5387195)
- AML modelling papers searched for time per alert and found to state none: Jensen and Iosifidis, "Fighting Money Laundering with Statistics and Machine Learning": https://arxiv.org/abs/2201.04207 ; Eddin and colleagues (Feedzai), "Anti-Money Laundering Alert Optimization Using Machine Learning with Graphs": https://arxiv.org/abs/2112.07508

**Vendor and product interface evidence**
- PeerSpot, NICE Actimize reviews (summary read; 4.1 of 5 from 18 reviews): https://www.peerspot.com/products/nice-actimize-reviews ; SAS Anti-Money Laundering (no reviews): https://www.peerspot.com/products/sas-anti-money-laundering-reviews
- Marble documentation (open source case manager): https://docs.checkmarble.com/docs/overview , https://docs.checkmarble.com/docs/ai-review , https://docs.checkmarble.com/docs/blocking-review , https://docs.checkmarble.com/docs/reviewing-a-sanction-check , https://docs.checkmarble.com/docs/rule-snoozes , https://docs.checkmarble.com/docs/auto-case-assignment ; index https://docs.checkmarble.com/llms.txt
- Vendor statements as recorded in files 08 to 16 (each with its own source list)

**Iraqi regulator texts used**
- CBI sanctions guidelines, 1 August 2023, page 21 (potential, confirmed and false-positive matches; freezing measures; Arabic): https://cbi.iq/static/uploads/up/file-169554063119385.pdf
- AML/CFT Office, Guidance on the Prompt Implementation of Targeted Financial Sanctions, uploaded 10 September 2026, art. 12 (true, potential and incorrect match; Arabic; local `rp/aml/guide-tfs-prompt`): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/e6fde006-431a-4fc7-8c9b-fdd61b99cd12_20260910121330.pdf ; Terrorists' Funds Freezing Regulation No. 6 of 2023 (as read in file 23 section 3.4): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/24d01159-7ec5-4274-bb22-cbc75a129214_20260810234152.pdf
- CBI Detailed Assessment Guide 2025, D2.1: https://cbi.iq/static/uploads/up/file-177615833416828.pdf
- CBI AML/CFT/CPF controls 2025: https://cbi.iq/static/uploads/up/file-173666439114818.pdf ; CBI circular of 26 August 2026: https://cbi.iq/static/uploads/up/file-178963291760044.pdf
- AML/CFT Office, UTR guide (first edition 2026): https://aml-iq.fra1.cdn.digitaloceanspaces.com/RegulatoryDocument/40c281c6-d5b7-480d-919d-2a88c454b2e3_20260811093728.pdf
- Digit counts were made on the text layers of these documents and of the CBI handbook 2025, Phase 1 guide, reform circular, cyber-resilience controls 2024, risk-management controls 2026, digital-bank controls 2026, the Office's STR, banking-indicator and CDD guides and Law 39 (local copies)

**Measurements made for this file (2026-10-07)**
- Digit usage in 16 Iraqi regulator and Office texts: 16,197 Western digit characters, 944 Arabic-Indic, 3 Persian
- Unicode CLDR behaviour through Node 22.23.1 ICU: default numbering systems, calendars and week data (`ar-IQ`, `ar`, `ar-SA`, `ar-EG`, `ar-AE`, `ckb-IQ`, `en-IQ`), month names, currency and plural categories, collation of 32 invented names (code-point order, `Intl.Collator('ar')`, `Intl.Collator('ckb')`, sensitivity levels), Hijri variants for 7 October 2026
- Bidirectional reordering with two independent implementations (Rust `unicode-bidi` crate 0.3 and Python `python-bidi`): https://crates.io/crates/unicode-bidi
- Font glyph coverage and digit widths of the vendored IBM Plex subsets through fontTools
- WCAG contrast ratios of the identity palette and of colours sampled from the eight mockup images (Pillow); recommended replacement tokens computed in the same way
- Keystroke-level arithmetic for a mouse and a keyboard disposition (section 3.2)

**Not reached, or reached only in part**
- G2, Gartner Peer Insights, TrustRadius and Capterra reviews of AML tools (blocked; no Wayback copy for the pages tried)
- The Material Design bidirectionality guidance and Apple's right-to-left guidance (script-rendered pages returned no content), so chart direction in right-to-left rests on the DBS convention and the planned RTL spike
- An independent measurement of minutes per AML alert or of liaison-officer review times (none found)
- Screen sizes, hardware and branch link quality in Iraqi banks (not published)
- Arabic screen-reader behaviour with a mixed-script interface (not tested)
- ISO 9241 and EN 301 549 texts (paywalled or not read); WCAG 2.2 is the working standard
