use anyhow::Context;
use sqlx::{SqlitePool};

pub async fn create_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect(database_url)
        .await
        .context("Failed to create DB pool")?;

    Ok(pool)
}