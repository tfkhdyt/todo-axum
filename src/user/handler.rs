use super::dto::AddUserRequest;
use crate::{error::HttpResult, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AddUserRequest>,
) -> HttpResult<impl IntoResponse> {
    state
        .user_repo
        .verify_username_availability(&state.pool, &payload.username)
        .await?;
    let new_user = payload.into_user(&state.salt, &state.argon2)?;
    state.user_repo.create(&state.pool, new_user).await?;

    let response = (
        StatusCode::CREATED,
        Json(json!({
            "message": "new user has been registered"
        })),
    );

    Ok(response)
}
