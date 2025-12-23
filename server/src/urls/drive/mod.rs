use crate::AppState;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(|| async { "drive home page" }))
}
