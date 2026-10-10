# Gap pass 1, run 5a (coverage: C, E, RP, S, TS; Haiku draft for lead review)

Scope: the 59 research IDs named by no phase header (tools/coverage.py). Class key: CARRIED (built in substance by a phase
that does not name the ID), Partly (built in part; missing part named), NOT-CARRIED (outside this release), GAP (in scope
and no phase builds it; full finding in Part 2). File references are docs/plan/phase-NN.md:LINE unless stated.

## Part 1. Classification

| ID | Research row (file:line) | Requirement | Class | Evidence (phase-NN.md:LINE "quote" or source) | Proposed action |
|---|---|---|---|---|---|
| C-05 | research/25-core-banking-integration.md:1138 | Publish CDC consumer lag and retention headroom; document stalled-consumer risk | Partly | phase-11.md:19 "freshness classes with ingest-lag measures" (C-29); phase-05.md:146 "lag_ceiling_s" (CT1 only) | Add C-05 to phase 11 header, line Partly implemented. Built: ingest-lag measures. Missing: CDC consumer lag and log retention headroom published to monitoring, and the stalled-consumer alarm. |
| C-11 | research/25-core-banking-integration.md:1144 | Debezium Server only where CDC agreed; sink to NATS JetStream or HTTP; AKR-run initial load on Oracle and SQL Server | Partly | phase-11.md:1323 "Change capture (opt-in). Compose profile `cdc` adds `akr-cdc`"; phase-11.md:866 "runs in the source's own network zone" | Add C-11 to phase 11 header, line Partly implemented. Built: opt-in sidecar behind the signed agreement. Missing: sink is a file spool (phase-11.md:869), not the NATS or HTTP sink of TS-08; the AKR-run initial load is not stated. |
| C-12 | research/25-core-banking-integration.md:1145 | Licence and legal gate for CDC components (Debezium, OpenLogReplicator, GoldenGate, IIDR) | Partly | phase-11.md:1323 (signed change-capture agreement gate, DR-91); phase-11.md:1520 "its licence and operating footprint, are not decided" | Add C-12 to phase 11 header, line Partly implemented. Built: the agreement gate. Missing: a licence check for the sidecar image and the other CDC products (phase-00.md:368 deny.toml covers crates only). |
| C-14 | research/25-core-banking-integration.md:1147 | Do not parse XML in the CBS; fetch raw record text by indexed range and decode in AKR-TMS | GAP | phase-10.md:1816 "`t24_*` ... refused ... (phase 11 and the parser phases add them)"; phase-11.md:107 only has `t24_date_time` as a watermark kind | Finding G1-11-01 (MEDIUM). |
| C-17 | research/25-core-banking-integration.md:1150 | Model lifecycle events (initiated to cancelled) so attempted and pending transactions are monitored | CARRIED | phase-10.md:1002 "kind IN ('initiated', 'pending', 'authorised', 'posted', 'reversed'"; phase-10.md:968 "status IN ('posted', 'cancelled')" | Add C-17 to phase 10 header, line Implemented here. |
| C-18 | research/25-core-banking-integration.md:1151 | Store booking, value, business dates, source local timestamp, bank zone and derived UTC; never convert silently | CARRIED | phase-10.md:962 "value_date DATE NOT NULL"; phase-10.md:965 "source_local_ts TEXT NOT NULL"; phase-00.md:235 "bank.timezone" = "Asia/Baghdad" | Add C-18 to phase 10 header, line Implemented here. |
| C-19 | research/25-core-banking-integration.md:1152 | EOD gate per source with day-close reconciliation and a day-reopen event for back-valued or late postings | Partly | phase-11.md:760 "The last closed business date of each branch"; phase-11.md:15 "a late posting is a new row" (C-22) | Add C-19 to phase 11 header, line Partly implemented. Built: closed-day gate and empty-unit watermark move. Missing: a day-reopen event when a late or back-valued posting reaches a closed day; the day-close rung is phase 12 (C-25). |
| C-31 | research/25-core-banking-integration.md:1164 | Certificate or passwordless auth; else password in OpenBao KV with no CBS admin credential; source-IP restriction | Partly | phase-05.md:244 "read only by `akr-ingestd`" (DR-25, DR-59); phase-05.md:765 "writes them to OpenBao KV `akr/sources/<id>`" | Add C-31 to phase 05 header, line Partly implemented. Built: password in OpenBao KV, read only by akr-ingestd. Missing: certificate or passwordless option and the source-IP restriction statement in the connect proof. |
| C-32 | research/25-core-banking-integration.md:1165 | TLS 1.2 or later validated against the bank CA; handshake self-test; NTP on ingestion hosts | Partly | phase-05.md:764 "TLS trust (CA bundle path or wallet directory reference)"; phase-05.md:829 "`tls_failed`" | Add C-32 to phase 05 header, line Partly implemented. Built: CA trust and the TLS failure code. Missing: TLS 1.2 floor, handshake self-test at install, NTP requirement on ingestion hosts (grep found no ntp in phases 00 to 05). |
| C-33 | research/25-core-banking-integration.md:1166 | Ingestion nodes in a dedicated segment; allow-listed path to CBS listener ports; default-deny egress | CARRIED | phase-00.md:710 "Egress to the CBS (zone Z0) is enforced on the host"; phase-00.md:711 nftables let only `akr-ingestd`'s address reach the CBS listeners | Add C-33 to phase 00 header, line Implemented here. |
| C-34 | research/25-core-banking-integration.md:1167 | Bank enables native audit on the AKR-TMS database account and forwards it to the bank SIEM | NOT-CARRIED | Bank DBA and SIEM action (research 25 s.10.5); no phase names native audit (grep found none); AKR's own SIEM export is phase 25 (phase-02.md:22) | Add C-34 to phase 05 header, line Not carried (bank DBA enables native audit on its read account; bank SIEM). |
| C-37 | research/25-core-banking-integration.md:1170 | Field maps generated from the bank's own dictionary; regenerate on change; alarm on renumbering | Partly | phase-10.md:1818 "`field:NAME` resolved through the adapter pack's dictionary positions"; phase-05.md:1118 "Code dictionaries (the pack's `field_dictionary`" | Add C-37 to phase 10 header, line Partly implemented. Built: field references resolved through dictionary positions. Missing: generated maps, regeneration on dictionary change, renumbering alarm. |
| C-39 | research/25-core-banking-integration.md:1172 | R4 landing contract: files with manifest, row counts, checksums and business date | CARRIED | phase-11.md:885 "Its manifest (schema `akr.extract.v1`) names"; phase-11.md:886 files with sha256, bytes, rows | Add C-39 to phase 11 header, line Implemented here. |
| C-40 | research/25-core-banking-integration.md:1173 | Read-only discovery kit per engine: tables, columns, counts, indexes, partitions, charset, replica status, grants | Partly | phase-05.md:1038 "**Catalogue run** (dictionary statements only"; phase-05.md:1041 "`ALL_TAB_PARTITIONS` (counts)" | Add C-40 to phase 05 header, line Partly implemented. Built: catalogue, partitions, charset, grants. Missing: replica status in the inventory. |
| C-41 | research/25-core-banking-integration.md:1174 | Ingest ISO 20022 and MT; retain raw message; versioned fuzz-tested Rust parsers; record parser version | GAP | phase-11.md:1535 "No ISO 20022 or MT parser ... (phases 12 and later under C-41 and C-42)"; no phase 12 to 31 parser (grep); phase-10.md:1025 `format IN ('iso20022', 'mt')` is storage only | Finding G1-11-02 (HIGH). |
| C-42 | research/25-core-banking-integration.md:1175 | Payment-gateway database adapters (RTGS, ACH, cheque items) as preferred source for domestic payments | GAP | phase-11.md:1535 (same deferral); phase-05.md:468 adapters are pg, mssql, oracle, odbc only | Finding G1-11-03 (HIGH). |
| C-43 | research/25-core-banking-integration.md:1176 | Payment-link table between messages and postings with match rules and confidence; unmatched to L4 | GAP | phase-10.md:1079 "CREATE TABLE IF NOT EXISTS canon.payment_link" (match_keys, confidence, rule); no writer or matcher in any phase; phase-12.md:1364 "no payment tie-out" | Finding G1-12-01 (MEDIUM). |
| C-44 | research/25-core-banking-integration.md:1177 | IBAN service: 23-character checksum, bank and branch decode, BIC lookup kept as admin data | Partly | phase-10.md:1859 "checks the 23-character Iraqi IBAN checksum and decodes the bank and branch (C-44)"; grep found no BIC lookup table | Add C-44 to phase 10 header, line Partly implemented. Built: checksum and decode. Missing: BIC lookup as admin data. |
| C-45 | research/25-core-banking-integration.md:1178 | Obtain CBI RTGS, ACH and switch formats and any ISO migration plan; keep canonical payment model ISO-shaped | Partly | phase-10.md:1025 "format IN ('iso20022', 'mt')" (ISO-shaped model); no phase lists the request to the CBI Payments Department | Add C-45 to phase 10 header, line Partly implemented. Built: ISO-shaped model. The CBI request is the bank's process and appears in no open doubt. |
| C-49 | research/25-core-banking-integration.md:1182 | Partition fact tables by booking month; encrypt personal fields with OpenBao-wrapped keys; keyed hashes for match keys | CARRIED | phase-10.md:1180 "Monthly partitions; 7 or 10 years by booking month"; phase-00.md:293 "AES-256-GCM ciphertext under an OpenBao-wrapped DEK"; phase-10.md:1085 `match_keys` | Add C-49 to phase 10 header, line Implemented here. |
| C-53 | research/25-core-banking-integration.md:1186 | Explicit, explainable survivorship rules; reversible maker-checker party merge and split | GAP | phase-10.md:1871 "Party merge and split stay maker-checker actions of phase 12 (C-53)"; phase-12.md has no merge or split (grep); phase-10.md:1869 longest_verified refused | Finding G1-12-02 (MEDIUM). |
| E-01 | research/04-basel-wolfsberg-egmont-un.md:1583 | Register FIU and authority requests; response package builder covering the INR.29 items; deadline tracking | Partly | phase-19.md:23 "`casework.internal_reports / authority_requests`" with phase-19.md:59 "deadline_at TIMESTAMPTZ NOT NULL"; phase-21.md:27 `office_reply` letter kind; grep found no INR.29 item list or response builder | Add E-01 to phase 19 header, line Partly implemented. Built: authority request register and deadline clock. Missing: INR.29 item list and the response package builder (the reply letter is phase 21). |
| E-02 | research/04-basel-wolfsberg-egmont-un.md:1584 | Register of suspension and freeze orders; read-only check that the CBS block is set; order-to-block KPI | Partly | phase-20.md:11 "M-34 (true match on a freeze list starts the freeze task"; phase-20.md:1079 "DR-160 (verification from the canonical copy; levels V0 to V3)" | Add E-02 to phase 20 header, line Partly implemented. Built: order register as tasks and verification levels. Missing: the order-to-block KPI (grep for KPI in phase 20 found none; the tile would sit in phase 24). |
| E-03 | research/04-basel-wolfsberg-egmont-un.md:1585 | STRs carry structured identifiers, all name spellings, phones, e-mails, IP and device data with foreign flags; pre-submission checks | Partly | phase-21.md:800 "Identifiers: `canon.party_identifier` rows with `superseded_at IS NULL`"; phase-21.md:808 "L0, canonical completeness (new in phase 21)" | Add E-03 to phase 21 header, line Partly implemented. Built: identifiers, names and the L0 completeness check. Missing: IP and device data with foreign flags in the report content (grep for ip and device in phases 07 and 21 found none). |
| E-04 | research/04-basel-wolfsberg-egmont-un.md:1586 | Timeliness KPIs: first alert to STR, escalation to STR | Partly | phase-21.md:12 "RP-31 (timeliness metrics: the intervals are computed"; phase-21.md:1406 "Timeliness explanation (RP-31)" | Add E-04 to phase 21 header, line Partly implemented. Built: filing interval metrics. Missing: named first-alert-to-STR and escalation-to-STR intervals (the anchor dates are not defined). |
| E-05 | research/04-basel-wolfsberg-egmont-un.md:1587 | FIU feedback register feeding tuning and STR usefulness indicators | Partly | phase-21.md:208 "FIU feedback register (RP-32, RP-84). Append-only."; phase-21.md:12 "RP-29 (the calibration sample is a monthly job" | Add E-05 to phase 21 header, line Partly implemented. Built: the register and the calibration sample. Missing: any feed into tuning (phase 16 parameter sets) and the usefulness indicators. |
| E-06 | research/04-basel-wolfsberg-egmont-un.md:1588 | CTRs at USD 10,000 through goAML; CFTF match and frozen-amount reports through IQTFS | Partly | phase-07.md:594 "BR-13 | CTR shape ... `threshold_usd` 10,000"; phase-21.md:307 letter kinds `freeze_report`, `freeze_second_report`; grep for IQTFS and CFTF found none | Add E-06 to phase 21 header, line Partly implemented. Built: CTR threshold and CTR batches, freeze letters. Missing: CFTF match report and IQTFS frozen-amount report. |
| E-07 | research/04-basel-wolfsberg-egmont-un.md:1589 | STR and authority-request data on a need-to-know basis, full access logging, encryption, tipping-off controls | CARRIED | phase-03.md:29 "the suspicion-tier grant rule is enforced from phase 18"; phase-18.md:587 "access event on a suspicion case" | Add E-07 to phase 18 header, line Implemented here. |
| RP-08 | research/23-regulatory-reporting-goaml.md:749 | Several profiles active in one installation, each with its own clocks, templates and numbering | CARRIED | phase-07.md:97 "CREATE UNIQUE INDEX ... uq_goaml_profiles_active ON rpt.goaml_profiles (code) WHERE status = 'active'" (one active per code, several codes allowed) | Add RP-08 to phase 07 header, line Implemented here. Check the per-profile clocks and numbering in the phase 07 profile body before the lead accepts. |
| RP-10 | research/23-regulatory-reporting-goaml.md:751 | Code maps from CBS codes to goAML lookups, effective-dated, maker-checker, with unmapped-code coverage report | Partly | phase-10.md:127 "CREATE TABLE IF NOT EXISTS admin.code_maps"; phase-10.md:144 "Entries are effective-dated"; phase-11.md:795 unmapped codes "open a data-quality issue" | Add RP-10 to phase 10 header, line Partly implemented. Built: effective-dated, maker-checker code maps. Missing: the goAML lookup kinds (funds, transaction mode, account type, legal form, party roles, identifier types, indicators) and the goAML unmapped-code coverage report. |
| RP-12 | research/23-regulatory-reporting-goaml.md:754 | MLRO decision has four outcomes, each with timestamp, reasons and evidence snapshot; UTR escalates to STR citing its number | Partly | phase-18.md:10 "M-01 (outcome set `no_report`, `utr`, `str`, `tfr`"; phase-21.md:174 "'escalated_from_utr'" | Add RP-12 to phase 18 header, line Partly implemented. Built: outcome set with reasons, the UTR-to-STR link. Missing: the "gather more evidence" outcome (RFI is a separate act) and SAR as a decision outcome. |
| RP-13 | research/23-regulatory-reporting-goaml.md:755 | Internal chain: observation to liaison to MLRO, written acknowledgment with tipping-off reminder, written decision notice, liaison report by the 10th | CARRIED | phase-22.md:62 "reporter_ack_code ... the tipping-off reminder version"; phase-22.md:613 "The staff decision notice (M-66)"; phase-22.md:307 "branch.liaison_due_day" = 10 | Add RP-13 to phase 22 header, line Implemented here. |
| RP-35 | research/23-regulatory-reporting-goaml.md:780 | MLRO and deputy registry with contact details as notified to the Office; task to notify the Office and the CBI on change | GAP | phase-04.md:708 "`NamedOfficers` ... mlro -> active holders of M" returns user ids only; grep for officer contact or Office notice found none | Finding G1-21-01 (MEDIUM). |
| S-11 | research/27-transaction-monitoring-scenarios.md:2360 | Resolved counterparty on every transaction with first-seen date, country, bank and class | Partly | phase-10.md:922 "CREATE TABLE IF NOT EXISTS canon.counterparty (" (first_seen, last_seen, bank_bic); phase-10.md:972 `counterparty_id` on canon.txn; phase-16.md:2119 "no phase writes the resolver" | Add S-11 to phase 10 header, line Partly implemented. Built: counterparty table and transaction link. Missing: the resolver that assigns the counterparty on each transaction and sets first-seen and class (unowned; DSL A.6 names it a feature writer). |
| S-14 | research/27-transaction-monitoring-scenarios.md:2363 | Effective-dated, versioned country and area risk table and a flagged-district table for TF, with owner, source and quarterly review; table version on every alert | Partly | phase-09.md:38 "risk_level TEXT NOT NULL CHECK"; phase-09.md:46 "CREATE TABLE IF NOT EXISTS ref.country_risk_overrides" | Add S-14 to phase 09 header, line Partly implemented. Built: country risk level with expiring overrides. Missing: area risk and flagged-district table for TF, its quarterly review, and the table version stamped on alerts (grep for district found none). |
| S-15 | research/27-transaction-monitoring-scenarios.md:2364 | Governed reference registers (dual-use and HS list, vessel and IMO list, sector-goods map, peer price tables, crypto counterparties, government-source flag, DNFBP classes) with owner, version and review date | GAP | phase-09.md:35 to 141 create only countries, overrides, governorates, currencies, fx, calendars; no register table (grep for dual-use, peer price and DNFBP found none); phase-10.md:1344 (vessel_imo only) | Finding G1-09-01 (MEDIUM). |
| S-16 | research/27-transaction-monitoring-scenarios.md:2365 | Cheque instrument record (drawer, payee, endorsers, clearing status, return reason) and available-versus-cleared balance | Partly | phase-10.md:902 "cleared_balance NUMERIC(28,6)"; phase-10.md:904 "available_balance ... null until a source column is confirmed"; grep for cheque in phases 05 to 13 found none | Add S-16 to phase 10 header, line Partly implemented. Built: cleared and available balance fields. Missing: the cheque instrument record and its clearing and return fields. |
| S-17 | research/27-transaction-monitoring-scenarios.md:2366 | Payroll and pension batch tags and a government-source flag so public-salary accounts form peer segments | Partly | DSL.md:374 "account.type" value `government_payroll_pool` used as an exclusion; phase-13.md:1100 "Segment-of-one. T-08 (large corporates and government bodies ...) is not built" | Add S-17 to phase 13 header, line Partly implemented. Built: the exclusion on a government payroll pool account type. Missing: payroll and pension batch tags, the government-source flag as a segment input, and the government segment-of-one (T-08). |
| S-18 | research/27-transaction-monitoring-scenarios.md:2367 | Facility event stream (disbursement, repayment with payer identity, early settlement, collateral and guarantor change) and collateral and guarantor graph | Partly | phase-10.md:1307 "CREATE TABLE IF NOT EXISTS canon.facility" | Add S-18 to phase 10 header, line Partly implemented. Built: facility table. Missing: the facility event table, payer identity on repayments, and the collateral and guarantor graph (grep for collateral and guarantor found none in phases 10 to 19). |
| S-19 | research/27-transaction-monitoring-scenarios.md:2368 | Optional connectors (customs, vessel, company registry, civil registry, NGO registry, adverse media, HR) shown in readiness as upgrading scenarios from C to B | Partly | phase-06.md:81 "external_needs TEXT[]"; phase-06.md:452 "External feeds the bank has connected ... Empty in P0" | Add S-19 to phase 06 header, line Partly implemented. Built: external needs on the register and the empty feed list. Missing: the named connector list and the C-to-B upgrade shown per connector in the readiness view. |
| S-20 | research/27-transaction-monitoring-scenarios.md:2369 | Optional read-only links to channel databases and the card processor, labelled grade C, with retention limits and a purpose statement | GAP | phase-06.md:453 says "device and channel data in 26", but phase-26.md:1 is the Islamic pack; no channel or card-processor source in phases 05 to 12 | Finding G1-11-04 (LOW). |
| S-21 | research/27-transaction-monitoring-scenarios.md:2370 | Teller event capture (cancelled, reduced and amended entries, depositor ID, denominations, counterfeit flag, operator ID) and depositor-ID capture rate per branch | Partly | phase-10.md:1255 "CREATE TABLE IF NOT EXISTS canon.cash_event" (denominations, declared_source, ctr_flag); phase-10.md:995 canon.txn `operator_id` | Add S-21 to phase 10 header, line Partly implemented. Built: cash event denominations and operator on the transaction. Missing: cancelled, reduced and amended entries, depositor ID, counterfeit flag and the capture-rate measure. |
| S-25 | research/27-transaction-monitoring-scenarios.md:2374 | Hard-rule pack with a CBI page behind each rule, configurable with effective dates and a "status unverified" label where applicable | Partly | phase-16.md:929 "each `cbi_refs` entry is an active item of the catalogue (`E_CBI_REF_UNKNOWN`)"; phase-16.md:1228 "recorded as `unverified`" (dark units only, not hard rules) | Add S-25 to phase 16 header, line Partly implemented. Built: CBI reference rule and configurable rules. Missing: the hard-rule content and the "status unverified" label for rules (phase 16 defers the content). |
| S-27 | research/27-transaction-monitoring-scenarios.md:2376 | Cash feature library and scenarios SCN-CASH-01 to -14, including limit-enquiry severity uplift and linked-party aggregation | Partly | phase-06.md:362 register families include CASH; phase-16.md:16 "S-08 (aggregation across linked parties through one linked-party resolver"; phase-16.md:1882 SCN-CASH-02 golden | Add S-27 to phase 16 header, line Partly implemented. Built: register family and linked-party aggregation. Missing: the cash feature definitions, the limit-enquiry uplift and the scenario content (phase 16 defers it to later phases or P2). |
| S-28 | research/27-transaction-monitoring-scenarios.md:2377 | Pass-through matching engine (cumulative, tolerance plus or minus 10 percent, 72 hours) with unit tests for the MAS and TD Zelle cases | GAP | DSL.md:284 "the pass-through matcher of S-28" writes features; no matcher in phases 13 to 17 (grep for pass-through found none); phase-16.md:2121 defers S-28 to "later phases or P2" | Finding G1-13-01 (MEDIUM). |
| S-29 | research/27-transaction-monitoring-scenarios.md:2378 | Public-funds family SCN-PUB-01 to -07 with restricted visibility, government-account segments and contract-chain graphs | Partly | phase-16.md:1704 "Restricted scenarios (the public-funds family ...) are invisible outside CFG and MRO"; phase-17.md:82 `public_funds` family; phase-13.md:1100 government segment not built; phase-19.md:345 network kinds have no contract kind | Add S-29 to phase 16 header, line Partly implemented. Built: restricted visibility and the family. Missing: government-account segments and the contract-chain graph. |
| S-30 | research/27-transaction-monitoring-scenarios.md:2379 | Front-company profile score with visible reasons; an alert opens a review, never an automatic exit | GAP | grep for front-company and profile score found none in phases 00 to 31; phase-06.md:362 register shape carries the scenario fields | Finding G1-16-01 (MEDIUM). |
| S-33 | research/27-transaction-monitoring-scenarios.md:2382 | NPO profile and scenarios: registry status, foreign funding, budget, audit status, board and signatories as related parties | GAP | grep for NPO and nonprofit found none in phases 00 to 31; phase-06.md:362 families list has no NPO family | Finding G1-16-02 (MEDIUM). |
| S-34 | research/27-transaction-monitoring-scenarios.md:2383 | Mule and wallet scenario set with device and phone graph, wallet and card features, and onboarding-channel field | Partly | phase-19.md:1204 "shared_contact" edges from canon.party_contact; phase-19.md:1209 "`shared_device`: no canonical source exists in phase 10; no edge of this kind is built" | Add S-34 to phase 19 header, line Partly implemented. Built: shared-contact edges. Missing: device edges, wallet and card features, the onboarding-channel field and the mule scenario set. |
| S-35 | research/27-transaction-monitoring-scenarios.md:2384 | Employee-risk view restricted to the compliance head: staff-linked flag, mandatory leave and rotation, neglect-of-duty clocks, gift and conflict registers | GAP | grep for mandatory leave, rotation, staff-linked and gift register found none in phases 00 to 31; role CH in phase-01.md:118 | Finding G1-24-01 (MEDIUM). |
| S-36 | research/27-transaction-monitoring-scenarios.md:2385 | Dormancy clock per account on the CBI rule (six months, current accounts) and a monthly reactivation audit report | Partly | phase-13.md:587 "`dormant_days` | account | days since the last posting"; phase-17.md:87 "('dormant', ... 'Dormant accounts')" | Add S-36 to phase 13 header, line Partly implemented. Built: dormant-days feature and the dormant family. Missing: the per-account dormancy clock on the six-month rule and the monthly reactivation audit report (grep for reactivation found no audit report). |
| TS-05 | research/32-tech-stack-mapping.md:1003 | Generate CBS SQL from mappings; verify with a SQL parser as one read-only SELECT over approved objects | CARRIED | phase-05.md:884 "parses with `sqlparser` in the engine's dialect"; phase-10.md:1814 "`source.object` equals an exact `OWNER.NAME` entry of `approved_read_objects`" | Add TS-05 to phase 05 header, line Implemented here. |
| TS-06 | research/32-tech-stack-mapping.md:1004 | Blocking drivers on a bounded blocking pool sized by the signed load budget | CARRIED | phase-05.md:584 "/// spawn_blocking pool sized to the budget's max_sessions." | Add TS-06 to phase 05 header, line Implemented here. |
| TS-08 | research/32-tech-stack-mapping.md:1006 | Debezium Server as the only JVM sidecar, HTTP or NATS sink into akr-ingestd, pinned image and SBOM entry | Partly | phase-11.md:1323 "Change capture (opt-in). Compose profile `cdc` adds `akr-cdc`"; phase-11.md:869 spool through `ChangeSpool` (file sink) | Add TS-08 to phase 11 header, line Partly implemented. Built: the opt-in sidecar. Missing: the HTTP or NATS sink (the file spool is used), and the pinned image and SBOM entry (phase-00.md:951 names the profile only). |
| TS-11 | research/32-tech-stack-mapping.md:1009 | Load-test rig with a synthetic CBS writer; prove the 10x and 100x salary-day cases for each hardware profile | Partly | phase-11.md:1328 "Development fixtures ... a `TRANSACT`-shaped fixture of about one million rows over ten business days" | Add TS-11 to phase 11 header, line Partly implemented. Built: the development fixture. Missing: the 10x and 100x salary-day proof. The per-profile load test is phase 30 section 3.15 (P2), which names TS-33 not TS-11. |
| TS-18 | research/32-tech-stack-mapping.md:1016 | Network edge tables, bounded recursive CTEs and petgraph, graph features in feat, no graph database | Partly | phase-19.md:28 "`casework.network_edges`: the DATA-MODEL columns (DR-145)"; grep for recursive CTE and petgraph in phases 13 to 25 found none | Add TS-18 to phase 19 header, line Partly implemented. Built: the edge table. Missing: the bounded traversal method and the graph features written to feat. |
| TS-20 | research/32-tech-stack-mapping.md:1018 | Investigation as a dedicated event-sourced case aggregate; reuse the workflow engine; sensitive-text guard on free-text notes | Partly | phase-17.md:330 "Append-only event trail. Per-case chain: prev_hash"; phase-18.md:299 `casework.notes_insert_guard()` (amends check only) | Add TS-20 to phase 18 header, line Partly implemented. Built: per-case event chain and the note guard. Missing: a sensitive-text guard on free-text notes (grep for sensitive text in phases 17 and 18 found none). |
| TS-21 | research/32-tech-stack-mapping.md:1019 | Notify only in-app and through the bank SMTP relay; no WhatsApp or push; no subject names in notification text | CARRIED | phase-22.md:963 "The bank's SMTP relay is the only sender (ARCHITECTURE section 15, TS-21" | Add TS-21 to phase 22 header, line Implemented here. |
| TS-35 | research/32-tech-stack-mapping.md:1033 | Reuse the DBS frontend stack and RTL rules; TanStack Table with virtual scrolling; Recharts default; xyflow with Sigma or Cytoscape; bidi-aware name component; no run-time fetch | Partly | phase-08.md:111 "`@tanstack/react-table` 9.1"; phase-08.md:166 "`bidi.ts` ... `scanControls(s)`"; phase-08.md:473 "RTL charts (DS-O3, DS-O7)" spike; phase-08.md:120 "`@xyflow/react` (spike only" | Add TS-35 to phase 08 header, line Partly implemented. Built: table, virtual, bidi helpers, spikes named. Missing: the Recharts default and the Sigma or Cytoscape choice are left to the spikes; no phase records the result. |

## Part 2. Findings (GAP rows)

### G1-11-01 Transact record source (`t24_xmlrecord`), XML decoded in AKR-TMS
- Severity: MEDIUM
- Category: Missing Logic
- Requirement: C-14; research/25-core-banking-integration.md:1147 ("Do not parse or shred XML inside the CBS; fetch the raw record text through an indexed range"). DSL.md:516 already names `t24_xmlrecord` as a `source.format`.
- Target: phase-11.md section 3.2 (watermark loop) and section 3.4 (delivery modes)
- Check: `grep -n "t24" docs/plan/phase-11.md` gives only the watermark kind `t24_date_time` (lines 107, 140, 756, 857); `grep -n -E "XMLRECORD|RECID" docs/plan/*.md` gives no hit; phase-10.md:1816 refuses `t24_*` with `format_not_in_phase` "(phase 11 and the parser phases add them)"; phase 11 never adds it.
- Proposed 7.x text:
  "### 7.N Transact record source (`t24_xmlrecord`, C-14)
  1. From phase 11 on, the phase 10 compile step (its step 3, Source object) accepts `source.format = t24_xmlrecord` for an approved object with one key column and one text column, both named by the adapter pack. Every other `t24_*` value stays refused.
  2. A load of such a table is the section 3.2 keyset loop with the key as the watermark (kind `sequence_id`, no overlap). The statement is `SELECT <key>, <text> FROM <approved object> WHERE <key> > $1 ORDER BY <key> FETCH FIRST $2 ROWS ONLY`, run through `SourceReader` (C-54). No XML function, no join, no predicate beyond the bound (C-35). The phase 05 SelectProof refuses any XML function in the statement.
  3. The text column lands unchanged in the batch raw rows. The mapping decodes each field from that text with the phase 10 transform `XmlField(path)`, in akr-ingestd or akr-worker, never on the CBS.
  4. Tests: a fixture record decodes to the expected field; a statement containing `extractvalue` or `XMLTABLE` is refused by the proof."

### G1-11-02 ISO 20022 and MT payment message parsers
- Severity: HIGH
- Category: Missing Logic
- Requirement: C-41; research/25-core-banking-integration.md:1174 (ingest native ISO 20022 and MT, retain raw message, versioned fuzz-tested parsers, record parser version). MODULES M5 P1 lists payment messages (ISO 20022 and MT) with raw kept.
- Target: phase-11.md new section 3.10 (payment message parsing)
- Check: `grep -n -i -E "iso ?20022|\bMT[0-9]{3}" docs/plan/phase-1[1-9].md docs/plan/phase-2*.md docs/plan/phase-3*.md` finds only phase-11.md:1535 (deferral) and phase-10.md:2444 (deferral); `grep -n parser_version docs/plan/phase-10.md` gives no hit; phase-10.md:1023 to 1050 has `format IN ('iso20022', 'mt')` and `raw_enc`, no parser version. Migrations 0036 to 0038 belong to phase 12 (phase-12.md:48), so the phase 11 migration number needs the lead.
- Proposed 7.x text:
  "### 7.N Payment message parsers (ISO 20022 and MT, C-41)
  1. A pure crate `crates/payments` (no IO) with `parse_iso20022(raw: &[u8], version: &str)` and `parse_mt(raw: &[u8])`, both returning `Result<ParsedMessage, ParseError>`. Parser versions are constants in the crate. Each parser has a fuzz target run in CI for a bounded time.
  2. Migration (number assigned by the lead): `ALTER TABLE canon.payment_message ADD COLUMN IF NOT EXISTS parser_version TEXT CHECK (parser_version IS NULL OR parser_version ~ '^[a-z0-9_.-]{1,40}$');`. Every insert from this phase sets it. The raw message stays in `raw_enc` (phase 10).
  3. Job `ingest_payment_parse` (new kind; payload `{source_id, source_table, batch_id}`), run by akr-ingestd over the landed rows of a payment source. Each parsed message inserts `canon.payment_message` and `canon.payment_message_party` in the same transaction as the batch receipt, idempotent on `(source_id, source_table, source_pk, received_at)` (C-24). A parse failure writes a receipt with reason `parse_failed` and no canonical row.
  4. The phase 10 refusal of `format = iso20022|mt` stays in force for every other path; from phase 11 the compiler accepts these formats for this job only.
  5. Audit: none new; the batch receipt is the record."

### G1-11-03 Payment-gateway database source (RTGS, ACH, cheque items)
- Severity: HIGH
- Category: Missing Integration
- Requirement: C-42; research/25-core-banking-integration.md:1175 (payment-gateway database adapters as the preferred structured source for domestic payments). MODULES M5 P1: "otherwise from the payment-gateway database (ARCHITECTURE 5.2) as a second source with its own freshness state".
- Target: phase-11.md section 3.9 (adapter packs) and section 3.4 (delivery modes)
- Check: phase-11.md:1535 "No ISO 20022 or MT parser and no payment-gateway source (phases 12 and later under C-41 and C-42)"; no phase 12 to 31 section defines a gateway pack (`grep -n -i gateway docs/plan/phase-1[2-9].md` gives only unrelated hits); phase-05.md:468 lists the adapters as pg, mssql, oracle, odbc.
- Proposed 7.x text:
  "### 7.N Payment-gateway source (C-42)
  1. A second `admin.sources` row per gateway database, with its own tier and grant proof (phase 05 unchanged), its own watermarks (section 3.2) and its own freshness state: `data_as_of` and the stale flag are computed per source (C-29).
  2. Adapter pack `payments_gateway@1.0.0` (signed release artifact, phase 05 section 3.6 hint schema) names the RTGS, ACH and cheque item objects as approved read objects. Raw message text lands and is parsed by the job of G1-11-02.
  3. The pack is admin-managed like `ics_banks` (section 3.9); a change of the pack is a change request (phase 04).
  4. Tests: a write grant on the gateway account is refused at connect; a halt on the gateway source does not change the CBS freshness."

### G1-11-04 Optional channel and card-processor read links (grade C)
- Severity: LOW
- Category: Missing Integration
- Requirement: S-20; research/27-transaction-monitoring-scenarios.md:2369 (optional read-only links, labelled grade C, with retention limits and a purpose statement).
- Target: phase-11.md section 3.9 (adapter packs and optional sources)
- Check: `grep -n -i -E "card processor|channel database|channel_db" docs/plan/*.md` gives no hit in phases 05 to 12; phase-06.md:453 says "device and channel data in 26" but phase-26.md:1 is the Islamic pack, so the pointer is wrong.
- Proposed 7.x text:
  "### 7.N Optional channel and card-processor sources (S-20)
  1. An optional source is off by default behind the setting `ingest.channel_sources_enabled` (FALSE). Its `admin.sources` row carries `optional_kind = channel` and grade `C` in the phase 06 readiness view.
  2. The source is read-only and grant-proved as any other source (phase 05). Its purpose statement and retention limit are stored on the row (`purpose_text`, `retention_days`) and shown on the readiness view.
  3. Scenarios that read it stay graded C in phase 06 until it is enabled.
  4. Phase 06 section 1 is corrected to name phase 11 (not phase 26) as the owner of device and channel data."

### G1-12-01 Payment-link matcher and reconciliation rung L4
- Severity: MEDIUM
- Category: Missing Logic
- Requirement: C-43; research/25-core-banking-integration.md:1176 (payment-link table with match rules and confidence; unmatched routed to reconciliation level L4).
- Target: phase-12.md section 1 (reconciliation ladder) and a new job subsection in section 3
- Check: `grep -n payment_link docs/plan/phase-1[1-9].md` finds no writer (phase-23.md:586 is retention only); the table exists at phase-10.md:1079 with `match_keys`, `confidence`, `rule`; phase-12.md:25 and 1364 defer L4 ("no payment tie-out"); `DataQualityRecorder` is in phase-12.md:21 and `admin.dq_issues` at phase-12.md:168.
- Proposed 7.x text:
  "### 7.N Payment-link matcher and rung L4 (C-43)
  1. Job `payment_link_match` (new kind; coalesce key `payment_link:<business date>`; concurrency 1), enqueued after the phase 11 landing of the business date and the parse job of G1-11-02.
  2. Rules in order: (a) UETR equal to `canon.payment_message.uetr`, amount and currency equal, `confidence` 1.0000, rule `uetr_amount`; (b) reference and amount equal with value date within `recon.payment_match_days` business days (a setting), `confidence` 0.8000, rule `reference_amount`.
  3. Each candidate inserts one `canon.payment_link` row (`message_id`, `message_received_at`, `txn_id`, `booking_date`, `match_keys`, `confidence`, `rule`).
  4. A released message with no link after the window is an L4 break: one `admin.dq_issues` row per message through `DataQualityRecorder`. Messages with status `rejected` or `cancelled` are excluded, the same set the release control skips (DR-389).
  5. Audit `reconciliation.payment_link_run {linked, unmatched}` (counts only)."

### G1-12-02 Party merge and split with maker-checker
- Severity: MEDIUM
- Category: Missing Logic
- Requirement: C-53; research/25-core-banking-integration.md:1186 (explicit survivorship rules; reversible maker-checker party merge and split).
- Target: phase-12.md new section 3 subsection (party merge and split); the survivorship part stays in phase 10 section 3.3
- Check: `grep -n -i -E "merge|split" docs/plan/phase-12.md` gives only router merging and the phase 11 route split; phase-10.md:1871 "Party merge and split stay maker-checker actions of phase 12 (C-53)" and phase-10.md:2443 defer to phase 12; no party merge table in DATA-MODEL.md (merged_into appears only for segments and cases). Scope note: MODULES M6 does not list C-53 (see Summary).
- Proposed 7.x text:
  "### 7.N Party merge and split (C-53)
  1. New table `canon.party_merge` (new; `id UUID PRIMARY KEY` with no default, UUIDv7 app-side; no foreign key to the bi-temporal `canon.party`, as phase 10 does for `reverses_id`):
  ```sql
  CREATE TABLE IF NOT EXISTS canon.party_merge (
      id                  UUID PRIMARY KEY,
      kind                TEXT NOT NULL CHECK (kind IN ('merge', 'split')),
      survivor_party_id   UUID NOT NULL,
      other_party_id      UUID NOT NULL,
      state               TEXT NOT NULL CHECK (state IN ('proposed', 'applied', 'rejected', 'reversed')),
      change_request_id   UUID NOT NULL,
      reason_code         TEXT NOT NULL,
      created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
      CHECK (survivor_party_id <> other_party_id)
  );
  ```
  2. A merge or split is a phase 04 change request; the maker never approves it. On approval, one transaction writes new canonical versions of the affected `canon.party_name`, `canon.party_identifier` and `canon.account_party_role` rows (phase 10 section 2 bi-temporal rules), supersedes the merged party, and sets `state = applied`.
  3. Reversal writes `state = reversed` and restores the superseded versions in one transaction.
  4. Survivorship for attributes that two sources supply: `source_priority`, then `most_recent` (phase 10 section 3.3). `longest_verified` stays refused.
  5. Audit: `party.merge_proposed`, `party.merge_applied`, `party.split_applied`, `party.merge_reversed`."

### G1-09-01 Governed reference registers for scenarios
- Severity: MEDIUM
- Category: Missing Table
- Requirement: S-15; research/27-transaction-monitoring-scenarios.md:2364 (dual-use and HS list, vessel and IMO list, sector-goods map, peer price tables, crypto counterparties, government-source flag, DNFBP classes; each with owner, version and review date).
- Target: phase-09.md section 3 (reference tables) and section 1 DDL
- Check: phase-09.md:35 to 141 create only `ref.countries`, `ref.country_risk_overrides`, `ref.governorates`, `ref.currencies`, `ref.fx_rates`, `ref.calendars`, `ref.calendar_days`, `ref.cbs_calendar_captures`; `grep -n -i -E "dual-use|peer price|DNFBP|sector" docs/plan/*.md` gives no register table; phase-10.md:1344 holds only `vessel_imo` on the trade instrument.
- Proposed 7.x text:
  "### 7.N Governed reference registers (S-15)
  1. Pattern of `ref.countries` (phase 09): changes by change request (phase 04), effective-dated versions.
  ```sql
  CREATE TABLE IF NOT EXISTS ref.registers (
      code            TEXT PRIMARY KEY CHECK (code ~ '^[a-z][a-z0-9_]{1,40}$'),
      name            JSONB NOT NULL,
      owner_role      TEXT NOT NULL,
      review_due_on   DATE NOT NULL,
      created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
  );
  CREATE TABLE IF NOT EXISTS ref.register_versions (
      id                  UUID PRIMARY KEY,
      register_code       TEXT NOT NULL REFERENCES ref.registers(code) ON DELETE RESTRICT,
      version             TEXT NOT NULL,
      source              TEXT NOT NULL,
      row_count           INTEGER NOT NULL CHECK (row_count >= 0),
      status              TEXT NOT NULL CHECK (status IN ('candidate', 'active', 'superseded', 'rejected')),
      change_request_id   UUID NOT NULL,
      activated_at        TIMESTAMPTZ,
      CONSTRAINT uq_register_version UNIQUE (register_code, version)
  );
  CREATE UNIQUE INDEX IF NOT EXISTS uq_register_active ON ref.register_versions (register_code) WHERE status = 'active';
  CREATE TABLE IF NOT EXISTS ref.register_rows (
      version_id      UUID NOT NULL REFERENCES ref.register_versions(id) ON DELETE RESTRICT,
      row_key         TEXT NOT NULL,
      attrs           JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(attrs) = 'object'),
      PRIMARY KEY (version_id, row_key)
  );
  ```
  2. Grants: `akr_app` SELECT on all three; INSERT on `ref.register_versions` and `ref.register_rows`; UPDATE (status, activated_at) on `ref.register_versions`. Each table gets a classification row (`internal`) and a retention row (R9).
  3. Scenarios name registers through their `needs`; the readiness view (phase 06) shows a register with no active version as missing."

### G1-13-01 Pass-through matcher (specialised feature writer)
- Severity: MEDIUM
- Category: Missing Logic
- Requirement: S-28; research/27-transaction-monitoring-scenarios.md:2377 (cumulative pass-through matching, tolerance plus or minus 10 percent, 72 hours, unit tests for the MAS case and the TD Zelle cap case).
- Target: phase-13.md section 3.1 (feature checks) and a new subsection for specialised writers
- Check: DSL.md:284 says the pass-through matcher of S-28 "writes features"; `grep -n -i -E "pass-through|pass through|matcher" docs/plan/phase-1[3-7].md` gives no matcher; phase-16.md:2121 defers S-28 to "later phases or P2"; phase 13 defines nine closed kinds (phase-13.md:440) and no matcher kind.
- Proposed 7.x text:
  "### 7.N Pass-through matcher (S-28)
  1. A specialised feature writer `PassThroughMatcher` (akr-worker) registered as a code-registered `feat.` namespace feature, as phase 13 registers its writers (DR-367). It writes rows of `feat.feature_values` (phase 13) per (party, account, month) for `passthrough.share_72h` (share of outflow matched within 72 hours) and `passthrough.matched_amount`.
  2. Matching is cumulative over the window: an inflow and an outflow pair when the outflow amount is within plus or minus 10 percent of the cumulative inflow not yet matched and the outflow falls within 72 hours of it. Settings: `passthrough.tolerance_pct` (10), `passthrough.window_hours` (72).
  3. Tests: the MAS case B and the TD Zelle cap case of research 27 S-28 as fixtures with expected shares.
  4. The features are read by scenarios like any other feature (DSL A.6), so no logic kind is added."

### G1-16-01 Front-company profile score with visible reasons
- Severity: MEDIUM
- Category: Missing Logic
- Requirement: S-30; research/27-transaction-monitoring-scenarios.md:2379 (front-company profile score with visible reasons; an alert opens a review, never an automatic exit).
- Target: phase-16.md section 3.1 (scenario document and register entries) with a feature input from phase-13.md section 3.1
- Check: `grep -n -i -E "front.company|front company|profile score" docs/plan/*.md` gives no hit; the register shape at phase-06.md:362 carries `needs` and `external_needs` per scenario; families are the closed list of phase 06 (no new family needed: `OTH`).
- Proposed 7.x text:
  "### 7.N Front-company profile score (S-30)
  1. Register entries `SCN-OTH-FRONT-01` (family `OTH`, an existing value) with `needs` on phase 13 features (account age at first transaction, turnover ratio, counterparty concentration from the S-11 resolver) and `external_needs` `company_registry` (graded C until a connector exists).
  2. A profile score written by a feature writer in the phase 13 pattern, with the reasons (each reason a feature code and value) stored in the alert payload (phase 17 detection payload).
  3. The alert opens a review task only. No account action, no exit, and no task that restricts an account is created by this scenario.
  4. Test: a document with a score and reasons produces an alert whose payload lists every reason."

### G1-16-02 NPO profile and scenarios
- Severity: MEDIUM
- Category: Missing Logic
- Requirement: S-33; research/27-transaction-monitoring-scenarios.md:2382 (registry status, foreign-funding flag, expected programme and budget, audit status against the IQD 75 million rule, board and signatories as related parties).
- Target: phase-16.md section 3.1 (register entries and parameter sets)
- Check: `grep -n -i -E "NPO|non-profit|nonprofit|charit" docs/plan/*.md` gives no scenario hit (phase-23 and phase-13 hits are unrelated words); the families at phase-06.md:362 have no NPO family.
- Proposed 7.x text:
  "### 7.N NPO profile and scenarios (S-33)
  1. Register entries `SCN-OTH-NPO-01` to `-03` (family `OTH`) with `needs` on the party relationship rows of phase 10 (`canon.party_relationship` for board members and signatories) and `external_needs` `ngo_registry` (grade D until the feed exists, phase 06).
  2. Setting `npo.audit_threshold_iqd` = 75000000 (bank-confirmed; the audit status is compared against it).
  3. Foreign-funding flag and the expected budget are admin-managed party attributes captured through the phase 10 mapping documents, not code constants."

### G1-21-01 Office officer registry (MLRO and deputy contact details)
- Severity: MEDIUM
- Category: Missing Table
- Requirement: RP-35; research/23-regulatory-reporting-goaml.md:780 (registry of the MLRO and deputy with contact details as notified to the Office; a task to notify the Office and the CBI on any change).
- Target: phase-21.md section 3 (new subsection: officer registry)
- Check: phase-04.md:708 `NamedOfficers::members` returns the user ids of the holders of role M (MLRO and deputy) and holds no contact data; `grep -n -i -E "officer|contact" docs/plan/phase-21.md` gives no registry table; no phase creates an Office notice.
- Proposed 7.x text:
  "### 7.N Office officer registry (RP-35)
  1. Table `rpt.officer_registry` (new):
  ```sql
  CREATE TABLE IF NOT EXISTS rpt.officer_registry (
      id                  UUID PRIMARY KEY,
      role_code           TEXT NOT NULL CHECK (role_code IN ('mlro', 'deputy')),
      user_id             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
      contact_enc         BYTEA NOT NULL,
      valid_from          DATE NOT NULL,
      valid_to            DATE,
      notified_office_at  TIMESTAMPTZ,
      notified_cbi_at     TIMESTAMPTZ,
      created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
  );
  CREATE UNIQUE INDEX IF NOT EXISTS uq_officer_open ON rpt.officer_registry (role_code) WHERE valid_to IS NULL;
  ```
  `contact_enc` is an AES-GCM envelope (family `personal`).
  2. A change of the holders of role M (phase 01 role grant or phase 04 delegation) writes the new row and closes the old one in one transaction, and registers a timer `officer_notice_due` (phase 00 timers) for the Office and CBI notices. The due interval is a setting with no research figure; the lead sets it (open doubt).
  3. Audit: `officer.registry_changed {role_code}` (no contact values)."

### G1-24-01 Employee-risk view restricted to the compliance head
- Severity: MEDIUM
- Category: Missing Logic
- Requirement: S-35; research/27-transaction-monitoring-scenarios.md:2384 (staff-linked flag, mandatory leave and rotation, neglect-of-duty clocks, gift and conflict registers; visible to the compliance head only).
- Target: phase-24.md section 3 (new subsection: staff-risk view and registers)
- Check: `grep -n -i -E "mandatory.leave|rotation|staff-linked|gift|conflict register|neglect" docs/plan/*.md` gives no staff-risk hit; `canon.staff` exists (phase-10.md, operator_id reference); role list at phase-01.md:118 includes `CH`, whose meaning as compliance head is not stated in the plan (see Summary).
- Proposed 7.x text:
  "### 7.N Employee-risk view (S-35)
  1. Route `GET /api/v1/staff-risk`, scope `staff.risk.read`, granted only to the role confirmed as compliance head. Every other role gets 404 (existence masked). Access event on every read, tier `restricted`.
  2. Derived view over `canon.staff` (phase 10) with `staff_linked` (the staff member is a party of a customer relationship row) and leave status from the leave source when loaded; otherwise `not declared`.
  3. Table `admin.gift_conflict_registers` (new; `id UUID PRIMARY KEY`; `admin.registers` of phase 24 holds findings and warnings only and does not fit; `kind CHECK (kind IN ('gift', 'conflict'))`; `staff_id`, `declared_on`, `detail_enc BYTEA`, `reviewed_by`, `reviewed_at`).
  4. Clocks `staff.neglect_review` and `staff.mandatory_leave` in the phase 24 clock registry (DR-370 units), due from the leave source.
  5. Audit: `staff_risk.viewed` and `staff_risk.register_reviewed`."

## Summary

Counts by class (59 IDs):
- CARRIED: 11 (C-17, C-18, C-33, C-39, C-49, E-07, RP-08, RP-13, TS-05, TS-06, TS-21)
- Partly: 35
- NOT-CARRIED: 1 (C-34)
- GAP: 12

GAP findings by severity:
- HIGH: 2 (G1-11-02 C-41; G1-11-03 C-42)
- MEDIUM: 9 (G1-11-01 C-14; G1-12-01 C-43; G1-12-02 C-53; G1-09-01 S-15; G1-13-01 S-28; G1-16-01 S-30; G1-16-02 S-33; G1-21-01 RP-35; G1-24-01 S-35)
- LOW: 1 (G1-11-04 S-20)

UNCLEAR (for the lead):
- C-53 scope: phase-10.md:1871 and 2443 assign party merge and split to phase 12, but MODULES M6 (docs/MODULES.md:250) does not list C-53. The finding treats it as in scope on the phase 10 cross-reference.
- S-35: the role `CH` (phase-01.md:118) is taken to be the compliance head; the plan does not define it. Confirm before the route scope is fixed.
- RP-08: the active-profile uniqueness is per code (phase-07.md:97); the per-profile clocks and numbering were not read in full. Marked CARRIED on the registry evidence only.
- RP-35: the notice interval for the Office and CBI has no research figure (proposal left to the lead).
- C-45: the request to the CBI Payments Department is a bank process; classed Partly on the ISO-shaped model alone.
- TS-11: whether a 10x and 100x salary-day proof is required for profile S in P1 or only in P2 (phase 30 section 3.15).
- Phase 06 line 453 points device and channel data to phase 26, which is the Islamic pack (phase-26.md:1). Corrected in G1-11-04.
- Migration numbers for G1-11-02 (and any migration from G1-09-01, G1-12-01, G1-12-02, G1-21-01, G1-24-01) need the lead: phase 11 uses 0033 to 0035 and phase 12 uses 0036 to 0038.
