use async_graphql::{Context, Object, Result};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::entities::{like, tweet, user};
use crate::store::Db;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 現在のユーザーのタイムラインを取得
    async fn timeline(&self, ctx: &Context<'_>) -> Result<Vec<TweetType>> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        let user = user::Entity::find_by_id(*user_id)
            .one(db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("User not found"))?;

        let tweets_with_likes = user
            .find_related(tweet::Entity)
            .order_by_desc(tweet::Column::CreatedAt)
            .find_with_related(like::Entity)
            .all(db)
            .await?;

        if tweets_with_likes.is_empty() {
            return Ok(Vec::new());
        }

        // ツイートIDを収集
        let tweet_ids: Vec<Uuid> = tweets_with_likes
            .iter()
            .map(|(tweet, _)| tweet.id)
            .collect();

        // いいね数を集計
        let mut like_counts: HashMap<Uuid, i64> = HashMap::new();
        for (tweet, likes) in &tweets_with_likes {
            like_counts.insert(tweet.id, likes.len() as i64);
        }

        // 現在のユーザーがいいねしたツイートを取得
        let user_likes = like::Entity::find()
            .filter(like::Column::TweetId.is_in(tweet_ids))
            .filter(like::Column::UserId.eq(*user_id))
            .all(db)
            .await?;

        let liked_tweet_ids: HashSet<Uuid> = user_likes.iter().map(|l| l.tweet_id).collect();

        let result: Vec<TweetType> = tweets_with_likes
            .into_iter()
            .map(|(tweet, _)| TweetType {
                id: tweet.id,
                user_id: tweet.user_id,
                content: tweet.content,
                created_at: tweet.created_at,
                like_count: *like_counts.get(&tweet.id).unwrap_or(&0),
                is_liked: liked_tweet_ids.contains(&tweet.id),
            })
            .collect();

        Ok(result)
    }

    /// 特定のツイートを取得
    async fn tweet(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<TweetType>> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>().ok();

        let tweet_with_likes = tweet::Entity::find_by_id(id)
            .find_with_related(like::Entity)
            .all(db)
            .await?;

        if let Some((tweet_model, likes)) = tweet_with_likes.first() {
            let like_count = likes.len() as i64;

            let is_liked = if let Some(uid) = user_id {
                likes.iter().any(|l| l.user_id == *uid)
            } else {
                false
            };

            Ok(Some(TweetType {
                id: tweet_model.id,
                user_id: tweet_model.user_id,
                content: tweet_model.content.clone(),
                created_at: tweet_model.created_at.clone(),
                like_count,
                is_liked,
            }))
        } else {
            Ok(None)
        }
    }

    /// 現在のユーザー情報を取得
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>().ok();

        if let Some(id) = user_id {
            let user = user::Entity::find_by_id(*id).one(db).await?;
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

impl From<user::Model> for UserType {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
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

impl From<tweet::Model> for TweetType {
    fn from(model: tweet::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            content: model.content,
            created_at: model.created_at,
            like_count: 0,
            is_liked: false,
        }
    }
}
