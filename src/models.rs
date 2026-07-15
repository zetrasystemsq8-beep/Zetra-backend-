use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub zetra_id: String,
    pub zetramail: String,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub zetra_id: String,
    pub zetramail: String,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            email: u.email,
            zetra_id: u.zetra_id,
            zetramail: u.zetramail,
            phone: u.phone,
            created_at: u.created_at,
        }
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MediaRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kind: String,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 { 50 }

/// A registered Zetra ecosystem app (NAI, Nigergram, ZTC, etc.)
/// allowed to send messages/verification codes to user inboxes.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ServiceApp {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing)]
    pub api_key_hash: String,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

/// A message delivered to a user's Zetra inbox — typically a
/// verification code sent from another Zetra app.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub from_app: String,
    pub kind: String,
    pub subject: String,
    pub body: String,
    pub code: Option<String>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
