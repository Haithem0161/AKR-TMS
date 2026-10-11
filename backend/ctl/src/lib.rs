//! `akr-ctl`: the operator's command line. It runs on the management host, and every command that changes
//! an installation records who ran it.

pub mod bundle;
pub mod cli;
pub mod conn;
pub mod db;
pub mod error;
pub mod net;
pub mod operator;
pub mod ops;
pub mod scram;
pub mod store;

pub use error::CtlError;
