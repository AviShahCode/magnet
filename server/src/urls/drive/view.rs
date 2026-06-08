use crate::AppState;
use crate::urls::drive::authorize_path;
use crate::utils::auth::get_session;
use crate::utils::global::DATA_DIR;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use base64::prelude::*;
use serde::Serialize;
use tokio::fs;

#[derive(sqlx::Type, Serialize, Debug, PartialOrd, PartialEq)]
#[sqlx(type_name = "drive.item_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub(super) enum ItemType {
    Folder,
    File,
}

// get on folder -> Vec<FolderItem>
#[derive(sqlx::FromRow, Debug, Serialize)]
pub(super) struct FolderItem {
    id: String,
    name: String,
    item_type: ItemType,
}

// actual response, one of File, Vec<FolderItem>
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum GetResponse {
    File { name: String, content: String }, // TODO: convert to Bytes/octet-stream
    Folder { items: Vec<FolderItem> },
}

pub(super) async fn get(
    State(mut state): State<AppState>,
    jar: CookieJar,
    path: Option<Path<String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let session = jar.get("session_id");
    if session.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let session_id = session.unwrap();
    let session = get_session(&mut state, session_id.value()).await?;
    let path = path.as_ref().map(|Path(p)| p);

    // if path specified, if not, its just home dir, which he would have access to
    if let Some(parent) = path {
        // ensure existence and ownership
        authorize_path(&state, session.user_id, parent).await?;

        // check if its a file
        let file = sqlx::query!(
            "SELECT name FROM drive.files \
            WHERE user_id = $1 AND id = $2 and item_type = 'file'",
            session.user_id,
            parent
        )
        .fetch_optional(&state.db)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

        // found file, file only holds name
        if let Some(file) = file {
            let path = DATA_DIR.join("drive").join(parent);
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
            let base64_content = BASE64_STANDARD.encode(content);
            return Ok((
                jar,
                (
                    StatusCode::OK,
                    Json(GetResponse::File {
                        name: file.name,
                        content: base64_content,
                    }),
                ),
            ));
        }
    }

    let files: Vec<FolderItem> = sqlx::query_as!(
        FolderItem,
        "SELECT id, name, item_type as \"item_type: ItemType\" FROM drive.files \
        WHERE user_id = $1 AND (parent = $2 OR (parent IS NULL AND $2 IS NULL)) \
        ORDER BY item_type, name",
        session.user_id,
        path
    )
    .fetch_all(&state.db)
    .await
    .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((
        jar,
        (StatusCode::OK, Json(GetResponse::Folder { items: files })),
    ))
}
