use crate::AppState;
use crate::urls::drive::authorize_path;
use crate::urls::drive::view::ItemType;
use crate::utils::auth::{generate_base64, get_session};
use crate::utils::global::DATA_DIR;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use base64::prelude::*;
use log::warn;
use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize, Debug)]
pub struct UploadItem {
    name: String,
    content: Option<String>,
}

pub async fn post(
    State(mut state): State<AppState>,
    jar: CookieJar,
    path: Option<Path<String>>,
    Json(payload): Json<UploadItem>,
) -> Result<impl IntoResponse, StatusCode> {
    let session = jar.get("session_id");
    if session.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let session_id = session.unwrap();
    let session = get_session(&mut state, session_id.value()).await?;
    let path = path.as_ref().map(|Path(p)| p);

    // TODO: already exists
    if let Some(path) = path {
        // ensure existence and ownership
        authorize_path(&state, session.user_id, path).await?;

        // assert parent is folder
        let item_type: ItemType = sqlx::query!(
            "SELECT item_type AS \"item_type: ItemType\" FROM drive.files WHERE id = $1",
            path
        )
        .fetch_optional(&state.db)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(StatusCode::NOT_FOUND)?
        .item_type;

        if item_type == ItemType::File {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let item_id = generate_base64::<16>();
    let item_type;
    match payload.content {
        Some(base64_content) => {
            let content = BASE64_STANDARD
                .decode(base64_content)
                .or(Err(StatusCode::BAD_REQUEST))?;
            fs::write(DATA_DIR.join("drive").join(&item_id), content)
                .await
                .map_err(|e| {
                    warn!(target: "drive", "Failed to write to file: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            item_type = ItemType::File;
        }
        None => {
            item_type = ItemType::Folder;
        }
    }
    sqlx::query!(
        "INSERT INTO drive.files (id, name, user_id, item_type, parent) VALUES ($1, $2, $3, $4, $5)",
        item_id, payload.name, session.user_id, item_type as ItemType, path
    )
        .execute(&state.db)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok((jar, (StatusCode::CREATED, item_id)))
}
