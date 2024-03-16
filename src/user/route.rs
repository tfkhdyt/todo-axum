use super::handler::{inspect, login, register};
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn user_route() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/inspect", get(inspect))
}
