use crate::AppState;
use crate::urls::drive::authorize_path;
use crate::utils::auth::get_session;
use crate::utils::global::BASE_DIR;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

pub async fn path_delete(
    State(state): State<AppState>,
    jar: CookieJar,
    path: Path<String>,
) -> Result<(CookieJar, impl IntoResponse), StatusCode> {
    let user_id = get_session(&state, &jar).await?;
    let Path(path) = path;

    // ensure existence and ownership
    authorize_path(&state, &jar, &path).await?;

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
        user_id,
        path
    )
    .fetch_all(&state.db_pool)
    .await
    .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    // delete all nested files
    for row in rows {
        let file_path = BASE_DIR.join("drive").join(row.id);
        let _ = tokio::fs::remove_file(file_path).await;
    }

    Ok((jar, StatusCode::NO_CONTENT))
}
