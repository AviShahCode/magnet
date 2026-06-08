use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Constants {
    pub username: UsernameConfig,
    pub password: PasswordConfig,
    pub session: SessionConfig,
}

#[derive(Deserialize, Clone)]
pub struct UsernameConfig {
    pub min_length: u64,
    pub max_length: u64,
    pub pattern: String,
}

#[derive(Deserialize, Clone)]
pub struct PasswordConfig {
    pub min_length: u64,
    pub max_length: u64,
}

#[derive(Deserialize, Clone)]
pub struct SessionConfig {
    pub duration: u64,
}
