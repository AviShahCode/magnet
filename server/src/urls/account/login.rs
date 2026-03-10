use crate::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use time::Duration;

fn generate_session_id() -> String {
    let mut bytes = [0u8; 16]; // TODO: constant
    rand::fill(&mut bytes);
    hex::encode(bytes)
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String, // TODO: argon2id
}

// TODO: error messages
pub async fn login_post(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), impl IntoResponse> {
    // logout first?

    let username = payload.username;

    // fetch user
    // Result<Option<Row>>
    let user = sqlx::query!("SELECT id, hashed, salt FROM users WHERE username = $1", username)
        .fetch_optional(&state.db_pool)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let hash = hex::encode(Sha256::digest(payload.password + &user.salt));

    // verify hash
    if user.hashed != hash {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // generate session
    let session_id = generate_session_id();
    sqlx::query!(
        "INSERT INTO sessions (id, user_id) VALUES ($1, $2)",
        session_id,
        user.id
    )
    .execute(&state.db_pool)
    .await
    .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    // TODO: add http-only, same_site, max_age, https
    let cookie = Cookie::build(("session_id", session_id))
        .path("/")
        .max_age(Duration::days(30)) // TODO: constant
        // .http_only(true)
        // .same_site(SameSite::Lax)
        // .secure(false)
        .build();

    Ok((jar.add(cookie), StatusCode::OK))
}
