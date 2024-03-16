use super::dto::{AddUserRequest, InspectResponse, LoginRequest};
use crate::{
    error::{AppError, HttpResult},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use cookie::time::Duration;
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
    jar: PrivateCookieJar,
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

    state
        .user_repo
        .set_token(&state.redis_client, &access_token, &refresh_token, &user.id)
        .await?;

    let access_cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .max_age(Duration::minutes(5));
    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .max_age(Duration::days(7));

    let cookie = jar.add(access_cookie).add(refresh_cookie);

    let response = (
        StatusCode::CREATED,
        cookie,
        Json(json!({
            "message": "login success",
        })),
    );

    Ok(response)
}

pub async fn inspect(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
) -> HttpResult<impl IntoResponse> {
    let Some(access_token) = jar.get("access_token") else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "access token is invalid",
        ));
    };

    let user = state
        .user_repo
        .find_one_by_access_token(&state.redis_client, &state.pool, access_token.value())
        .await?;

    let response = Json(InspectResponse {
        id: user.id,
        name: user.name,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
    });

    Ok(response)
}

pub async fn refresh_token(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
) -> HttpResult<impl IntoResponse> {
    let Some(old_refresh_token) = jar.get("refresh_token") else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "access token is invalid",
        ));
    };

    let user = state
        .user_repo
        .find_one_by_refresh_token(&state.redis_client, &state.pool, old_refresh_token.value())
        .await?;

    let access_token = Uuid::new_v4().to_string();
    let refresh_token = Uuid::new_v4().to_string();

    state
        .user_repo
        .set_token(&state.redis_client, &access_token, &refresh_token, &user.id)
        .await?;

    let access_cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .max_age(Duration::minutes(5));
    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .max_age(Duration::days(7));

    let cookie = jar.add(access_cookie).add(refresh_cookie);

    let response = (
        StatusCode::CREATED,
        cookie,
        Json(json!({
            "message": "refresh success",
        })),
    );

    Ok(response)
}
