use crate::AppState;
use axum::Router;
use axum::routing::{get, post};

pub mod login;
mod logout;
pub mod profile;

pub fn account_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::login_post))
        .route("/logout", get(logout::logout))
        .route("/@{username}", get(profile::profile))
}
