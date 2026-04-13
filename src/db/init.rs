use anyhow::Context;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use log::{error, info};
use sqlx::sqlite::SqlitePoolOptions;

/// Initializes the database
///
/// For testing, use `sqlite::memory:` as `database_url` value.
pub async fn init_db(database_url: &str) -> anyhow::Result<SqlitePool> {
    // 1. Create DB if not exists
    match Sqlite::database_exists(database_url).await {
        Ok(false) => {
            info!("Creating database...");
            Sqlite::create_database(database_url).await?;
        }
        Ok(true) => {
            info!("Database already exists");
        }
        Err(e) => {
            error!("Failed to check if database exists: {}", e);
            return Err(e.into())
        },
    }

    // 2. Create pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to database")?;

    // 3. Run migrations
    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    Ok(pool)
}