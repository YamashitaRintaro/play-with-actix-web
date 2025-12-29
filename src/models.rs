use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::error::AppError;

/// ユーザーモデル（データベース用）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// SQLite行からUserを構築
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, AppError> {
        let id_str: String = row.get("id");
        let id = Uuid::parse_str(&id_str)
            .map_err(|_| AppError::Internal("Invalid user ID".to_string()))?;

        let created_at_str: String = row.get("created_at");
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_| AppError::Internal("Invalid date format".to_string()))?
            .with_timezone(&Utc);

        Ok(Self {
            id,
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at,
        })
    }
}

/// ツイートモデル（データベース用）
#[derive(Debug, Clone)]
pub struct Tweet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Tweet {
    pub fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, AppError> {
        let id_str: String = row.get("id");
        let id = Uuid::parse_str(&id_str)
            .map_err(|_| AppError::Internal("Invalid tweet ID".to_string()))?;

        let user_id_str: String = row.get("user_id");
        let user_id = Uuid::parse_str(&user_id_str)
            .map_err(|_| AppError::Internal("Invalid user ID".to_string()))?;

        let created_at_str: String = row.get("created_at");
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_| AppError::Internal("Invalid date format".to_string()))?
            .with_timezone(&Utc);

        Ok(Self {
            id,
            user_id,
            content: row.get("content"),
            created_at,
        })
    }
}

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

impl From<Tweet> for TweetResponse {
    fn from(tweet: Tweet) -> Self {
        Self {
            id: tweet.id,
            user_id: tweet.user_id,
            content: tweet.content,
            created_at: tweet.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}
