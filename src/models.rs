use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{tweet, user};

// リクエスト/レスポンス用の構造体
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTweetRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct TweetResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

// SeaORMのModelからRust型への変換ヘルパー
impl From<user::Model> for UserResponse {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
        }
    }
}

impl From<tweet::Model> for TweetResponse {
    fn from(model: tweet::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            content: model.content,
            created_at: DateTime::parse_from_rfc3339(&model.created_at)
                .expect("Invalid date format")
                .with_timezone(&Utc),
        }
    }
}
