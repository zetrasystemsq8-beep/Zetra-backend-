use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};

use bytes::Bytes;
use serde_json::json;
use uuid::Uuid;

use crate::state::AppState;

pub async fn upload_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, StatusCode> {

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        let file_name = field
            .file_name()
            .unwrap_or("file.bin")
            .to_string();

        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let data = field
            .bytes()
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let key = format!(
            "uploads/{}/{}",
            Uuid::new_v4(),
            file_name
        );

        state
            .storage
            .upload(
                key.clone(),
                Bytes::from(data),
                content_type,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(Json(json!({
            "success": true,
            "key": key
        })));
    }

    Err(StatusCode::BAD_REQUEST)
}


pub async fn delete_file(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {

    state
        .storage
        .delete(id.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": true,
        "deleted": id
    })))
}
