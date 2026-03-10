use crate::AppState;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;

pub async fn get_session(state: &AppState, jar: &CookieJar) -> Result<i32, StatusCode> {
    let session_id = jar.get("session_id").ok_or(StatusCode::UNAUTHORIZED)?;
    sqlx::query!(
        "SELECT user_id FROM sessions WHERE id = $1 AND now() < expires_at",
        session_id.value()
    )
    .fetch_optional(&state.db_pool)
    .await
    .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
    .map(|r| r.user_id)
    .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_username(state: &AppState, jar: &CookieJar) -> Result<String, StatusCode> {
    let user_id = get_session(&state, &jar).await?;
    Ok(
        sqlx::query!("SELECT username FROM users WHERE id = $1", user_id)
            .fetch_optional(&state.db_pool)
            .await
            .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
            .expect("user_id exists, username HAS to as well")
            .username,
    )
}

pub async fn user_has_role(
    state: &AppState,
    jar: &CookieJar,
    role: &str,
) -> Result<bool, StatusCode> {
    let user_id = get_session(&state, &jar).await?;
    let user = sqlx::query!(
        "SELECT u.id FROM users u, user_roles ur, roles r WHERE u.id = $1 AND r.name = $2 AND ur.user_id = u.id AND ur.role_id = r.id",
        user_id, role
    ).fetch_optional(&state.db_pool).await.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    if user.is_some() {
        return Ok(true);
    }
    Ok(false)
}
