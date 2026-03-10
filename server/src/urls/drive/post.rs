use super::get::ItemType;
use crate::AppState;
use crate::urls::drive::authorize_path;
use crate::utils::auth::{get_session};
use crate::utils::global::BASE_DIR;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use log::{warn};
use serde::Deserialize;
use tokio::fs;

fn generate_file_id() -> String {
    let mut bytes = [0u8; 16]; // TODO: constant
    rand::fill(&mut bytes);
    hex::encode(bytes)
}

#[derive(Deserialize, Debug)]
pub struct UploadItem {
    name: String,
    content: Option<Vec<u8>>,
}

pub async fn path_post(
    State(state): State<AppState>,
    jar: CookieJar,
    path: Option<Path<String>>,
    Json(payload): Json<UploadItem>,
) -> Result<(CookieJar, impl IntoResponse), impl IntoResponse> {
    let user_id = get_session(&state, &jar).await?;
    let path = path.as_ref().map(|Path(p)| p);

    // TODO: has write access in parent, already exists
    if let Some(path) = path {
        // ensure existence and ownership
        authorize_path(&state, &jar, path).await?;

        // assert parent is folder
        let item_type: ItemType = sqlx::query!(
            "SELECT item_type AS \"item_type: ItemType\" FROM drive.files WHERE id = $1",
            path
        )
        .fetch_optional(&state.db_pool)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        .ok_or(StatusCode::NOT_FOUND)?
        .item_type;

        if item_type == ItemType::File {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let item_id = generate_file_id();
    let item_type;
    match payload.content {
        Some(content) => {
            fs::write(BASE_DIR.join("drive").join(&item_id), content)
                .await
                .map_err(|e| {
                    warn!("Failed to write to file: {}", e);
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
        item_id, payload.name, user_id, item_type as ItemType, path
    )
        .execute(&state.db_pool)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok((jar, (StatusCode::CREATED, item_id)))
}
