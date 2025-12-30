use async_graphql::{Context, InputObject, Object, Result};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entities::{tweet, user};
use crate::graphql::query::{TweetType, UserType};
use crate::store::Db;
use crate::utils::{create_jwt, hash_password, verify_password};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// ユーザー登録
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthPayload> {
        let db = ctx.data::<Db>()?;

        // メールアドレスの重複チェック
        let existing = user::Entity::find()
            .filter(user::Column::Email.eq(&input.email))
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(async_graphql::Error::new("Email already exists"));
        }

        let password_hash =
            hash_password(&input.password).map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let user_id = Uuid::new_v4();
        let created_at = Utc::now();

        let new_user = user::ActiveModel {
            id: Set(user_id),
            username: Set(input.username.clone()),
            email: Set(input.email.clone()),
            password_hash: Set(password_hash),
            created_at: Set(created_at.to_rfc3339()),
        };

        new_user.insert(db).await?;

        let token =
            create_jwt(user_id).map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(AuthPayload {
            token,
            user: UserType {
                id: user_id,
                username: input.username,
                email: input.email,
            },
        })
    }

    /// ログイン
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let db = ctx.data::<Db>()?;

        let user_model = user::Entity::find()
            .filter(user::Column::Email.eq(&input.email))
            .one(db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Invalid email or password"))?;

        let valid = verify_password(&input.password, &user_model.password_hash)
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if !valid {
            return Err(async_graphql::Error::new("Invalid email or password"));
        }

        let token =
            create_jwt(user_model.id).map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(AuthPayload {
            token,
            user: UserType::from(user_model),
        })
    }

    /// ツイート作成
    async fn create_tweet(&self, ctx: &Context<'_>, content: String) -> Result<TweetType> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        if content.is_empty() || content.len() > 280 {
            return Err(async_graphql::Error::new(
                "Tweet content must be between 1 and 280 characters",
            ));
        }

        let tweet_id = Uuid::new_v4();
        let created_at = Utc::now();

        let new_tweet = tweet::ActiveModel {
            id: Set(tweet_id),
            user_id: Set(*user_id),
            content: Set(content.clone()),
            created_at: Set(created_at.to_rfc3339()),
        };

        new_tweet.insert(db).await?;

        Ok(TweetType {
            id: tweet_id,
            user_id: *user_id,
            content,
            created_at: created_at.to_rfc3339(),
        })
    }

    /// ツイート削除
    async fn delete_tweet(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let db = ctx.data::<Db>()?;
        let user_id = ctx.data::<Uuid>()?;

        let tweet_model = tweet::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Tweet not found"))?;

        if tweet_model.user_id != *user_id {
            return Err(async_graphql::Error::new("Not authorized"));
        }

        tweet_model.delete(db).await?;

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

