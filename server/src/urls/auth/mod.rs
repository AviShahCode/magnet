mod login;
mod logout;
mod whoami;
pub mod signup;

use crate::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::post))
        .route("/whoami", get(whoami::get))
        .route("/logout", get(logout::get))
        .route("/signup", post(signup::post))
}
