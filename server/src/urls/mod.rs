pub mod account;
pub mod drive;

use crate::AppState;
use crate::utils::auth::user_has_role;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum_extra::extract::CookieJar;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(|| async { (StatusCode::OK, "hello ".repeat(100)) }),
        )
        .route("/admin", get(admin))
        .merge(account::account_router())
        .nest("/drive", drive::drive_router())
}

pub async fn admin(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), impl IntoResponse> {
    if user_has_role(&state, &jar, "admin").await? {
        return Ok((jar, StatusCode::OK));
    }
    Err(StatusCode::FORBIDDEN)
}
