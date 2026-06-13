use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use crate::AppState;

pub async fn get(
    State(mut state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(session) = jar.get("session_id") {
        let _: () = redis::cmd("DEL")
            .arg(session.value())
            .query_async(&mut state.redis_session)
            .await
            .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    }
    Ok((jar.remove(Cookie::from("session_id")), StatusCode::NO_CONTENT))
}