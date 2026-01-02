use async_graphql::{Context, Object, Result};
use std::collections::HashSet;
use uuid::Uuid;

use crate::models::{LikeTweetId, Tweet, User};
use crate::store::Db;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 現在のユーザーのタイムラインを取得
    async fn timeline(&self, ctx: &Context<'_>) -> Result<Vec<TweetType>> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        // ユーザーのツイートを取得
        let tweets: Vec<Tweet> =
            sqlx::query_as("SELECT * FROM tweets WHERE user_id = ? ORDER BY created_at DESC")
                .bind(user_id)
                .fetch_all(db)
                .await?;

        if tweets.is_empty() {
            return Ok(Vec::new());
        }

        let tweet_ids: Vec<Uuid> = tweets.iter().map(|t| t.id).collect();

        // SQLiteでIN句を使うため、プレースホルダを動的に生成
        let placeholders = tweet_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let like_count_query = format!(
            "SELECT tweet_id, COUNT(*) as count FROM likes WHERE tweet_id IN ({}) GROUP BY tweet_id",
            placeholders
        );

        let mut query = sqlx::query_as::<_, (Uuid, i64)>(&like_count_query);
        for id in &tweet_ids {
            query = query.bind(id);
        }
        let like_counts: Vec<(Uuid, i64)> = query.fetch_all(db).await?;
        let like_count_map: std::collections::HashMap<Uuid, i64> =
            like_counts.into_iter().collect();

        // 現在のユーザーがいいねしたツイートを取得
        let user_likes_query = format!(
            "SELECT tweet_id FROM likes WHERE tweet_id IN ({}) AND user_id = ?",
            placeholders
        );
        let mut query = sqlx::query_as::<_, LikeTweetId>(&user_likes_query);
        for id in &tweet_ids {
            query = query.bind(id);
        }
        query = query.bind(user_id);
        let user_likes: Vec<LikeTweetId> = query.fetch_all(db).await?;
        let liked_tweet_ids: HashSet<Uuid> = user_likes.into_iter().map(|l| l.tweet_id).collect();

        let result: Vec<TweetType> = tweets
            .into_iter()
            .map(|tweet| {
                let like_count = *like_count_map.get(&tweet.id).unwrap_or(&0);
                let is_liked = liked_tweet_ids.contains(&tweet.id);
                TweetType::from_tweet(tweet, like_count, is_liked)
            })
            .collect();

        Ok(result)
    }

    async fn tweet(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<TweetType>> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>().ok();

        let tweet: Option<Tweet> = sqlx::query_as("SELECT * FROM tweets WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?;

        if let Some(tweet) = tweet {
            // いいね数を取得
            let (like_count,): (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM likes WHERE tweet_id = ?")
                    .bind(tweet.id)
                    .fetch_one(db)
                    .await?;

            let is_liked = if let Some(uid) = user_id {
                let exists: Option<(i32,)> =
                    sqlx::query_as("SELECT 1 FROM likes WHERE tweet_id = ? AND user_id = ?")
                        .bind(tweet.id)
                        .bind(uid)
                        .fetch_optional(db)
                        .await?;
                exists.is_some()
            } else {
                false
            };

            Ok(Some(TweetType::from_tweet(tweet, like_count, is_liked)))
        } else {
            Ok(None)
        }
    }

    /// 現在のユーザー情報を取得
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>().ok();

        if let Some(id) = user_id {
            let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_optional(db)
                .await?;
            Ok(user.map(UserType::from))
        } else {
            Ok(None)
        }
    }
}

/// GraphQL用のユーザー型
#[derive(Clone)]
pub struct UserType {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[Object]
impl UserType {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn email(&self) -> &str {
        &self.email
    }
}

// UserからUserTypeへの変換（Uuid::parse_str不要）
impl From<User> for UserType {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

/// GraphQL用のツイート型
#[derive(Clone)]
pub struct TweetType {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: String,
    pub like_count: i64,
    pub is_liked: bool,
}

#[Object]
impl TweetType {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn user_id(&self) -> Uuid {
        self.user_id
    }

    async fn content(&self) -> &str {
        &self.content
    }

    async fn created_at(&self) -> &str {
        &self.created_at
    }

    async fn like_count(&self) -> i64 {
        self.like_count
    }

    async fn is_liked(&self) -> bool {
        self.is_liked
    }
}

impl TweetType {
    // TweetからTweetTypeへの変換（Uuid::parse_str不要）
    pub fn from_tweet(tweet: Tweet, like_count: i64, is_liked: bool) -> Self {
        Self {
            id: tweet.id,
            user_id: tweet.user_id,
            content: tweet.content,
            created_at: tweet.created_at,
            like_count,
            is_liked,
        }
    }
}

impl From<Tweet> for TweetType {
    fn from(tweet: Tweet) -> Self {
        Self::from_tweet(tweet, 0, false)
    }
}
