pub mod drive;
pub mod login;
pub mod mail;
pub mod admin;

use crate::AppState;
use axum::Router;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::routing::{get, post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "magnet home page" }))
        .route("/admin", get(admin::admin))
        .route(
            "/@{username}",
            get(
                |headers: HeaderMap, Path(username): Path<String>| async move {
                    println!("{:?}", headers);
                    format!("hello {}", username)
                },
            ),
        )
        .route("/login", post(login::login))
        .nest("/drive", drive::router())
        .nest("/mail", mail::router())
}
