use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), StatusCode> {
    if let Some(session) = jar.get("session_id") {
        sqlx::query!("DELETE FROM sessions WHERE id = $1", session.value())
            .execute(&state.db_pool)
            .await
            .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    }
    Ok((jar.remove(Cookie::from("session_id")), StatusCode::OK))
}
