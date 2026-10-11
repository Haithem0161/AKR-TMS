//! `akr-assess`: the pre-transaction decision service (compose profile `phase2`). In this phase it starts,
//! connects as `akr_assess` and answers its probes; the decision path arrives with phase 29.

pub mod readiness;
