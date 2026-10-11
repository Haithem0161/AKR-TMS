use std::process::ExitCode;
use std::sync::Arc;

use akr_api::config::{DB_POOL_SIZE, VALKEY_POOL_SIZE};
use akr_api::{ApiConfig, AppState, Dependencies, build_app, openapi, valkey};
use akr_audit::TracingSink;
use akr_authz::AllowAllAuthorizer;
use akr_common::config::{Binary, CoreConfig, DEV_SEED, Wiring, boot, harden_process};
use akr_common::infra::{self, MetricsRender};
use akr_crypto::{Crypto, InMemoryCrypto, OpenBaoCrypto, Purpose};
use akr_db::{PgClassificationRepository, PgDataKeyStore};
use akr_domain::repositories::ClassificationRepository;
use akr_telemetry::Telemetry;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

/// `akr-api openapi` prints the OpenAPI document of the production surface and exits.
const OPENAPI_COMMAND: &str = "openapi";

fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some(OPENAPI_COMMAND) {
        return print_openapi();
    }
    if std::env::args().nth(1).as_deref() == Some(infra::HEALTHCHECK_COMMAND) {
        return infra::healthcheck(Binary::Api);
    }
    match serve() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("akr-api: {e}");
            ExitCode::FAILURE
        }
    }
}

fn print_openapi() -> ExitCode {
    match serde_json::to_string_pretty(&openapi()) {
        Ok(json) => {
            println!("{json}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("akr-api: {e}");
            ExitCode::FAILURE
        }
    }
}

#[tokio::main]
async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let core = CoreConfig::from_env(Binary::Api)?;
    harden_process(core.env)?;
    let telemetry = Telemetry::init(&core)?;

    let secrets = boot(&core, Purpose::ALL).await?;
    core.validate(
        secrets.db.as_ref(),
        &Wiring {
            tracing_sink: true,
            in_memory_crypto: secrets.bao.is_none(),
            allow_all_authorizer: true,
        },
    )?;

    let db = secrets.db.ok_or("no database credential was read")?;
    let pool = akr_db::connect(&db, Binary::Api.name(), DB_POOL_SIZE).await?;
    telemetry.load_classification(&PgClassificationRepository::new(pool.clone()).list().await?);

    let crypto: Arc<dyn Crypto> = match secrets.bao {
        Some(session) => Arc::new(OpenBaoCrypto::new(
            session.client,
            Arc::new(PgDataKeyStore::new(pool.clone())),
            session.hash_keys,
        )),
        None => Arc::new(InMemoryCrypto::from_seed(DEV_SEED)),
    };

    let valkey_url = core.valkey_url()?.to_owned();
    let valkey_pool = valkey::pool(&valkey_url, VALKEY_POOL_SIZE)?;
    valkey::connect(&valkey_pool);

    let state = AppState::new(Dependencies {
        pool,
        valkey: valkey_pool,
        crypto,
        store: akr_objectstore::connect(&core, secrets.s3)?,
        audit: Arc::new(TracingSink::new()),
        authz: Arc::new(AllowAllAuthorizer::new(core.env)?),
        cursor_key: secrets.cursor_key.ok_or("no cursor key was read")?,
        config: ApiConfig::new(core.clone()),
    })?;

    let public = TcpListener::bind(core.bind).await?;
    let management = TcpListener::bind(core.metrics_bind).await?;
    let handle = telemetry.metrics();
    let render: MetricsRender = Arc::new(move || handle.render());
    tracing::info!(
        env = core.env.as_str(),
        bind = %core.bind,
        metrics_bind = %core.metrics_bind,
        "akr-api starting"
    );

    let stop = CancellationToken::new();
    tokio::spawn({
        let stop = stop.clone();
        async move {
            infra::shutdown_signal().await;
            stop.cancel();
        }
    });
    tokio::spawn(valkey::listen_for_settings(
        valkey_url,
        state.settings.clone(),
        stop.clone(),
    ));
    tokio::try_join!(
        infra::serve(public, build_app(state), stop.clone().cancelled_owned()),
        infra::serve(
            management,
            infra::management_router(render),
            stop.clone().cancelled_owned()
        ),
    )?;

    tracing::info!("akr-api stopped");
    telemetry.flush();
    Ok(())
}
