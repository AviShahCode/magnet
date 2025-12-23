use crate::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
enum AdminResponse {
    NoSessionToken,
    InvalidSessionToken,
    NotAuthorized,
    Error(String),
    Pass(String),
}

pub async fn admin(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let token = headers.get("session_token");
    println!("{:?}", token);
    if token.is_none() {
        return (StatusCode::BAD_REQUEST, Json(AdminResponse::NoSessionToken)).into_response();
    }
    let value = token.unwrap().to_str();
    if value.is_err() {
        return (StatusCode::BAD_REQUEST, Json(AdminResponse::InvalidSessionToken)).into_response();
    }

    let row = sqlx::query!(
        "select user_id from sessions where token = $1 and exp > now()",
        value.unwrap()
    )
    .fetch_optional(&state.db_pool)
    .await;
    if let Err(e) = row {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(AdminResponse::Error(format!("{:?}", e)))).into_response();
    }
    let row = row.unwrap();
    if row.is_none() {
        return (StatusCode::BAD_REQUEST, Json(AdminResponse::InvalidSessionToken)).into_response();
    }
    let row = row.unwrap();
    println!("{:?}", row);
    let row = sqlx::query!(
        "select u.username, r.role
    from users u, user_roles ur, roles r
    where u.id = ur.user_id and ur.id = r.id and r.role = 'admin' and u.id = $1;"
    , row.user_id)
    .fetch_optional(&state.db_pool)
    .await;
    if let Err(e) = row {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(AdminResponse::Error(format!("{:?}", e))))
        .into_response();
    }
    let row = row.unwrap();
    if row.is_none() {
        return (StatusCode::UNAUTHORIZED, Json(AdminResponse::NotAuthorized)).into_response();
    }
    (StatusCode::OK, Json(AdminResponse::Pass(format!("Hello admin {}", row.unwrap().username)))).into_response()
}
