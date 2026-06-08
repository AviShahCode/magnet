use crate::config::CONFIG;
use crate::utils::auth::Session;
use crate::{AppState, utils};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use time::Duration;

#[derive(Deserialize)]
pub(super) struct LoginRequest {
    username: String,
    password: String,
}

pub(super) async fn post(
    State(mut state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // logout first?

    let username = payload.username;

    let user = sqlx::query!(
        "SELECT id, hashed, salt FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&state.db)
    .await
    .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let hash = hex::encode(Sha256::digest(payload.password + &user.salt));

    // verify hash
    if user.hashed != hash {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let duration = CONFIG.constants.session.duration;
    let session = Session::new(username, user.id, duration);
    let session_data = serde_json::to_string(&session).unwrap();

    // generate session
    let session_id = utils::auth::generate_hex::<16>();
    let _: () = redis::cmd("SET")
        .arg(&session_id)
        .arg(&session_data)
        .arg("EX")
        .arg(duration)
        .query_async(&mut state.redis)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    // TODO: add http-only, same_site, max_age, https
    let cookie = Cookie::build(("session_id", session_id))
        .path("/")
        .max_age(Duration::seconds(duration as i64)) // TODO: overflow
        // .http_only(true)
        // .same_site(SameSite::Lax)
        // .secure(false)
        .build();

    Ok((jar.add(cookie), StatusCode::OK))
}
