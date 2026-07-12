use axum::{extract::{Multipart, State}, Json};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    error::{ApiError, ApiResult},
    models::MediaRecord,
    state::AppState,
};

const IMAGE_MIMES: &[&str] = &["image/jpeg", "image/png", "image/webp", "image/gif"];
const VIDEO_MIMES: &[&str] = &["video/mp4", "video/webm", "video/quicktime"];

fn ext_for(mime: &str) -> &'static str {
    match mime {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "video/mp4" => "mp4",
        "video/webm" => "webm",
        "video/quicktime" => "mov",
        _ => "bin",
    }
}

async fn save(state: &AppState, user_id: Uuid, kind: &str, allowed: &[&str], mut mp: Multipart) -> ApiResult<MediaRecord> {
    while let Some(field) = mp.next_field().await.map_err(|e| ApiError::BadRequest(e.to_string()))? {
        if field.name() != Some("file") { continue; }
        let mime = field.content_type().unwrap_or("application/octet-stream").to_string();
        if !allowed.contains(&mime.as_str()) {
            return Err(ApiError::UnsupportedMediaType);
        }
        let filename = format!("{}.{}", Uuid::new_v4(), ext_for(&mime));
        let path = std::path::Path::new(&state.cfg.upload_dir).join(&filename);
        let mut file = tokio::fs::File::create(&path).await?;
        let mut total: usize = 0;
        let mut stream = field;
        while let Some(chunk) = stream.chunk().await.map_err(|e| ApiError::BadRequest(e.to_string()))? {
            total += chunk.len();
            if total > state.cfg.max_upload_bytes {
                let _ = tokio::fs::remove_file(&path).await;
                return Err(ApiError::PayloadTooLarge);
            }
            file.write_all(&chunk).await?;
        }
        file.flush().await?;
        let rec = sqlx::query_as::<_, MediaRecord>(
            "INSERT INTO media (user_id, kind, filename, mime_type, size_bytes) VALUES ($1,$2,$3,$4,$5) RETURNING *"
        )
        .bind(user_id)
        .bind(kind)
        .bind(&filename)
        .bind(&mime)
        .bind(total as i64)
        .fetch_one(&state.db)
        .await?;
        return Ok(rec);
    }
    Err(ApiError::BadRequest("missing 'file' field".into()))
}

pub async fn upload_image(State(state): State<AppState>, user: AuthUser, mp: Multipart) -> ApiResult<Json<MediaRecord>> {
    Ok(Json(save(&state, user.id, "image", IMAGE_MIMES, mp).await?))
}

pub async fn upload_video(State(state): State<AppState>, user: AuthUser, mp: Multipart) -> ApiResult<Json<MediaRecord>> {
    Ok(Json(save(&state, user.id, "video", VIDEO_MIMES, mp).await?))
}
