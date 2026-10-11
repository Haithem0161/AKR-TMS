use std::process::ExitCode;
use std::sync::Arc;

use akr_audit::TracingSink;
use akr_common::config::{Binary, CoreConfig, DEV_SEED, Wiring, boot, harden_process};
use akr_common::idempotency::IdempotencyService;
use akr_common::infra::{self, MetricsRender};
use akr_common::settings::{NoNotifier, SettingsService};
use akr_crypto::{Crypto, InMemoryCrypto, OpenBaoCrypto, Purpose};
use akr_db::{
    PgClassificationRepository, PgDataKeyStore, PgIdempotencyStore, PgJobStore, PgOutboxStore,
    PgSettingsRepository, PgTimerStore, listen_for_jobs,
};
use akr_domain::audit::AuditSink;
use akr_domain::repositories::ClassificationRepository;
use akr_jobs::{
    JobQueue, JobRegistry, JobStore, OutboxDispatcher, OutboxStore, TimerRegistry, TimerStore,
    TimerSweeper, TopicRegistry, Worker, kinds,
};
use akr_telemetry::Telemetry;
use akr_worker::handlers::{BackupCheck, MaintenanceSweep};
use akr_worker::readiness::WorkerProbe;
use chrono::Utc;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

/// The poll loop, the dispatcher, the sweeper and the two jobs share this pool.
const DB_POOL_SIZE: u32 = 10;

#[tokio::main]
async fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some(infra::HEALTHCHECK_COMMAND) {
        return infra::healthcheck(Binary::Worker);
    }
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("akr-worker: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let core = CoreConfig::from_env(Binary::Worker)?;
    harden_process(core.env)?;
    let telemetry = Telemetry::init(&core)?;

    let secrets = boot(&core, Purpose::ALL).await?;
    core.validate(
        secrets.db.as_ref(),
        &Wiring {
            tracing_sink: true,
            in_memory_crypto: secrets.bao.is_none(),
            allow_all_authorizer: false,
        },
    )?;

    let db = secrets.db.ok_or("no database credential was read")?;
    let pool = akr_db::connect(&db, Binary::Worker.name(), DB_POOL_SIZE).await?;
    telemetry.load_classification(&PgClassificationRepository::new(pool.clone()).list().await?);

    let bao = secrets.bao.as_ref().map(|session| session.client.clone());
    let keys = secrets.bao.map(|session| {
        Arc::new(OpenBaoCrypto::new(
            session.client,
            Arc::new(PgDataKeyStore::new(pool.clone())),
            session.hash_keys,
        ))
    });
    if let Some(keys) = &keys {
        keys.ensure_keys(Utc::now()).await?;
    }
    let crypto: Arc<dyn Crypto> = match &keys {
        Some(keys) => keys.clone(),
        None => Arc::new(InMemoryCrypto::from_seed(DEV_SEED)),
    };

    let audit: Arc<dyn AuditSink> = Arc::new(TracingSink::new());
    let settings = SettingsService::new(
        Arc::new(PgSettingsRepository::new(pool.clone())),
        audit.clone(),
        Arc::new(NoNotifier),
    );
    let job_store: Arc<dyn JobStore> = Arc::new(PgJobStore::new(pool.clone()));
    let outbox_store: Arc<dyn OutboxStore> = Arc::new(PgOutboxStore::new(pool.clone()));
    let timer_store: Arc<dyn TimerStore> = Arc::new(PgTimerStore::new(pool.clone()));
    let store = akr_objectstore::connect(&core, secrets.s3)?;

    let mut registry = JobRegistry::new();
    registry.register(
        &kinds::MAINTENANCE_SWEEP,
        MaintenanceSweep {
            jobs: job_store.clone(),
            outbox: outbox_store.clone(),
            timers: timer_store.clone(),
            idempotency: IdempotencyService::new(
                Arc::new(PgIdempotencyStore::new(pool.clone())),
                crypto,
                settings.clone(),
            ),
            settings: settings.clone(),
            keys,
        },
    )?;
    registry.register(
        &kinds::BACKUP_CHECK,
        BackupCheck {
            settings: settings.clone(),
        },
    )?;

    let worker = Worker::new(
        job_store.clone(),
        settings.clone(),
        audit,
        registry,
        core.instance_id.clone(),
    );
    let wake = worker.wake_handle();
    let dispatcher = OutboxDispatcher::new(outbox_store, settings.clone(), TopicRegistry::new());
    let sweeper = TimerSweeper::new(
        timer_store,
        JobQueue::new(job_store),
        settings,
        TimerRegistry::new(),
    );

    let public = TcpListener::bind(core.bind).await?;
    let management = TcpListener::bind(core.metrics_bind).await?;
    let handle = telemetry.metrics();
    let render: MetricsRender = Arc::new(move || handle.render());
    let probe = Arc::new(WorkerProbe::new(pool.clone(), bao, store));
    tracing::info!(
        env = core.env.as_str(),
        worker_id = %core.instance_id,
        bind = %core.bind,
        metrics_bind = %core.metrics_bind,
        "akr-worker starting"
    );

    let stop = CancellationToken::new();
    tokio::spawn({
        let stop = stop.clone();
        async move {
            infra::shutdown_signal().await;
            stop.cancel();
        }
    });
    let loops = async {
        tokio::join!(
            worker.run(stop.clone()),
            dispatcher.run(stop.clone()),
            sweeper.run(stop.clone()),
            listen_for_jobs(pool, wake, stop.clone()),
        );
        Ok(())
    };
    tokio::try_join!(
        infra::serve(public, infra::router(probe), stop.clone().cancelled_owned()),
        infra::serve(
            management,
            infra::management_router(render),
            stop.clone().cancelled_owned()
        ),
        loops,
    )?;

    tracing::info!("akr-worker stopped");
    telemetry.flush();
    Ok(())
}
