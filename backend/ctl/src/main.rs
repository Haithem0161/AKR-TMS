use std::process::ExitCode;
use std::sync::Arc;

use akr_audit::TracingSink;
use akr_common::config::{Binary, CoreConfig, harden_process};
use akr_ctl::CtlError;
use akr_ctl::cli::{self, Cli, Outcome};
use akr_ctl::operator::{self, Operator, Trail};
use akr_telemetry::Telemetry;
use clap::Parser;

/// A refusal, or a check that failed: the command ran and the answer is no.
const EXIT_REFUSED: u8 = 2;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(code) => code,
        Err(e) => {
            eprintln!("akr-ctl: {e}");
            ExitCode::FAILURE
        }
    }
}

fn finish((lines, result): Outcome) -> ExitCode {
    for line in lines {
        println!("{line}");
    }
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("akr-ctl: {e}");
            match e {
                CtlError::Refused { .. } | CtlError::ChecksFailed { .. } => {
                    ExitCode::from(EXIT_REFUSED)
                }
                _ => ExitCode::FAILURE,
            }
        }
    }
}

async fn run(cli: Cli) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let command = cli.command;
    if !command.acts_on_installation() {
        return Ok(finish(cli::run_offline(command)));
    }

    let core = CoreConfig::from_env(Binary::Ctl)?;
    harden_process(core.env)?;
    let telemetry = Telemetry::init(&core)?;
    let trail = Trail::new(Arc::new(TracingSink::new()), Operator::from_env());

    let name = command.name();
    let changes = command.changes();
    let outcome = cli::run(command, core.env, &trail).await;
    if changes {
        let how = if outcome.1.is_ok() {
            operator::Outcome::Ok
        } else {
            operator::Outcome::Failed
        };
        trail.command_run(name, how).await?;
    }
    telemetry.flush();
    Ok(finish(outcome))
}
