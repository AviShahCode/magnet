use crate::AppState;
use crate::config::CONFIG;
use crate::utils::auth::generate_base64;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::sync::OnceLock;
use serde::Deserialize;

static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();

#[derive(Deserialize)]
pub(super) struct SignupRequest {
    pub code: String,
    pub username: String,
    pub password: String,
}

pub(super) async fn post(
    State(mut state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let code_exists: bool = redis::cmd("EXISTS")
        .arg(&payload.code)
        .query_async(&mut state.redis_signup)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !code_exists {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let username_len = payload.username.len() as u64;
    if CONFIG.constants.username.min_length > username_len {
        return Err(StatusCode::BAD_REQUEST);
    }
    if CONFIG.constants.username.max_length < username_len {
        return Err(StatusCode::BAD_REQUEST);
    }

    let regex = USERNAME_REGEX.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());
    if !regex.is_match(&payload.username) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let username_len = payload.password.len() as u64;
    if CONFIG.constants.password.min_length > username_len {
        return Err(StatusCode::BAD_REQUEST);
    }
    if CONFIG.constants.password.max_length < username_len {
        return Err(StatusCode::BAD_REQUEST);
    }

    let username_taken: Option<bool> = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
        payload.username
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if username_taken == Some(true) {
        return Err(StatusCode::CONFLICT);
    }

    let salt = generate_base64::<6>();

    let mut hasher = Sha256::new();
    hasher.update(payload.password.as_bytes());
    hasher.update(salt.as_bytes()); // Append salt to password
    let hashed_password = hex::encode(hasher.finalize());

    // insert into Postgres using a Transaction
    let mut tx = state.db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = sqlx::query!(
        "INSERT INTO users (username, hashed, salt) VALUES ($1, $2, $3) RETURNING id",
        payload.username,
        hashed_password,
        salt
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)",
        user_id.id,
        2
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _: () = redis::cmd("DEL")
        .arg(&payload.code)
        .query_async(&mut state.redis_signup)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
