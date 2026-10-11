use akr_crypto::{DataKeyRow, DataKeyStore, Family};
use akr_db::PgDataKeyStore;
use sqlx::PgPool;
use uuid::Uuid;

fn key(family: Family, period: &str, wrapped: &str) -> DataKeyRow {
    DataKeyRow {
        id: Uuid::now_v7(),
        family,
        period: period.to_owned(),
        wrapped: wrapped.to_owned(),
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_first_key_of_a_family_and_month_wins(pool: PgPool) {
    let store = PgDataKeyStore::new(pool);
    let first = key(Family::Personal, "2026-10", "vault:v1:Zmlyc3Q=");
    let second = key(Family::Personal, "2026-10", "vault:v1:c2Vjb25k");

    assert_eq!(store.insert_if_absent(first.clone()).await.unwrap(), first);
    assert_eq!(store.insert_if_absent(second.clone()).await.unwrap(), first);

    assert_eq!(store.find(first.id).await.unwrap(), Some(first.clone()));
    assert_eq!(store.find(second.id).await.unwrap(), None);
    assert_eq!(
        store
            .find_period(Family::Personal, "2026-10")
            .await
            .unwrap(),
        Some(first)
    );
    assert_eq!(
        store
            .find_period(Family::Personal, "2026-11")
            .await
            .unwrap(),
        None
    );
    assert_eq!(
        store.find_period(Family::Card, "2026-10").await.unwrap(),
        None
    );
}
