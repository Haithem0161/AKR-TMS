//! `akr-ingestd`: the only process that reaches the bank's core banking system. In this phase it starts,
//! connects as `akr_ingest` and answers its probes; the source adapters and their jobs arrive with phase 05.

pub mod readiness;
