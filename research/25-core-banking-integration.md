# 25 — Core banking integration: read-only access to the CBS database, payment data, the canonical model and the phase-2 pre-release API

Status: done (2026-10-06)

Stack alignment (2026-10-07, file 32 sections 12-13): the user chose **OpenBao** instead of HashiCorp Vault, **Valkey** instead of Redis, and to **keep MinIO** as a pinned, maintained and renamed build (Silo, built from source; Chainguard's fork as second source). Mentions below were aligned on that date; where the text describes the digital-banking-system (DBS), its own Vault and Redis are kept as they are.

Scope: how AKR-TMS gets complete, correct and timely data out of an Iraqi bank's core banking system (CBS) through a READ-ONLY database connection, without touching the CBS backend or APIs and without writing to the CBS; how it proves that nothing was missed; which CBS products Iraqi and regional banks run and what their databases look like; where payment messages come from; the canonical AML data model and the metadata-driven mapping layer; and the optional phase-2 synchronous screening and scoring API. Requirement IDs use the prefix C. Cross-references: file 01 (CBI controls 2025, "Automatic Integration" with the CBS, records, hosting), file 04 (B-01 coverage reconciliation; Wolfsberg payment transparency), file 05 (NYDFS Part 504 data integrity and ETL completeness; OFAC 10-year records), file 27 (scenarios and the data they need), file 32 (Rust drivers and streaming choices), file 30 (security of the link), files 17-18 (Iraqi CBS market, written by another agent).

## Contents

1. Premises: what "read-only" must mean in practice (and what it cannot cover)
2. The integration patterns compared and a decision matrix
3. Engine by engine: Oracle, SQL Server, PostgreSQL, IBM Db2, others
4. CDC and replication tooling: Debezium, GoldenGate, Qlik Replicate and others
5. Load on the CBS, resource governance and the bank's kill switch
6. Isolation levels, consistency and the CBS business day
7. Inserts, updates, deletes, reversals; watermarks; late data; idempotent loading
8. Reconciliation and control totals: proving completeness
9. Latency targets and freshness classes
10. Securing the link: least privilege, segmentation, TLS, OpenBao, bank-side audit
11. The CBS products used in Iraq and the region, and their database quirks
12. Payment data sources: SWIFT MT, ISO 20022, Iraq RTGS, ACH, national switch, cards
13. The canonical AML data model
14. The metadata-driven mapping layer and per-bank adapters
15. The phase-2 pre-release screening and scoring API
16. What it means for AKR-TMS
17. Derived requirements
18. Sources

Evidence marks used throughout: [V] read in a primary document during this research; [S] secondary source; [K] practitioner knowledge to be confirmed in each bank's discovery. Section 16 lists the open questions and the sources that could not be reached.

---

## 1. Premises: what "read-only" must mean in practice (and what it cannot cover)

Evidence marks used in this file: **[V]** read in a primary vendor, regulator or standards document during this research (URL in section 18); **[S]** from reputable secondary material (trade press, consultancy, community post); **[K]** practitioner knowledge that could not be confirmed in a document this session and must be confirmed in each bank's discovery phase. Versions and prices are as of 2026-10-06.

### 1.1 The premises

- The brief fixes the data path: AKR-TMS reads the bank's CBS database through a read-only connection only. It never calls the CBS backend or its APIs and never writes to the CBS. A CBS-agnostic mapping layer turns each bank's schema into the canonical model. Freezes and blocks become tasks for bank operations and are verified afterwards from the same read-only data (order-to-block time, post-order debit checks).
- CBI controls 2025, chapter 5 article 5 (file 01 section 2d) ask for a "direct link to the core banking system" ("Automatic Integration"), "instant, real-time follow-up" of all account movements, and a system that "stops any transaction that matches one or more suspicion scenarios until the MLRO verifies it is sound". A read-only database link can deliver the first two (as near-real-time, in minutes). It cannot deliver the third. Section 15 (the phase-2 pre-release API) is how a bank gets the third.
- Completeness is a regulatory property, not a nicety. BCBS 2020 para 28 (monitoring must cover all accounts and transactions; requirement B-01 in file 04). NYDFS Part 504 ("data extraction and loading processes that ensure a complete and accurate transfer from source systems"; file 05 section 1f). And the CBI's own risk-management controls of 13 July 2026, whose internal-control section (second, "internal control framework") requires IT systems that secure "the validity, completeness and accuracy of data" (item 4.2.3, PDF page 26; read from the Arabic text of the commercial-banks edition) [V].
- The CBI expects every bank to have a CBS at all: standards handbook 2025, standard B2, says each bank "must use a comprehensive banking system" meeting the CBI's assessment guide, verified by "a competent firm from the list of firms approved by the Central Bank", and defines it as "a digital platform or set of platforms that enable banks to carry out core banking operations such as account management, transaction processing, loan management and compliance reporting across branches in real time" [V, Arabic text of the handbook; translation ours]. The same handbook (B7) requires data to be protected "from unauthorised access and tampering" and encryption to be verified by an approved firm; B9 requires business-continuity and disaster-recovery plans that are audited (file 30). A bank's CBS is therefore the system of record that AKR-TMS reads, and the CBI-approved assessor will examine how AKR-TMS touches it.

### 1.2 Three different meanings of "read-only"

Banks, DBAs and CBS vendors use the phrase loosely. The product should state which of the three it delivers in each deployment, and prove it.

| Level | Meaning | How AKR-TMS proves it |
|---|---|---|
| 1. Data-plane read-only | The AKR-TMS database identity holds SELECT (or the engine's equivalent) on a named list of views or tables and nothing else: no INSERT, UPDATE, DELETE, DDL, EXECUTE on procedures, no temporary objects in CBS schemas | Catalogue check at every connection (Oracle `SESSION_PRIVS` and `USER_TAB_PRIVS`, SQL Server `sys.fn_my_permissions` and `sys.database_permissions`, PostgreSQL `has_table_privilege` and `pg_roles`, Db2 `SYSCAT.*AUTH`); refuse to start if any write-class privilege is found |
| 2. Footprint-free | AKR-TMS needs no object, setting or background job inside the CBS database (no flush table, no signalling table, no change tables, no replication slot, no supplemental logging) | The "footprint register" (1.4) is empty |
| 3. Load-free | The reads do not slow the CBS | Reads go to a replica, or run under hard resource caps set by the bank's DBA (section 5) |

Log-based change data capture (CDC) delivers level 1 and breaks level 2, and a stalled consumer can break level 3 on the primary:

- **Oracle with Debezium LogMiner** [V]: the database must be in archive-log mode with supplemental logging (`ALTER DATABASE ADD SUPPLEMENTAL LOG DATA`; per-table `ADD SUPPLEMENTAL LOG DATA (ALL) COLUMNS`, which "increases the volume of the Oracle redo logs"). The connector account in Debezium's own example holds `FLASHBACK ANY TABLE`, `SELECT ANY TABLE`, `SELECT ANY TRANSACTION`, `LOGMINING`, `EXECUTE_CATALOG_ROLE`, `CREATE TABLE`, `CREATE SEQUENCE` and `LOCK ANY TABLE`, because it creates a flush table (`LOG_MINING_FLUSH`) in the source database (switchable off with `log.mining.read.only=true`, which Debezium documents for logical standbys). Incremental snapshots on Oracle need write permission on a signalling table; Debezium states that write permission is avoidable "only for connectors that can be configured to perform read-only incrementals snapshots (MariaDB, MySQL, or PostgreSQL)".
- **SQL Server CDC** [V]: CDC must be enabled on the database and on every captured table (`sys.sp_cdc_enable_db`, `sys.sp_cdc_enable_table`); the SQL Server Agent capture job copies log records into change tables inside the CBS database. Debezium: SQL Server 2016 SP1 and later, Standard or Enterprise edition.
- **IBM Db2 with Debezium** [V]: needs SQL Replication (ASN capture and apply programs, control tables and change-data tables in the database) and, per Debezium's documentation, "a separate license to IBM Infosphere Data replication (IIDR)".
- **PostgreSQL logical decoding** [V]: a replication slot is created on the server it decodes. The PostgreSQL manual: replication slots "know nothing about the state of their consumer(s). They will prevent removal of required resources even when there is no connection using them... In extreme cases this could cause the database to shut down to prevent transaction ID wraparound." A stopped AKR-TMS consumer can therefore fill the disk of a CBS primary.

So the honest statement for a CDC deployment is: "AKR-TMS writes no business data to the CBS, but the bank's DBA applies a one-time configuration change, runs the capture components, and carries a new failure mode (a stalled consumer) that must be capped by the bank (for PostgreSQL `max_slot_wal_keep_size`; for Oracle the archive-log retention and fast recovery area; for SQL Server log-truncation alarms)." This belongs in the bank's risk-acceptance paper, not in a footnote.

### 1.3 Four connection tiers

The market is heterogeneous (section 11): some banks run Oracle with a licensed standby, some run a single SQL Server, some state banks have no comprehensive CBS at all (the CBI itself describes an "internal clearing system between the branches of the same bank, which enables government banks that do not have a comprehensive banking system to automate all transfers of their branches" [V], CBI payment-systems statement). One canonical pipeline must therefore accept four tiers:

| Tier | Name | Source | Typical bank |
|---|---|---|---|
| R1 | Replica read | A standby or replica that the bank already runs and that accepts queries (Oracle Active Data Guard, SQL Server readable secondary, PostgreSQL hot standby, Db2 reads on standby) or a reporting clone the bank's DBA refreshes | Larger private banks with HA/DR designed by an integrator |
| R2 | View read on the primary | A read-only account on the primary reading designated views, small batches, off-peak or throttled | The default for first deployments; the easiest for a DBA and a CBS vendor to approve |
| R3 | Log-based CDC | CDC from the primary or, better, from a standby (section 4) | Banks with DBA capacity, strict latency needs, or very large tables |
| R4 | Bank-delivered extract | The bank's own team (or the CBS vendor's batch) writes files or loads a landing database that AKR-TMS reads; includes a vendor "operational data store" if the bank owns one | CBSs with no SQL access (multi-value or proprietary stores), banks that refuse any direct connection, state-bank branches |

R4 still respects the brief: AKR-TMS touches no CBS process and reads no CBS API; the bank decides what to put in the landing area. Everything downstream of the "change batch" (mapping, reconciliation, monitoring) is identical for all four tiers.

### 1.4 AKR-TMS hooks

- **Integration profile per bank**: tier, engine and exact version, replica availability and its licence status, DBA contact, change window, resource caps, kill-switch owner. Stored as admin-managed data (DBS rule "nothing is a fixture").
- **Read-only attestation**: the connector inspects its own grants on every (re)connect and writes a signed report to the evidence vault ("what this account can do on 2026-xx-xx"); any write-class privilege fails the connector closed unless a named bank officer has recorded an exception.
- **Footprint register**: every object or setting AKR-TMS needs in the CBS database (supplemental logging, CDC enablement, slot, flush table, grants), each with an owner on the bank side, a justification, and a removal script. Exportable to the bank's change-management system.
- **Consumer-lag guard** (R3 only): AKR-TMS publishes its consumer lag and slot or log retention to the bank's monitoring and stops advancing nothing silently; the bank's DBA owns the cap that protects the CBS.

---

## 2. The integration patterns compared and the decision matrix

### 2.1 The patterns

| Pattern | How it works | CBS footprint | Latency | Updates and deletes | Completeness proof | Cost and licence | Main failure mode |
|---|---|---|---|---|---|---|---|
| A. Replica polling (R1) | Poll views on a replica by watermark, overlap window, idempotent upsert | None on the primary beyond what the replica already costs (SQL Server adds 14 bytes per row version on the primary when readable secondaries exist) | Replica lag (seconds) + poll interval (10-60 s) | Updates seen through the version column; hard deletes invisible unless the table keeps a tombstone or is reconciled (section 7) | Reconciliation by counts, sums and balances (section 8) | Oracle needs the Active Data Guard option; SQL Server readable secondaries need Enterprise edition; Db2 reads on standby are UR only | Replica lag; query conflicts cancel long reads; replica opened by the bank's DBA for the wrong role |
| B. Primary polling through views (R2) | Same, against the primary | A read-only account and views | Poll interval | Same as A | Same as A | None beyond the CBS licence | Reads compete with the CBS at peak and at end of day; long scans |
| C. Log-based CDC on the primary (R3) | Read redo or WAL or transaction log; emit row-level changes | Supplemental logging or CDC tables or slot; extra privileges (1.2) | Seconds | Inserts, updates and deletes are all seen, with commit order and before/after images | Per-transaction ordering; still needs reconciliation | Debezium is free (Apache-2.0); Oracle XStream needs GoldenGate licence; Db2 needs IIDR; GoldenGate and Qlik Replicate are commercial | Stalled consumer fills the primary's log disk; schema changes break decoding; supplemental logging raises redo volume |
| D. Log-based CDC on a standby (R3) | Same, reading the standby | Less on the primary; configuration still applied there | Seconds + standby lag | Same as C | Same as C | As C plus the standby licence | Oracle physical-standby capture is "incubating" in Debezium; GoldenGate Extract cannot connect to an ADG standby directly |
| E. Triggers or journal tables written by the CBS | The CBS database writes change rows | Writes inside CBS transactions | Immediate | All | Good | None | **Excluded**: it writes to the CBS and adds latency to every posting |
| F. Vendor feeds the bank chooses to switch on (for example a CBS event-streaming or ODS product) | The CBS vendor's own replication feeds a landing store or queue | Vendor-side | Seconds | Vendor-defined | Vendor-defined | Often a separate licence (for Temenos: Transact Data Hub) | Mapping now depends on the vendor's simplified model; **allowed only as an R4 source the bank provides, never called by AKR-TMS** |
| G. Application API or message interfaces | Call CBS services | CBS backend | Real time | Full | Weak | n/a | **Excluded by the brief** |

[V] for the Oracle, SQL Server, PostgreSQL, Db2 and Debezium details (section 3 and 4 give the sources). [S] for Qlik Replicate.

### 2.2 Decision matrix by the bank's situation

| Situation found in discovery | First choice | Second choice | Note |
|---|---|---|---|
| Oracle with a licensed, open-read Active Data Guard standby | R1 polling on the standby | R3 CDC from the standby (Debezium physical-standby mode, incubating; or Qlik Replicate Log Reader on Data Guard) | Oracle names the standby-open-read capability a licensed option (section 3.1) |
| Oracle with a mounted DR standby (plain Data Guard) or none | R2 polling on the primary with strict caps | Ask the bank for a refreshed reporting clone (R1) | Do not assume the DR standby can be read |
| SQL Server with an Always On readable secondary (Enterprise) | R1 polling on the secondary | R3: CDC enabled on the primary; AKR-TMS reads the change tables on the secondary | CDC cannot be enabled on the secondary alone |
| SQL Server Standard or a single instance | R2 | R3 CDC on the primary (works in Standard from 2016 SP1) | Row-version columns, if present, give a cheap watermark |
| PostgreSQL (rare in CBS, common in digital channels) | R1 on a hot standby | R3 logical decoding on a PG16+ standby (slot on the standby) | See 3.3 for query-conflict settings |
| Db2 LUW | R1 only if the DBA enables reads on standby and accepts uncommitted reads; else R2 | R3 only with IIDR licence | Section 3.4 |
| Multi-value or proprietary store (for example jBASE), or no SQL access | R4: bank-delivered extracts or an ODS | none | Section 11 |
| State-bank branches without a comprehensive CBS | R4 from the branch or head-office reporting data | none | CBI statement above |

### 2.3 Recommended default: "poll first, add CDC where it earns its keep"

Reasons, with the evidence:

1. **Approval friction decides the schedule.** A read-only account on views is the smallest ask of a DBA and of a CBS vendor whose support terms usually forbid database-level changes [K]. CDC needs supplemental logging, extra privileges and agents running inside the CBS database (1.2).
2. **Completeness can be proved independently of the mechanism.** Reconciliation against control totals (section 8) is required in any design. A polling design makes it a first-class part of the loop; a CDC design adds a second source of truth to reconcile.
3. **CBS ledgers rarely delete.** Financial entries are reversed by new entries, not deleted [K]; hard deletes occur in staging and in-flight tables (for example unauthorised-entry files in Temenos, section 11.1), which a polling design handles by reading the authorised state.
4. **Minutes are enough for post-event monitoring.** The CBI asks for "instant, real-time follow-up" but the system cannot hold a payment from a read-only connection anyway; real-time interdiction is the pre-release API (section 15). A 30-60 second poll plus replica lag is within what Wolfsberg and BCBS describe as effective post-event monitoring.
5. **A change-source abstraction keeps the door open.** The ingestion framework consumes "change batches with a watermark" whether they come from a poll, a CDC stream or a landing file, so a bank can move from R2 to R3 later without touching the mapping or the monitoring.

### 2.4 AKR-TMS hooks

- **Change-source interface**: `poll(view, watermark, overlap) -> batch`, `stream(source, position) -> batch`, `file(drop, manifest) -> batch`; every batch carries source id, watermark range, row count and checksums (section 8).
- **Discovery checklist** (printable, one per bank) that fills the integration profile: engine and version, edition, replica present and open, ADG or equivalent licence, CDC allowed, supplemental logging allowed, time zone and business-date rules, end-of-day window, maximum concurrent sessions, who can kill a session.
- **Pattern switch without redeploy**: moving a bank from R2 to R1 or R3 is a configuration change plus a reconciliation run, not a new adapter.

### 2.5 The whole path in one picture

```text
BANK ZONE (CBS network)                         AKR-TMS (in the bank's data centre, single tenant)

  CBS database  ----+                           +--------------------------------------------------+
  (primary)         |  R2 poll views            |  ingestion workers (Rust)                        |
  replica / standby-+  R1 poll replica   -----> |   change source: poll | stream | file            |
  log (R3 CDC) -----+  R3 Debezium sidecar      |     -> batch envelope + receipt (count, hash)    |
  payment gateway DB   R2 poll views            |   raw landing (verbatim rows, kept)              |
  SWIFT archive / ODS  R4 files / landing       |   mapping engine (pinned mapping_version)        |
                                                |   canonical store (PostgreSQL, partitioned)      |
  bank-owned VIEWS  <== DBA applies DDL pack    |   reconciliation ladder L0-L6 -> coverage cert.  |
  kill switch (DBA) ==> pauses the account      |   features -> scenarios / ML -> alerts -> cases  |
                                                +----------------------+---------------------------+
                                                                       | (phase 2, optional)
  payment gateway / SWIFT interface  --- HTTPS or MQ, mTLS --->  assess API (separate binary)
  (holds, releases, rejects; AKR-TMS never writes to it)  <--- signed decision (RELEASE|HOLD|REJECT)
```

---

## 3. Engine by engine: Oracle, SQL Server, PostgreSQL, IBM Db2, others

### 3.1 Oracle Database

**Reading a replica.** A physical standby that is open read-only while redo apply continues is Oracle's "real-time query", and it is a licensed option. Oracle's own Licensing Information manual lists "Physical Standby with Real-time Query" among the features that need Oracle Active Data Guard, and says: "Oracle Active Data Guard must be licensed on any standby databases with any of the above features in use, as well as the primary database." The same page says the container root of a standby "may be opened read-only without requiring an Oracle Active Data Guard license", and that the licence "is only required if any of the pluggable databases in the standby is opened for read" [V, Oracle Database 19c Licensing Information]. Licensing consultancies quote a list price of about USD 11,500 per processor for 2026 [S; confirm against the bank's Oracle contract]. Consequence: never assume a bank's DR standby can be read. A mounted (not open) standby is useless to AKR-TMS; asking the bank to open it may trigger a licence purchase.

**Consistency.** Oracle "always enforces statement-level read consistency, which guarantees that data returned by a single query is committed and consistent for a single point in time"; in the default read-committed level that point is the time the statement opened. A read-only transaction (`SET TRANSACTION READ ONLY`) gives the whole transaction one point in time and "read-only transactions are not susceptible to the ORA-08177 error". Long reads risk `ORA-01555 snapshot too old` when undo is reused, so the extraction batch size must fit inside the bank's `undo_retention` [V, Oracle Concepts]. Flashback query (`SELECT ... AS OF SCN n`) reads a table as of a past SCN; Debezium uses it for its initial snapshot after taking short `ROW SHARE MODE` locks to read a consistent SCN [V].

**Change detection without CDC.** `ORA_ROWSCN` is not a safe row-level watermark: "In the absence of row-level dependencies, ORA_ROWSCN reflects block-level dependencies", and it "should not be considered to be an exact SCN... any value greater than or equal to [the commit SCN] could be returned" [V]. Row-level tracking needs `ROWDEPENDENCIES` at table creation, which a CBS vendor will not have set. So the usable watermarks are the CBS's own columns: a posting sequence or key that only grows, a last-modified timestamp, a status column with version counter (section 7).

**CDC.**

| Option | Status | Notes |
|---|---|---|
| Debezium + LogMiner | Apache-2.0; current stable Debezium is 3.7 (29 September 2026), Java 17 or later for connectors [V] | Oracle 19, 21, 23 and 26 are the supported releases in Red Hat's build [S]; "Oracle 11g is not supported; however, we do aim to be backward compatible with Oracle 11g on a best-effort basis" [V]. Two LogMiner modes: uncommitted-changes mode buffers in the connector (low database load, high connector memory) and committed-changes mode shifts memory to the database PGA and can hit `PGA_AGGREGATE_LIMIT` [V] |
| Debezium + XStream | Needs a GoldenGate licence: "To use the XStream API, you must have a license for the GoldenGate product" [V] | Not realistic for most Iraqi banks |
| Debezium + OpenLogReplicator | OpenLogReplicator reads redo and archive logs directly "with minimal impact on the database"; AGPL-3.0; release 2.0.0 on 2026-07-19 [V, GitHub API] | AGPL needs legal review before bundling; operational maturity must be tested |
| Debezium on a physical standby | "Incubating... can change without notice"; needs Active Data Guard, a flat-file dictionary exported from the primary (`log.mining.strategy=dictionary_from_file`), `capture.mode=physical_standby`, `log.mining.archive.log.only.mode=true`; the connector still connects to the primary for snapshots and metadata [V] | The dictionary is a one-time export, so schema changes on the primary need a rebuild |
| Oracle GoldenGate | Commercial; the Extract for Oracle "cannot connect to a Data Guard standby or Active Data Guard standby to capture data directly"; the supported pattern is a cascaded downstream mining database [S, Oracle docs excerpt]. GoldenGate "Free" is Oracle-to-Oracle only and capped at 20 GB [S] | Heavy for a bank that only wants to read |
| Qlik Replicate | Commercial; Oracle Data Guard "is supported when using Replicate Log Reader to access the redo logs only" (LogMiner is not supported with Data Guard) [V, Qlik help, May 2026] | A separate endpoint is needed for primary and standby |

**Footprint and privileges.** The Debezium LogMiner account needs the wide set of privileges listed in 1.2, plus supplemental logging and archive-log mode. Supplemental logging at database level is a change the CBS vendor may refuse to support [K].

**Traps.** (1) Character sets: Arabic-region Oracle databases are often not AL32UTF8 (AR8MSWIN1256 and AR8ISO8859P6 exist) [K]; the client's `NLS_LANG` must be tested with a round-trip string containing Arabic letters, tashkeel and Persian digits at install time. (2) `RECID` is a reserved word, which bites Temenos tables (section 11.1) [S]. (3) `DATE` has no time zone; the CBS business date is a separate concept (section 6). (4) Old releases: Finacle and FLEXCUBE estates frequently sit on 11g or 12c [K]; plan for polling there.

### 3.2 Microsoft SQL Server

**Reading a replica.** An Always On readable secondary gives near real-time data: "In many circumstances, data latency between a primary database and the corresponding secondary database is only a few seconds." Two points matter [V, Microsoft Learn]:

- "All queries that run against the secondary databases are automatically mapped to snapshot isolation transaction level, even when other transaction isolation levels are explicitly set. Also, all locking hints are ignored." Good for AKR-TMS: no reader/writer contention and consistent statement views.
- The cost lands on the primary and the secondary: "the primary databases add 14 bytes of overhead on deleted, modified, or inserted data rows" (page splits can follow); long queries on the secondary can block ghost-record cleanup on the primary, and if the secondary disconnects "this state also prevents log truncation". So the bank's DBA must cap read-query duration on the secondary.

Basic availability groups (Standard edition) have "No read access on secondary replica" [V]. Readable secondaries therefore mean Enterprise edition [V by exclusion]; a Standard-edition bank falls back to R2 (or to readable log-shipping standby or a refreshed clone, which are [K]).

**Consistency on the primary.** The default is `READ COMMITTED` with shared locks unless `READ_COMMITTED_SNAPSHOT` is on: "the Database Engine uses shared locks to prevent other transactions from modifying rows while the current transaction is running a read operation. The shared locks also block the statement from reading rows modified by other transactions until the other transaction is completed." `SNAPSHOT` needs `ALLOW_SNAPSHOT_ISOLATION ON` set by the bank [V]. `READ UNCOMMITTED` or `NOLOCK` allows dirty reads, "rows can appear or disappear in the data set" [V]. Rule for AKR-TMS: on a primary with RCSI or snapshot enabled, use them; otherwise use `READ COMMITTED` with short statements, small batches and a short `LOCK_TIMEOUT`, and never use `NOLOCK` for the monitoring feed (a posting that is read dirty and then rolled back would raise a false alert or hide a real one).

**CDC.** CDC "cannot be enabled only on a secondary replica database. Change Data Capture can be enabled on the primary replica database and the changes can be read from the CDC tables using the functions on the secondary replica database"; change tracking "is explicitly disabled on secondary databases" [V]. Debezium reads from a readable secondary with `driver.applicationIntent=ReadOnly`, forces `snapshot.isolation.mode=snapshot`, and then "emits a transaction END event only after it detects the arrival of the first event from a subsequent transaction", which delays END markers on quiet systems [V]. The SQL Server Agent must be running; the capture job's polling interval trades CPU on the database against latency [V]. Debezium's SQL Server connector requires 2016 SP1 or later, Standard or Enterprise [V]. Qlik Replicate lists SQL Server 2014 to 2022 with MS-CDC [S].

**Watermark.** `rowversion` plus `MIN_ACTIVE_ROWVERSION()` is the safe pattern: "If an application uses @@DBTS rather than MIN_ACTIVE_ROWVERSION, it is possible to miss changes that are active when synchronization occurs" [V]. It only works if the CBS tables have a `rowversion` column, which most vendors do not add [K].

**Traps.** Collation and code page for Arabic (`Arabic_CI_AS`, `SQL_Latin1_General_CP1256_CI_AS`, `varchar` versus `nvarchar`) [K]; `datetime` rounding to 3.33 ms; `money` types; identifiers with spaces.

### 3.3 PostgreSQL

**Reading a replica.** A hot standby accepts read queries. The PostgreSQL manual [V]: "The Serializable transaction isolation level is not yet available in hot standby", so use `REPEATABLE READ READ ONLY`. A query that conflicts with replay is cancelled after `max_standby_streaming_delay` (default 30 s [K]); the alternative, `hot_standby_feedback`, "will delay cleanup of dead rows on the primary, which may result in undesirable table bloat". The AKR-TMS reader must therefore use short statements and retry on conflict cancellation (SQLSTATE 40001 or 57014 class) without alarm.

**CDC.** Logical decoding is available on a standby from PostgreSQL 16: "A logical replication slot can also be created on a hot standby. To prevent VACUUM from removing required rows from the system catalogs, hot_standby_feedback should be set on the standby... It's highly recommended to use a physical slot between the primary and the standby"; creating the slot "may need to wait for some activity to happen on the primary" [V]. Publications cannot be created on a standby, so they must exist on the primary [S, Decodable/Morling]. PostgreSQL 17 adds failover slots, which Debezium 3.0.5 and later can request with `slot.failover=true` [S]. Debezium's PostgreSQL connector can run read-only incremental snapshots (no write to a signalling table), unlike Oracle or SQL Server [V].

**Rust-native option.** Supabase ETL (Apache-2.0, "high-performance Postgres replication, written in Rust", with a note that it is under active development before a first stable release [V, GitHub]) can read a publication; it is relevant only when a CBS or channel database is PostgreSQL.

### 3.4 IBM Db2 (LUW; z/OS in older state-bank estates)

**Reading a standby.** HADR "reads on standby" has a hard restriction: "Only the uncommitted read (UR) isolation level is supported on the standby", and applications that ask for a higher level receive an error [V, IBM documentation for 11.1; confirm for 11.5 and 12.1]. If the replay-only window is enabled (`DB2_HADR_ROS_AVOID_REPLAY_ONLY_WINDOW=OFF`), the standby "is inaccessible to all application connections during the replay of DDL log records or maintenance operations" [V]. The feature is not supported in pureScale [V]. Consequence: a read on the standby can show rows from a transaction that later rolls back. AKR-TMS must mark every row read at UR as provisional until a later read confirms it, or read the primary under cursor stability at low volume.

**CDC.** Debezium needs SQL Replication (ASN capture and apply programs, control schema, change-data tables) and, per its documentation, a separate IBM InfoSphere Data Replication licence; tested on Db2 for Linux; Db2 for z/OS support is incubating and excludes z/OS versions with 6-byte LSNs; SQL Replication does not support BOOLEAN columns [V]. Qlik Replicate lists Db2 LUW 10.5, 11.1 and 11.5 as source [S].

**Where it appears.** Temenos Transact is documented in community posts as running on Db2 with the XML record layout (section 11.1) [S]. Db2 on z/OS in an Iraqi state bank is possible but not confirmed anywhere in the research so far [K].

### 3.5 Other engines and stores

- **MySQL and MariaDB** [V, Debezium]: the connector "can follow one of the primary servers, or one of the replicas (if that replica has its binlog enabled)"; GTIDs enable read-only incremental snapshots using executed GTID sets as watermarks. Local Iraqi systems on MySQL or MariaDB are plausible [K].
- **SAP Sybase ASE** (iMAL in older estates, per file 17): no maintained pure-Rust driver; ODBC through SAP's driver or FreeTDS, or a bank-side replica on another engine [K]. CDC options exist only through SAP replication tooling the bank would have to own [K].
- **Db2 for i** (Misys Equation): ODBC through the IBM i Access driver; CCSID for Arabic text; packed and zoned decimals; journaling is the native change log, so a bank-delivered journal extract is the realistic R3 or R4 route [K].
- **Multi-value and proprietary stores** (the jBASE file system under Temenos, proprietary core builds, Progress, FoxPro, Access) [K]: usually no safe SQL path for ad hoc reads. Use R4 (bank-delivered extracts or an operational data store).

### 3.6 Engine capability matrix (summary)

| Engine | Replica read | Isolation seen on the replica | CDC from a standby | CDC licence | Safe watermark primitive | Biggest trap |
|---|---|---|---|---|---|---|
| Oracle | Active Data Guard (licensed option) | Read-consistent to the standby apply point | Debezium physical standby (incubating); Qlik Log Reader | LogMiner free; XStream and GoldenGate licensed | CBS columns; not `ORA_ROWSCN` | Mounted DR standby cannot be read; old releases |
| SQL Server | Readable secondary (Enterprise) | Snapshot, hints ignored | CDC on primary, read change tables on secondary | Included (Standard or Enterprise from 2016 SP1) | `rowversion` with `MIN_ACTIVE_ROWVERSION()` | 14-byte row overhead and blocked ghost cleanup on the primary |
| PostgreSQL | Hot standby | Read committed or repeatable read (no serializable) | Logical slot on PG16+ standby | Free | CBS columns; commit timestamps if enabled | Query conflicts; slot retention |
| Db2 LUW | HADR reads on standby | UR only (dirty reads) | Not from the standby in practice | IIDR for SQL Replication | Row-change timestamps if defined [K] | Dirty reads; replay-only window |
| MySQL, MariaDB | Replica with binlog | Repeatable read [K] | Yes, from a replica | Free | GTID | Binlog format must be ROW [K] |
| SAP Sybase ASE | Replication Server standby [K] | Locking reads [K] | Via SAP tooling only [K] | Customer-owned [K] | Timestamp columns if defined | Driver gap in Rust |
| Db2 for i | Remote journal replica [K] | Commitment control levels [K] | Journal-based [K] | HA tooling licences [K] | Journal sequence number [K] | CCSID, packed decimals |
| Multi-value, proprietary | None | n/a | n/a | n/a | n/a | No SQL path |

### 3.7 AKR-TMS hooks

- **Engine profile registry**: one record per engine, version and edition with feature flags (replica readable, CDC allowed, snapshot isolation allowed, row-version column present) filled by the discovery step; the extractor picks its read-consistency policy from it:
  - Oracle: one `SET TRANSACTION READ ONLY` per extraction batch, batch size bounded by the bank's undo retention.
  - SQL Server: snapshot or RCSI if enabled; otherwise short read-committed statements with a lock timeout; never `NOLOCK`.
  - PostgreSQL: `REPEATABLE READ READ ONLY`; retry on recovery-conflict cancellation.
  - Db2: UR on a standby produces rows flagged `provisional`, confirmed by a later read.
- **Character-set round-trip test** at install: Arabic letters, tashkeel, Arabic-Indic and Persian digits, Kurdish letters, mother's-name examples; must come back byte-identical after normalisation.
- **Licence questions on the discovery sheet** (is Active Data Guard licensed? is GoldenGate licensed? is the standby open?) so that a design never depends on a purchase the bank has not made.

---

## 4. CDC and replication tooling: Debezium, GoldenGate, Qlik Replicate and others

### 4.1 Debezium

- **What it is.** Open source (Apache-2.0), current stable release 3.7 (29 September 2026); connectors need Java 17 or later, and Debezium Server, the Operator and the Outbox extension need Java 21 or later [V, debezium.io/releases]. Connectors: Oracle, SQL Server, Db2, PostgreSQL, MySQL, MariaDB and others; an incubating YashanDB source was added in 3.6 [S].
- **Runtimes.** Kafka Connect, the embedded engine (Java), and **Debezium Server**, "a ready-to-use application that streams change events from a source database directly to messaging and storage systems without requiring Apache Kafka". Its sinks include Kafka, Redis Stream, NATS JetStream, HTTP client, Pulsar, RabbitMQ, Kinesis, Pub/Sub, JDBC and Iceberg [V]. This matters for the DBS stack, which avoids Kafka (file 32 section 5): Debezium Server can feed NATS JetStream or Redis Streams, or call an HTTP endpoint of an AKR-TMS ingestion service, with no Kafka and no ZooKeeper.
- **Snapshots.** Initial and incremental snapshots exist, but incremental snapshots on Oracle and SQL Server write to a signalling table, which breaks level-1 read-only. AKR-TMS should do its own initial load (flashback or snapshot-isolated reads) and use Debezium only for the stream, or pass `snapshot.mode=no_data`-style settings after its own load [K on exact property names; verify against the release in use].
- **Position and failover.** Offsets are stored by the runtime; after a failover to a physical standby "the presence of consistent SCN values ensure that the connector can find the last processed SCN value" [V]. Position loss means a re-snapshot, so the reconciliation loop (section 8) is what tells AKR-TMS that a gap exists.
- **Cost of running it.** A JVM process per source database, memory sized to the largest in-flight transaction (Oracle uncommitted mode), and a schema-history store. This is the main "foreign body" in an otherwise Rust stack (section 4.6).

### 4.2 Oracle GoldenGate

Oracle's own, deep and fast, and expensive. The "Free" edition is Oracle-to-Oracle and limited to 20 GB, so it is not an option for a bank [S]. GoldenGate is justified only if the bank already owns it for another purpose and offers the trail files as an R3 feed.

### 4.3 Qlik Replicate (formerly Attunity)

Commercial, agentless log readers for Oracle, SQL Server, Db2 LUW and others [S]. For Oracle with Data Guard it supports only its own Replicate Log Reader [V]. Appears in the market as the tool a bank's data team already runs for a warehouse feed; if it does, AKR-TMS should accept its output (an R4 landing area) rather than bring a second CDC product.

### 4.4 OpenLogReplicator

An open-source (AGPL-3.0) redo-log reader that Debezium can use as its Oracle adapter [V]. Interesting for banks on Oracle without GoldenGate licences and without Active Data Guard, because it reads archived and online redo directly, but it is a young project with one principal maintainer [K], and AGPL bundling needs legal review. Treat as a lab option, not a baseline.

### 4.5 Others

IBM InfoSphere Data Replication (IIDR), Striim, Fivetran HVR, AWS DMS (cloud only) and Maxwell are all [K] and none is an on-prem default. For PostgreSQL sources, Supabase ETL is a Rust-native choice (3.3).

### 4.6 Where a non-Rust CDC component is justified (feeds file 32)

| Source | Use | Reason |
|---|---|---|
| Oracle or Db2 at a bank that agrees to log-based CDC | Debezium Server as a sidecar container, sinking to NATS JetStream or an HTTP endpoint | No maintained Rust crate reads Oracle redo or Db2 ASN change tables; writing a LogMiner client in Rust would duplicate years of Debezium edge-case handling (transactions, schema evolution, LOB columns) |
| SQL Server with CDC enabled | Optional: read the `cdc.*_CT` tables with a Rust TDS client | The change tables are ordinary tables read with `fn_cdc_get_all_changes_*`; no JVM needed |
| PostgreSQL | Optional: Rust-native replication client | Logical replication protocol is simple enough for a Rust client |
| Everything else | Polling in Rust | No JVM |

### 4.7 AKR-TMS hooks

- **CDC adapter as a container image in the same Harbor registry**, versioned with the Rust services, running as a separate K3s deployment with its own resource limits and network policy (it is the only component allowed to talk to the CBS in R3).
- **Normalising output**: whatever the source (poll, Debezium event, vendor file), the adapter emits one internal envelope: `source_id`, `table`, `op` (insert, update, delete, snapshot), `key`, `before`, `after`, `commit_ts`, `position`, `batch_id`, `checksum`.
- **Licence and legal gate** in the product's bill of materials: Debezium (Apache-2.0), OpenLogReplicator (AGPL-3.0), GoldenGate and IIDR (customer-owned licences), Oracle Instant Client and IBM CLI driver redistribution terms (file 32 section 19).

---

## 5. Load on the CBS, resource governance and the bank's kill switch

A monitoring product that slows the core banking system will be switched off by the bank's IT within a week, and rightly. Load must be designed, budgeted, signed by the bank's DBA and enforced on both sides.

### 5.1 Where the load comes from

| Source | When | What drives it | Control |
|---|---|---|---|
| Initial history load | Once per bank, and again after a re-onboarding | Years of postings: tens to hundreds of millions of rows | Date-partitioned slices, checkpoints, a bank-approved window, throttling, replica where available |
| Steady polling | Continuously | Number of tables times poll frequency | Index-supported watermark predicates, bounded batches, adaptive back-off |
| End-of-day (EOD, close of business) | Nightly | The CBS batch holds locks, accrues interest, rolls the date | "EOD gate" (section 6.5) |
| Reconciliation queries | After each batch and nightly | Aggregates over day partitions | Run on the replica; aggregate on indexed date columns only; incremental (today plus lookback), full history only monthly and off-peak |
| Re-extraction after an incident | Ad hoc | Size of the gap | Automatic but budget-limited; large gaps wait for a window |
| CDC capture | Continuously | Redo or log volume (Oracle supplemental logging "increases the volume of the Oracle redo logs" [V]) | The DBA's job (1.2) |
| Discovery and metadata reads | At onboarding | Catalogue scans | Light, once |

Arithmetic the discovery step should do for every bank: postings per day times days of history gives the backfill size. Example: 100,000 postings a day for five years is 182.5 million ledger rows; at 5,000 rows a second the pure read time is about 10 hours, and at a polite 500 rows a second it is about four days of windows. The throttle, not the network, decides the go-live date.

### 5.2 The load budget: a record the bank's DBA signs

Stored per bank, admin-managed, audited:

| Field | Default (adjust per bank) |
|---|---|
| Maximum concurrent sessions | 2 (4 for backfill windows) |
| Maximum statement time | 30 s (120 s in backfill windows) |
| Maximum rows per fetch / per batch | 5,000 / 50,000 |
| Maximum rows per minute | set from the DBA's measured headroom |
| Allowed windows | By calendar; Iraq's working week is Sunday to Thursday [K], the CBS holiday table is the authority |
| EOD blackout | From the CBS EOD start until EOD complete plus a safety margin |
| Replica-lag ceiling | Stop reading the replica above a set lag (for example 120 s) and alarm |
| Slow-down triggers | Probe-query latency above twice baseline; lock timeouts; resource-busy errors |

The bank-side backstop is the database's own governor. Oracle Resource Manager can "create an active session pool", can detect "when a session or call consumes more than a specified amount of CPU, physical I/O, logical I/O, or elapsed time, and then automatically either terminating the session or call, or switching to a consumer group with a lower resource allocation", can "prevent the execution of operations that the optimizer estimates will run for a longer time than a specified limit", and can "limit the amount of time that a session can be idle" [V, Oracle Database 19c Administrator's Guide]. SQL Server Resource Governor and Db2 Workload Manager are the equivalents [K]. PostgreSQL has role-level `statement_timeout`, `idle_in_transaction_session_timeout` and `default_transaction_read_only`, which is exactly how the DBS provisions its own read-only reporting role (migration 0019, "defence in depth over the grants") [V, DBS repository].

### 5.3 Adaptive behaviour inside AKR-TMS

- **AIMD controller** per extractor: additive increase of batch size while probe latency and replica lag are healthy; multiplicative decrease on slowness, lock timeouts, resource-busy, snapshot-too-old or recovery-conflict errors; hard floor and ceiling from the load budget.
- **Keyset pagination** (`WHERE (wm, pk) > (:w, :k) ORDER BY wm, pk FETCH FIRST n ROWS ONLY`), never `OFFSET`; no long-lived cursors; bind variables (avoids hard-parse floods on Oracle [K]); explicit column lists through bank-approved views.
- **No XML shredding on the CBS.** Temenos tables store each record as XML in one column (section 11.1). Extracting fields with `extractvalue` or `XMLTABLE` inside the query makes the CBS server parse every record; community posts already warn that the function is slow [S]. Fetch the raw XML text and parse it in AKR-TMS (a streaming XML parser in Rust), so the CBS pays for one indexed range scan and nothing else.
- **Error taxonomy**: busy or timeout errors back off and retry; "snapshot too old" shrinks the batch; "object does not exist" or "permission denied" stops the table and raises a mapping alarm (the CBS schema changed or a grant was removed); authentication failure means "paused by the bank" (5.4).

### 5.4 The kill switch belongs to the bank

The DBA can stop AKR-TMS instantly: lock the account, remove the grant, close the firewall rule. AKR-TMS must treat that as a normal state, not a fault:

1. Authentication or authorisation failures move the source into **paused-by-bank**, with a long exponential back-off ceiling (no tight retry loops that would trigger the bank's own lockout policies or fill logs).
2. A **coverage gap** opens at the last good watermark and is shown to the MLRO and to AKR-TMS operations as "monitoring blind for source X since time T". The gap is itself reportable (file 04 B-01).
3. On resume, the gap is backfilled automatically from the last good watermark within the load budget, and the gap closes only when reconciliation (section 8) passes.
4. The contract says plainly: the bank may pause at any time; the platform shows the gap; the bank owns the decision to monitor blind.

### 5.5 AKR-TMS hooks

- **Load budget screen** and API, signed off by a named bank DBA, versioned and audited; every extractor reads the active version.
- **Self-measured impact**: each extractor records its own query durations, rows per second and error classes to Prometheus, so the bank's DBA and AKR-TMS support can show who was running what at the time of any CBS slowdown.
- **Backfill planner**: given row estimates and the load budget, proposes slices and windows and shows the calendar days to completion before anyone starts.

---

## 6. Isolation levels, consistency and the CBS business day

### 6.1 Why a monitoring system cares

A monitoring system joins data that the CBS wrote at different moments (ledger, balances, master data, holds) and then decides about people. Three failure modes follow: reading half a transaction (read skew across tables), reading something that never became real (a dirty or provisional row), and assigning an event to the wrong day (business date versus calendar date).

### 6.2 Read-consistency policy per batch

Defined per engine in 3.7: one read-only transaction per Oracle batch; snapshot or read-committed-snapshot on SQL Server, otherwise short read-committed statements and never `NOLOCK` (a dirty read of "rows can appear or disappear" [V]); `REPEATABLE READ READ ONLY` on PostgreSQL; unconfirmed rows flagged `provisional` on a Db2 standby (UR only [V]). Across tables, AKR-TMS does not pretend to have a global snapshot; it records the `as_of` marker of each batch and lets reconciliation (section 8) catch skew.

### 6.3 Status lifecycle: attempted, pending, posted, reversed

The CBI wants STRs that cover attempted transactions of any amount (file 01) and unusual-transaction reviews of what was tried as well as what was posted. CBS tables mix states: in-flight or unauthorised records awaiting a checker, held or earmarked amounts, authorised and posted entries, reversals, rejected and cancelled items [K; Temenos keeps unauthorised records in a separate `$NAU` file and superseded versions in `$HIS`, FLEXCUBE carries authorisation and record-status flags on master records, Finacle has posted and deleted flags on transaction rows, section 11]. The canonical model therefore stores **transaction events with a lifecycle**, not only final ledger lines:

`initiated -> pending_authorisation -> authorised -> posted -> (reversed | corrected)`, plus `rejected` and `cancelled`, each with its own timestamp, actor and source row. Scenarios choose which states they count (structuring counts posted cash deposits; an "attempted wire to a listed party" scenario counts initiated and rejected items).

### 6.4 Business date, value date, booking date, calendar date, time zone

- Every CBS has a **business date** (T24 "TODAY", FLEXCUBE branch date, Finacle business date) that differs from the wall-clock date during end of day and on weekends and holidays [K]. Postings carry a **booking date** (when posted), a **value date** (when interest and availability start, possibly in the past: back-valued entries) and sometimes an **input time**. All three go into the canonical model.
- Iraq uses UTC+3 all year (no daylight saving since 2008) [K]; the Kurdistan Region uses the same [K]. CBS timestamps are usually zone-less local time. Store `source_local_ts` exactly as read, the bank's profile zone, and a derived UTC value; never convert silently.
- The CBS holiday table is the calendar authority; AKR-TMS reads it rather than keeping its own, and reuses the DBS business-calendar machinery for its timers (file 32).

### 6.5 End of day and the "EOD gate"

EOD (also "close of business", COB) is a batch that accrues interest and profit, posts charges, rolls the GL, recomputes balances and advances the business date. During it, tables are locked or rewritten, balances change without matching postings in the same instant, and back-valued items appear [K].

**EOD gate**, per source:

1. Detect EOD from, in order of preference: the CBS's own status or date table, a bank-provided schedule, or a change-rate signature (rows modified per minute above N times baseline).
2. While the gate is closed: pause balance and aggregate reads; keep reading new postings with a larger overlap; stop reconciliation for the open day.
3. When the business date advances, run the **day-close reconciliation** for the day that just closed (section 8, levels L1 to L3) and issue its coverage certificate.
4. A posting that appears in an already closed day (back-valued or late) emits a **day-reopen** event so the affected accounts' windows are recomputed (section 7.4).

### 6.6 Branch-level dates

Some CBSs keep a business date per branch (Oracle FLEXCUBE does at least in some modules [K]). The branch dimension then carries its own date, and "yesterday" is branch-relative.

### 6.7 Internal consistency in AKR-TMS

Landing into PostgreSQL follows the DBS golden rule (same-transaction enqueue): in one transaction AKR-TMS writes the raw rows, upserts canonical rows, advances the watermark and enqueues the follow-up jobs (feature computation, scenario evaluation). A crash replays the batch, and idempotent upserts (7.7) make the replay harmless.

### 6.8 AKR-TMS hooks

- **Per-batch as-of record**: SCN, rowversion, LSN, snapshot timestamp or "none" with the extraction start and end, stored with the batch receipt.
- **Lifecycle-aware canonical transaction** with all dates and the zone, plus a "scenario view" per scenario saying which lifecycle states it counts.
- **EOD gate** with a per-bank detector configuration and a visible state on the integration dashboard.

---

## 7. Inserts, updates, deletes, reversals; watermarks; late data; idempotent loading

### 7.1 Four behaviours of CBS tables

| Behaviour | Typical tables | What AKR-TMS must do |
|---|---|---|
| Append-only ledger, corrections by reversal | Posting or statement-entry tables | Read new rows; link reversals to originals; never expect updates to old rows [K] |
| Update-in-place master data with a version counter | Customer, account, limits, signatories | Read by change timestamp or version; keep history of every version; nightly key-set reconciliation |
| In-flight or staging tables with deletes on completion | Unauthorised entries, queues, holds | Read the authorised state; treat disappearance as a state change, not a delete of history |
| Archive and purge jobs that move or drop old rows | Online ledger tables, partitions | Never treat absence as deletion; archived rows must already be in AKR-TMS |

### 7.2 Watermark strategy by table class

| Table class | Strategy |
|---|---|
| Reference and code tables (branches, currencies, product codes, holidays) | Full read daily and compare a row hash; small |
| Master data (customer, account) | Version or last-change column with overlap; hourly; plus a nightly key-set reconciliation to catch rows the column missed |
| Ledger postings | Monotonic key plus posting timestamp plus overlap (7.3); daily count and sum reconciliation |
| Balances | Snapshot by business date after EOD; intraday balance derived from the ledger and checked against the snapshot |
| Payment messages | Message id or sequence plus creation time; counts against RTGS or SWIFT logs (section 12) |

Preference order where several exist: CDC position, then a row version with a safe minimum (`MIN_ACTIVE_ROWVERSION()` on SQL Server [V]), then a monotonic key, then a last-update timestamp, then a full compare.

### 7.3 The commit-order problem and the lookback window

A transaction can take a key, a timestamp or a row version early and commit late. A poller that remembers "highest value seen" then skips it forever. Microsoft's documentation states the effect plainly: with `@@DBTS` instead of `MIN_ACTIVE_ROWVERSION`, "it is possible to miss changes that are active when synchronization occurs" [V]. Oracle's `ORA_ROWSCN` is only an upper bound, "any value greater than or equal to [the commit SCN] could be returned" [V].

Where the engine offers no safe minimum, AKR-TMS uses a **lookback window** `L`: every poll re-reads the last `L` of the watermark range, and idempotent upserts absorb the duplicates. Choose `L` at least as large as the longest transaction the CBS allows, plus clock skew, plus replica lag. Measure it: record the delay between a row's own timestamp and the moment AKR-TMS first saw it, track the 99.99th percentile per table, alarm when a row arrives later than `L`, and widen `L` automatically after such an event. The cost of a lookback is a re-read of recent rows through an index, which is cheap.

### 7.4 Late data and the scenario engine

- Scenarios evaluate an event at its **settled time** (event time plus a settling delay `D`, for example two minutes) so that most late rows are already in. A fast path can evaluate immediately for high-severity scenarios and be re-evaluated at the settled time.
- A row that arrives after settling is marked `late = true`, triggers a **recompute of the affected account windows**, and feeds a data-quality metric.
- If a late row changes an alert decision (creates an alert that should have existed, or removes the basis of an existing one), the alert is **amended with an explanation**, never silently changed. Every alert stores the data-as-of at the time of decision (bi-temporal model, section 13).

### 7.5 Deletes and purges

- AKR-TMS never deletes monitored data because the source did: retention is governed by AKR-TMS's own retention classes (5 years from account closure per the CBI, 10 years option for sanctions-related records per OFAC; files 01, 05, 30).
- **Key-set reconciliation** finds hard deletes and missed rows: for each table and day partition, compare count plus an order-independent hash (sum of per-row hashes modulo 2^64) between source and landing, and on mismatch bisect by key range to locate the differing keys. Rows missing in the source are marked `source_deleted` (tombstone) and kept; rows missing in AKR-TMS are re-fetched.
- **Archive and purge detection**: a sudden drop in source row counts for old partitions with matching landing counts is an archive event, logged as such, not a finding.
- **CBS time regression**: when the bank restores its CBS to an earlier point, or fails over to a DR copy that lost recent commits, the source's high-water mark (highest key, row version or posting time per table) falls **below** AKR-TMS's watermark and the day's counts and hashes disagree. The loader treats this as a `source_regression` event: it stops advancing the watermarks of that source, raises a priority-one notice to the MLRO and the bank's DBA, keeps the rows that no longer exist in the source (marked `source_deleted` only after key-set reconciliation confirms they are gone), re-opens reconciliation from the earliest affected business date, and reports the window as an inconsistent period in the coverage statement (8.2; file 30 section 16.5).

### 7.6 Reversals and corrections

Reversal and correction entries are linked to the original (`reverses_id`, `corrected_by_id`) in the canonical model. Scenario logic nets them where appropriate (structuring should ignore a reversed deposit); a reversal after an alert is itself a signal worth showing to the investigator.

### 7.7 Idempotent loading

Natural key: `(source_id, source_table, source_pk)`. Change detection: `source_version` (version counter, row version or posting sequence) and `row_hash` over the business columns. The upsert only moves forward:

```sql
INSERT INTO canon.txn_raw (source_id, source_table, source_pk, source_version, row_hash, payload, extracted_at, batch_id)
VALUES (...)
ON CONFLICT (source_id, source_table, source_pk) DO UPDATE
   SET source_version = EXCLUDED.source_version, row_hash = EXCLUDED.row_hash,
       payload = EXCLUDED.payload, extracted_at = EXCLUDED.extracted_at, batch_id = EXCLUDED.batch_id
 WHERE EXCLUDED.source_version > canon.txn_raw.source_version
    OR (EXCLUDED.source_version = canon.txn_raw.source_version AND EXCLUDED.row_hash <> canon.txn_raw.row_hash);
```

Every accepted change also appends to a history table so the bank can show what the source said when. The raw landing layer keeps the original row image (partitioned by day) so mapping bugs can be fixed and replayed without going back to the CBS.

### 7.8 AKR-TMS hooks

- **Watermark and lookback registry** per table with measured late-arrival percentiles and automatic widening.
- **Key-set reconciliation job** with bisection and tombstones; archive-event classifier.
- **Late-arrival ledger**: every late row, its delay, and the alerts it amended, reportable to the MLRO.
- **Replay tool**: re-run mapping from the raw landing layer for a date range, producing a diff of canonical changes before it is applied.

---

## 8. Reconciliation and control totals: proving completeness

The requirement is stated three times in the evidence base: BCBS (monitoring must cover all accounts and transactions; file 04 B-01 asks for "daily reconciliation of ingested record counts and sums against the CBS, with any gap shown as a control failure"), NYDFS Part 504 (data extraction and loading that "ensure a complete and accurate transfer from source systems"; file 05 section 1f), and the CBI's 2026 internal-control item on "validity, completeness and accuracy of data" (1.1). The BCBS 239 principles on risk data aggregation give the vocabulary: accuracy and integrity (principle 3), completeness (4), timeliness (5), adaptability (6) [K: the BIS PDF could not be retrieved in this session; titles quoted from memory]. A system that cannot show, per source and per day, that it holds everything the CBS posted has no standing to say it monitored "all" transactions.

### 8.1 The reconciliation ladder

| Level | What is compared | How | Frequency | What a failure means |
|---|---|---|---|---|
| L0 Batch receipt | Rows read versus rows landed in each batch | Manifest: source id, table, `as_of`, watermark range, row count, order-independent hash of row hashes, query hash, start and end time; signed by AKR-TMS and stored | Every batch | Transport or mapping loss inside AKR-TMS |
| L1 Table and day | Count and sum of debit and credit amounts per source table per business date, source versus landing | Aggregate query on the source (replica, indexed date column) and on the landing table | After each batch for today minus settling; nightly for the previous N days (N at least the lookback); monthly for full history | Missed, duplicate or altered rows |
| L2 Account continuity | Opening balance plus postings equals closing balance, per account per day | Needs a balance-snapshot table (for example Finacle's end-of-day balance table, FLEXCUBE's account-balance history [K]); computed in AKR-TMS from landed data | Nightly after day-close | Missing or late postings for that account, or balance changes that bypass the ledger |
| L3 General ledger tie-out | Sum of customer balances by product equals the control account in the GL; debits equal credits per day | Read the CBS trial balance or GL balance tables | Nightly after day-close | Incomplete extraction, or a CBS-internal imbalance (a finding for the bank, not for AKR-TMS) |
| L4 Payment tie-out | Messages in the SWIFT or RTGS or ACH logs versus payment records in the CBS and in AKR-TMS | Count and amount by day, message type and direction | Daily | Payment data missing from the monitored set (section 12) |
| L5 Sequence gaps | Voucher numbers, SWIFT sequence numbers, ATM trace numbers | Gap detector per monotonic sequence | Continuous | A hole in a sequence is a missing record or a CBS fault |
| L6 Cross-system | Nostro statements (MT950 or camt.053) versus ledger | The bank's own reconciliation output, if exposed | Daily | Outside AKR-TMS scope but a rich data-quality signal |

### 8.2 Outcomes and the coverage statement

Each (source, business date) ends in one of four states: **reconciled**; **open difference** (inside the lookback window, being retried); **confirmed gap** (triggers an automatic re-extraction of the affected range); **source anomaly** (the CBS itself is internally inconsistent, for example an unbalanced day: a finding for the bank, recorded and escalated, not "fixed").

The MLRO dashboard shows a **coverage statement**: the percentage of accounts and of transactions covered per source, the data-as-of time, and every **blind period** (windows with no or incomplete data, including bank-initiated pauses, 5.4). A case that investigates activity inside a blind period shows a banner saying so. Each night the system signs a **daily coverage certificate** (hash of that day's reconciliation results) and stores it in the evidence vault; the year's certificates feed the annual certification pack (file 05 section 1f), the CBDDQ evidence pack (file 04 section 2c) and the CBI-approved assessor's request (file 01 section 2e).

### 8.3 Data-quality rules that run with the load

Beyond counts: null or default values in fields that scenarios depend on (customer id, currency, counterparty name, channel); invalid currency or branch codes against the reference tables; referential integrity (account to customer, transaction to account); placeholder dates (for example 1900-01-01 or 9999-12-31) and future-dated postings; sign and decimal-shift outliers (an amount 1,000 times the account's history); IBAN checksum failures on counterparty data (the Iraqi IBAN is 23 characters: `IQ` + 2 check digits + 4-letter bank code + 3-digit branch + 12-digit account [S, bank-code.net and the CBI payment-systems statement]); and mojibake (Arabic text containing replacement or Latin-1 look-alike characters). Each rule has an owner, a threshold, and a place in the data-source inventory (file 05 section 1f: "identification of all data sources").

### 8.4 Explained differences register

Some differences are legitimate: IQD rounding at fractional units, suspense and clearing accounts, items the CBS excludes from its own reports. A **known-adjustments register** (bank-signed, expiring, with a reason) lets reconciliation pass with an explanation instead of a silent tolerance. Tolerances are never global; they are per table and per reason.

### 8.5 AKR-TMS hooks

- **Reconciliation service** as a first-class bounded context (own tables, own jobs, own UI), not a script: ladder definitions are data (admin-managed), results immutable, certificates signed with an OpenBao transit key and anchored like the DBS audit trail.
- **Gap-to-backfill automation** with the load budget respected (5.2); a confirmed gap above a size threshold waits for a bank-approved window and shows on the dashboard until closed.
- **Coverage report** for CBI inspection: per source and per day, with the blind-period list and the reasons.
- **Bisection tool** for analysts and support: pick a source, a day and a table; see where the key sets diverge.

---

## 9. Latency targets and freshness classes

### 9.1 What "near-real-time" has to mean

The CBI asks for "instant, real-time follow-up" (file 01). Over a read-only link the honest definition is: data in AKR-TMS within about a minute of the CBS commit (poll tiers) or within seconds (CDC tier), alerts within minutes, and real-time interdiction only through the phase-2 pre-release API. Wolfsberg's guidance on sanctions screening says the same about scope: "Screening cross-border payments prior to completing the transaction is common practice and known as screening in real-time. By contrast, screening domestic payments in real-time may be unnecessary for FIs that are subject to the same local regulatory requirements" [V, Wolfsberg Sanctions Screening Guidance 2019, section 5.1]. EU practice for instant payments goes further: Article 5d of Regulation (EU) 2024/886 requires verification of the bank's own customers against targeted financial sanctions "immediately after the entry into force of any new" measure "and at least once every calendar day" and forbids a further per-transaction check in the instant-payment flow, because Article 5a gives the payee's bank 10 seconds to make the funds available [V, EUR-Lex]. The design lesson: keep the synchronous path short by moving what can be pre-computed (customer screening, risk rating, counterparty history) out of it.

### 9.2 Freshness classes

| Class | Used for | Target (CBS commit to evaluable in AKR-TMS) | How it is met |
|---|---|---|---|
| F0 Pre-release (phase 2) | Screening and scoring of a payment before release | p95 150 ms, p99 400 ms, service-side (budget in section 15) | Local list index and pre-computed features; no CBS access |
| F1 Near-real-time monitoring | Per-event scenarios, account-activity rules | Ingest p95 under 60 s and p99 under 3 min on poll tiers; p95 under 10 s on CDC; alert evaluation at ingest plus 5 s on the fast path, re-evaluated at the settled time (7.4) | Poll interval 15-30 s, overlap, small batches |
| F2 Intraday aggregates | Rolling windows, velocity, risk re-rating triggers | Hourly | Incremental feature jobs |
| F3 End of day | Daily cash, structuring and "expected versus actual" scenarios, day-close reconciliation, regulatory daily lists | Within 60 minutes of the CBS EOD completing | EOD gate (6.5) |
| F4 Periodic | CBI monthly reports (liaison officer report by the 10th), KYC refresh queues, board packs | Calendar | Scheduler with the bank's calendar |
| F5 Reference data | Sanctions and PEP lists | CBI controls 2025 require lists updated "every 12 hours at least" [file 01 2d]; the CBI sanctions guidelines of 1 August 2023 require lists to be kept and updated "continuously (24 hours)" and the **customer database to be screened daily** against them [V, Arabic scan, pp. 20 and 23]; AKR-TMS aims at 1-4 hours and re-screens customers on every change (BCBS para 50, file 04 B-09) | List ingestion with diff, delta re-screen |

### 9.3 A latency budget for a poll tier (illustrative)

| Step | Time |
|---|---|
| Replica lag (SQL Server readable secondary: "usually only a matter of seconds" [V]) | 1-5 s |
| Poll interval (average half of it) | 7-15 s of a 15-30 s interval |
| Query, transfer and mapping | 1-3 s |
| Commit and enqueue | under 1 s |
| Fast-path scenario evaluation | 1-5 s |
| Total p95 | about 25-40 s |

The settled-time re-evaluation (7.4) adds the settling delay on top for scenarios that need it.

### 9.4 Measuring it

Ingest lag per source is `landing_time - source_commit_time`, where the commit time is the CDC commit timestamp, or the row's own posting timestamp when no better value exists. It is published as a histogram per source, alarmed against the class target, and shown to analysts as a **freshness banner** ("data current to 10:32:15"). Lag that exceeds target opens a coverage-gap warning (section 8.2).

### 9.5 AKR-TMS hooks

- **Freshness SLOs per source and per class** as admin-managed data, with burn-rate alerts (the DBS observability pattern: Prometheus histograms, Grafana, Loki).
- **Banner and API field** `data_as_of` on every analyst screen and every exported report.
- **Latency regression test** in the integration test rig: synthetic postings written to a test CBS schema, measured end to end for each tier.

---

## 10. Securing the link: least privilege, segmentation, TLS, OpenBao, bank-side audit

### 10.1 Least privilege per engine

| Engine | Minimum for polling (R1, R2) | Extra for CDC (R3) | Avoid |
|---|---|---|---|
| Oracle | `CREATE SESSION` plus `SELECT` on named views | `LOGMINING`, `SELECT ANY TRANSACTION`, `FLASHBACK ANY TABLE`, `EXECUTE_CATALOG_ROLE`, `LOCK ANY TABLE`, `CREATE TABLE`, `CREATE SEQUENCE`, `SELECT` on `V_$LOG`, `V_$ARCHIVED_LOG`, `V_$LOGMNR_*` and others (Debezium's list [V]) | `SELECT ANY TABLE`, `SELECT_CATALOG_ROLE`, roles inherited from `PUBLIC` |
| SQL Server | A custom role with `SELECT` on named views; `VIEW DATABASE STATE` only if lag is monitored | `db_owner` to enable CDC is a bank DBA act; AKR-TMS reads `cdc.*` functions with `SELECT` | `db_datareader` on the whole database; `sysadmin` |
| PostgreSQL | A role with `SELECT` on named views and `default_transaction_read_only = on`, `statement_timeout`, `idle_in_transaction_session_timeout` (the DBS pattern [V]) | `REPLICATION` attribute and publication ownership by the bank | `pg_read_all_data`, superuser |
| Db2 | `CONNECT` plus `SELECT` on named views | ASN privileges and IIDR (bank DBA) | `DBADM`, `DATAACCESS` |

Rule: SELECT on **views the bank's DBA controls**, never on base tables. The views are the contract: they expose only mapped columns, apply the masking decisions of section 11 of file 30 (card numbers, secrets), and let the bank change a base table without breaking AKR-TMS.

### 10.2 Credentials

- **Prefer no password.** Certificate-based authentication where the engine supports it (PostgreSQL `cert` with `verify-full`; Oracle TLS client certificate or wallet; Db2 with SSL) [K].
- **Where a password is unavoidable**, it lives in OpenBao (KV, versioned; the open-source fork of HashiCorp Vault that AKR-TMS uses, file 32 section 13) under a path only the ingestion workload identity can read; the DBS pattern already gives the api and worker separate Vault tokens with narrow policies, and refuses root tokens in production [V, DBS deploy README].
- **Do not give OpenBao a CBS-side administrative credential** to rotate the password. The database secrets engine (HashiCorp Vault's, inherited by OpenBao) can rotate "static role" passwords automatically, but that needs a root credential able to alter users on the CBS, which no bank should grant to a monitoring product. Vault lists plugins for Oracle, MSSQL, PostgreSQL, MySQL/MariaDB and IBM Db2 [V; edition and external-plugin requirements for Oracle and Db2 unverified]. Instead the bank's DBA (or the bank's privileged-access tool) rotates the AKR-TMS account on the bank's schedule and updates the OpenBao secret; AKR-TMS reloads without restart and alarms if a rotation is overdue.
- **Source restriction**: the account may connect only from the ingestion hosts (Oracle `tcp.invited_nodes`, SQL Server logon triggers or firewall, PostgreSQL `pg_hba.conf` with `hostssl`) [K].

### 10.3 Transport

- TLS 1.2 or later on every CBS connection with certificate validation against the **bank's own CA bundle** (the DBS sets `SSL_CERT_FILE` to a deployment-built trust bundle for exactly this reason [V]); never "trust server certificate". Oracle Net native encryption is an acceptable alternative to TCPS if the bank already uses it [K].
- Verify, do not assume, that the bank's network devices do not strip or downgrade the session (a periodic handshake self-test records protocol, cipher and certificate fingerprint).
- Time: NTP-synchronised hosts on both sides; clock skew is a lookback input (7.3) and an evidence-quality input.

### 10.4 Network segmentation

- A dedicated segment for the AKR-TMS ingestion nodes; firewall rules allow-list the ingestion node addresses to the specific listener ports (1521, 1433, 5432, 50000 or the bank's non-default values) and nothing else; no inbound connection from the CBS network into AKR-TMS except the optional CDC sink and the phase-2 gateway calls (section 15).
- Default-deny egress from AKR-TMS (the DBS decision D6), with an allow-list for list providers only if the bank approves (file 01 section 8: pull in lists, push out nothing).
- The CBS database is never reachable from the analyst UI or from any user workstation segment through AKR-TMS: only the ingestion workers hold a path to it, and they run with no interactive shell.

### 10.5 The bank audits the reader

The bank's DBA should enable the engine's native audit on the AKR-TMS account (Oracle Unified Auditing, SQL Server Audit, pgAudit, Db2 audit facility [K]) and keep it in the bank's SIEM. This gives the bank evidence of exactly what AKR-TMS read, which supports the outsourcing duty in the CBI's 2026 controls (the bank must be able to monitor activities handed to a service provider, and its internal audit and CBI examiners must be allowed to assess them; file 30 section 2).

### 10.6 Tipping-off: do not query one customer at a time

A CBS-side audit or slow-query log shows which keys AKR-TMS fetched and when. If analysts' case work caused AKR-TMS to query the CBS for the one customer under suspicion, a database administrator or a curious insider would learn who is being investigated, which is a tipping-off channel (file 30 section 8). Rule: **AKR-TMS never fetches by customer on demand.** It reads everything in bulk, continuously, by watermark, and serves investigators from its own store. The CBS sees the same bulk pattern whether or not any customer is under review.

### 10.7 Data minimisation at the source

Ask the bank's DBA to expose views without: full card numbers (masked or keyed-hash only, to stay out of PCI scope; file 30 section 11), PIN-related fields, authentication secrets, free-text customer notes unless mapped, document images, and any column no scenario or screening rule uses. If the CBS encrypts fields at application level with keys the CBS holds, AKR-TMS cannot read them: the bank must provide a masked or tokenised view.

### 10.8 AKR-TMS hooks

- **Connection hardening checklist** executed automatically at connect time and re-run daily: TLS version, certificate chain to the bank CA, source IP, account privileges (the read-only attestation, 1.4), password age, audit enabled (if the bank shares that metadata).
- **OpenBao policy templates** per bank: one KV path per source, readable only by the ingestion workload; no database secrets engine against the CBS.
- **"No per-customer CBS query" lint**: an automated test that fails the build if any code path issues a CBS query keyed on a single customer, account or case.

---

## 11. The CBS products used in Iraq and the region, and their database quirks

*Update 2026-10-06 (file 18):* add **Capital Banking Solutions** to the adapter list: CapitalBanker "running on Oracle
technologies" and PowerBanker "running on IBM Power platforms" (vendor pages, Wayback 2022); Iraqi Middle East Investment
Bank since 2006 and Sumer Commercial Bank (2022) are named, and the vendor claims "more than 25 Iraqi" banks. CapitalBanker
follows the Oracle path below. For PowerBanker, "IBM Power" may mean IBM i or AIX: confirm the engine with the bank's
DBA, then use the Db2 path (ODBC or bank extract, C-61) or the matching driver.

Vendor and market facts (which bank runs what, AML modules bundled with each CBS, tenders) belong to files 17 and 18 (written by another agent; the bank-by-bank map is theirs). This section records what the **integration layer** has to handle for each CBS family.

**Build order, from file 17.** ICS BANKS (Oracle; "over 30 active customers" in Iraq by the vendor's own count) is the first adapter family; Path/Azentio iMAL (Oracle or SAP Sybase ASE) and BML Istisharat ICBS (also used by the CBI itself) follow; Oracle FLEXCUBE matters for the state banks (Rafidain was reported to be moving to it); Temenos Transact appears at a few banks (National Bank of Iraq migrated in 2024); the older Misys/Finastra cores at state banks (Db2 for i) need an R4 or ODBC path; Infosys Finacle has no confirmed Iraqi customer and stays low priority. The Temenos section below is first only because its storage model is the most unusual and shapes the generic design. Public documentation of CBS database schemas is thin, because vendors treat them as proprietary: most table-level statements below are [K] or [S] and must be confirmed in each bank's discovery step. The one design conclusion that holds regardless: **mapping must be driven by the bank's own data dictionary and local-field definitions, not by a vendor-generic schema**, because Iraqi KYC data that matters for screening (mother's name, family number, national ID number, tribe, Kurdish-script name) usually lives in bank-defined local fields [K].

### 11.1 Temenos Transact (T24) on TAFJ or jBASE

- **Where it is documented.** Temenos sells "Transact Data Hub" with "Data Event Streaming" (to "Apache Kafka, Amazon Kinesis, and Azure Event Hub"), an "Operational Data Store" that "provides a real-time replication of data from Temenos Transact", and a "Snapshot Data Store", "a curated end of day snapshot of banking data" [V, Temenos brochure 2020]. Those products exist because the core database is not friendly to ad hoc SQL.
- **Storage.** In an RDBMS deployment the application tables carry two columns, `RECID` (the record key) and `XMLRECORD` (the whole record as XML) [S: IBM community thread on Guardium and T24 on Db2: "whatever Guardium is picking from the set of run queries is only two fields, RECID and XMLRECORD"; Qlik community thread with a working Oracle example `SELECT RECID, extractvalue(XMLRECORD,'/row/c26') ... FROM FBNK_ACCOUNT`, describing `RECID` as `VARCHAR2(255)` and `XMLRECORD` as `SYS.XMLTYPE`]. Field values are elements `c<N>` where `N` is the field's position in the application's dictionary; multi-values and sub-values are repeated elements with positional attributes [S: Oracle forum posts on multi-value XML; the page itself could not be retrieved]. The tables are named after the company mnemonic and the application (for example `FBNK_ACCOUNT`) [S]. Engines: Oracle, Db2, SQL Server [S], or the jBASE file system in classic (TAFC) installations, where "the Unix files representing tables... are stubs" when an RDBMS is used and the data otherwise sits in jBASE files [S].
- **Consequences for the mapping layer.**
  1. **Field numbers are release- and bank-specific.** `c26` means "cleared balance" only in one dictionary. The adapter must read the bank's own dictionary (the `STANDARD.SELECTION` application, itself stored the same way) at discovery and on every change, and generate its field map from it, including local reference fields [K].
  2. **Parse XML in AKR-TMS, not in the CBS** (5.3). Fetch `XMLRECORD` as text through an indexed `RECID` range and decode it in Rust.
  3. **Authorised versus pending versus history.** T24 keeps unauthorised records in a parallel file (suffix `$NAU`) and superseded ones in `$HIS` [K]. Read the live (authorised) file for state, read `$NAU` only if the scenario needs attempted or pending items (6.3), and use `$HIS` for field-level history of master data.
  4. **Keys carry meaning.** Transaction ids and statement-entry ids embed prefixes and dates [K]; do not parse them for semantics, but do use them as stable natural keys.
  5. **Dates are `YYYYMMDD` strings, amounts are decimal strings**, and the business date comes from the system's dates record, not the clock [K].
  6. **jBASE.** There is no standard SQL surface for ad hoc reading of jBASE files in the bank's hands [K]. These banks are R4: the bank's batch (close-of-business extract jobs) or Transact Data Hub's ODS writes a relational landing area that AKR-TMS reads. If neither exists, the bank must add one: this is a precondition of the project, not a mapping detail.
- **Event feeds.** Data Event Streaming is a Temenos-side configuration that publishes committed events. It is not a database read, so it is outside the brief unless the bank chooses to deliver the stream into a landing queue (R4).

### 11.2 Infosys Finacle

- **Engine.** Finacle's reference deployment is on Oracle Database; Oracle's 2014 white paper documents Finacle 10.2.13 on an Oracle RAC database [V]. Finacle estates in the region often run old Oracle releases [K].
- **Schema.** Table names are short three-letter or four-letter codes (account master `GAM`, transaction detail history `HTD` and daily detail `DTD`, with account id `ACID` and the printed account number `FORACID`) [K, from practitioner material; no primary document retrieved]. Transaction rows carry posted and deleted flags (`PSTD_FLG`, `DEL_FLG`) and part-transaction serial numbers (`PART_TRAN_SRL_NUM`) because a transaction has debit and credit parts [K].
- **Consequences.** (1) Read posted and non-deleted rows for the ledger and the flag history for attempted or deleted items (6.3). (2) The daily detail table moves into history at end of day, so a poller that reads only history sees nothing for the current day and one that reads only daily detail misses yesterday: the adapter reads both and de-duplicates on transaction id and part serial [K]. (3) Account balance history by end of day is the natural L2 reconciliation source (8.1) [K]. (4) Table names are not self-describing, so the bank's Finacle data dictionary (or the vendor's table description sheet) is a discovery deliverable.

### 11.3 Oracle FLEXCUBE (Universal Banking)

- **Engine.** Oracle Database [K]. The product has long tables with module prefixes: customer and account masters (`STTM_*`), accounting entries and history (`ACTB_*`), message and contract tables per module [K, practitioner and Oracle data-model references; no schema retrieved]. User-defined fields hold bank-specific attributes [K].
- **Consequences.** (1) Authorisation and record status flags on master records mean the adapter must read only authorised states by default and keep the maker-checker history. (2) Per-branch business dates (6.6). (3) The accounting-entry history is the ledger source, with balance history tables as reconciliation sources. (4) SWIFT messages are stored in message tables with the text in large-object columns [K]; read them as text and parse in AKR-TMS (section 12).

### 11.4 Path Solutions iMAL (Islamic core banking)

- **What is public.** A trade-press summary reports that an Iraqi greenfield Islamic bank (named there as Al-Rajih Islamic Bank; the name is unconfirmed, the article itself could not be fetched) signed for iMAL; modules named include core, clearing and settlement, credit facilities, risk management and FATCA, trade finance, personnel and payroll, Islamic invest, Islamic profit calculation, electronic cheque clearing, delivery channels including mobile banking, central bank reporting and ATM management [S]. A trade headline records that Path Solutions "beats ICS Financial Systems to core banking system deal in Iraq" [S], which shows both vendors are competing there. Path (now part of Azentio, file 17 section 3) describes iMAL as an AAOIFI-certified, n-tier suite [S]. A 2017 review quoted in file 17 says iMAL runs on "either Oracle or SAP Sybase as core database servers" [S, via file 17]; older Iraqi iMAL banks may therefore be on Sybase ASE, which tiberius cannot read (ASE speaks an older TDS dialect than SQL Server) and which needs an ODBC driver (SAP's or FreeTDS) or a bank-side Oracle or PostgreSQL replica [K].
- **Consequences.** Islamic product semantics (murabaha, ijara, mudaraba, musharaka, istisna, salam, wakala, profit instead of interest, profit-distribution runs) must appear in the canonical product model (section 13) and in the scenario mapping (file 24). Profit and Sharia-reserve postings are system-generated and must be classified so they do not trigger customer-behaviour rules.

### 11.5 ICS BANKS (ICS Financial Systems, Amman, Jordan)

- **In Iraq.** ICSFS publishes a success story titled "The First and Only Digital Bank in Iraq to Partner with ICSFS": First Iraqi Bank (FIB) went live on ICS BANKS Digital Banking in "less than four months"; customers "open a full bank account, licensed by the Central Bank of Iraq, from your mobile phone, in less than 5 minutes" with an ID, "send and receive money within Iraq instantly", "receive your salary in seconds", and businesses can "create terminals as sub-accounts" so employees act as PoS terminals with "limited cash services" [V, icsfs.com]. Each of those is an AML data fact: remote onboarding on one ID, instant domestic transfers, salary inflows, and sub-account terminals that behave like agent points. ICSFS also states that it obtained ISO/IEC 27001:2022 certification for its information security management system in 2025 [V]. A trade headline records that Path Solutions "beats ICS Financial Systems to core banking system deal in Iraq" [S], so both vendors compete for Iraqi banks.
- **Engine.** ICS Financial Systems presents itself as an Oracle ISV partner, describes "SOA Architecture", and publishes work on ICS BANKS with Oracle Exadata ("ICSFS 27X faster with Oracle Exadata") and on an IBM Power platform; it positions the product line as "modular", "cloud-native and cloud-agnostic" with "Open APIs" [V, icsfs.com]. The engine is therefore Oracle Database in the reference architecture [V by implication; confirm per bank].
- **Consequences.** Treat as Oracle (section 3.1) with the vendor's modular table families; the product has a regulatory-reports module and KYC/AML-related screens [V, icsfs.com], so the bank may already have AML-tagged data (flags, risk codes) that AKR-TMS can read and should not overwrite conceptually: AKR-TMS keeps its own rating and shows the CBS's as a source attribute.

### 11.6 Local and regional systems, state banks, and what the CBI says about them

- The CBI's own description: "the Central Bank also implemented the internal clearing system between the branches of the same bank, which enables government banks that do not have a comprehensive banking system to automate all transfers of their branches and provide a comprehensive database" [V, CBI payment-systems statement]. State-bank branches may therefore run partial or branch-level systems, with the head office receiving data by clearing rather than by a single ledger.
- **BML Istisharat ICBS** (Lebanon): the third regional core with Iraqi references; its users reportedly include the Central Bank of Iraq itself, Erbil Bank, Minare Bank, National Bank of Iraq (before its 2024 Temenos move) and Al Qurtas Islamic Bank; no public data on its database or AML module [S, via file 17 section 9]. Treat as an unknown-schema family: discovery and a reverse-engineered dictionary first.
- **Misys and Finastra Equation, Fusionbanking and Bankmaster** at state banks (Rafidain's history, file 17 section 7): Equation is an IBM i application on Db2 for i [K, file 17 could not confirm from vendor documents]. A read-only role over ODBC (IBM i Access ODBC driver), a CCSID check for Arabic, packed and zoned decimals, short field names and bank-supplied logical-file definitions are the integration facts; if the bank will not create an ODBC read-only user on the IBM i, the route is R4 (journal-based or batch extract delivered by the bank).
- **ProgressSoft PS-GatePay** connects Iraqi banks to the CBI's ACH, RTGS and cheque clearing, "integrating with the banks' Core Banking Systems (CBS)": nine banks used it in January 2019 (National Bank of Iraq, Mosul Bank for Development and Investment, RT Bank, BLOM Bank, Economy Bank for Investment and Finance, Standard Chartered, Ashur International Bank, Bank of Baghdad, Credit Bank of Iraq) [V, ProgressSoft]. Banks on it have a **payment gateway database** whose tables are an additional, structured source of RTGS and ACH messages (section 12).
- Local builds on SQL Server, MySQL, FoxPro or Access exist in the region [K]; they are mapped like any other source but their schemas have no vendor documentation at all, so the first deliverable is a reverse-engineered data dictionary co-signed by the bank.

### 11.7 Summary matrix

| CBS | Engine (expected) | How data is stored | Tier | Main mapping risk | Main read risk |
|---|---|---|---|---|---|
| Temenos Transact (TAFJ on RDBMS) | Oracle, Db2, SQL Server | `RECID` plus XML record per application table | R2 or R1; R4 via Transact Data Hub ODS | Field numbers per release and bank; local reference fields | XML parsing cost if pushed to the CBS; unauthorised and history files |
| Temenos Transact (TAFC on jBASE) | jBASE files | Multi-value files | R4 only | Dictionary-driven decoding of multi-values | No SQL path |
| Infosys Finacle (no confirmed Iraqi customer; low priority) | Oracle (often old releases) | Short-coded tables, daily to history at end of day | R2, R1 if standby is open | Two-table ledger (daily plus history); flags | Old Oracle releases; character sets |
| Oracle FLEXCUBE | Oracle | Module-prefixed relational tables with authorisation flags | R1 or R2 | Authorised versus unauthorised; user-defined fields | Per-branch dates |
| Path/Azentio iMAL | Oracle or SAP Sybase ASE (2017 review) | Relational (Islamic modules) | R2; R4 if Sybase and no ODBC | Islamic product semantics | Sybase ASE driver gap |
| BML Istisharat ICBS | Unknown | Unknown | R2 or R4 after discovery | No public schema | Everything: unknown |
| Misys/Finastra Equation, Fusionbanking | Db2 for i (Equation) [K] | Fixed-length short-named files, packed decimals | R4 or R2 over ODBC | CCSID for Arabic, EBCDIC-era types | ODBC read-only role may be refused |
| ICS BANKS (first adapter family; 30+ Iraqi customers by vendor claim) | Oracle | Relational, modular | R2 or R1 | Local fields; vendor AML tags | Oracle licensing of standby |
| State-bank branch systems | Mixed | Partial, branch-level | R4 | No single ledger; clearing data as source | Coverage: proving "all accounts" |
| Local builds | SQL Server, MySQL, others | Unknown | R2 or R4 | No documentation | Everything |

### 11.8 AKR-TMS hooks

- **Discovery kit** (read-only scripts per engine): table inventory with row counts and sizes, column catalogue, indexes on candidate watermark columns, grant list of the AKR-TMS account, partitioning, character set and collation, replica status. Results become the **data-source inventory** (file 05 section 1f) stored with the bank profile.
- **Dictionary-driven adapter generation** for Transact: read `STANDARD.SELECTION` and local reference definitions, generate the field map, diff it on every run, and alarm when the dictionary changes (a new local field or a renumbering).
- **Per-CBS adapter packs** (section 14) shipped with golden test data sets and mapping conformance tests, never with production data.
- **Gateway-database adapters** for PS-GatePay-style payment hubs, so RTGS and ACH messages come from the structured source, not from CBS narrative fields.

---

## 12. Payment data sources: SWIFT MT, ISO 20022, Iraq RTGS, ACH, national switch, cards

The CBS ledger shows the bank's side of a payment: a debit on a customer account and a credit on a nostro or clearing account. It does not hold the full message: the originator and beneficiary names and addresses as sent, the banks in the chain, the purpose, the remittance text, the reference that links to the sender's records. Sanctions screening, payment-transparency checks (FATF R.16, Wolfsberg PTS 2023; file 03 F-12 and file 04 W-12), trade-based ML scenarios and the phase-2 pre-release API all need the message. Screening-engine behaviour on MT and ISO 20022 fields is in file 14 section 10; this section is about where the data comes from and how it joins the ledger.

### 12.1 The Iraqi rails, as far as public documents go

| Rail | What it is | Evidence | Data AKR-TMS needs |
|---|---|---|---|
| CBI RTGS | High-value gross settlement, launched 2006; the CBI is owner and operator; "uses international standards designed through the SWIFT network"; settles USD and IQD; Arabic and English | [V] CBI paper "Electronic payment systems in Iraq" (2013) and the CBI statement on payment-system progress | Each RTGS order with participant, amount, currency, reference, status |
| C-ACH | Electronic clearing launched 2011: small-value payment orders (SVPO, same-day), electronic cheques (CH, settled within two days), "VIP" clearing cheques (same day if accepted), direct debits (DD, "not currently activated for lack of instructions and legislation"); net settlement instructions are sent to RTGS | [V] same CBI paper | Items by type, drawer and payee banks and accounts, cheque numbers, return reasons |
| National switch | Retail infrastructure supporting "inter-bank clearing for transfers made by customers and transactions through the use of ATMs and points of sale through credit cards linked to the bank account"; first transaction 2016 | [V] CBI statement; [S] trade press | ATM and POS authorisations and clearing records |
| Mobile payments and e-wallets | "Funds will be transferred through e-wallets without the need for a bank account", also bill payment | [V] CBI statement | Wallet top-ups and cash-outs seen as counterparties on bank accounts |
| Internal clearing between branches of one bank | For government banks without a comprehensive banking system | [V] CBI statement | Head-office clearing data |
| IBAN | "The IBAN number for Iraq consists of 23 character"; Iraq deals only in international-standard account numbers on transfers | [V] CBI statement; structure `IQkk BBBB SSS CCCCCCCCCCCC` [S] | Validate the checksum; derive bank and branch from the IBAN |
| Cross-border SWIFT | Customer and bank-to-bank payments, trade finance messages, statements | [K] universal for Iraqi banks with correspondents (file 01 section 4: USD correspondent relationships) | MT or MX messages (12.2, 12.3) |

CBI standards handbook 2025 (B8) requires every bank to have "the internal capabilities" to integrate with payment systems and defines payment systems as the "local and cross-border platforms currently used in Iraq... (including the real-time settlement system and the electronic clearing systems) in addition to any platforms that may be adopted in Iraq in the future (including instant payment systems)" [V, handbook text read from the Arabic PDF]. So as of 2025 the CBI itself treats instant payment as a future platform. Banks nevertheless offer "instant" domestic transfers inside their own channels (First Iraqi Bank advertises "send and receive money within Iraq instantly", 11.5 [V]); those are on-us transfers or switch-routed, and in every case a source that AKR-TMS must read from the bank's channel database.

**Gap, stated plainly:** no public CBI document was found that sets the CBI's own ISO 20022 migration timetable for RTGS and ACH. The 2013 paper says the RTGS follows SWIFT standards. The CBI Payments Department must be asked for the current message formats and any migration plan; until then the canonical payment model (section 13) is ISO 20022-shaped and the adapters convert RTGS messages into it.

### 12.2 SWIFT FIN (MT) messages

MT messages remain in bank archives and in some CBS tables even after cross-border instructions moved to ISO 20022. The ones that matter for AML:

| Message | Use | Fields AKR-TMS must extract [K] |
|---|---|---|
| MT103 (and 103 STP) | Single customer credit transfer | 20 sender reference; 23B bank operation code; 32A value date, currency, amount; 33B instructed amount; 36 exchange rate; 50a ordering customer (50F carries numbered lines for name, address, country); 52a ordering institution; 53a and 54a correspondents; 56a intermediary; 57a account-with institution; 59a beneficiary (59F structured); 70 remittance information; 71A charges; 72 sender to receiver information; 77B regulatory reporting; the UETR in the user header (block 3, tag 121) |
| MT202 and MT202 COV | Bank-to-bank transfer; the COV variant is the cover for an underlying customer payment and carries sequence B "underlying customer credit transfer details" (50a, 52a, 56a, 57a, 59a, 70, 72) so intermediaries can screen the real parties | All of the above for both sequences; link the COV to its MT103 by reference |
| MT199, MT299, MT999 | Free-text queries, replies and notices (often carrying the crucial facts in an investigation) | Whole text; they are retired for exceptions and investigations after November 2026 in favour of Swift's structured case messages [S, ACI summary of Swift's timeline] |
| MT940, MT950, MT900, MT910 | Statements and confirmations for nostro and vostro accounts | Entries for L6 reconciliation and correspondent activity reports (the CBI's third mandatory report, file 01 section 2d) |
| MT700, 707, 710, 720, 760, 767, 799 | Letters of credit and guarantees: issuance, amendment, transfer, standby | Applicant, beneficiary, advising bank, goods description, ports, amounts; feed the trade-based ML and PF scenarios (file 27; CBI PF indicators) |
| MT400, 410, 412, 420 | Documentary collections | Drawer, drawee, documents |

Format traps [K]: names and addresses are limited to four lines of 35 characters in 50a and 59a, so long Iraqi names are truncated or abbreviated; Latin transliteration of Arabic names varies by clerk (file 22); field 70 and 72 hold free text with party names, vessel names and addresses; SWIFT's restricted character set removes diacritics and Arabic letters entirely.

### 12.3 ISO 20022 (CBPR+) messages

Swift's coexistence period for cross-border payment instructions ended on 22 November 2025. Per the Swift timeline summarised by ACI, MT 103, MT 103 STP, MT 202 and MT 202 COV (CBPR+ equivalents pacs.008, pacs.009 and pacs.009 COV) became subject to additional technical validation and charges, some types such as MT 103 REMIT were withdrawn, and reporting messages such as MT 940 and MT 950 (equivalent camt.053) were deprecated but remain supported; MT 199, 299 and 999 use for exceptions and investigations ends after November 2026 [S, ACI Worldwide; Swift's own pages were not retrievable]. A second deadline follows: from the Swift release live on 14 November 2026, unstructured postal addresses are rejected in CBPR+ payment messages, with at least town name and country as structured fields and "hybrid" addresses allowed [S, trade guides; confirm in Swift's documentation].

| ISO 20022 message | Replaces or covers | AML relevance |
|---|---|---|
| pacs.008 | MT 103 customer credit transfer | Full debtor and creditor blocks (name, structured address, identification such as LEI or organisation or private ID, account IBAN or other), agents (BIC, LEI), instructed and settled amounts, charge bearer, purpose, structured and unstructured remittance, regulatory reporting, UETR in the payment identification |
| pacs.009 and pacs.009 COV | MT 202 and 202 COV | Bank-to-bank; the COV variant carries the underlying customer credit transfer block |
| pacs.004, pacs.002 | Returns and status reports | Rejections and returns: attempted and failed payments are STR-relevant in Iraq (file 01) |
| camt.053, camt.052, camt.054 | MT 940, 942 and debit/credit notifications | Nostro and vostro activity, reconciliation |
| camt.056, camt.029, camt.110, camt.111, trck.* | Cancellation, investigation, tracker | Investigation trails, RFIs from correspondents (file 04 section 2p) |
| pain.001 | Customer initiation (corporate files) | Bulk payment files from corporates: many beneficiaries in one file |

Design rule: ingest **the native message** whenever it exists, keep the raw message text in the evidence store, and parse it into the canonical model. An MX converted down to an MT loses data (structured address, LEI, purpose, 140-character names); ACI's summary of Swift's position says that "converting ISO 20022 transactions to MT standard risks data loss" [S].

### 12.4 Where to read payment data inside the bank

| Source | Access | Tier | Notes |
|---|---|---|---|
| CBS payment and message tables | Read-only views (FLEXCUBE and others keep message text in large-object columns [K]; Temenos delivery tables [K]) | R2 | Easy to reach, often truncated or normalised |
| Payment hub or gateway database (for example a PS-GatePay-type RTGS and ACH gateway [V, ProgressSoft]) | Read-only views on the gateway's database | R2 | Structured RTGS and ACH data, the cleanest domestic source |
| SWIFT interface (Alliance Access or equivalent) message archive | Not a direct database read: the interface's own database and security zone are governed by Swift's Customer Security Programme (file 30 section 3) [K]. Use the bank's message-archive export (files) or a copy delivered by the payment application | R4 | Reading the interface's database directly is not appropriate and may breach the bank's Swift assurance |
| Pre-release interception (phase 2) | A message-partner or routing hook configured by the bank that sends a copy to AKR-TMS and waits for a verdict | API | Section 15 |
| Card processor or national-switch database (ISO 8583 authorisations and clearing) | Read-only views on the processor's database | R2 or R4 | PANs only masked or keyed-hashed (file 30 section 11) |
| Channel databases (mobile app, internet banking, digital bank) | Read-only views | R2 | Device and IP data for KYC trigger rules (file 04 section 2q) |
| Cash and teller | CBS teller module | R2 | Denominations and currency; the USD 10,000 CTR threshold (file 01) |

### 12.5 Linking messages to ledger postings

A message and a posting are different records: a customer credit transfer produces a ledger debit, charges, perhaps FX, and a credit to a nostro; a batch file produces one posting and many beneficiary lines. AKR-TMS therefore keeps a **payment-link** table: `(payment_message_id, ledger_txn_id, link_type, confidence, rule)`, matched by UETR, end-to-end id, transaction reference, "our" and "their" reference, value date and amount, with fuzzy matching only as a labelled fallback. Messages without a posting and postings without a message are exceptions routed to reconciliation level L4 (8.1). Bundled and "on behalf of" payments (Wolfsberg; file 04) become one message with many parties.

### 12.6 Cards and wallets

- ISO 8583 authorisation and clearing records give MTI, masked PAN, processing code, amount, currency, STAN and RRN, terminal and merchant identifiers, merchant category code, acquirer and response code [K]. Cash withdrawals at ATMs and POS purchases join the ledger by account and reference.
- Digital-bank models add agent-like constructs (FIB's "terminals as sub-accounts" with "limited cash services", 11.5 [V]): the canonical model needs a relationship from a sub-account to its owning merchant account, because the AML risk sits with the owner.
- E-wallet providers are outside the bank; the bank sees their funding and cash-out as counterparties. The CBI's separate AML controls for e-payment providers (20 April 2025; file 01 section 2e) were not read in detail.

### 12.7 AKR-TMS hooks

- **Payment adapters** that emit one internal `payment_message` with a `format` (MT103, pacs.008, RTGS, ACH-SVPO, cheque, ISO 8583) and keep the raw text.
- **Message parsers in Rust** for MT (block structure, field tags) and ISO 20022 (streaming XML with the CBPR+ schemas), both fuzz-tested because they will see malformed real-world messages; parser version recorded on every parsed record.
- **IBAN service**: checksum, bank and branch decode, BIC lookup table maintained as admin data.
- **Payment-quality monitor** (file 04 W-12) fed from parsed fields (missing or meaningless fields, name truncation, unstructured addresses after November 2026).
- **Link table and L4 reconciliation** (8.1) as built-in controls.
- **A "CBI formats" request** on the project checklist: the RTGS, ACH and switch message specifications and migration dates, obtained from the CBI Payments Department.

---

## 13. The canonical AML data model

The canonical model is the contract between the per-bank adapters (section 14) and everything else: scenarios (file 27), screening (files 14 and 22), risk rating (file 26), cases (file 28), reports (file 23). It must be rich enough to run the CBI's scenario catalogue (file 27: cash, transfers, LCs, LGs, credit, digital, cards, FX, lockers, securities, customer and employee behaviour) and small enough that a bank can fill it from a read-only database.

### 13.1 Design rules

1. **Four layers.** Raw landing (source rows as read, kept; the one exception is that card-number-like digit runs in free-text columns are masked before landing, file 30 section 11.5) -> canonical (typed, normalised, entity-resolved) -> features and aggregates (windows, peer groups) -> outputs (alerts, scores, cases). Each layer is rebuildable from the one below it.
2. **Lineage on every row**: `source_id` (the bank connection), `source_table`, `source_pk`, `source_version`, `row_hash`, `batch_id`, `extracted_at`, `mapping_version`.
3. **Bi-temporal for slowly changing things** (party, customer, account, relationship, product assignment): business validity `valid_from` and `valid_to`, system time `recorded_from` and `recorded_to`. Facts (transactions, messages) are immutable once settled; corrections arrive as new events linked to the original (7.6).
4. **Identity**: canonical ids are UUIDv7 (the DBS convention); a cross-reference table maps `(source_id, source_key)` to canonical ids; a party can be resolved across several source records and several CBSs (a bank with an Islamic window on one system and conventional banking on another; an acquired bank).
5. **Amounts are positive with a debit/credit indicator**, never signed; `numeric(28,6)` with ISO currency; the local-currency equivalent and the USD equivalent are stored with the rate source and rate date. IQD's minor unit is 0 in most CBSs and 3 in ISO 4217 [K]; the adapter normalises and records which it found.
6. **Names are never overwritten**: raw as read, normalised for matching, with script, language and name type (file 22).
7. **Extensible without migrations**: bank-specific fields live in a typed attribute table `attribute_definition` (name, type, owner, personal-data class, documentation) and `attribute_value` rows; an attribute that many scenarios use is promoted to a column by a normal migration later. This is how Iraqi fields (mother's name, family number, tribe) enter the model per bank.
8. **Every column carries a data-classification tag** (public, internal, personal, financial, detection-logic, secret-STR, card) that drives masking, field-level encryption and access policy (file 30 sections 7, 8, 11).
9. **Reference data is first-class**: countries and governorates, currencies, holidays, branches, product maps, transaction-code maps, purpose codes, MCCs, all admin-managed with effective dates.

### 13.2 Entity catalogue

| Entity | Purpose | Key attributes | Typical sources |
|---|---|---|---|
| `party` | A natural person, a legal entity or a legal arrangement, resolved across sources | type, status, nationality, residence country, date and place of birth or incorporation, gender, occupation or industry (ISIC), legal form, registration and tax numbers, mother's name, family number, PEP status and source, staff flag and related-to-staff flag | CBS customer master, KYC forms, registries |
| `party_name` | All names of a party | raw, normalised, script (Arabic, Latin, Kurdish), language, type (legal, trade, alias, as-in-payment-message, former), components (given name, father's name, grandfather's name, family or tribe name) | CBS (often several name fields and a local-language field), payment messages |
| `party_identifier` | Identity documents and registry numbers | type (national ID card, civil ID, family card, passport, residence card, commercial registration, tax id, LEI, BIC), value (keyed hash plus encrypted value), issuer, country, issue and expiry dates | CBS, KYC records |
| `party_contact` | Phones, e-mails, addresses, devices | type, normalised value (phone in E.164 with +964), structured address (governorate, district, neighbourhood, street, house), first and last seen, shared-count | CBS, channel databases |
| `party_relationship` | Party to party | type (owner with share percentage, controller, director, signatory, attorney, guardian, family with degree up to 2nd or 4th, employee-of, associate), effective dates, evidence source | CBS ownership screens, KYC documents, analysts |
| `customer` | The relationship with this bank | CIF, branch of relationship, relationship manager, segment, onboarding date and channel, status, CDD level (SDD, CDD, EDD), expected-activity profile (declared monthly turnover, cash share, countries, products, source of funds and wealth, purpose), risk rating link, review dates | CBS customer and KYC tables |
| `account` | A deposit, current, savings, investment, loan, nostro or vostro account | account number and IBAN, type, product, currency, branch, open and close dates, status (active, dormant, frozen, blocked, closed), dormancy dates, parent account (sub-accounts), staff-account flag, correspondent flag | CBS account master |
| `account_party_role` | Who relates to an account and how | owner, joint holder, signatory, attorney, beneficial owner, nominee, guarantor, beneficiary; signing rule; effective dates | CBS mandate and signatory tables |
| `account_hold` | Freezes, blocks, earmarks, dormancy freezes | type (court order, AML freeze, sanctions freeze, CBI order, branch hold), authority, reference, start, end, amount | CBS account-status and hold tables; needed to verify bank freeze actions (file 04) |
| `balance_snapshot` | Balances by business date | ledger and available balance, currency | CBS end-of-day balance history |
| `transaction` | A ledger line | account, dr_cr, amount, currency, local and USD equivalents, booking date, value date, posting time, business date, type (canonical taxonomy), channel, branch, operator and authoriser ids, narrative (raw and normalised), references, status, reversal and correction links, payment link, product, purpose code, cash flag | CBS ledger |
| `transaction_event` | Lifecycle of a transaction | initiated, pending, authorised, posted, reversed, rejected, cancelled, each with time and actor | CBS pending and history files, gateway logs (6.3) |
| `counterparty` | The other side of a transaction, resolved | names, account or IBAN, bank (BIC, name, country), type (internal customer, external person or organisation, bank, PSP or wallet, merchant, government), first and last seen, link to an internal `party` when it is one | Message parties, narrative parsing, clearing data |
| `product` | What was sold | bank code mapped to a canonical family (current, savings, term deposit, Islamic investment account, financing types, card, LC, LG, remittance, locker, FX, securities), Islamic contract type, risk attributes | CBS product tables plus the admin product map |
| `channel` | How it was done | branch teller, ATM, POS, internet, mobile, call centre, agent or sub-account terminal, SWIFT, RTGS, ACH, switch, partner API, wallet; channel risk attribute | CBS channel codes plus map |
| `branch` | Service outlet | code, name (Arabic and English), governorate and region (Kurdistan Region flagged), type (branch, sub-branch, head office, digital), open dates, liaison officer, branch business date | CBS branch table |
| `card` and `card_transaction` | Cards and their authorisations | masked PAN (first six and last four) plus keyed hash, scheme, product, status, limits, holder, account link; card transaction: MCC, merchant and terminal ids, acquirer, country, entry mode, e-commerce flag, response code, amount; never the full PAN, CVV or PIN data | Card system, switch, processor |
| `facility` | Loans, financing, limits | borrower, product, limit, outstanding, disbursements, schedule, rate or profit, purpose, collateral and guarantor links, status, early settlement and third-party repayment events; Islamic financing contract fields | CBS lending modules |
| `trade_instrument` | LCs, LGs, collections | type, applicant, beneficiary, issuing, advising and confirming banks, amount, currency, tenor, goods description, HS codes, ports, vessel and IMO, shipment dates, incoterms, amendments, presentations, settlements, linked messages | CBS trade module, MT 7xx and 4xx |
| `payment_message` | A payment instruction or status as sent or received | format, rail, direction, raw text, UETR, references, instructed and settled amounts, charge bearer, purpose, regulatory reporting, status, parsed version | Gateway, SWIFT archive, CBS message tables (section 12) |
| `payment_message_party` | Every party and agent in a message | role (debtor, creditor, ultimate debtor, ultimate creditor, debtor agent, creditor agent, intermediaries, instructing agent), name, structured address, identifier (LEI, BIC, national ID), account | Parsed message |
| `payment_link` | Message to ledger | match keys, confidence, rule | Derived (12.5) |
| `cash_event` | Cash handling | denominations, currency, declared source of cash, CTR flag (USD 10,000 or equivalent, file 01), branch vault movements | Teller module |
| `fx_deal` | FX transactions | pair, rates, amounts, value date, counterparty, exchange-house flag | Treasury and FX modules |
| `locker` and `locker_access` | Safe deposit lockers and visits | holder, access times | Locker module |
| `staff` | Know-your-employee data | employee id and party link, branch, role, authority limits, leave and rotation history, own and related accounts | HR and CBS user tables |
| `cbs_user_activity` | Optional: CBS operator actions | user, action, object, time, override flags | CBS audit tables (insider-threat scenarios, file 30 section 9) |
| `kyc_document` | Evidence items | type, expiry, vault pointer | DMS, uploads |
| Reference tables | Countries, governorates, currencies, FX rates, holidays, codes, maps | effective-dated | Bank and public sources |
| Lineage tables | `source_catalog`, `mapping_version`, `batch_receipt`, `reconciliation_result`, `quality_issue` | | Section 8 |

### 13.3 How the entities join

```text
party --< party_name / party_identifier / party_contact
party >--< party_relationship >--< party
party --< customer --< account >--< account_party_role >-- party
account --< transaction --< transaction_event
transaction >-- counterparty        transaction >-- channel, branch, product
transaction >--< payment_link >-- payment_message --< payment_message_party
account --< balance_snapshot        account --< account_hold
customer --< facility >-- collateral        customer --< trade_instrument >--< payment_message
card >-- account        card --< card_transaction >-- transaction
staff >-- party         staff --< cbs_user_activity >-- transaction
```

### 13.4 Storage conventions on PostgreSQL (follows the DBS rules)

- Migrations in one SQLx chain; `TIMESTAMPTZ` only; `UUID` keys; compile-time checked queries; every table has `created_at` and `updated_at` per the DBS migration rules.
- `transaction` and `transaction_event` are **range-partitioned by booking month** with BRIN indexes on time and B-tree on `(account_id, booking_date)`; partition creation and retention run as worker jobs like the DBS `partition_maintenance` job. The raw landing layer is partitioned by extraction day.
- Personal data columns are encrypted at field level with OpenBao-wrapped data keys (DBS pattern), searchable fields use keyed hashes (national id, phone) so matching works without decrypting.
- A sketch of the central fact table:

```sql
CREATE TABLE IF NOT EXISTS txn (
  id            UUID NOT NULL DEFAULT gen_random_uuid(),
  source_id     UUID NOT NULL,                 -- bank connection
  source_table  TEXT NOT NULL,
  source_pk     TEXT NOT NULL,
  source_version TEXT,
  row_hash      BYTEA NOT NULL,
  account_id    UUID NOT NULL,
  dr_cr         CHAR(1) NOT NULL CHECK (dr_cr IN ('D','C')),
  amount        NUMERIC(28,6) NOT NULL CHECK (amount >= 0),
  currency      CHAR(3) NOT NULL,
  amount_lcy    NUMERIC(28,6),
  amount_usd    NUMERIC(28,6),
  booking_date  DATE NOT NULL,
  value_date    DATE,
  business_date DATE NOT NULL,
  posted_at     TIMESTAMPTZ,
  source_local_ts TIMESTAMP,                   -- exactly as read, no zone
  txn_type      TEXT NOT NULL,                 -- canonical taxonomy
  status        TEXT NOT NULL,                 -- lifecycle (6.3)
  channel_id    UUID, branch_id UUID, product_id UUID,
  counterparty_id UUID, operator_id TEXT, authoriser_id TEXT,
  narrative_raw TEXT, narrative_norm TEXT,
  reverses_id   UUID, corrected_by_id UUID,
  late          BOOLEAN NOT NULL DEFAULT false,
  batch_id      UUID NOT NULL, extracted_at TIMESTAMPTZ NOT NULL,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (id, booking_date),
  UNIQUE (source_id, source_table, source_pk, booking_date)
) PARTITION BY RANGE (booking_date);
```

### 13.5 Islamic banking and Iraqi specifics

- **Islamic products**: financing contracts (murabaha, ijara, istisna, salam, musharaka, mudaraba, wakala) have their own parties (seller and buyer roles, agent), asset records and deferred prices; investment accounts share profit rather than earn interest; the canonical `product` and `facility` carry `contract_type`; profit-distribution and Sharia-reserve postings are marked `system_generated` so customer-behaviour scenarios skip them (file 24).
- **Iraqi identity**: national card, civil ID and "family number" (the CBI's PEP and KYC practice uses family cards; file 01 section 7) are separate identifiers; the mother's name is a first-class attribute because the national list carries it with birth year (file 01 section 6); names carry tribal components (file 22).
- **Two main currencies**: IQD and USD, with the USD dimension subject to US scrutiny (file 01 section 4); every amount has both equivalents for reporting and for the OFAC 10-year record class (file 05 section 1e).
- **Kurdistan Region**: branches flagged by region; Kurdish-script names (file 22); salary-localisation (MyAccount) accounts as a segment (file 01 section 9).

### 13.6 Scenario coverage matrix (a product feature, not only a model)

Every scenario (CBI catalogue and bank-defined) declares the canonical attributes it needs. After mapping, AKR-TMS computes for each scenario whether it can run: **operable**, **degraded** (some attribute missing, with the loss stated), or **blocked** (named attribute unmapped, for example no teller id so employee scenarios cannot run). The matrix is shown to the MLRO and exported for the CBI-approved assessor: it answers "does the system really cover the 86 CBI scenarios with this bank's data?" without a vendor's say-so. It also tells the bank which fields to ask its CBS vendor to expose.

### 13.7 AKR-TMS hooks

- **Canonical schema as code and documentation**: the SQLx migrations are the single source; a generated data dictionary (entity, column, class, description in Arabic and English, source mappings) is published in the product and exported for assessors.
- **Attribute registry** with promotion workflow.
- **Scenario-to-attribute dependency table** and the coverage matrix (13.6).
- **Synthetic data generator** that fills the canonical model (and, through the adapters' reverse maps, each CBS schema's test copy) with Iraqi-shaped data: Arabic and Kurdish names, tribal components, family numbers, salary-day peaks, cash-heavy patterns, seeded typologies; used for tests, demos and pre-go-live tuning, never real data.

---

## 14. The metadata-driven mapping layer and per-bank adapters

Goal: onboarding a new bank on a known CBS family in weeks, on an unknown one in a few more, with every mapping decision visible, versioned, testable and reversible, and with no code deployment to change a mapping. The DBS rule "nothing is a fixture" applies: the product owns the engine and the pack templates; the bank (with the implementer) owns the mapping data, edited on screens, audited, maker-checker approved.

### 14.1 Components

| Component | What it is | Owned by |
|---|---|---|
| **Source catalogue** | Introspected metadata: tables, columns, types, keys, indexes, partitions, row estimates, character set; plus imported vendor dictionaries (for Transact the `STANDARD.SELECTION` records); versioned snapshots with a drift diff | Generated at discovery and on every run |
| **Mapping specification** | A declarative document per canonical entity: source object, key, change strategy and watermark, field mappings, transforms, filters, validations. JSON validated against a JSON Schema (the DBS `dsl` crate pattern: strict schema, loud failure, no remote `$ref`) | Stored as rows in the product database; seeded from adapter packs; edited in a mapping workbench |
| **Transform library** | A closed, versioned, unit-tested set of functions in Rust: `parse_date(fmt)`, `decimal_scale(n)`, `xml_field(path)`, `multivalue_split`, `code_map(map_id)`, `normalise_phone(+964)`, `keyed_hash`, `split_arabic_name`, `iban_validate`, `currency_minor_units`, `tz_convert(zone)`, `coalesce`, `concat`, `mask_pan` | Product (code) |
| **Safe expression language** | For small computed fields and filters: a non-Turing-complete expression language (CEL is the candidate; file 32 evaluates the Rust implementation); no user scripts, no I/O, evaluation limits | Product |
| **Code maps** | Bank code to canonical code: transaction codes, product codes, channel codes, status codes, branch types; effective-dated; unmapped values become `unknown` plus a data-quality issue, never a failed batch | Bank (admin screens) |
| **SQL generator and dialect plugins** | Builds only `SELECT` statements from the spec for each engine (keyset pagination, fetch-first, bind variables); a SQL parser verifies that every statement is a single read-only `SELECT` over approved objects before it is sent | Product |
| **View DDL pack** | A reviewable script that the **bank's DBA** runs to create the read-only views AKR-TMS reads; generated from the mapping and the dictionary; each view appears in the footprint register | Generated; applied by the bank |
| **Adapter pack** | The bundle for one CBS family and engine: dictionary reader, view DDL templates, mapping templates, code-map seeds, EOD-gate configuration, reconciliation ladder templates, golden test data, known-quirks document | Product |
| **Conformance suite** | Property tests for transforms, contract tests that every adapter output satisfies the canonical constraints, golden-data tests per pack, fuzz tests for message parsers | Product |

### 14.2 What the mapping document looks like (sketch)

For a Transact `ACCOUNT` table the field references are dictionary names, resolved to `cNN` through the bank's own dictionary at generation time:

```json
{
  "entity": "account",
  "source": { "object": "FBNK_ACCOUNT", "key": "RECID", "format": "t24_xmlrecord" },
  "change": { "strategy": "poll", "watermark": { "kind": "t24_date_time", "field": "DATE.TIME" },
              "overlap": "PT10M", "settle": "PT2M" },
  "fields": {
    "account_number": { "from": "RECID" },
    "customer_ref":   { "from": "field:CUSTOMER" },
    "currency":       { "from": "field:CURRENCY" },
    "open_date":      { "from": "field:OPENING.DATE", "do": ["parse_date:yyyyMMdd"] },
    "status":         { "from": "field:POSTING.RESTRICT", "do": ["code_map:t24_account_status"] },
    "branch_code":    { "from": "field:CO.CODE", "do": ["code_map:t24_company_to_branch"] },
    "mothers_name":   { "from": "local:MOTHER.NAME", "attribute": true }
  },
  "validate": [ { "rule": "not_null", "field": "customer_ref" }, { "rule": "ref", "field": "currency", "table": "currency" } ]
}
```

For a relational CBS the `source.object` is a bank-owned view and `from` names its columns. Everything else (change strategy, overlap, validation, code maps) has the same shape.

### 14.3 Lifecycle: draft, test, approve, publish, pin

1. **Draft** in the workbench; every edit audited with actor and reason.
2. **Test**: run against a sample of real source rows (read-only) and against the pack's golden data; view the canonical output side by side with the source; automated checks (types, ranges, nulls, referential integrity, row counts).
3. **Impact analysis**: how many rows change canonical value versus the active version, which scenarios depend on the changed fields, whether the coverage matrix (13.6) improves or degrades.
4. **Approve**: maker-checker through Cedar policies (an implementer proposes, a bank compliance or IT officer approves; the proposer cannot approve).
5. **Publish and pin**: each batch records the `mapping_version` it ran under, so a later change never silently re-interprets history (the DBS "a published definition stays pinned" rule).
6. **Replay** (optional, explicit): re-run mapping over the raw landing layer for a date range and review a diff before applying (7.8).
7. **Rollback**: activate the previous version; replay if needed.

### 14.4 Drift and bulkheads

At every run the extractor compares the catalogue hash of the columns a mapping uses with the snapshot the mapping was approved against. On drift it pauses **only the affected tables** (bulkhead), keeps all others running, opens a mapping alarm naming the table and column, and shows the diff. A CBS upgrade that renumbers fields or renames a column therefore degrades one entity, loudly, instead of corrupting data quietly. Dictionary-driven packs (Transact) regenerate the field map after the dictionary changes and propose the new version for approval.

### 14.5 Multi-source banks and survivorship

A bank can have several sources (Islamic and conventional systems, a card processor, a payment hub, a digital-channel database, a branch network on a legacy system). Each is a `source_id` with its own mapping set. Conflicts on one party's attributes are resolved by **survivorship rules** per attribute (most recent, most trusted source, longest verified), recorded and explainable; party resolution across sources follows file 22 (names) and file 21 (entity resolution) and is always reversible (split and merge are maker-checker actions).

### 14.6 Onboarding plan (planning assumptions to be replaced by experience)

| Stage | Output | Indicative duration [K] |
|---|---|---|
| Discovery | Integration profile, tier choice, licence questions answered, calendar and EOD rules, character-set test | 1-2 weeks of work, often 2-6 weeks elapsed because of bank change management |
| Access | Read-only account, views applied by the DBA, network path, TLS, OpenBao secret, read-only attestation | 1-4 weeks elapsed |
| Dictionary and mapping | Catalogue, dictionary import, mapping workshop, code maps, validation rules | 1-3 weeks for a known family; 3-6 for an unknown one |
| Backfill and reconciliation | History loaded under the load budget; L0 to L3 green; gaps closed | 1-3 weeks |
| Parallel run and tuning | Scenarios in shadow, alert volumes reviewed, mapping fixes | 2-6 weeks |
| Go-live | Coverage certificate, signed load budget, kill-switch procedure rehearsed | 1 week |

The long pole is the bank's own approvals and DBA time, not engineering. The product should shorten it by making each request small, explicit and reversible (views to review, a read-only attestation, a footprint register, a kill switch).

### 14.7 What AKR-TMS will not do

- No stored procedures, triggers, tables, database links, linked servers or jobs in the CBS database (the only creations are bank-applied views and, in R3, the capture configuration the bank's DBA chooses to apply).
- No ad hoc SQL typed by an implementer against the CBS from the product UI; the mapping workbench runs generated, parsed, read-only statements only.
- No transformation logic that runs inside the CBS database (XML shredding, joins that scan large tables) beyond what the bank's own approved views contain.

### 14.8 AKR-TMS hooks

- **Mapping workbench** and API (OpenAPI-first, Orval-generated client, Cedar-gated) with the lifecycle above; mappings and code maps are admin-managed data with effective dates.
- **Adapter packs as signed release artifacts** in Harbor (file 32 section 16): the pack version is part of the release bill of materials.
- **Golden data and conformance suite** run in the vendor's CI on every change to a pack or to the canonical model.
- **Coverage matrix and data dictionary export** generated from the mapping (13.6, 13.7).

---

## 15. The phase-2 pre-release screening and scoring API

Phase 1 watches the CBS from the replica and raises alerts after the event. Phase 2 adds an **optional** synchronous service that the bank's payment or SWIFT gateway (or a CBS exit that the bank configures) calls **before** it releases a payment. AKR-TMS stays passive: it never reaches into the gateway or the CBS; the bank's component calls AKR-TMS, receives a decision, and itself holds, releases or rejects. This is the only way a read-only product can meet the CBI's requirement that the system "stops any transaction that matches one or more suspicion scenarios until the MLRO verifies it is sound" (file 01 section 2d).

### 15.1 Scope: what is worth calling for

- **Cross-border payments** (SWIFT, in and out) and trade finance messages: Wolfsberg: "Screening cross-border payments prior to completing the transaction is common practice and known as screening in real-time" [V, section 5.1]. Highest value, lowest volume.
- **Domestic payments** (RTGS, ACH): Wolfsberg says real-time screening of domestic payments "may be unnecessary for FIs that are subject to the same local regulatory requirements", but an FI "may decide to screen a defined set of transactions, where it assesses the sanctions risks within the local economy... to be outside of its own risk tolerance" [V]. Iraq's dollar and militia-linked risks (files 01, 02, 05) argue for screening large and high-risk domestic items, and for scoring (not only list screening) before release.
- **Channel pre-confirmation** (mobile or internet banking beneficiary checks) and **cash-out at agents or sub-account terminals**: optional, later.
- **Intermediaries** can "only be expected to conduct real time sanctions screening on the information that is included in the message" [V, Wolfsberg Payment Transparency 2023], so the API screens what the message carries and flags what it lacks (file 04 W-12).

### 15.2 Integration patterns (the bank's choice)

| Pattern | How the call happens | Notes |
|---|---|---|
| Gateway callout over HTTPS | The payment hub or gateway posts the message or its parsed fields to AKR-TMS and waits | Preferred: simplest, testable, observable |
| Message-queue interposition | A routing rule in the SWIFT interface or hub sends a copy to a queue and holds the message until a verdict arrives on a reply queue | Common in SWIFT interfaces [K]; AKR-TMS provides an MQ adapter (file 32 section 5) next to the HTTPS API |
| CBS exit or channel call | A user exit or workflow step in the CBS or channel calls AKR-TMS before posting | Needs the CBS vendor; the bank owns the change |
| Shadow tap | The same copy is sent but the verdict is ignored | Used for roll-out (15.9) |

SWIFT infrastructure sits inside the bank's Swift Customer Security Programme scope; any component connected to it, including an adapter that receives copies of messages, must be treated as part of that secure zone (file 30 section 3) [K].

### 15.3 Contract

```text
POST /v1/assess                               (mTLS, bank client certificate)
Idempotency-Key: <bank_id>:<rail>:<UETR or internal reference>:<attempt>
Content-Type: application/json
{
  "format": "MT103" | "pacs.008" | "RTGS" | "ACH" | "canonical",
  "raw": "<message text>",              // or "canonical": {...}
  "context": { "rail": "...", "direction": "out", "branch": "...", "channel": "...",
               "operator_id": "...", "debit_account": "...", "value_date": "..." }
}

200 OK
{
  "decision_id": "<uuid>",
  "decision": "RELEASE" | "HOLD" | "REJECT",
  "reasons": [ { "code": "SANCTIONS_HIT", "list": "OFAC-SDN", "field": "59a", "score": 0.93 },
               { "code": "HIGH_RISK_CORRIDOR" }, { "code": "SCORE_ABOVE_THRESHOLD", "score": 0.81 } ],
  "risk_score": 81,
  "case_ref": "AML-2026-004711",       // present when a HOLD created or joined a case
  "degraded": [],                       // e.g. ["ml_unavailable", "lists_stale"]
  "expires_at": "2026-10-06T12:34:56Z", // a RELEASE is valid only for this window
  "signature": "<key-version||ed25519 over canonical decision bytes>",
  "customer_message_code": "PENDING_REVIEW"   // generic, never the reason
}
```

- **Idempotency.** Same key and same payload returns the stored response **byte for byte** (the DBS idempotency ledger already does this: migration 0003 stores raw response bytes so a replay is exact); same key with a different payload is refused (the DBS `E_IDEMPOTENCY_REUSE`, 422); the same key while the first call is still running gets `E_IDEMPOTENCY_IN_FLIGHT` (409). The IETF Idempotency-Key header draft (draft-ietf-httpapi-idempotency-key-header, revision 07 of October 2025, now expired) describes the same semantics [V].
- **Natural keys.** The UETR (Swift gpi) identifies a cross-border payment end to end; domestic rails use the bank's own reference with the rail and bank id.
- **A score alone never rejects.** A model or rule score can produce at most HOLD for human review. REJECT is returned without a person only for deterministic conditions the bank has configured as prohibited (an exact identifier match to a designated person, a destination barred by a CBI order), with the rule id in `reasons`; every other reject is a person's decision on a held payment (file 30 section 19.5).
- **Decision expiry.** A RELEASE is valid for a short window (for example 10 minutes); a gateway that releases later must call again, so a payment cannot be assessed once and held for days.
- **Signed decision.** The decision bytes are signed in-process with an Ed25519 key unwrapped from OpenBao at start-up and rotated by version prefix, the same scheme the DBS uses for QR tokens (OpenBao is never in the request hot path). The gateway verifies the signature and refuses an unsigned or expired RELEASE, so a compromised proxy cannot forge a release.
- **Versioning.** `/v1`, OpenAPI generated from utoipa annotations; the bank's gateway integration uses the OpenAPI document.

### 15.4 Latency budget

The DBS target for its interactive endpoints is p99 below 40 ms [V, DBS tech-stack]. This API is a different class: it screens names against large indices and scores, so it gets its own service-side objectives, proposed here and **to be proven by load test**:

| Step | Budget (p95) |
|---|---|
| TLS keep-alive, request parse and validation | 5-10 ms |
| Message parse (MT or MX) | 2-10 ms |
| Candidate generation from the list index (normalised names, phonetic and n-gram keys) for each party and agent | 15-40 ms |
| Match scoring and explanation | 10-30 ms |
| Features: customer, counterparty, corridor (in-memory cache, PostgreSQL fallback) | 2-10 ms |
| Rules and thresholds | 2-5 ms |
| ML inference (tree ensemble via ONNX) | 2-10 ms |
| Durable write of the decision (synchronous commit) | 3-10 ms |
| **Total service-side** | **about 40-125 ms; target p95 150 ms, p99 400 ms** |

The gateway's own timeout should be set well above that (1-3 seconds is typical for hub screening callouts [K]) with the fail policy in 15.5. Context from other regimes: EU instant payments allow 10 seconds in total for the whole credit transfer (Regulation (EU) 2024/886 Article 5a) and the EU resolved the screening-latency problem by removing per-transaction screening from that flow in favour of daily and on-change customer screening (Article 5d) [V, EUR-Lex]. Iraq's rails are not instant (RTGS and ACH are batch-like or near-real-time [V CBI]), so a sub-second synchronous call is comfortable, and **precomputation** (customer screening results, risk scores, counterparty history) is how the budget stays small.

**Volumes are not the problem.** The CBI's 2013 paper reports about 3,000 RTGS transfers in IQD and about 800 in USD per month for the whole country [V]; today's volumes are larger but cross-border and RTGS flows per bank are still small. Plan for 50 requests per second per bank with 500 per second burst headroom on one node; the constraints are latency, availability and correctness.

### 15.5 Fail-open versus fail-closed

The decision is a **policy of the bank**, set per rail, corridor, currency and amount, approved by the board or the MLRO and recorded as data. AKR-TMS publishes the recommended matrix; the **gateway** enforces it, because the gateway is the component that can hold a message.

| Class of payment | Recommended behaviour when AKR-TMS cannot answer in time | Reason |
|---|---|---|
| Cross-border, any USD, trade finance, correspondent flows | **Fail closed**: the message stays in a "pending screening" state, is neither released nor rejected, and is retried; after a configured wait it goes to a manual four-eyes screening queue using the bank's own degraded procedure | A released payment that was never screened is a potential sanctions breach; OFAC-style and UK regimes can be strict liability (file 05 sections 1e and 3) |
| Large or high-risk domestic (above a threshold, or to or from high-risk customers) | Fail closed, shorter wait | The CBI's requirement to hold until cleared (file 01) |
| Low-value domestic between known customers (for example ACH small-value orders) | Bank may choose **fail open with mandatory retrospective screening** within minutes | Wolfsberg: real-time screening of domestic payments may be unnecessary; compensating control is post-event monitoring and the release-without-decision report (15.7) |
| Channel pre-confirmation | Fail open (the payment still passes through the rail check later) | Convenience control only |

Degraded modes inside AKR-TMS (each marks the response with a `degraded` flag and a metric):
1. ML unavailable: rules and lists only; decision is more conservative on borderline scores.
2. Feature cache unavailable: use PostgreSQL features or conservative defaults.
3. **Sanctions index older than its threshold** (CBI: lists refreshed "every 12 hours at least"): cross-border decisions become HOLD with reason `lists_stale`; AKR-TMS does not say RELEASE on stale lists.
4. Persistence unavailable: no decision can be recorded, so no RELEASE is issued (the decision record is the audit evidence); the gateway's fail policy applies.

### 15.6 When AKR-TMS is down

1. **Detection**: the gateway's circuit breaker trips on timeouts and errors; AKR-TMS health endpoints and external probes alarm separately (dead-man probes from the bank's monitoring).
2. **Continuity**: two stateless nodes behind the bank's load balancer (active-active); a **local fallback screen** (a small container deployed beside the gateway with the compiled list index, refreshed from AKR-TMS, no dependency on the central service) can answer the list-screening half alone and mark the response `degraded: central_unavailable`; this lets cross-border flow continue with lists-only protection under the bank's approved procedure instead of stopping.
3. **Recording**: every period during which the API did not answer is a **pre-release blind period** in the coverage statement (8.2); payments released during it under a fail-open rule are queued for priority retrospective review; the outage report with timeline is evidence for the CBI.
4. **Recovery targets**: RTO 15 minutes for the API (restart or second node); RPO zero for decisions (synchronous commit for the decision table to a standby when one exists, per the DBS note on per-transaction `synchronous_commit`); the monitoring pipeline (phase 1) continues independently from the replica.
5. **Rehearsal**: quarterly failover drill and a weekly **control test payment** (a synthetic message with a known listed name injected through the real gateway, expected to be held), reported to the MLRO.

### 15.7 Holds without writing to the CBS, and the controls that prove the gateway behaves

- **Hold** is the gateway not forwarding the message (or a CBS authorisation hold created by the bank's own flow). **Release** happens when an authorised person decides in AKR-TMS (maker-checker for HOLD clearance where policy requires) and the gateway learns of it by **pull** (poll or long-poll on the decision id) or an optional **webhook** from AKR-TMS to the gateway zone (an outbound call to the gateway, never to the CBS; the bank decides whether to open that path).
- State machine of a held payment: `ASSESSING -> HOLD -> RELEASED | REJECTED | RETURNED | EXPIRED`, with timers and escalation (reusing the DBS timers and business calendar), and the CBI and AML/CFT Office clocks (the Office can suspend a transaction, reported as up to seven days; file 01).
- **A REJECT on a sanctions or listing match triggers the freeze process**: "without delay" means within hours for UN-designated persons (file 03); the CBI sanctions guidelines of 2023 say freezing measures are taken "immediately" on a **confirmed match**, define three match states (potential match, confirmed match, false positive; a match is also confirmed "when there is any suspicion or doubt"), forbid staff to tell the customer or any third party that a freeze will be applied, and quote the Terrorist Funds Freezing System No. 5 of 2016 under which the Committee for Freezing Terrorists' Funds must be informed **within three days** of the freeze [V, guidelines pp. 15, 21]. *Correction (2026-10-07, from files 23 and 28):* Terrorist Funds Freezing Regulation No. 6 of 2023 (in force 22 August 2023) replaced the 2016 system. It defines "without delay" as within 24 hours of the listing decision, and requires a first report to the Committee (copy to the AML/CFT Office) within **48 hours** of the freeze and a second report within **two months** [V, Arabic text checked by the lead; file 23 section 8]. AKR-TMS therefore creates the freeze task for bank operations with the freeze clock (default 8 hours, the strictest of the Office's texts; file 28 section 3.4) and the 48-hour and two-month report clocks (file 28 M-34), and later verifies from the replica that the account block is in place and that no debit followed the order (the brief's order-to-block time and post-order debit checks).
- **Release-without-decision control** (read-only, after the fact): every payment in scope that appears in the CBS or gateway as released must have a matching signed decision (L4 reconciliation, 8.1). Any release without a decision is an exception for the MLRO and the bank's internal audit. This detects a gateway silently configured fail-open, a bypass route, or an operator override.
- **Tipping-off**: the response returns only a generic `customer_message_code`; reason codes stay inside the bank's compliance zone (file 30 section 8).

### 15.8 Security of the call

mTLS with client certificates from the bank's CA, one client identity per gateway; Cedar policies bind that identity to a bank, rails and actions; request size and rate limits; payload kept out of logs (the DBS rule of never logging secrets extends to message content); signed responses (15.3); network path only from the gateway zone; the API host sits inside the bank's data centre next to the gateway, not in a shared zone with analysts' workstations (file 30 section 4).

### 15.9 Roll-out: shadow, advisory, enforcing

Per rail and corridor, in stages: **shadow** (assess and log, always answer RELEASE) to measure latency, hit volumes and false positives with real traffic; **advisory** (HOLD decisions create cases but the bank still releases manually); **enforcing** (the gateway honours HOLD and REJECT). Each stage change is a maker-checker approval with evidence (alert volumes, latency percentiles, test results) and is itself audited. Screening tuning and testing follow Wolfsberg and NYDFS expectations (files 04 and 05): documented thresholds, pre- and post-implementation tests, change control.

### 15.10 AKR-TMS hooks

- **Assessment service** (Axum, OpenAPI-first) as a separate Rust binary from the analyst API so it can be scaled, restarted and secured independently; shares the domain crates; own SLO dashboards.
- **In-memory list and feature caches** rebuilt from PostgreSQL, with the staleness guard (15.5).
- **Decision ledger** (immutable, signed, partitioned) and the **release-without-decision report**.
- **Fail-policy matrix** as admin-managed data with maker-checker; exported to the gateway team as configuration guidance.
- **Local fallback screen** image published beside the service in Harbor.
- **Shadow-mode switch**, **control-test-payment scheduler** and **outage report generator**.

---

## 16. What it means for AKR-TMS

1. **The integration is a product line of its own, not a connector.** The decisive questions are not technical (every engine can be read) but about permission, proof and safety: what the bank's DBA will approve, how AKR-TMS proves it saw everything, and how it proves it did no harm. The design should make those three things visible screens and artifacts: the integration profile and read-only attestation, the reconciliation ladder with daily coverage certificates, and the signed load budget with a bank-owned kill switch.
2. **Say what "read-only" really means, per deployment.** Level 1 (data-plane) is always delivered and always proved. Level 2 (footprint-free) holds for poll tiers and fails for CDC; level 3 (load-free) holds on a replica and is controlled by budget elsewhere. Honesty here is what lets a DBA say yes.
3. **Poll first, keep CDC optional.** A read-only account on bank-owned views is the smallest request a bank can approve; reconciliation proves completeness whatever the mechanism; CBS ledgers rarely delete; minutes of latency are enough for post-event monitoring, and real-time interdiction belongs to phase 2. CDC is justified where a bank has the DBA capacity and a licence-free path, or where tables are too large to poll. When it is used on Oracle or Db2, a Debezium Server sidecar is the pragmatic choice because no maintained Rust crate reads those logs (file 32).
4. **Some CBSs cannot be read through SQL at all.** Temenos on jBASE, branch systems of state banks and local builds need an R4 landing contract. The product must offer a documented file and landing-database contract with manifests and checksums, so that "no SQL access" is a delivery pattern rather than a blocker. For Temenos banks that run the vendor's data hub, its operational data store is a ready, relational R1 or R2 source.
5. **Completeness proof is the differentiator.** BCBS, NYDFS and the CBI all ask for it; most AML products treat it as the customer's problem. A ladder from batch receipts to GL tie-out, signed daily coverage certificates, a blind-period list, and a scenario coverage matrix driven by mapping completeness turn "we monitor everything" into evidence an assessor can check.
6. **The canonical model is lifecycle-aware, bi-temporal and lineage-rich.** It must record attempted and pending transactions (the CBI asks for attempted transactions to be reported), keep every date a CBS has (booking, value, business, local timestamp) and never overwrite a source's statement of what it said when.
7. **Mapping is data, owned by the bank, pinned and diff-able.** A closed transform library, a safe expression language, dictionary-driven generation for Transact, drift detection with per-table bulkheads, maker-checker publishing and replay-with-diff give a new bank a weeks-long path and protect against CBS upgrades.
8. **Payment data needs its own sources.** Domestic RTGS and ACH data live best in the gateway database; cross-border data live in message archives; MX is richer than MT and should be ingested natively. The CBI's own ISO 20022 plans are not public and must be requested.
9. **Phase 2 is a policy product as much as an API.** The fail-open or fail-closed choice belongs to the bank and varies by rail; AKR-TMS supplies the recommended matrix, shadow and advisory stages, signed decisions with expiry, a local fallback screen, and the control that detects payments released without a decision.
10. **Security rules shape the integration.** No on-demand per-customer queries (tipping-off by DBA), masked views, no CBS-admin credential in OpenBao, bank-side audit of the AKR-TMS account, default-deny egress.
11. **The long pole is the bank's approvals.** Plan elapsed time in weeks for access and change management; the product reduces it by making each request small, explicit, reversible and documented.

### Open questions and gaps (could not be settled from public sources)

- Which CBS, engine, version and edition each target bank runs, whether Active Data Guard (or equivalent) is licensed and open, and which banks run Temenos on jBASE: file 18 holds the market map; discovery must confirm per bank.
- The CBI's RTGS, ACH and switch message formats and any ISO 20022 migration plan (not found publicly).
- Table-level schemas for Finacle, FLEXCUBE, iMAL and ICS BANKS: only practitioner knowledge was available [K]; the vendor data dictionaries are a discovery deliverable.
- Db2 HADR reads-on-standby restrictions were confirmed from IBM's 11.1 documentation; the 11.5 and 12.1 pages were not retrieved.
- The BCBS 239 PDF could not be downloaded in this session; principle titles are quoted from memory.
- The Swift pages on structured-address timing (14 November 2026) and on MT retirement were not retrievable; trade-press summaries were used and are marked [S].
- Web search capacity ran out during this research (the session's 200-search budget), so later items rely on direct reads of known URLs and are marked accordingly.

---

## 17. Derived requirements

| ID | Requirement | Source |
|---|---|---|
| C-01 | Support four connection tiers (R1 replica read, R2 read-only views on the primary, R3 log-based CDC, R4 bank-delivered extract or landing) feeding one canonical pipeline, selected per bank in an admin-managed integration profile | Sections 1.3, 2; CBI controls 2025 ch. 5 art. 5; CBI handbook 2025 B2 |
| C-02 | AKR-TMS never writes to the CBS: no triggers, tables, jobs, database links or stored code created by AKR-TMS; the only CBS-side objects are views and capture settings applied by the bank's DBA, each listed in a footprint register with a bank owner | Brief; 1.2; 14.7 |
| C-03 | Read-only attestation on every (re)connect: inspect own grants in the engine catalogue, fail closed on any write-class privilege unless a named bank officer recorded an exception, store the signed report in the evidence vault | 1.4 |
| C-04 | State, per deployment, which of the three read-only levels (data-plane, footprint-free, load-free) is delivered and prove it | 1.2 |
| C-05 | For CDC tiers, publish consumer lag and retention headroom to the bank's monitoring and document the stalled-consumer risk and the bank-owned cap in the risk-acceptance paper | 1.2; PostgreSQL logical decoding manual; Debezium docs |
| C-06 | Default to poll-first ingestion behind a change-source interface (`poll`, `stream`, `file`) so a bank can move between tiers by configuration, with the same batch envelope, reconciliation and mapping | 2.3 |
| C-07 | Provide a per-bank discovery checklist and engine profile registry (engine, version, edition, replica status and licence, CDC permission, time zone, business-date and EOD rules, session limits, kill-switch owner) | 2.4, 3.7 |
| C-08 | Apply a per-engine read-consistency policy: Oracle read-only transaction per batch sized to undo retention; SQL Server snapshot or RCSI if enabled, else short read-committed statements with lock timeout and never NOLOCK; PostgreSQL REPEATABLE READ READ ONLY with conflict retry; Db2 standby reads flagged provisional | 3.7; Oracle, Microsoft, PostgreSQL, IBM documentation |
| C-09 | Never assume a DR standby is readable or that an Oracle Active Data Guard, GoldenGate or IIDR licence exists; the design must not depend on an option the bank has not bought | 3.1; Oracle Licensing Information; Debezium Db2 docs |
| C-10 | Run a character-set round-trip test at install (Arabic letters, tashkeel, Arabic-Indic and Persian digits, Kurdish letters) and record the result | 3.1, 3.2 |
| C-11 | Use Debezium Server as a sidecar only where CDC is agreed and no Rust-native path exists; sink to NATS JetStream or HTTP (not Kafka); perform AKR-TMS's own initial load on Oracle and SQL Server because Debezium incremental snapshots write to a signalling table | 4.1, 4.6; Debezium docs |
| C-12 | Maintain a licence and legal gate for CDC components (Debezium Apache-2.0, OpenLogReplicator AGPL-3.0, GoldenGate and IIDR as customer-owned licences) | 4.7 |
| C-13 | Enforce a signed per-bank load budget (sessions, statement time, rows per fetch and per minute, windows, EOD blackout, lag ceiling, slow-down triggers) with an adaptive controller, keyset pagination, bind variables and explicit column lists | 5.2, 5.3; Oracle Resource Manager documentation |
| C-14 | Do not parse or shred XML inside the CBS; fetch the raw record text through an indexed range and decode it in AKR-TMS | 5.3, 11.1 |
| C-15 | Treat account lock, grant removal and firewall closure by the bank's DBA as a normal "paused by bank" state: bounded back-off, an open coverage gap shown to the MLRO, automatic backfill on resume | 5.4 |
| C-16 | Provide a backfill planner that turns row estimates and the load budget into slices, windows and calendar days, with checkpoints | 5.1, 5.5 |
| C-17 | Model transaction lifecycle events (initiated, pending authorisation, authorised, posted, reversed, rejected, cancelled) so attempted and pending transactions are monitored and reportable | 6.3; file 01 (STR includes attempted transactions) |
| C-18 | Store booking date, value date, business date, source local timestamp, the bank's zone (Iraq UTC+3) and derived UTC; never convert silently | 6.4 |
| C-19 | Implement an EOD gate per source with day-close reconciliation and a day-reopen event for back-valued or late postings | 6.5 |
| C-20 | Record an as-of marker (SCN, row version, LSN, timestamp or none) with every batch and land data, watermark and follow-up jobs in one transaction (DBS golden rule) | 6.2, 6.7 |
| C-21 | Keep a watermark registry per table with a lookback window derived from measured late-arrival percentiles, automatic widening and use of safe-minimum primitives where the engine offers them | 7.2, 7.3; Microsoft MIN_ACTIVE_ROWVERSION; Oracle ORA_ROWSCN |
| C-22 | Evaluate scenarios at a settled time, recompute affected windows on late data, and amend alerts with an explanation instead of changing them silently; store data-as-of at decision time | 7.4 |
| C-23 | Never delete monitored data because the source did; keep tombstones; run key-set reconciliation with bisection; classify archive and purge events | 7.5 |
| C-24 | Load idempotently with `(source, table, pk)` keys, version and row-hash guards and a history table; retain the raw landing layer; provide replay with a diff before applying | 7.7, 7.8 |
| C-25 | Implement a reconciliation ladder (L0 batch receipt, L1 table and day, L2 account continuity, L3 GL tie-out, L4 payment tie-out, L5 sequence gaps, L6 cross-system) as a first-class service with four outcomes (reconciled, open, confirmed gap, source anomaly) | 8.1, 8.2; file 04 B-01; file 05 section 1f |
| C-26 | Sign a daily coverage certificate per source (OpenBao transit key) and anchor it in the evidence vault; feed it into the annual certification pack and the CBDDQ evidence pack | 8.2 |
| C-27 | Show an MLRO coverage statement (percent of accounts and transactions covered, data-as-of, blind periods with reasons) and a banner on any case that touches a blind period | 8.2 |
| C-28 | Run data-quality rules with owners and thresholds (nulls, referential integrity, placeholder dates, outliers, IBAN checksum, mojibake) and a bank-signed, expiring known-adjustments register | 8.3, 8.4; file 05 section 1f |
| C-29 | Define freshness classes F0 to F5 with SLOs, publish ingest-lag histograms per source and show `data_as_of` on every analyst screen, export and API response | 9 |
| C-30 | Request least-privilege access on bank-owned views (never base tables), with the per-engine minimum grants listed in the discovery pack | 10.1 |
| C-31 | Prefer certificate-based or passwordless authentication; otherwise keep the password in OpenBao KV, rotated by the bank's process, with no CBS-side administrative credential held by OpenBao and source-IP restriction on the account | 10.2; Vault documentation |
| C-32 | Use TLS 1.2 or later with validation against the bank's own CA on every CBS connection, run a handshake self-test, and require NTP on ingestion hosts | 10.3 |
| C-33 | Place ingestion nodes in a dedicated segment with an allow-listed firewall path to the CBS listener ports only, default-deny egress, and no path from analyst workstations through AKR-TMS to the CBS | 10.4; DBS decision D6 |
| C-34 | Ask the bank to enable native audit on the AKR-TMS database account and forward it to the bank's SIEM | 10.5; CBI 2026 controls (outsourcing) |
| C-35 | Never query the CBS by customer, account or case on demand; enforce with an automated test; investigators are served from AKR-TMS's own store | 10.6 |
| C-36 | Minimise data at the source: no full card numbers, PIN data, authentication secrets or document images in the views; require masked or tokenised views where the CBS encrypts fields at application level | 10.7; PCI DSS (file 30) |
| C-37 | Generate Temenos-style field maps from the bank's own dictionary (including local reference fields), regenerate on dictionary change and alarm on renumbering | 11.1 |
| C-38 | Ship adapter packs per CBS family and engine (dictionary reader, view DDL, mapping templates, code maps, EOD-gate and reconciliation templates, golden test data, quirks notes) as signed release artifacts with a conformance suite | 11, 14 |
| C-39 | Define and document an R4 landing contract (files or tables with manifest, row counts, checksums, business date) for CBSs without SQL access and for branch-level systems | 11.1, 11.6 |
| C-40 | Provide a read-only discovery kit per engine that produces the data-source inventory (tables, columns, row counts, indexes, partitions, character set, replica status, grants) | 11.8; file 05 section 1f |
| C-41 | Ingest native ISO 20022 (CBPR+) and MT messages, retain the raw message, parse in Rust with versioned and fuzz-tested parsers, and record the parser version on each parsed record | 12.2, 12.3, 12.7 |
| C-42 | Support payment-gateway database adapters (RTGS, ACH, cheque items) as the preferred structured source for domestic payments | 12.4; ProgressSoft PS-GatePay |
| C-43 | Keep a payment-link table between messages and ledger postings with match rules and confidence; route unmatched items to reconciliation level L4 | 12.5 |
| C-44 | Provide an IBAN service for the 23-character Iraqi IBAN (checksum, bank and branch decode, BIC lookup maintained as admin data) | 12.1 |
| C-45 | Obtain from the CBI Payments Department the RTGS, ACH and national-switch message formats and any ISO 20022 migration plan; keep the canonical payment model ISO 20022-shaped | 12.1 |
| C-46 | Build the canonical model with lineage on every row, bi-temporal slowly changing entities, UUIDv7 ids, positive amounts with debit/credit indicator, local and USD equivalents, a data-classification tag on every column and a typed attribute registry for bank-specific fields | 13.1 |
| C-47 | Cover the entity set in 13.2 (party and names, identifiers, contacts, relationships, customer, account and roles, holds, balances, transactions and events, counterparties, products, channels, branches, cards, facilities, trade instruments, payment messages and parties, cash, FX, lockers, staff, optional CBS user activity) | 13.2; file 27 |
| C-48 | Compute and export a scenario coverage matrix (operable, degraded, blocked) from mapping completeness against the CBI scenario catalogue and bank-defined scenarios | 13.6; file 01 section 2d; file 27 |
| C-49 | Partition fact tables by booking month with partition-maintenance jobs, encrypt personal fields with OpenBao-wrapped keys and use keyed hashes for match keys | 13.4; DBS conventions |
| C-50 | Hold mappings as admin-managed, JSON-Schema-validated data using a closed, versioned transform library and a non-Turing-complete expression language; map unknown codes to `unknown` plus a data-quality issue | 14.1, 14.2 |
| C-51 | Run a mapping lifecycle: draft, test on real and golden data, impact analysis, maker-checker approval, publish and pin by batch, replay with diff, rollback | 14.3 |
| C-52 | Detect schema and dictionary drift at every run and pause only the affected tables (bulkheads) with a named alarm | 14.4 |
| C-53 | Resolve multi-source attribute conflicts with explicit, explainable survivorship rules and reversible, maker-checker party merge and split | 14.5 |
| C-54 | Generate only parsed, single-statement read-only SELECTs over approved objects; no ad hoc SQL against the CBS from any UI | 14.1, 14.7 |
| C-55 | Phase 2: expose a synchronous assess endpoint with mTLS, Cedar-bound client identities, idempotent byte-exact replay, signed decisions with an expiry and generic customer-facing codes | 15.3, 15.8 |
| C-56 | Phase 2: meet service-side objectives of p95 150 ms and p99 400 ms (to be proven by load test) and roll out per rail through shadow, advisory and enforcing stages with maker-checker approval | 15.4, 15.9 |
| C-57 | Phase 2: hold the fail-open and fail-closed policy per rail, corridor, currency and amount as admin-managed data with recommended defaults, flag every degraded mode in the response, and never answer RELEASE on stale lists | 15.5; CBI lists refresh rule; Wolfsberg sanctions guidance |
| C-58 | Phase 2: run two stateless nodes, a local fallback screen beside the gateway, a recorded pre-release blind period for any outage, quarterly failover drills and a weekly control-test payment | 15.6 |
| C-59 | Phase 2: report every payment released without a matching signed decision (release-without-decision control) | 15.7 |
| C-60 | Phase 2: on a sanctions REJECT create the freeze task for bank operations and verify the block and the absence of post-order debits from the replica; keep a held-payment state machine with timers and pull or webhook release | 15.7; file 03 ("without delay"); brief |
| C-61 | Reach engines without a pure-Rust driver (Db2 LUW and Db2 for i, Sybase ASE) through ODBC with bank-supplied drivers behind the same source interface, and fall back to the R4 landing contract when the bank refuses an ODBC read-only role | 3.4, 3.5, 11.6; file 32 section 4 |
| C-62 | Order adapter packs by market evidence (ICS BANKS on Oracle first, then iMAL, BML ICBS, FLEXCUBE, Temenos; Finacle only on demand) and let the first customer in each family fund and validate the pack | 11; files 17 and 18 |
| C-63 | Support the CBI sanctions-guideline duties: screen the whole customer database daily against lists refreshed at least every 24 hours (12 hours under the 2025 controls), model match states (potential, confirmed, false positive), start a freeze task on a confirmed match with the freeze clock (default 8 hours, the strictest of the AML/CFT Office's texts; at most 24 hours from the listing decision under Regulation No. 6 of 2023) and the Committee report clocks of Regulation No. 6 of 2023 (first report within 48 hours, second within two months; the three days in the 2023 guidelines quote the repealed 2016 system; file 28 M-34), and block staff-visible signals that would tip off the customer | 9.2, 15.7; CBI sanctions guidelines 2023 (pp. 15, 20, 21, 23); Regulation No. 6 of 2023 (file 23 section 8) |
| C-64 | Detect CBS time regression (restore to an earlier point, or failover with lost commits): halt watermark advance for the source, alert the MLRO and the bank's DBA, keep rows that disappeared, re-run reconciliation from the earliest affected date and report the window in the coverage statement | 7.5; file 30 section 16.5 |
| C-65 | Phase 2: a score can produce at most HOLD; REJECT without a person only for deterministic, bank-configured prohibited conditions with the rule id recorded; every other reject is a recorded human decision | 15.3; file 30 section 19.5 |

---

## 18. Sources

Regulators and standards
- CBI, General guidelines for banks and non-bank financial institutions on local and international sanctions (1 August 2023, Arabic scan, 23 pages): https://cbi.iq/static/uploads/up/file-169554063119385.pdf
- CBI standards handbook for private commercial and Islamic banks, 2025 (Arabic; standards B2, B7, B8, B9, D2), via https://cbi.iq/ (supervision documents page 139; local copy cbi-handbook-2025)
- CBI risk-management controls for commercial banks, 13 July 2026 (Arabic; internal control 4.2.3, outsourcing, IT risk): https://cbi.iq/static/uploads/up/file-178480215427134.pdf
- CBI, "Progress in payment system led to controlling the flow of funds" (English statement): https://cbi.iq/static/uploads/up/file-152326652419124.pdf
- CBI, "Electronic payment systems in Iraq: components, role, risk management" (Arabic, 2013): https://cbi.iq/static/uploads/up/file-152232072727068.pdf
- Regulation (EU) 2024/886 (instant payments; Articles 5a and 5d), EUR-Lex: https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32024R0886
- Wolfsberg Group Guidance on Sanctions Screening (2019): https://db.wolfsberg-group.org/assets/29c751c6-f406-4329-a267-1b05aabdd241/Wolfsberg%20Guidance%20on%20Sanctions%20Screening.pdf
- Wolfsberg Group Payment Transparency Standards 2023: https://db.wolfsberg-group.org/assets/13422898-fba1-44b3-9679-a8c7406e9e78/Wolfsberg%20Group%20Payment%20Transparency%20Standards%202023.pdf
- IETF draft-ietf-httpapi-idempotency-key-header (revision 07, expired): https://datatracker.ietf.org/doc/draft-ietf-httpapi-idempotency-key-header/

Databases and replication
- Debezium Oracle connector: https://debezium.io/documentation/reference/stable/connectors/oracle.html
- Debezium SQL Server connector: https://debezium.io/documentation/reference/stable/connectors/sqlserver.html
- Debezium Db2 connector: https://debezium.io/documentation/reference/stable/connectors/db2.html
- Debezium PostgreSQL connector: https://debezium.io/documentation/reference/stable/connectors/postgresql.html
- Debezium MySQL connector: https://debezium.io/documentation/reference/stable/connectors/mysql.html
- Debezium Server: https://debezium.io/documentation/reference/stable/operations/debezium-server.html
- Debezium releases (3.7, 29 September 2026): https://debezium.io/releases/
- Red Hat build of Debezium supported configurations (Oracle 19, 21, 23, 26) [S, from search snippet]: https://access.redhat.com/articles/4938181
- Oracle Database 19c Licensing Information (Active Data Guard features): https://docs.oracle.com/en/database/oracle/oracle-database/19/dblic/Licensing-Information.html
- Oracle Database Concepts, data concurrency and consistency: https://docs.oracle.com/en/database/oracle/oracle-database/19/cncpt/data-concurrency-and-consistency.html
- Oracle SQL Language Reference, ORA_ROWSCN: https://docs.oracle.com/en/database/oracle/oracle-database/19/sqlrf/ORA_ROWSCN-Pseudocolumn.html
- Oracle Database Resource Manager: https://docs.oracle.com/en/database/oracle/oracle-database/19/admin/managing-resources-with-oracle-database-resource-manager.html
- Oracle GoldenGate, downstream Extract with Active Data Guard [S, search snippet]: https://docs.oracle.com/en/middleware/goldengate/core/21.3/coredoc/extract-cascaded-downstream-capture-extract-cascaded-downstream-capture-using-adg.html
- Oracle GoldenGate Free documentation [S]: https://docs.oracle.com/en/middleware/goldengate/free/23/index.html
- Active Data Guard price and trigger summary (consultancy) [S]: https://redresscompliance.com/oracle-active-data-guard-licensing-guide-2026.html
- Qlik Replicate, Oracle Data Guard (May 2026): https://help.qlik.com/en-US/replicate/May2026/Content/Global_Common/Content/SharedReplicateHDD/Oracle/OracleDBSource_DataGuard.htm
- Qlik Replicate supported source endpoints [S, search snippet]: https://help.qlik.com/en-US/replicate/May2026/Content/Replicate/Main/Support%20Matrix/supported_source_endpoints.htm
- Microsoft Learn, readable secondary replicas: https://learn.microsoft.com/en-us/sql/database-engine/availability-groups/windows/active-secondaries-readable-secondary-replicas-always-on-availability-groups
- Microsoft Learn, basic availability groups: https://learn.microsoft.com/en-us/sql/database-engine/availability-groups/windows/basic-availability-groups-always-on-availability-groups
- Microsoft Learn, SET TRANSACTION ISOLATION LEVEL: https://learn.microsoft.com/en-us/sql/t-sql/statements/set-transaction-isolation-level-transact-sql
- Microsoft Learn, MIN_ACTIVE_ROWVERSION: https://learn.microsoft.com/en-us/sql/t-sql/functions/min-active-rowversion-transact-sql
- PostgreSQL 17, hot standby: https://www.postgresql.org/docs/17/hot-standby.html
- PostgreSQL 17, logical decoding concepts: https://www.postgresql.org/docs/17/logicaldecoding-explanation.html
- Morling, logical replication from standby servers and failover slots [S]: https://www.morling.dev/blog/logical-replication-from-postgres-stand-by-servers-debezium-and-failover-slots/
- Decodable, logical replication from Postgres 16 standby servers [S]: https://www.decodable.co/blog/logical-replication-from-postgres-16-stand-by-servers-part-1-of-2
- IBM Db2, reads on standby restrictions (11.1 documentation): https://www.ibm.com/support/knowledgecenter/en/SSEPGG_11.1.0/com.ibm.db2.luw.admin.ha.doc/doc/c0054258.html
- Supabase ETL: https://github.com/supabase/etl
- OpenLogReplicator: https://github.com/bersler/OpenLogReplicator
- HashiCorp Vault, database secrets engine: https://developer.hashicorp.com/vault/docs/secrets/databases

Core banking systems and payment infrastructure
- Temenos Transact Data Hub brochure (2020): https://archive.temenos.com/wp-content/uploads/2020/09/temenos-transact-data-hub.pdf
- IBM community, T24 data stored as XML (Guardium) [S]: https://community.ibm.com/community/user/discussion/how-can-guardium-parse-t24-application-data-which-is-stored-in-xml-string
- Qlik community, T24 RECID and XMLRECORD query example [S; page blocked, content seen in a search summary]: https://community.qlik.com/t5/Talend-Studio/Select-RECID-In-Oracle-Giving-Wrong-Data/td-p/2358154
- Oracle white paper, Finacle on Oracle SuperCluster (2014): https://www.oracle.com/docs/tech/infosys-finacle-wr-osc.pdf
- ICS Financial Systems: https://www.icsfs.com/ , https://www.icsfs.com/products/core-banking/ , https://www.icsfs.com/about/why-icsfs/ , https://icsfs.com/insights/success-stories/the-first-and-only-digital-bank-in-iraq-to-partner-with-icsfs/
- Fintech Futures, "Path Solutions beats ICS Financial Systems to core banking system deal in Iraq" [S, headline only; page blocked]: https://www.fintechfutures.com/core-banking-technology/path-solutions-beats-ics-financial-systems-to-core-banking-system-deal-in-iraq
- ProgressSoft, PS-GatePay in Iraq (22 January 2019): https://www.progressoft.com/news/banks-in-iraq-automate-their-clearing-and-settlement-processes-using-ps-gatepay
- Iraq IBAN structure [S]: https://bank-code.net/iban/structure/iraq-international-bank-account-number
- ACI Worldwide, end of MT/MX coexistence (summarising Swift's timeline) [S]: https://www.aciworldwide.com/blog/the-end-of-coexistence-is-near-banks-take-an-existential-leap-towards-iso-20022
- DPS, 100 days to end of MT/MX coexistence [S]: https://dps.de/en/news/iso-20022-only-100-days-left-until-the-end-of-mt-mx-coexistence/
- Tazapay, ISO 20022 November 2026 structured address guide [S, search snippet]: https://tazapay.com/guides/iso-20022-november-2026-structured-address-compliance-guide

Local repositories and companion files
- digital-banking-system (read only): `tech-stack.md`, `docs/ARCHITECTURE.md`, `.claude/rules/jobs.md`, `backend/migrations/0003_idempotency_byte_exact_replay.sql`, `backend/migrations/0019_readonly_reporting_grants.sql`, `deploy/prod/README.md`
- AKR-TMS research files 01, 03, 04, 05, 14, 17, 18, 22, 24, 27 (cross-references in the text)
