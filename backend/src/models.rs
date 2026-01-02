use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    #[allow(dead_code)]
    pub created_at: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Tweet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: String,
}

/// いいね済みツイートIDのみ取得用
#[derive(Debug, Clone, FromRow)]
pub struct LikeTweetId {
    pub tweet_id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
pub struct HashtagName {
    pub name: String,
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

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

impl From<Tweet> for TweetResponse {
    fn from(tweet: Tweet) -> Self {
        Self {
            id: tweet.id,
            user_id: tweet.user_id,
            content: tweet.content,
            created_at: DateTime::parse_from_rfc3339(&tweet.created_at)
                .expect("Invalid date format")
                .with_timezone(&Utc),
        }
    }
}
