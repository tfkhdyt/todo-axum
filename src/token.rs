use crate::error::{AppError, HttpResult};
use axum::http::StatusCode;
use axum_extra::extract::PrivateCookieJar;
use cookie::{time::Duration, Cookie};
use redis::{Client, Commands};
use uuid::Uuid;

#[derive(Clone)]
pub struct TokenManager;

impl TokenManager {
    pub fn new() -> Self {
        Self {}
    }

    async fn set_token(
        &self,
        redis_client: &Client,
        access_token: &str,
        refresh_token: &str,
        user_id: &str,
    ) -> HttpResult<()> {
        let mut conn = redis_client.get_connection().map_err(|err| {
            println!("Error: {}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to get redis connection",
            )
        })?;

        conn.set_ex(format!("access_token:{}", &access_token), user_id, 60 * 5)
            .map_err(|err| {
                println!("Error: {}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to store token to cache",
                )
            })?;
        conn.set_ex(
            format!("refresh_token:{}", &refresh_token),
            user_id,
            60 * 60 * 24 * 7,
        )
        .map_err(|err| {
            println!("Error: {}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to store token to cache",
            )
        })?;

        Ok(())
    }

    pub async fn create_token(
        &self,
        redis_client: &Client,
        jar: PrivateCookieJar,
        user_id: &str,
    ) -> HttpResult<PrivateCookieJar> {
        let access_token = Uuid::new_v4().to_string();
        let refresh_token = Uuid::new_v4().to_string();

        self.set_token(redis_client, &access_token, &refresh_token, user_id)
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

        Ok(cookie)
    }

    pub async fn find_user_id_by_access_token(
        &self,
        client: &Client,
        access_token: &str,
    ) -> HttpResult<String> {
        let mut conn = client.get_connection().map_err(|err| {
            println!("Error: {}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to get redis connection",
            )
        })?;

        let user_id: String =
            conn.get(format!("access_token:{}", access_token))
                .map_err(|err| {
                    println!("Error: {}", err);
                    AppError::new(StatusCode::UNAUTHORIZED, "access token is not found")
                })?;

        Ok(user_id)
    }

    pub async fn find_user_id_by_refresh_token(
        &self,
        client: &Client,
        refresh_token: &str,
    ) -> HttpResult<String> {
        let mut conn = client.get_connection().map_err(|err| {
            println!("Error: {}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to get redis connection",
            )
        })?;

        let user_id: String = conn
            .get(format!("refresh_token:{}", refresh_token))
            .map_err(|err| {
                println!("Error: {}", err);
                AppError::new(StatusCode::UNAUTHORIZED, "refresh token is not found")
            })?;

        Ok(user_id)
    }
}
