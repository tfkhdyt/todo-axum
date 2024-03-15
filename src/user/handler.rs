use super::dto::{AddUserRequest, LoginRequest};
use crate::{
    error::{AppError, HttpResult},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use redis::Commands;
use serde_json::json;
use uuid::Uuid;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AddUserRequest>,
) -> HttpResult<impl IntoResponse> {
    state
        .user_repo
        .verify_username_availability(&state.pool, &payload.username)
        .await?;
    let new_user = payload.into_user()?;
    state.user_repo.create(&state.pool, new_user).await?;

    let response = (
        StatusCode::CREATED,
        Json(json!({
            "message": "new user has been registered"
        })),
    );

    Ok(response)
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> HttpResult<impl IntoResponse> {
    payload.validate()?;
    let user = state
        .user_repo
        .find_one_by_username(&state.pool, &payload.username)
        .await?;
    user.check_password(&payload.password)?;

    let access_token = Uuid::new_v4().to_string();
    let refresh_token = Uuid::new_v4().to_string();

    let mut conn = state.redis_client.get_connection().map_err(|err| {
        println!("Error: {}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to get redis connection",
        )
    })?;

    conn.set_ex(format!("access_token:{}", &access_token), &user.id, 60 * 5)
        .map_err(|err| {
            println!("Error: {}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to store token to cache",
            )
        })?;
    conn.set_ex(
        format!("refresh_token:{}", &refresh_token),
        &user.id,
        60 * 60 * 24 * 7,
    )
    .map_err(|err| {
        println!("Error: {}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to store token to cache",
        )
    })?;

    let response = (
        StatusCode::CREATED,
        Json(json!({
            "access_token": access_token,
            "refresh_token": refresh_token,
        })),
    );

    Ok(response)
}
