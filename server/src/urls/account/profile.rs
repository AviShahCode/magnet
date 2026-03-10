use crate::AppState;
use crate::utils::auth::get_username;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

pub async fn profile(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(username): Path<String>,
) -> Result<(CookieJar, impl IntoResponse), StatusCode> {
    let mut output = "".to_string();
    match get_username(&state, &jar).await {
        Ok(u) => output.push_str(format!("hello {} ", u).as_str()),
        Err(_) => {}
    };
    Ok((
        jar,
        (StatusCode::OK, format!("{}here is {}", output, username)),
    ))
}
