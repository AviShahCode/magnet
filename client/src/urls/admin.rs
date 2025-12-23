use crate::ApiClient;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub enum AdminResponse {
    NoSessionToken,
    InvalidSessionToken,
    NotAuthorized,
    Error(String),
    Pass(String),
}

pub async fn admin(client: &ApiClient) -> AdminResponse {
    if client.session_token.is_none() {
        return AdminResponse::NoSessionToken;
    }

    let res = client
        .client
        .get(client.base_url.clone() + "/admin")
        .header("session_token", client.session_token.as_ref().unwrap())
        .send()
        .await;

    match res {
        Ok(res) => {
            let response: AdminResponse = res.json().await.unwrap();
            if let AdminResponse::Pass(s) = response.clone() {
                // println!("Pass: {}", s);
            }
            response
        }
        Err(e) => AdminResponse::Error(e.to_string()),
    }
}
