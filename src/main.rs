use anyhow::Result;
use axum::{
    extract::FromRef,
    routing::{delete, get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use redis::Client;
use sqlx::SqlitePool;
use std::env;
use todo::repo::TodoRepo;
use user::repo::UserRepo;

mod db;
mod error;
mod todo;
mod user;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    redis_client: Client,
    key: Key,
    todo_repo: TodoRepo,
    user_repo: UserRepo,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let port = env::var("PORT")?;
    let app_secret = env::var("APP_SECRET")?;

    let pool = db::get_sqlite_pool().await?;
    let redis_client = db::get_redis_client()?;

    let todo_repo = TodoRepo::new();
    let user_repo = UserRepo::new();
    let shared_state = AppState {
        pool,
        redis_client,
        key: Key::from(app_secret.as_bytes()),
        todo_repo,
        user_repo,
    };

    let app = Router::new()
        .route(
            "/todos",
            get(todo::handler::find_all_todos).post(todo::handler::add_todo),
        )
        .route(
            "/todos/:id",
            delete(todo::handler::delete_todo).put(todo::handler::update_todo),
        )
        .route("/auth/register", post(user::handler::register))
        .route("/auth/login", post(user::handler::login))
        .route("/auth/inspect", post(user::handler::inspect))
        .with_state(shared_state);

    println!("Running on http://localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
