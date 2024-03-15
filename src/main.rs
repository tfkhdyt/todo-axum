use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
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
    todo_repo: TodoRepo,
    user_repo: UserRepo,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let port = env::var("PORT")?;

    let pool = db::get_sqlite_connection().await?;

    let todo_repo = TodoRepo::new();
    let user_repo = UserRepo::new();
    let shared_state = AppState {
        pool,
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
        .with_state(shared_state);

    println!("Running on http://localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
