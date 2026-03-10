use crate::AppState;
use crate::urls::drive::authorize_path;
use crate::utils::auth::{get_session};
use crate::utils::global::BASE_DIR;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use serde::Serialize;
use tokio::fs;

#[derive(sqlx::Type, Serialize, Debug, PartialOrd, PartialEq)]
#[sqlx(type_name = "drive.item_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Folder,
    File,
}

// get on folder -> Vec<FolderItem>
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct FolderItem {
    id: String,
    name: String,
    item_type: ItemType,
}

// actual response, one of File, Vec<FolderItem>
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GetResponse {
    File { name: String, content: Vec<u8> },  // TODO: convert to Bytes/octet-stream
    Folder { items: Vec<FolderItem> },
}

pub async fn path_get(
    State(state): State<AppState>,
    jar: CookieJar,
    path: Option<Path<String>>,
) -> Result<(CookieJar, impl IntoResponse), StatusCode> {
    let user_id = get_session(&state, &jar).await?;
    let path = path.as_ref().map(|Path(p)| p);

    // if path specified, if not, its just home dir, which he would have access to
    if let Some(parent) = path {
        // ensure existence and ownership
        authorize_path(&state, &jar, parent).await?;

        // check if its a file
        let file = sqlx::query!(
            "SELECT name FROM drive.files \
            WHERE user_id = $1 AND id = $2 and item_type = 'file'",
            user_id,
            parent
        )
        .fetch_optional(&state.db_pool)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

        // found file, file only holds name
        if let Some(file) = file {
            let path = BASE_DIR.join("drive").join(parent);
            // check existence of file in drive folder
            if !fs::try_exists(&path)
                .await
                .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
            {
                return Err(StatusCode::NOT_FOUND);
            }
            let content = fs::read(path)
                .await
                .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
            return Ok((
                jar,
                (
                    StatusCode::OK,
                    Json(GetResponse::File {
                        name: file.name,
                        content,
                    }),
                ),
            ));
        }
    }

    // TODO: has read access
    let files: Vec<FolderItem> = sqlx::query_as!(
        FolderItem,
        "SELECT id, name, item_type as \"item_type: ItemType\" FROM drive.files \
        WHERE user_id = $1 AND (parent = $2 OR (parent IS NULL AND $2 IS NULL)) \
        ORDER BY item_type, name",
        user_id,
        path
    )
    .fetch_all(&state.db_pool)
    .await
    .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((
        jar,
        (StatusCode::OK, Json(GetResponse::Folder { items: files })),
    ))
}
