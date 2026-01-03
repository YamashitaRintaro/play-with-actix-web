use async_graphql::{Context, Object, Result};
use std::collections::HashSet;
use uuid::Uuid;

use crate::models::{Comment, HashtagName, LikeTweetId, Tweet, User};
use crate::store::Db;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 現在のユーザーのタイムラインを取得（自分 + フォロー中のユーザーのツイート）
    async fn timeline(&self, ctx: &Context<'_>) -> Result<Vec<TweetType>> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        // 自分とフォロー中のユーザーのツイートを取得
        let tweets: Vec<Tweet> = sqlx::query_as(
            r#"
            SELECT t.* FROM tweets t
            WHERE t.user_id = ?
               OR t.user_id IN (SELECT following_id FROM follows WHERE follower_id = ?)
            ORDER BY t.created_at DESC
            "#,
        )
        .bind(user_id)
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

        // 各ツイートのハッシュタグを取得
        let hashtags_query = format!(
            r#"
            SELECT th.tweet_id, h.name 
            FROM tweet_hashtags th 
            JOIN hashtags h ON th.hashtag_id = h.id 
            WHERE th.tweet_id IN ({})
            "#,
            placeholders
        );
        let mut query = sqlx::query_as::<_, (Uuid, String)>(&hashtags_query);
        for id in &tweet_ids {
            query = query.bind(id);
        }
        let tweet_hashtags: Vec<(Uuid, String)> = query.fetch_all(db).await?;

        // ツイートIDごとにハッシュタグをグループ化
        let mut hashtag_map: std::collections::HashMap<Uuid, Vec<String>> =
            std::collections::HashMap::new();
        for (tweet_id, tag_name) in tweet_hashtags {
            hashtag_map.entry(tweet_id).or_default().push(tag_name);
        }

        let result: Vec<TweetType> = tweets
            .into_iter()
            .map(|tweet| {
                let like_count = *like_count_map.get(&tweet.id).unwrap_or(&0);
                let is_liked = liked_tweet_ids.contains(&tweet.id);
                let hashtags = hashtag_map.remove(&tweet.id).unwrap_or_default();
                TweetType::from_tweet(tweet, like_count, is_liked, hashtags)
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

            // ハッシュタグを取得
            let hashtags: Vec<HashtagName> = sqlx::query_as(
                r#"
                SELECT h.name 
                FROM tweet_hashtags th 
                JOIN hashtags h ON th.hashtag_id = h.id 
                WHERE th.tweet_id = ?
                "#,
            )
            .bind(tweet.id)
            .fetch_all(db)
            .await?;
            let hashtag_names: Vec<String> = hashtags.into_iter().map(|h| h.name).collect();

            Ok(Some(TweetType::from_tweet(
                tweet,
                like_count,
                is_liked,
                hashtag_names,
            )))
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

    /// ツイートへのコメント一覧を取得
    async fn comments(&self, ctx: &Context<'_>, tweet_id: Uuid) -> Result<Vec<CommentType>> {
        let db = ctx.data::<Db>()?;

        let comments: Vec<Comment> =
            sqlx::query_as("SELECT * FROM comments WHERE tweet_id = ? ORDER BY created_at ASC")
                .bind(tweet_id)
                .fetch_all(db)
                .await?;

        Ok(comments.into_iter().map(CommentType::from).collect())
    }

    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<UserType>> {
        let db = ctx.data::<Db>()?;
        let current_user_id = ctx.data::<Uuid>().ok();

        let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await?;

        if let Some(user) = user {
            // フォロワー数を取得
            let (followers_count,): (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM follows WHERE following_id = ?")
                    .bind(id)
                    .fetch_one(db)
                    .await?;

            // フォロー中の数を取得
            let (following_count,): (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM follows WHERE follower_id = ?")
                    .bind(id)
                    .fetch_one(db)
                    .await?;

            // 現在のユーザーがこのユーザーをフォローしているか
            let is_following = if let Some(current_id) = current_user_id {
                if *current_id == id {
                    false // 自分自身の場合はフォロー不可
                } else {
                    let exists: Option<(i32,)> = sqlx::query_as(
                        "SELECT 1 FROM follows WHERE follower_id = ? AND following_id = ?",
                    )
                    .bind(current_id)
                    .bind(id)
                    .fetch_optional(db)
                    .await?;
                    exists.is_some()
                }
            } else {
                false
            };

            Ok(Some(UserType {
                id: user.id,
                username: user.username,
                email: user.email,
                followers_count,
                following_count,
                is_following,
            }))
        } else {
            Ok(None)
        }
    }

    /// ユーザーのフォロワー一覧を取得
    async fn followers(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<Vec<UserType>> {
        let db = ctx.data::<Db>()?;
        let current_user_id = ctx.data::<Uuid>().ok();

        // フォロワーのユーザー情報を取得
        let followers: Vec<User> = sqlx::query_as(
            r#"
            SELECT u.* FROM users u
            JOIN follows f ON u.id = f.follower_id
            WHERE f.following_id = ?
            ORDER BY f.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(db)
        .await?;

        let mut result = Vec::new();
        for follower in followers {
            let (followers_count,): (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM follows WHERE following_id = ?")
                    .bind(follower.id)
                    .fetch_one(db)
                    .await?;

            let (following_count,): (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM follows WHERE follower_id = ?")
                    .bind(follower.id)
                    .fetch_one(db)
                    .await?;

            let is_following = if let Some(current_id) = current_user_id {
                if *current_id == follower.id {
                    false
                } else {
                    let exists: Option<(i32,)> = sqlx::query_as(
                        "SELECT 1 FROM follows WHERE follower_id = ? AND following_id = ?",
                    )
                    .bind(current_id)
                    .bind(follower.id)
                    .fetch_optional(db)
                    .await?;
                    exists.is_some()
                }
            } else {
                false
            };

            result.push(UserType {
                id: follower.id,
                username: follower.username,
                email: follower.email,
                followers_count,
                following_count,
                is_following,
            });
        }

        Ok(result)
    }

    /// ユーザーがフォローしているユーザー一覧を取得
    async fn following(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<Vec<UserType>> {
        let db = ctx.data::<Db>()?;
        let current_user_id = ctx.data::<Uuid>().ok();

        // フォロー中のユーザー情報を取得
        let following: Vec<User> = sqlx::query_as(
            r#"
            SELECT u.* FROM users u
            JOIN follows f ON u.id = f.following_id
            WHERE f.follower_id = ?
            ORDER BY f.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(db)
        .await?;

        let mut result = Vec::new();
        for user in following {
            let (followers_count,): (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM follows WHERE following_id = ?")
                    .bind(user.id)
                    .fetch_one(db)
                    .await?;

            let (following_count,): (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM follows WHERE follower_id = ?")
                    .bind(user.id)
                    .fetch_one(db)
                    .await?;

            let is_following = if let Some(current_id) = current_user_id {
                if *current_id == user.id {
                    false
                } else {
                    let exists: Option<(i32,)> = sqlx::query_as(
                        "SELECT 1 FROM follows WHERE follower_id = ? AND following_id = ?",
                    )
                    .bind(current_id)
                    .bind(user.id)
                    .fetch_optional(db)
                    .await?;
                    exists.is_some()
                }
            } else {
                false
            };

            result.push(UserType {
                id: user.id,
                username: user.username,
                email: user.email,
                followers_count,
                following_count,
                is_following,
            });
        }

        Ok(result)
    }
}

/// GraphQL用のユーザー型
#[derive(Clone)]
pub struct UserType {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub is_following: bool,
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

    async fn followers_count(&self) -> i64 {
        self.followers_count
    }

    async fn following_count(&self) -> i64 {
        self.following_count
    }

    async fn is_following(&self) -> bool {
        self.is_following
    }
}

// UserからUserTypeへの変換（認証レスポンス用のシンプルな変換）
impl From<User> for UserType {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            followers_count: 0,
            following_count: 0,
            is_following: false,
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
    pub hashtags: Vec<String>,
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

    async fn hashtags(&self) -> &[String] {
        &self.hashtags
    }

    /// ツイート投稿者の情報を取得
    async fn user(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let db = ctx.data::<Db>()?;

        let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(self.user_id)
            .fetch_optional(db)
            .await?;

        Ok(user.map(UserType::from))
    }
}

impl TweetType {
    pub fn from_tweet(
        tweet: Tweet,
        like_count: i64,
        is_liked: bool,
        hashtags: Vec<String>,
    ) -> Self {
        Self {
            id: tweet.id,
            user_id: tweet.user_id,
            content: tweet.content,
            created_at: tweet.created_at,
            like_count,
            is_liked,
            hashtags,
        }
    }
}

impl From<Tweet> for TweetType {
    fn from(tweet: Tweet) -> Self {
        Self::from_tweet(tweet, 0, false, Vec::new())
    }
}

/// GraphQL用のコメント型
#[derive(Clone)]
pub struct CommentType {
    pub id: Uuid,
    pub tweet_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: String,
}

#[Object]
impl CommentType {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn tweet_id(&self) -> Uuid {
        self.tweet_id
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

    /// コメント投稿者の情報を取得
    async fn user(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let db = ctx.data::<Db>()?;
        let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(self.user_id)
            .fetch_optional(db)
            .await?;
        Ok(user.map(UserType::from))
    }
}

impl From<Comment> for CommentType {
    fn from(comment: Comment) -> Self {
        Self {
            id: comment.id,
            tweet_id: comment.tweet_id,
            user_id: comment.user_id,
            content: comment.content,
            created_at: comment.created_at,
        }
    }
}
