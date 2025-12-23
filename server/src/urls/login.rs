use crate::AppState;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    hashed: String
}

#[derive(Serialize, Deserialize)]
enum LoginResponse {
    Token(String),
    InvalidUsername,
    WrongPassword,
    Error(String),
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let username = payload.username;
    let row = sqlx::query!(
        r"SELECT id, username, hashed FROM users WHERE username = $1",
        username
    )  // TODO: password check
    .fetch_optional(&state.db_pool)
    .await;

    if let Err(e) = row {
        return Json(LoginResponse::Error(format!("{:?}", e)));
    }
    let row = row.unwrap();
    if row.is_none() {
        return Json(LoginResponse::InvalidUsername);
    }
    let row = row.unwrap();
    if payload.hashed != row.hashed {
        return Json(LoginResponse::WrongPassword);
    }

    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    let token = hex::encode(bytes);
    let status = sqlx::query!(
        r"insert into sessions (user_id, token) values ($1, $2)",
        row.id,
        token
    )
    .execute(&state.db_pool)
    .await;
    match status {
        Ok(_) => {
            Json(LoginResponse::Token(token))
        }
        Err(e) => {
            Json(LoginResponse::Error(format!("{:?}", e)))
        }
    }
}
