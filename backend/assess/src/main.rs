use std::process::ExitCode;
use std::sync::Arc;

use akr_assess::readiness::AssessProbe;
use akr_common::config::{Binary, CoreConfig, Wiring, boot, harden_process};
use akr_common::infra::{self, MetricsRender};
use akr_db::PgClassificationRepository;
use akr_domain::repositories::ClassificationRepository;
use akr_telemetry::Telemetry;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

/// The probes only, until the decision path of phase 29.
const DB_POOL_SIZE: u32 = 5;

#[tokio::main]
async fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some(infra::HEALTHCHECK_COMMAND) {
        return infra::healthcheck(Binary::Assess);
    }
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("akr-assess: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let core = CoreConfig::from_env(Binary::Assess)?;
    harden_process(core.env)?;
    let telemetry = Telemetry::init(&core)?;

    let secrets = boot(&core, &[]).await?;
    core.validate(secrets.db.as_ref(), &Wiring::default())?;

    let db = secrets.db.ok_or("no database credential was read")?;
    let pool = akr_db::connect(&db, Binary::Assess.name(), DB_POOL_SIZE).await?;
    telemetry.load_classification(&PgClassificationRepository::new(pool.clone()).list().await?);

    let public = TcpListener::bind(core.bind).await?;
    let management = TcpListener::bind(core.metrics_bind).await?;
    let handle = telemetry.metrics();
    let render: MetricsRender = Arc::new(move || handle.render());
    tracing::info!(
        env = core.env.as_str(),
        bind = %core.bind,
        metrics_bind = %core.metrics_bind,
        "akr-assess starting"
    );

    let stop = CancellationToken::new();
    tokio::spawn({
        let stop = stop.clone();
        async move {
            infra::shutdown_signal().await;
            stop.cancel();
        }
    });
    tokio::try_join!(
        infra::serve(
            public,
            infra::router(Arc::new(AssessProbe::new(pool))),
            stop.clone().cancelled_owned()
        ),
        infra::serve(
            management,
            infra::management_router(render),
            stop.clone().cancelled_owned()
        ),
    )?;

    tracing::info!("akr-assess stopped");
    telemetry.flush();
    Ok(())
}
