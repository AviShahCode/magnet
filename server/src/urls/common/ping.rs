use axum::response::IntoResponse;

pub(super) async fn get() -> impl IntoResponse {
    "pong"
}