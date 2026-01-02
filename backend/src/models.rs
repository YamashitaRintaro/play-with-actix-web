use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// データベースから取得するための構造体（sqlx::FromRow）
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: String, // SQLiteではTEXTで保存
    pub username: String,
    pub email: String,
    pub password_hash: String,
    #[allow(dead_code)] // SELECT * で取得するが、UserResponseでは使用しない
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Tweet {
    pub id: String, // SQLiteではTEXTで保存
    pub user_id: String,
    pub content: String,
    pub created_at: String,
}

/// いいねテーブルの全カラム（将来の拡張用に保持）
#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
pub struct Like {
    pub user_id: String,
    pub tweet_id: String,
    pub created_at: String,
}

/// いいね済みツイートIDのみ取得用
#[derive(Debug, Clone, FromRow)]
pub struct LikeTweetId {
    pub tweet_id: String,
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

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

// UserからUserResponseへの変換
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: Uuid::parse_str(&user.id).expect("Invalid UUID"),
            username: user.username,
            email: user.email,
        }
    }
}

// TweetからTweetResponseへの変換
impl From<Tweet> for TweetResponse {
    fn from(tweet: Tweet) -> Self {
        Self {
            id: Uuid::parse_str(&tweet.id).expect("Invalid UUID"),
            user_id: Uuid::parse_str(&tweet.user_id).expect("Invalid UUID"),
            content: tweet.content,
            created_at: DateTime::parse_from_rfc3339(&tweet.created_at)
                .expect("Invalid date format")
                .with_timezone(&Utc),
        }
    }
}
