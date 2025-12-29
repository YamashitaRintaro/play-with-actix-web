use crate::entities::{tweet, user};
use crate::error::AppError;
use crate::models::*;
use crate::store::Db;
use crate::utils::{authenticate, create_jwt, hash_password, verify_password};
use actix_web::{HttpRequest, HttpResponse, web};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

type Result<T> = std::result::Result<T, AppError>;

/// ユーザー登録
pub async fn register(db: web::Data<Db>, req: web::Json<RegisterRequest>) -> Result<HttpResponse> {
    // メールアドレスの重複チェック
    let existing = user::Entity::find()
        .filter(user::Column::Email.eq(&req.email))
        .one(db.as_ref())
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }

    let password_hash = hash_password(&req.password)?;
    let user_id = Uuid::new_v4();
    let created_at = Utc::now();

    // SeaORMでユーザーを作成
    let new_user = user::ActiveModel {
        id: Set(user_id),
        username: Set(req.username.clone()),
        email: Set(req.email.clone()),
        password_hash: Set(password_hash),
        created_at: Set(created_at.to_rfc3339()),
        ..Default::default()
    };

    new_user.insert(db.as_ref()).await?;

    let token = create_jwt(user_id)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            username: req.username.clone(),
            email: req.email.clone(),
        },
    }))
}

/// ログイン
pub async fn login(db: web::Data<Db>, req: web::Json<LoginRequest>) -> Result<HttpResponse> {
    let user_model = user::Entity::find()
        .filter(user::Column::Email.eq(&req.email))
        .one(db.as_ref())
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    if !verify_password(&req.password, &user_model.password_hash)? {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    let token = create_jwt(user_model.id)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse::from(user_model),
    }))
}

/// ツイート投稿
pub async fn create_tweet(
    req_http: HttpRequest,
    db: web::Data<Db>,
    req: web::Json<CreateTweetRequest>,
) -> Result<HttpResponse> {
    let user_id = authenticate(&req_http)?;

    if req.content.is_empty() || req.content.len() > 280 {
        return Err(AppError::BadRequest(
            "Tweet content must be between 1 and 280 characters".to_string(),
        ));
    }

    let tweet_id = Uuid::new_v4();
    let created_at = Utc::now();

    // SeaORMでツイートを作成
    let new_tweet = tweet::ActiveModel {
        id: Set(tweet_id),
        user_id: Set(user_id),
        content: Set(req.content.clone()),
        created_at: Set(created_at.to_rfc3339()),
        ..Default::default()
    };

    new_tweet.insert(db.as_ref()).await?;

    Ok(HttpResponse::Created().json(TweetResponse {
        id: tweet_id,
        user_id,
        content: req.content.clone(),
        created_at,
    }))
}

/// ツイート取得
pub async fn get_tweet(db: web::Data<Db>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let tweet_model = tweet::Entity::find_by_id(*path)
        .one(db.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Tweet not found".to_string()))?;

    Ok(HttpResponse::Ok().json(TweetResponse::from(tweet_model)))
}

/// タイムライン取得
pub async fn get_timeline(req_http: HttpRequest, db: web::Data<Db>) -> Result<HttpResponse> {
    let user_id = authenticate(&req_http)?;

    let user = user::Entity::find_by_id(user_id)
        .one(db.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let tweets = user
        .find_related(tweet::Entity)
        .order_by_desc(tweet::Column::CreatedAt)
        .all(db.as_ref())
        .await?;

    let timeline: Vec<TweetResponse> = tweets.into_iter().map(TweetResponse::from).collect();

    Ok(HttpResponse::Ok().json(timeline))
}
