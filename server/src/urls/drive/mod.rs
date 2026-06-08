pub mod delete;
pub mod upload;
pub mod view;

use crate::AppState;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use log::debug;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(view::get))
        .route("/{path}", get(view::get))
        .route("/", post(upload::post))
        .route("/{path}", post(upload::post))
        .route("/{path}", delete(delete::delete))
}

// TODO: transform to middleware
// check existence and ownership of path
pub async fn authorize_path(
    state: &AppState,
    user_id: i32,
    path: &str,
) -> Result<(), StatusCode> {
    let item = sqlx::query!("select user_id, name from drive.files where id = $1", path)
        .fetch_optional(&state.db)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if item.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let item = item.unwrap();
    if item.user_id != user_id {
        debug!(
            target: "drive",
            "FORBIDDEN: user {} is trying to access {}",
            user_id,
            path
        );
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(())
}
