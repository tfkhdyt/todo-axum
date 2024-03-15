use anyhow::Result;
use redis::Client;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

pub async fn get_sqlite_pool() -> Result<Pool<Sqlite>> {
    let db_url = env::var("DATABASE_URL")?;

    let pool = SqlitePool::connect(&db_url).await?;

    Ok(pool)
}
pub fn get_redis_client() -> Result<Client> {
    let redis_url = env::var("REDIS_URL")?;
    let redis_client = redis::Client::open(redis_url)?;

    Ok(redis_client)
}
