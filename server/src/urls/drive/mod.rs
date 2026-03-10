mod delete;
mod get;
mod post;

use crate::AppState;
use crate::utils::auth::{get_session, get_username};
use axum::Router;
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum_extra::extract::CookieJar;
use log::debug;

pub fn drive_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get::path_get))
        .route("/{path}", get(get::path_get))
        .route("/", post(post::path_post))
        .route("/{path}", post(post::path_post))
        .route("/{path}", delete(delete::path_delete))
}

// check existence and ownership of path
pub async fn authorize_path(
    state: &AppState,
    jar: &CookieJar,
    path: &String,
) -> Result<(), StatusCode> {
    let user_id = get_session(&state, &jar).await?;
    let item = sqlx::query!("select user_id, name from drive.files where id = $1", path)
        .fetch_optional(&state.db_pool)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if item.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let item = item.unwrap();
    if item.user_id != user_id {
        debug!(
            "FORBIDDEN: user {} is trying to get {}",
            get_username(&state, &jar).await?,
            path
        );
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(())
}
