use async_graphql::{Context, InputObject, Object, Result};
use chrono::Utc;
use uuid::Uuid;

use crate::graphql::query::{TweetType, UserType};
use crate::models::{Tweet, User};
use crate::store::Db;
use crate::utils::{create_jwt, hash_password, verify_password};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthPayload> {
        let db = ctx.data::<Db>()?;

        // メールアドレスの重複チェック
        let existing: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = ?")
            .bind(&input.email)
            .fetch_optional(db)
            .await?;

        if existing.is_some() {
            return Err(async_graphql::Error::new("Email already exists"));
        }

        let password_hash =
            hash_password(&input.password).map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let user_id = Uuid::new_v4();
        let created_at = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(user_id.to_string())
        .bind(&input.username)
        .bind(&input.email)
        .bind(&password_hash)
        .bind(&created_at)
        .execute(db)
        .await?;

        let user = User {
            id: user_id.to_string(),
            username: input.username.clone(),
            email: input.email.clone(),
            password_hash,
            created_at,
        };

        let token = create_jwt(user_id).map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(AuthPayload {
            token,
            user: UserType::from(user),
        })
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let db = ctx.data::<Db>()?;

        let user: User = sqlx::query_as("SELECT * FROM users WHERE email = ?")
            .bind(&input.email)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Invalid email or password"))?;

        let valid = verify_password(&input.password, &user.password_hash)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if !valid {
            return Err(async_graphql::Error::new("Invalid email or password"));
        }

        let user_id =
            Uuid::parse_str(&user.id).map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let token = create_jwt(user_id).map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(AuthPayload {
            token,
            user: UserType::from(user),
        })
    }

    async fn create_tweet(&self, ctx: &Context<'_>, content: String) -> Result<TweetType> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        if content.is_empty() || content.len() > 280 {
            return Err(async_graphql::Error::new(
                "Tweet content must be between 1 and 280 characters",
            ));
        }

        let tweet_id = Uuid::new_v4();
        let created_at = Utc::now().to_rfc3339();

        sqlx::query("INSERT INTO tweets (id, user_id, content, created_at) VALUES (?, ?, ?, ?)")
            .bind(tweet_id.to_string())
            .bind(user_id.to_string())
            .bind(&content)
            .bind(&created_at)
            .execute(db)
            .await?;

        Ok(TweetType {
            id: tweet_id,
            user_id: *user_id,
            content,
            created_at,
            like_count: 0,
            is_liked: false,
        })
    }

    async fn delete_tweet(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        let tweet: Tweet = sqlx::query_as("SELECT * FROM tweets WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Tweet not found"))?;

        let tweet_user_id = Uuid::parse_str(&tweet.user_id)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        if tweet_user_id != *user_id {
            return Err(async_graphql::Error::new("Not authorized"));
        }

        sqlx::query("DELETE FROM tweets WHERE id = ?")
            .bind(id.to_string())
            .execute(db)
            .await?;

        Ok(true)
    }

    async fn like_tweet(&self, ctx: &Context<'_>, tweet_id: Uuid) -> Result<bool> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        // 既にいいね済みかチェック（存在確認のみなので1を選択）
        let already_liked: Option<(i32,)> =
            sqlx::query_as("SELECT 1 FROM likes WHERE tweet_id = ? AND user_id = ?")
                .bind(tweet_id.to_string())
                .bind(user_id.to_string())
                .fetch_optional(db)
                .await?;

        if already_liked.is_some() {
            return Err(async_graphql::Error::new("Already liked"));
        }

        // ツイートが存在するか確認（存在確認のみなので1を選択）
        let tweet_exists: Option<(i32,)> = sqlx::query_as("SELECT 1 FROM tweets WHERE id = ?")
            .bind(tweet_id.to_string())
            .fetch_optional(db)
            .await?;

        if tweet_exists.is_none() {
            return Err(async_graphql::Error::new("Tweet not found"));
        }

        // いいねを作成
        let created_at = Utc::now().to_rfc3339();
        sqlx::query("INSERT INTO likes (user_id, tweet_id, created_at) VALUES (?, ?, ?)")
            .bind(user_id.to_string())
            .bind(tweet_id.to_string())
            .bind(&created_at)
            .execute(db)
            .await?;

        Ok(true)
    }

    async fn unlike_tweet(&self, ctx: &Context<'_>, tweet_id: Uuid) -> Result<bool> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        // いいねが存在するかチェック（存在確認のみなので1を選択）
        let like_exists: Option<(i32,)> =
            sqlx::query_as("SELECT 1 FROM likes WHERE tweet_id = ? AND user_id = ?")
                .bind(tweet_id.to_string())
                .bind(user_id.to_string())
                .fetch_optional(db)
                .await?;

        if like_exists.is_none() {
            return Err(async_graphql::Error::new("Like not found"));
        }

        sqlx::query("DELETE FROM likes WHERE tweet_id = ? AND user_id = ?")
            .bind(tweet_id.to_string())
            .bind(user_id.to_string())
            .execute(db)
            .await?;

        Ok(true)
    }
}

/// 登録入力
#[derive(InputObject)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// ログイン入力
#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

/// 認証レスポンス
pub struct AuthPayload {
    pub token: String,
    pub user: UserType,
}

#[Object]
impl AuthPayload {
    async fn token(&self) -> &str {
        &self.token
    }

    async fn user(&self) -> UserType {
        self.user.clone()
    }
}
