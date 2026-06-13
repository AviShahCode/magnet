mod ping;
mod signup_code;

use crate::AppState;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ping", get(ping::get))
        .route("/signup_code", get(signup_code::get))
}
