use async_graphql::{Context, Object, Result};
use sea_orm::{EntityTrait, ModelTrait, QueryOrder};
use uuid::Uuid;

use crate::entities::{tweet, user};
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

        let tweets = user
            .find_related(tweet::Entity)
            .order_by_desc(tweet::Column::CreatedAt)
            .all(db)
            .await?;

        Ok(tweets.into_iter().map(TweetType::from).collect())
    }

    /// 特定のツイートを取得
    async fn tweet(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<TweetType>> {
        let db = ctx.data::<Db>()?;

        let tweet = tweet::Entity::find_by_id(id).one(db).await?;

        Ok(tweet.map(TweetType::from))
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
}

impl From<tweet::Model> for TweetType {
    fn from(model: tweet::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            content: model.content,
            created_at: model.created_at,
        }
    }
}

