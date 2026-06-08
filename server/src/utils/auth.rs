use crate::AppState;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

pub fn generate_hex<const N: usize>() -> String {
    let mut bytes = [0u8; N];
    rand::fill(&mut bytes);
    hex::encode(bytes)
}

pub async fn get_session(state: &mut AppState, session_id: &str) -> Result<Session, StatusCode> {
    let session_data: Option<String> = redis::cmd("GET")
        .arg(&session_id)
        .query_async(&mut state.redis)
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
