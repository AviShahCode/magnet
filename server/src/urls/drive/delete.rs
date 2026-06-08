use crate::AppState;
use crate::urls::drive::authorize_path;
use crate::utils::auth::get_session;
use crate::utils::global::DATA_DIR;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

pub async fn delete(
    State(mut state): State<AppState>,
    jar: CookieJar,
    path: Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let session = jar.get("session_id");
    if session.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let session_id = session.unwrap();
    let session = get_session(&mut state, session_id.value()).await?;

    let Path(path) = path;

    // ensure existence and ownership
    authorize_path(&state, session.user_id, &path).await?;

    let rows = sqlx::query!(
        r#"
        WITH RECURSIVE tree AS (
            SELECT id, item_type
            FROM drive.files
            WHERE user_id = $1 AND id = $2

            UNION ALL

            SELECT f.id, f.item_type
            FROM drive.files f
            INNER JOIN tree t ON f.parent = t.id
            WHERE f.user_id = $1
        ),
        deleted AS (
            DELETE FROM drive.files
            WHERE id IN (SELECT id FROM tree)
            RETURNING id, item_type
        )
        SELECT id FROM deleted WHERE item_type = 'file';
        "#,
        session.user_id,
        path
    )
    .fetch_all(&state.db)
    .await
    .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    // delete all nested files
    for row in rows {
        let file_path = DATA_DIR.join("drive").join(row.id);
        let _ = tokio::fs::remove_file(file_path).await;
    }

    Ok((jar, StatusCode::NO_CONTENT))
}
