use crate::AppState;
use axum::Router;

pub mod admin;
pub mod auth;
pub mod common;
pub mod drive;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(auth::router())
        .merge(common::router())
        .nest("/drive", drive::router())
        .nest("/admin", admin::router())
}
