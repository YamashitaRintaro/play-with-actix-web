use async_graphql::{Context, InputObject, Object, Result};
use chrono::Utc;
use uuid::Uuid;

use crate::graphql::query::{CommentType, TweetType, UserType};
use crate::models::User;
use crate::store::Db;
use crate::utils::{create_jwt, extract_hashtags, hash_password, verify_password};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthPayload> {
        let db = ctx.data::<Db>()?;

        let exists: Option<(i32,)> = sqlx::query_as("SELECT 1 FROM users WHERE email = ?")
            .bind(&input.email)
            .fetch_optional(db)
            .await?;

        if exists.is_some() {
            return Err(async_graphql::Error::new("Email already exists"));
        }

        let password_hash =
            hash_password(&input.password).map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let user_id = Uuid::new_v4();
        let created_at = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(user_id)
        .bind(&input.username)
        .bind(&input.email)
        .bind(&password_hash)
        .bind(&created_at)
        .execute(db)
        .await?;

        let user = User {
            id: user_id,
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

        let token = create_jwt(user.id).map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(AuthPayload {
            token,
            user: UserType::from(user),
        })
    }

    async fn create_tweet(&self, ctx: &Context<'_>, content: String) -> Result<TweetType> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        if content.is_empty() || content.len() > 280 {
            return Err("Tweet content must be between 1 and 280 characters".into());
        }

        let tweet_id = Uuid::new_v4();
        let created_at = Utc::now().to_rfc3339();

        sqlx::query("INSERT INTO tweets (id, user_id, content, created_at) VALUES (?, ?, ?, ?)")
            .bind(tweet_id)
            .bind(user_id)
            .bind(&content)
            .bind(&created_at)
            .execute(db)
            .await?;

        let hashtag_names = extract_hashtags(&content);
        for tag_name in &hashtag_names {
            sqlx::query("INSERT OR IGNORE INTO hashtags (id, name) VALUES (?, ?)")
                .bind(Uuid::new_v4())
                .bind(tag_name)
                .execute(db)
                .await?;

            let (hashtag_id,): (Uuid,) = sqlx::query_as("SELECT id FROM hashtags WHERE name = ?")
                .bind(tag_name)
                .fetch_one(db)
                .await?;

            sqlx::query("INSERT INTO tweet_hashtags (tweet_id, hashtag_id) VALUES (?, ?)")
                .bind(tweet_id)
                .bind(hashtag_id)
                .execute(db)
                .await?;
        }

        Ok(TweetType {
            id: tweet_id,
            user_id: *user_id,
            content,
            created_at,
            like_count: 0,
            is_liked: false,
            hashtags: hashtag_names,
        })
    }

    async fn delete_tweet(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        let result = sqlx::query("DELETE FROM tweets WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err("Tweet not found or not authorized".into());
        }

        Ok(true)
    }

    async fn like_tweet(&self, ctx: &Context<'_>, tweet_id: Uuid) -> Result<bool> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM tweets WHERE id = ?")
            .bind(tweet_id)
            .fetch_optional(db)
            .await?
            .ok_or("Tweet not found")?;

        let result = sqlx::query(
            "INSERT OR IGNORE INTO likes (user_id, tweet_id, created_at) VALUES (?, ?, ?)",
        )
        .bind(user_id)
        .bind(tweet_id)
        .bind(Utc::now().to_rfc3339())
        .execute(db)
        .await?;

        if result.rows_affected() == 0 {
            return Err("Already liked".into());
        }

        Ok(true)
    }

    async fn unlike_tweet(&self, ctx: &Context<'_>, tweet_id: Uuid) -> Result<bool> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        let result = sqlx::query("DELETE FROM likes WHERE tweet_id = ? AND user_id = ?")
            .bind(tweet_id)
            .bind(user_id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err("Like not found".into());
        }

        Ok(true)
    }

    async fn create_comment(
        &self,
        ctx: &Context<'_>,
        tweet_id: Uuid,
        content: String,
    ) -> Result<CommentType> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        if content.is_empty() || content.len() > 280 {
            return Err("Comment content must be between 1 and 280 characters".into());
        }

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM tweets WHERE id = ?")
            .bind(tweet_id)
            .fetch_optional(db)
            .await?
            .ok_or("Tweet not found")?;

        let comment_id = Uuid::new_v4();
        let created_at = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO comments (id, tweet_id, user_id, content, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(comment_id)
        .bind(tweet_id)
        .bind(user_id)
        .bind(&content)
        .bind(&created_at)
        .execute(db)
        .await?;

        Ok(CommentType {
            id: comment_id,
            tweet_id,
            user_id: *user_id,
            content,
            created_at,
        })
    }

    async fn delete_comment(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        let result = sqlx::query("DELETE FROM comments WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err("Comment not found or not authorized".into());
        }

        Ok(true)
    }

    async fn follow_user(&self, ctx: &Context<'_>, target_id: Uuid) -> Result<Uuid> {
        let db = ctx.data::<Db>()?;
        let current_user_id = ctx.data::<Uuid>()?;

        if *current_user_id == target_id {
            return Err("Cannot follow yourself".into());
        }

        sqlx::query_as::<_, (i32,)>("SELECT 1 FROM users WHERE id = ?")
            .bind(target_id)
            .fetch_optional(db)
            .await?
            .ok_or("User not found")?;

        let result = sqlx::query(
            "INSERT OR IGNORE INTO follows (follower_id, following_id, created_at) VALUES (?, ?, ?)",
        )
        .bind(current_user_id)
        .bind(target_id)
        .bind(Utc::now().to_rfc3339())
        .execute(db)
        .await?;

        if result.rows_affected() == 0 {
            return Err("Already following this user".into());
        }

        Ok(target_id)
    }

    async fn unfollow_user(&self, ctx: &Context<'_>, target_id: Uuid) -> Result<Uuid> {
        let db = ctx.data::<Db>()?;
        let current_user_id = ctx.data::<Uuid>()?;

        let result = sqlx::query("DELETE FROM follows WHERE follower_id = ? AND following_id = ?")
            .bind(current_user_id)
            .bind(target_id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err("Not following this user".into());
        }

        Ok(target_id)
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
