use std::process::ExitCode;
use std::sync::Arc;

use akr_common::config::{Binary, CoreConfig, Wiring, harden_process};
use akr_common::infra::{self, MetricsRender};
use akr_s3guard::rules::rule_book;
use akr_s3guard::{Guard, GuardConfig, register_metrics};
use akr_telemetry::Telemetry;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some(infra::HEALTHCHECK_COMMAND) {
        return infra::healthcheck(Binary::S3Guard);
    }
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("akr-s3guard: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let core = CoreConfig::from_env(Binary::S3Guard)?;
    harden_process(core.env)?;
    let telemetry = Telemetry::init(&core)?;
    core.validate(None, &Wiring::default())?;

    let config = GuardConfig::from_env()?;
    let book = rule_book()?;
    tracing::info!(
        upstream = %config.upstream,
        installer_blocks = config.installer_cidrs.len(),
        rules = book.rules.len(),
        rules_for = %book.store,
        bind = %core.bind,
        "akr-s3guard starting"
    );
    let guard = Arc::new(Guard::new(config)?);
    register_metrics();

    let public = TcpListener::bind(core.bind).await?;
    let management = TcpListener::bind(core.metrics_bind).await?;
    let handle = telemetry.metrics();
    let render: MetricsRender = Arc::new(move || handle.render());

    let stop = CancellationToken::new();
    tokio::spawn({
        let stop = stop.clone();
        async move {
            infra::shutdown_signal().await;
            stop.cancel();
        }
    });
    tokio::try_join!(
        infra::serve(public, guard.router(), stop.clone().cancelled_owned()),
        infra::serve(
            management,
            infra::management_router(render),
            stop.clone().cancelled_owned()
        ),
    )?;

    tracing::info!("akr-s3guard stopped");
    telemetry.flush();
    Ok(())
}
