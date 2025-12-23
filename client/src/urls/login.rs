use crate::ApiClient;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    hashed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoginResponse {
    Token(String),
    InvalidUsername,
    WrongPassword,
    Error(String),
}

pub async fn login(client: &mut ApiClient, username: String, password: String) -> LoginResponse {
    let payload = LoginRequest {
        username,
        hashed: hex::encode(Sha256::digest(password)),
    };
    let res = client
        .client
        .post(client.base_url.clone() + "/login")
        .json(&serde_json::json!(payload))
        .send()
        .await;

    match res {
        Ok(res) => {
            let t = res.json::<LoginResponse>().await;
            let response: LoginResponse = t.unwrap();
            if let LoginResponse::Token(token) = response.clone() {
                client.session_token = Some(token);
            }
            response
        }
        Err(e) => {
            LoginResponse::Error(e.to_string())
        }
    }
}
