use crate::AppState;
use crate::config::CONFIG;
use crate::utils::auth;
use crate::utils::auth::{generate_base64, user_has_role};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

pub async fn get(
    State(mut state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(session) = jar.get("session_id") {
        let session = auth::get_session(&mut state, session.value()).await?;
        if !user_has_role(&state, session.user_id, "admin").await? {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let code = generate_base64::<16>();
    let duration = CONFIG.constants.signup.code_duration;

    let _: () = redis::cmd("SET")
        .arg(&code)
        .arg("")
        .arg("EX")
        .arg(duration)
        .query_async(&mut state.redis_signup)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, code))
}
