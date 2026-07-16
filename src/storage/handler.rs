use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    Json,
};
use serde_json::json;

pub async fn upload_file(
    _multipart: Multipart,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "success": true,
        "message": "Storage upload endpoint created. Backblaze integration coming next."
    })))
}

pub async fn delete_file(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "success": true,
        "deleted": id
    })))
}
