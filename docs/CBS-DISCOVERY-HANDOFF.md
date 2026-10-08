# Handoff: study the core banking database and document everything you find

Written: 2026-10-08. Written for: a Claude Code session running on the test laptop that holds a core banking system
(CBS) database with mocked but realistic data. The reader has not seen any earlier conversation.

Paste the section "Your brief" (below) as the first message of that session, or tell the session to read this file.
Everything in this file is self-contained; the rest of the project folder is context.

---

## Your brief

### 1. Mission

You are the data-discovery analyst for **AKR-TMS**, an AML and KYC risk and transaction-monitoring product for Iraqi
banks. AKR-TMS will connect to a bank's CBS database **read-only**, copy what it needs into its own store, and run
screening, customer risk rating, transaction-monitoring scenarios, cases and regulatory reporting on that copy. Nobody
has seen a real CBS database from an Iraqi bank yet. This laptop has a mocked one. Your job is to **study that system
and its database and document everything you find in files**, so the product plan can be corrected against reality
before any code is written.

You do not build anything and you do not change anything in the database. You observe, measure, and write down.

### 2. Hard rules

1. **Read-only, always.** Run only `SELECT` and catalogue reads. Never `INSERT`, `UPDATE`, `DELETE`, `MERGE`, DDL,
   `GRANT`, `TRUNCATE`, `ANALYZE`/`DBMS_STATS` gathers, `VACUUM`, `CREATE` of any object (no temp tables, no views, no
   indexes, no sequences), no stored procedure that is not provably read-only, no `EXPLAIN ANALYZE` of anything that
   writes. Open every session as read-only where the engine allows it (PostgreSQL `SET default_transaction_read_only
   = on`; Oracle `SET TRANSACTION READ ONLY`; SQL Server `ApplicationIntent=ReadOnly` or a read-only login; Db2
   `FOR READ ONLY`). Set a statement timeout (30 seconds default, never above 5 minutes) and a row cap on every query.
2. **Your first finding is your own account's privileges.** List what the account you were given can do (roles,
   object grants, system privileges, admin options). If it holds any write-class privilege, write that down at once
   in the report: AKR-TMS must refuse such a connection in production, so this is a real finding. Still use it only for
   reads. If a read-only account can be created by the user, ask them to create one and use that.
3. **Do not hammer the system.** Use catalogue statistics for row counts and sizes (not `COUNT(*)` on large tables
   unless the table is small or you have no choice; then say how long it took). Sample large tables
   (`TABLESAMPLE`, Oracle `SAMPLE`, SQL Server `TABLESAMPLE`, or a keyed range) instead of full scans. Prefer indexed
   ranges. Never `SELECT *` without a `LIMIT`/`FETCH FIRST`. Record the elapsed time of every heavy query: AKR-TMS has
   to plan a load budget from your numbers.
4. **Treat the data as real personal and financial data**, even though it is mocked. In the report:
   - give structure, counts, formats, distributions and rule findings, not rows;
   - an example value is allowed only masked (keep the shape: `A1234567` becomes `A#######`; Arabic names show the
     script and the pattern, for example "three parts, family name last", not the name itself);
   - never paste a full row, never dump a table to a file, never write ID numbers, phone numbers, account numbers or
     full names into any file you create;
   - the only exception is code-table and reference data (transaction type codes, product codes, status codes, branch
     codes, currency codes): list those in full, because they are the dictionary.
5. **Nothing leaves this laptop through you.** No web requests with data, no uploading, no external services, no
   email. Your output is local files in the folder named in section 8, which the user will carry back.
6. **Write as you go.** The laptop or the editor can crash. After every stage, and at least every 20 minutes, append
   to `cbs-discovery/CBS-DISCOVERY.md` and to `cbs-discovery/_progress.md` (what is done, what is next, anything
   unresolved). On restart, read `_progress.md` first and resume.
7. **Do the work yourself, step by step.** Do not launch subagents or parallel workers unless the user asks in so many
   words. If your usage looks high, say so and stop rather than queue more work.
8. **No guessing presented as fact.** Every statement in the report is tagged one of: **observed** (you saw it in the
   catalogue or data), **inferred** (you reasoned from names or patterns; give the reasoning), or **unknown**. Where
   column names are cryptic (many CBSs use abbreviations or numeric field names), say what you believe and how
   confident you are, and add it to the questions list.
9. No emojis. Plain sentences. Write in English; keep Arabic text only where it is needed to show a script or
   encoding issue (and masked as in rule 4).
10. Do not edit anything in the AKR-TMS project folder outside `cbs-discovery/`. The planning documents are owned by
    the planning session; you give it findings, it changes the plan.

### 3. Orient yourself first (30 minutes at most)

Read these, in this order, only for the parts named. They tell you what AKR-TMS needs, so you know what to look for.
Paths are relative to the AKR-TMS project root.

| File | Read | Why |
|---|---|---|
| `docs/README.md` | all | One-page map of the plan |
| `docs/ARCHITECTURE.md` | section 5 (CBS access), 5.1 (connection tiers CT1-CT4), section 23 | How AKR-TMS reads the CBS and what is still open |
| `docs/MODULES.md` | M5, M6, M7 | Discovery, mapping, reconciliation: what the product does with your findings |
| `docs/DATA-MODEL.md` | section 3 (sources and landing), section 5 (canonical model) | **The target.** Every canonical table and column below is something you must try to find in this CBS |
| `research/25-core-banking-integration.md` | sections 1.3, 3 (your engine), 6 (business day), 7 (inserts, updates, deletes, reversals, watermarks, late data) | The questions to ask of a CBS schema |
| `research/17-vendor-cbs-native-aml.md` | skim | What CBS vendors expose |
| `docs/plan/phase-05.md`, `phase-06.md`, `phase-10.md` | only if they exist | The phases that will use your findings |

If a file is missing, say so in `_progress.md` and continue.

### 4. Ask the user these first, then start

1. Which engine and version, and how do I connect (host, port, service or database name, tool installed)? Which
   client tools exist on this laptop (`psql`, `sqlplus`/`sqlcl`, `sqlcmd`, `db2`, `mysql`, Python with a driver)? Do
   not install anything without asking.
2. Which account am I using, and may it be replaced by a read-only one?
3. Is this the whole CBS database, a replica, or an extract? Which schemas/owners belong to the CBS and which to
   other things (reports, staging, vendor tools)?
4. Which CBS product and version is it supposed to mimic (for example ICS BANKS on Oracle, iMAL, Temenos, Flexcube,
   a custom system)? Is there a data dictionary, manual, ER diagram, or vendor documentation on this laptop? Read it
   first and treat it as a source to check against the data, not as truth.
5. Is there an application (screens, reports, batch jobs) I may look at to see how the data is used, read-only?
6. Any tables or schemas I must stay out of?

Write the answers at the top of `CBS-DISCOVERY.md` under "Environment".

### 5. Method: ten stages

Do them in order. Each stage ends with a written section in the report. Do not skip a stage because the previous one
was large; do a thinner version and mark it incomplete.

**Stage 1 - Environment and privileges.** Engine, version, edition, character set and national character set,
collation, time zone settings, default date/time types and formats, client library used, server uptime, whether it
is a primary or a standby, whether the system is mid-business-day or after end of day. The account's privileges and
roles (rule 2). Which schemas the account can see.

**Stage 2 - Inventory.** For every CBS schema: every table, view, materialised view, synonym, sequence, procedure,
function, package, trigger, scheduled job, database link, and partitioned table. For each table: estimated row count,
size on disk, number of columns, primary key, unique keys, foreign keys (declared), indexes (columns and order,
uniqueness), partitioning scheme, comments, last stats date. Save the machine-readable inventory as
`cbs-discovery/data/inventory.csv` (one row per table/column) and a readable summary in the report. Use the engine's
catalogue, not guesses:

| Engine | Catalogue starting points |
|---|---|
| PostgreSQL | `information_schema`, `pg_catalog` (`pg_class`, `pg_attribute`, `pg_constraint`, `pg_index`, `pg_stats`, `pg_description`, `pg_partitioned_table`) |
| Oracle | `ALL_TABLES`, `ALL_TAB_COLUMNS`, `ALL_CONSTRAINTS`, `ALL_CONS_COLUMNS`, `ALL_INDEXES`, `ALL_IND_COLUMNS`, `ALL_TAB_COMMENTS`, `ALL_COL_COMMENTS`, `ALL_PART_TABLES`, `ALL_SOURCE`, `ALL_TRIGGERS`, `ALL_SCHEDULER_JOBS`, `ALL_TAB_PRIVS`, `SESSION_PRIVS`, `SESSION_ROLES`, `NLS_DATABASE_PARAMETERS` |
| SQL Server | `INFORMATION_SCHEMA`, `sys.tables`, `sys.columns`, `sys.indexes`, `sys.foreign_keys`, `sys.partitions`, `sys.dm_db_partition_stats`, `sys.extended_properties`, `sys.database_permissions`, `fn_my_permissions` |
| Db2 | `SYSCAT.TABLES`, `SYSCAT.COLUMNS`, `SYSCAT.INDEXES`, `SYSCAT.REFERENCES`, `SYSCAT.TABAUTH` |
| MySQL/MariaDB | `information_schema` |

**Stage 3 - Relationships.** Declared foreign keys first. Then infer the undeclared ones (many CBSs declare none):
matching column names and types, matching value sets (inclusion test on a sample: what fraction of the child values
exist in the parent key), index patterns, naming conventions. For each inferred relationship give the evidence and the
inclusion percentage. Produce an entity-relationship summary per business domain (stage 4) and a Mermaid diagram per
domain in the report. Note orphans (child rows with no parent) as data-quality findings.

**Stage 4 - Business domains.** Assign every table to a domain, or mark it "unknown/technical". Domains AKR-TMS cares
about (this list mirrors `DATA-MODEL.md` section 5; add any you find that are not on it):
customers and parties (individuals, companies, joint holders, directors, owners, signatories, guardians); identity
documents and identifiers; addresses and contacts; accounts and account-to-party roles; balances and daily balance
history; transactions (postings), their status lifecycle, reversals and corrections; transaction types and codes;
channels (teller, ATM, POS, online, mobile, SWIFT, standing orders, clearing); branches and the branch hierarchy;
products; general ledger; cash (denominations, cash declarations); foreign exchange deals and exchange houses;
payment messages (SWIFT MT or ISO 20022, domestic Iraqi rails, raw message text); counterparties and beneficiaries;
cards and card transactions; loans, facilities, collateral; trade finance (letters of credit, guarantees,
collections; goods, HS codes, ports, vessels); safe-deposit lockers; staff, operators, authorisers, approval limits,
user activity logs, overrides; holds, freezes, court and CBI orders; KYC fields (risk rating, PEP flag, expected
income, source of funds, occupation, cdd level, document expiry, last review); AML flags the CBS already carries
(its own alerts, watch lists, blacklist, sanctions hits); Islamic banking contract types (murabaha, ijara, mudaraba,
musharaka, and so on); reference/code tables; audit/history/log tables; batch and end-of-day control tables;
anything else (reports, staging, interfaces, vendor tools).
For each domain write: the tables, the one-paragraph story of how data flows (who writes it, when, how it changes),
and the key columns with meaning.

**Stage 5 - Column semantics and profiling.** For every table in the domains above (largest and most important
first), and every column you will map, profile with sampled, bounded queries:
null rate; distinct count (estimate is fine); min and max; top 20 values with frequency (code columns: all values);
length distribution for text; format patterns for identifiers, phone numbers and dates stored as text or numbers
(regex families, masked); value ranges for money; for codes: the full code list and, if a code table exists, its
descriptions. Also record data types exactly as declared. Then answer the questions that decide the design, each
with evidence:

- **Money:** Which column holds the amount? Integer minor units, decimal, float, text? Scale? Is there a separate
  currency column? Is the sign (debit/credit) in the amount or a separate indicator? Are there local-currency and
  foreign-currency amounts side by side, and the rate used? Which currencies appear (IQD, USD, others) and their
  shares?
- **Dates and time:** Which date columns exist on a transaction (entry, booking/posting, value, business date,
  system timestamp)? Types (date, timestamp, number such as `20260131`, text, Hijri or Gregorian)? Time zone
  (server local, UTC, none)? Is the time of day stored? What is the business-day/EOD mechanism (control table,
  system date, "day open/close" flags)? Do branches have their own date? Weekend and holiday behaviour (Iraq: Friday
  and Saturday weekend, check the data)?
- **Transaction lifecycle:** statuses (pending, authorised, posted, rejected, reversed, cancelled), how a reversal is
  stored (a new negative row, a flag on the original, a link column), how corrections are stored, whether rows are
  ever updated in place, whether rows are ever deleted or purged, whether back-dated entries exist (booking date much
  earlier than the insert time), whether the same transaction appears more than once (header/lines, double entry:
  one row per leg or per transaction?).
- **Identity of things:** the stable key of a customer, account, transaction; whether account numbers are reused;
  whether customer numbers can merge; the format and check-digit scheme of account numbers and IBANs; whether one
  person can have several customer numbers (duplicates); how joint accounts are represented.
- **Names:** how stored (single column, parts, separate Arabic and Latin columns, transliteration), character set,
  whether Arabic is stored correctly (round-trip a few values: read as bytes, compare with the displayed text;
  look for mojibake, reversed order, visual-order storage, mixed Arabic/Persian letters such as ي/ى/ی and ك/ک, presence
  of diacritics and tatweel, Kurdish letters, Eastern Arabic digits vs Western digits). Whether father's and
  grandfather's names and the mother's name are captured. Whether company names, trade names, aliases are separate.
- **Dates of birth:** completeness, year-only values, placeholder dates (01/01/1900, 01/01 of a year), impossible
  dates.
- **Identity documents:** types (national ID, residency card, passport, civil status ID, nationality certificate,
  company registration), formats, where stored, uniqueness, expiry.
- **Geography:** governorate, district and branch coding; whether Kurdistan region is identifiable; countries coded
  how (ISO alpha-2, alpha-3, numeric, local).
- **Cash:** is a cash transaction identifiable (transaction code, channel, flag)? Are denominations kept? Any declared
  source of funds or purpose on deposits and transfers?
- **Payments:** where outgoing and incoming transfers sit; whether the SWIFT message text, the ordering and
  beneficiary parties, the intermediary and correspondent banks, the purpose code and the remittance information are
  stored, and in what form; how a payment message links to the ledger posting (a reference, UETR, or nothing).
- **Operators:** whether each posting records who entered it and who authorised it (maker-checker), at which branch
  and terminal, and whether overrides and forced postings are logged.
- **Risk and KYC fields:** what the CBS already stores about customer risk, PEP, sanctions, expected activity, source
  of wealth, review dates, document expiry, and how complete those fields are.
- **Islamic banking:** whether contract types and profit-sharing structures appear, and where.
- **Existing AML data:** any alert, case, STR, watch-list or blacklist tables the CBS keeps.

**Stage 6 - Change tracking and loading.** For each large or important table decide how AKR-TMS could read changes
safely with bounded load: monotonically increasing keys (sequence/identity), `created`/`updated` timestamps and
whether they are reliable (are updates really stamping them? any rows with `updated < created`? any future dates?),
row versions, SCN/LSN pseudo-columns, history/audit tables, soft-delete flags, hard deletes, partitioning by date,
and whether late or back-dated rows appear (compare insert order with booking date). State the proposed watermark
column and its risk. Measure: how many transactions per day (use the sampled distribution over the last 90 days of
data), peak hour, rows per table per day, table growth, the size of one day's extract, the time to read one day with
an indexed range on a representative table (bounded, with a timeout; report the elapsed time and the plan if you can
read it). Note which tables have no usable index for incremental reads.

**Stage 7 - Behaviour in the database.** Triggers, stored procedures, jobs and views that carry business logic
(posting rules, interest, fees, EOD batch, status changes, archiving or purging). Read their source (read-only) and
summarise what they do to the data AKR-TMS will read. Pay special attention to: anything that deletes or archives rows,
anything that rewrites history, and any view that already looks like a reporting layer (these are candidates for the
"designated views" connection tier CT2).

**Stage 8 - Draft canonical mapping.** For every table and column in `docs/DATA-MODEL.md` section 5 (canonical
model), say where it comes from in this CBS. Use one table per canonical table:

| Canonical column | Source (schema.table.column or expression) | Transform needed | Confidence (high/medium/low/none) | Evidence | Note |
|---|---|---|---|---|---|

Mark canonical columns with no source as **absent**. Also list CBS columns that look useful for AML/KYC but have no
canonical home (candidates to add to the model).

**Stage 9 - Data-readiness view.** Grade each canonical attribute family, and then each of the following scenario
families, A to D: **A** the data is present, unambiguous and of good quality; **B** present but needs a documented
transform or join; **C** present but partial, unreliable, or only for some products/branches; **D** absent. Name the
missing or weak fields and who in the bank or the vendor would have to provide them. Scenario families (the full list
of 173 CBI items is in the plan; here is the practical grouping): cash structuring and large cash; rapid movement of
funds (in and out); dormant account reactivation; activity inconsistent with profile; high-risk geography
counterparties; cross-border and correspondent payments; trade-based; round-amount and just-below-threshold;
transfers between related accounts; use of many accounts or branches; staff and insider activity; PEP and high-risk
customer monitoring; sanctions and name screening on onboarding and on payments; KYC completeness and periodic review;
card and ATM patterns; loan and collateral anomalies; Islamic contract anomalies.

**Stage 10 - Quality, risks and questions.** A ranked list of data-quality findings (orphans, duplicates, placeholder
values, impossible dates, mixed encodings, inconsistent codes, truncated names, negative balances where impossible,
transactions on closed accounts, accounts without owners, customers without accounts, unbalanced entries), each with
count or rate, tables, severity (blocks monitoring / degrades / cosmetic), and a suggested handling. A list of
**questions for the bank and for the CBS vendor**, each tied to a finding. A list of **surprises**: anything that
contradicts an assumption in the plan (section 6 below lists the main assumptions).

### 6. Assumptions in the plan that you are testing

Mark each CONFIRMED, CONTRADICTED, or UNKNOWN, with evidence:

1. A CBS table or view exists from which a stable transaction id, account, signed amount, currency, booking date,
   value date and transaction type can be read for every posting.
2. Customers have a stable key and a single owner record per person, with name, date of birth, nationality, an
   identity document, and a branch.
3. Names are available in Arabic script and can be read back correctly (no mojibake).
4. Transaction type codes are enumerable and mappable to canonical types (cash deposit, cash withdrawal, transfer,
   fee, interest, FX, loan, card, and so on).
5. A business date exists separately from the system timestamp, and an end-of-day marker can be detected.
6. Postings are append-only, or changes can be detected from a timestamp or history table; deletions are rare or
   visible.
7. Reversals are linked to the originals.
8. The maker and the authoriser of each posting are recorded.
9. Payment messages (SWIFT or domestic) are stored with party details and linked to postings.
10. Branch, governorate and region can be derived for every account and transaction.
11. A bounded incremental read of one day's postings completes well inside a 10-minute window with an indexed range.
12. No table needed by AKR-TMS holds card numbers in clear (full PANs); if one does, that is a finding to report
    immediately because AKR-TMS must mask or exclude them before landing.
13. The CBS carries risk rating, PEP and expected-activity fields at least for some customers.
14. Historical depth: at least 12 months of postings are available (state the real depth per table).

### 7. Style of findings

- Short paragraphs and tables. A table beats a paragraph where the content is a list of facts.
- Every finding has: what, where (schema.table.column), evidence (query shape and result summary, masked), tag
  (observed/inferred/unknown), consequence for AKR-TMS (one line), and a suggested action.
- Keep the queries you ran in `cbs-discovery/data/queries.sql` (one file, commented with the stage and purpose, with
  the timing). The user and the planning session must be able to rerun them. Queries must be read-only by inspection.
- Put wide tables and long lists (inventory, code lists, profiles) in `cbs-discovery/data/*.csv` and link them from
  the report; keep the report readable.

### 8. Deliverables (all inside `cbs-discovery/` in the project folder; create it)

```
cbs-discovery/
  CBS-DISCOVERY.md        the main report, written incrementally (skeleton below)
  _progress.md            running log: stage done, next step, open items (newest first)
  data/
    inventory.csv         every table and column: engine types, nullability, keys, row estimate, size
    relationships.csv     declared and inferred links with inclusion percentages and evidence
    code-tables.csv       every code list found (table, code, description, row count)
    profiles/             one csv per major table: column profile (nulls, distinct, min, max, top values masked)
    mapping.csv           the canonical mapping from stage 8 in flat form
    readiness.csv         stage 9 grades
    queries.sql           every query run, with purpose and timing
  diagrams/               Mermaid sources, one file per domain
```

Skeleton of `CBS-DISCOVERY.md` (keep these headings, so the planning session can find things):

```
# CBS discovery report
0. Summary for the planner (one page: system in one paragraph, ten most important findings, readiness verdict,
   the five biggest risks, the ten most important questions)
1. Environment and the account's privileges
2. Inventory (schemas, size, counts, what is where)
3. Relationships (declared, inferred, orphans)
4. Domains (one subsection each: tables, data flow, key columns, diagram)
5. Column semantics and profiling (money, dates, lifecycle, identity, names and Arabic, documents, geography, cash,
   payments, operators, risk fields, Islamic, existing AML data)
6. Change tracking and loading (watermark choice per table, volumes, timings, index gaps)
7. Behaviour in the database (triggers, procedures, jobs, views, purge/archive)
8. Canonical mapping (tables from stage 8; absent fields; candidate model additions)
9. Data-readiness grades (attributes and scenario families)
10. Data-quality findings (ranked)
11. Assumption check (the 14 assumptions)
12. Questions for the bank and the CBS vendor
13. Surprises and recommended plan changes (what the AKR-TMS docs should change, by file and section if you can)
14. Appendix: glossary of CBS terms found, abbreviations decoded, tools used
```

### 9. When you finish

1. Update section 0 of the report so it can be read alone.
2. Tell the user, in plain short sentences: what the system is, the readiness verdict, the three worst problems, the
   questions that need a human, and which files to carry back.
3. Do not start fixing or building anything. If you noticed an opportunity or risk outside the brief, add it to
   section 13 and say so.

---

## Notes for the planning session (not part of the brief)

- This is the "discovery" step that phase 05 (read-only connection and discovery) and phase 06 (readiness grades and
  the CBI gap report) will automate. The human-readable report from the laptop is the ground truth to test their
  specs against: field availability, code maps, watermark choices, load budget numbers, name data-quality profile.
- When the report comes back, update: `docs/DATA-MODEL.md` section 5 (columns that do not exist or exist differently),
  `docs/MODULES.md` M5-M7, `docs/ARCHITECTURE.md` section 23 (first CBS adapter), `docs/plan/research.md` (new DR
  rows for decisions the findings force), and write the findings into phases 05, 06 and 10 before they are authored.
- Keep the mocked data out of the repository if it is ever put under version control: the report holds masked
  samples only, but check before committing.
