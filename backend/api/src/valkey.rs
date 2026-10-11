//! Valkey: the cache, the rate limits and the message bus between processes.
//!
//! Nothing durable lives there. A process starts while Valkey is down and connects when it comes back; a
//! command sent meanwhile fails after [`COMMAND_TIMEOUT`] instead of waiting in a queue.

use std::time::Duration;

use akr_common::settings::{SETTINGS_CHANGED_CHANNEL, SettingsNotifier, SettingsService};
use async_trait::async_trait;
use fred::clients::Pool;
use fred::prelude::{
    Builder, ClientLike, Config, Error, EventInterface, PubsubInterface, ReconnectPolicy,
};
use tokio_util::sync::CancellationToken;

pub const COMMAND_TIMEOUT: Duration = Duration::from_secs(2);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

/// How long the settings listener waits before it tries to subscribe again.
const SUBSCRIBE_RETRY: Duration = Duration::from_secs(5);

const RECONNECT_MIN_MS: u32 = 100;
const RECONNECT_MAX_MS: u32 = 5_000;

fn builder(url: &str) -> Result<Builder, Error> {
    let mut config = Config::from_url(url)?;
    config.fail_fast = false;
    let mut builder = Builder::from_config(config);
    builder
        .with_performance_config(|performance| {
            performance.default_command_timeout = COMMAND_TIMEOUT;
        })
        .with_connection_config(|connection| {
            connection.connection_timeout = CONNECT_TIMEOUT;
        })
        .set_policy(ReconnectPolicy::new_exponential(
            0,
            RECONNECT_MIN_MS,
            RECONNECT_MAX_MS,
            2,
        ));
    Ok(builder)
}

/// A pool that is not connected yet. [`connect`] starts it.
pub fn pool(url: &str, size: usize) -> Result<Pool, Error> {
    builder(url)?.build_pool(size)
}

/// Starts the pool's connections in the background and returns at once.
pub fn connect(pool: &Pool) {
    drop(pool.connect());
}

pub async fn ping(pool: &Pool) -> Result<(), Error> {
    pool.ping::<String>(None).await.map(|_| ())
}

/// Tells the other processes that a setting changed. Best effort: a process that misses the message
/// reads the new value when its cache ages out.
pub struct ValkeySettingsNotifier {
    pool: Pool,
}

impl ValkeySettingsNotifier {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsNotifier for ValkeySettingsNotifier {
    async fn settings_changed(&self, key: &str) {
        let sent: Result<i64, Error> = self
            .pool
            .next()
            .publish(SETTINGS_CHANGED_CHANNEL, key.to_owned())
            .await;
        if let Err(e) = sent {
            tracing::warn!(key, error = %e, "the settings change was not published");
        }
    }
}

/// Drops the settings cache whenever another process publishes a change. Runs until `shutdown`.
pub async fn listen_for_settings(
    url: String,
    settings: SettingsService,
    shutdown: CancellationToken,
) {
    let subscriber = match builder(&url).and_then(|b| b.build_subscriber_client()) {
        Ok(subscriber) => subscriber,
        Err(e) => {
            tracing::error!(error = %e, "the settings listener cannot be built");
            return;
        }
    };
    let mut messages = subscriber.message_rx();
    drop(subscriber.connect());
    drop(subscriber.manage_subscriptions());

    loop {
        match subscriber.subscribe(SETTINGS_CHANGED_CHANNEL).await {
            Ok(()) => break,
            Err(e) => {
                tracing::warn!(error = %e, "cannot subscribe to settings changes yet");
                tokio::select! {
                    () = shutdown.cancelled() => return,
                    () = tokio::time::sleep(SUBSCRIBE_RETRY) => {}
                }
            }
        }
    }

    loop {
        tokio::select! {
            () = shutdown.cancelled() => break,
            message = messages.recv() => match message {
                Ok(_) => settings.invalidate().await,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    settings.invalidate().await;
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            },
        }
    }
    if let Err(e) = subscriber.quit().await {
        tracing::debug!(error = %e, "the settings listener did not close cleanly");
    }
}
