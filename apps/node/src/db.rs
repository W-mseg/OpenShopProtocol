use anyhow::Result;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use std::str::FromStr;

pub async fn init(database_url: &str) -> Result<SqlitePool> {
    // Parse options and enable auto-creation of the DB file
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Run migrations automatically at startup
    sqlx::migrate!("./migrations").run(&pool).await?;

    tracing::info!("Database ready");
    Ok(pool)
}
