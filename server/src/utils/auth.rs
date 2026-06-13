use crate::AppState;
use axum::http::StatusCode;
use base64::prelude::*;
use serde::{Deserialize, Serialize};

pub fn generate_base64<const N: usize>() -> String {
    // Generate a random base64 string with N bytes of entropy
    let mut bytes = [0u8; N];
    rand::fill(&mut bytes);
    BASE64_URL_SAFE_NO_PAD.encode(bytes)
}

pub async fn get_session(state: &mut AppState, session_id: &str) -> Result<Session, StatusCode> {
    let session_data: Option<String> = redis::cmd("GET")
        .arg(&session_id)
        .query_async(&mut state.redis_session)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    match session_data {
        Some(data) => {
            let session: Session = serde_json::from_str(&data).unwrap();
            Ok(session)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub username: String,
    pub user_id: i32,
    pub iat: u64,
    pub exp: u64,
}

impl Session {
    pub fn new(username: String, user_id: i32, duration: u64) -> Self {
        let iat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp = iat + duration;
        Self {
            username,
            user_id,
            iat,
            exp,
        }
    }
}

pub async fn user_has_role(state: &AppState, user_id: i32, role: &str) -> Result<bool, StatusCode> {
    let user = sqlx::query!(
        "SELECT u.id FROM users u, user_roles ur, roles r WHERE u.id = $1 AND r.name = $2 AND ur.user_id = u.id AND ur.role_id = r.id",
        user_id, role
    ).fetch_optional(&state.db).await.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    if user.is_some() {
        return Ok(true);
    }
    Ok(false)
}
