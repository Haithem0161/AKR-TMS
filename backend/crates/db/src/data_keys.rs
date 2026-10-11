//! `admin.data_keys`: the registry of wrapped data keys (DR-424). Rows are inserted and read, never changed.

use akr_crypto::{CryptoError, DataKeyRow, DataKeyStore, Family};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgDataKeyStore {
    pool: PgPool,
}

impl PgDataKeyStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn unavailable(error: sqlx::Error) -> CryptoError {
    CryptoError::Unavailable(format!("the data key registry cannot be read: {error}"))
}

fn row(id: Uuid, family: &str, period: String, wrapped: String) -> Result<DataKeyRow, CryptoError> {
    Ok(DataKeyRow {
        id,
        family: family.parse::<Family>()?,
        period,
        wrapped,
    })
}

#[async_trait]
impl DataKeyStore for PgDataKeyStore {
    async fn find(&self, id: Uuid) -> Result<Option<DataKeyRow>, CryptoError> {
        sqlx::query!(
            "SELECT id, family, period, wrapped FROM admin.data_keys WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(unavailable)?
        .map(|r| row(r.id, &r.family, r.period, r.wrapped))
        .transpose()
    }

    async fn find_period(
        &self,
        family: Family,
        period: &str,
    ) -> Result<Option<DataKeyRow>, CryptoError> {
        sqlx::query!(
            "SELECT id, family, period, wrapped FROM admin.data_keys WHERE family = $1 AND period = $2",
            family.as_str(),
            period
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(unavailable)?
        .map(|r| row(r.id, &r.family, r.period, r.wrapped))
        .transpose()
    }

    async fn insert_if_absent(&self, new: DataKeyRow) -> Result<DataKeyRow, CryptoError> {
        sqlx::query!(
            "INSERT INTO admin.data_keys (id, family, period, wrapped) VALUES ($1, $2, $3, $4)
             ON CONFLICT (family, period) DO NOTHING",
            new.id,
            new.family.as_str(),
            new.period,
            new.wrapped
        )
        .execute(&self.pool)
        .await
        .map_err(unavailable)?;
        self.find_period(new.family, &new.period)
            .await?
            .ok_or_else(|| {
                CryptoError::Unavailable("the data key registry lost a row it just held".to_owned())
            })
    }
}
