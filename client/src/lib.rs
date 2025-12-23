use reqwest::Client;

pub mod urls;

pub struct ApiClient {
    pub client: Client,
    base_url: String,
    session_token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String, session_token: Option<String>) -> Self {
        ApiClient {
            client: Client::new(),
            base_url,
            session_token,
        }
    }
}
