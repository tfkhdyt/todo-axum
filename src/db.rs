use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

pub async fn get_sqlite_connection() -> Result<Pool<Sqlite>> {
    let db_url = env::var("DATABASE_URL")?;

    let pool = SqlitePool::connect(&db_url).await?;

    Ok(pool)
}
