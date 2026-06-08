use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use crate::AppState;
use crate::utils::auth;

pub async fn get (
    State(mut state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(session) = jar.get("session_id") {
        let session = auth::get_session(&mut state, session.value())
            .await?;
        return Ok(session.username);
    }
    Err(StatusCode::UNAUTHORIZED)
}