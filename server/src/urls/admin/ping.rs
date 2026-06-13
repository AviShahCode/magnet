use crate::AppState;
use crate::utils::auth;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use crate::utils::auth::user_has_role;

pub(super) async fn get(
    State(mut state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(session) = jar.get("session_id") {
        let session = auth::get_session(&mut state, session.value()).await?;
        if !user_has_role(&state, session.user_id, "admin").await? {
            return Err(StatusCode::FORBIDDEN);
        }
        Ok("pong")
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
