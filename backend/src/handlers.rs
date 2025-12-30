use crate::entities::{tweet, user};
use crate::error::AppError;
use crate::models::*;
use crate::store::Db;
use crate::utils::{authenticate, create_jwt, hash_password, verify_password};
use actix_web::{HttpRequest, HttpResponse, web};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set};
use uuid::Uuid;

type Result<T> = std::result::Result<T, AppError>;

// ビジネスロジック層

/// ユーザー登録のビジネスロジック
async fn register_user(
    db: &Db,
    username: &str,
    email: &str,
    password: &str,
) -> Result<(Uuid, String)> {
    // メールアドレスの重複チェック
    let existing = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }

    let password_hash = hash_password(password)?;
    let user_id = Uuid::new_v4();
    let created_at = Utc::now();

    let new_user = user::ActiveModel {
        id: Set(user_id),
        username: Set(username.to_string()),
        email: Set(email.to_string()),
        password_hash: Set(password_hash),
        created_at: Set(created_at.to_rfc3339()),
    };

    new_user.insert(db).await?;

    let token = create_jwt(user_id)?;

    Ok((user_id, token))
}

/// ログインのビジネスロジック
async fn login_user(db: &Db, email: &str, password: &str) -> Result<(user::Model, String)> {
    let user_model = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    if !verify_password(password, &user_model.password_hash)? {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    let token = create_jwt(user_model.id)?;

    Ok((user_model, token))
}

/// ツイート作成のビジネスロジック
async fn create_tweet_internal(db: &Db, user_id: Uuid, content: &str) -> Result<TweetResponse> {
    if content.is_empty() || content.len() > 280 {
        return Err(AppError::BadRequest(
            "Tweet content must be between 1 and 280 characters".to_string(),
        ));
    }

    let tweet_id = Uuid::new_v4();
    let created_at = Utc::now();

    let new_tweet = tweet::ActiveModel {
        id: Set(tweet_id),
        user_id: Set(user_id),
        content: Set(content.to_string()),
        created_at: Set(created_at.to_rfc3339()),
    };

    new_tweet.insert(db).await?;

    Ok(TweetResponse {
        id: tweet_id,
        user_id,
        content: content.to_string(),
        created_at,
    })
}

// HTTPハンドラー層

/// ユーザー登録（JSON API）
pub async fn register(db: web::Data<Db>, req: web::Json<RegisterRequest>) -> Result<HttpResponse> {
    let result = register_user(db.as_ref(), &req.username, &req.email, &req.password).await;

    match result {
        Ok((user_id, token)) => Ok(HttpResponse::Ok().json(AuthResponse {
            token,
            user: UserResponse {
                id: user_id,
                username: req.username.clone(),
                email: req.email.clone(),
            },
        })),
        Err(e) => {
            eprintln!("Register error: {}", e);
            Err(e)
        }
    }
}

/// ログイン（JSON API）
pub async fn login(db: web::Data<Db>, req: web::Json<LoginRequest>) -> Result<HttpResponse> {
    let (user_model, token) = login_user(db.as_ref(), &req.email, &req.password).await?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse::from(user_model),
    }))
}

/// ログアウト（JSON API）
pub async fn logout() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Logged out successfully" })))
}

/// ツイート投稿（JSON API）
pub async fn create_tweet(
    req_http: HttpRequest,
    db: web::Data<Db>,
    req: web::Json<CreateTweetRequest>,
) -> Result<HttpResponse> {
    let user_id = authenticate(&req_http)?;
    let tweet = create_tweet_internal(db.as_ref(), user_id, &req.content).await?;

    Ok(HttpResponse::Created().json(tweet))
}

/// ツイート取得
pub async fn get_tweet(db: web::Data<Db>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let tweet_model = tweet::Entity::find_by_id(*path)
        .one(db.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Tweet not found".to_string()))?;

    Ok(HttpResponse::Ok().json(TweetResponse::from(tweet_model)))
}

/// ツイート削除（JSON API）
pub async fn delete_tweet(
    req_http: HttpRequest,
    db: web::Data<Db>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = authenticate(&req_http)?;

    let tweet_model = tweet::Entity::find_by_id(*path)
        .one(db.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Tweet not found".to_string()))?;

    // 所有者のみ削除可能
    if tweet_model.user_id != user_id {
        return Err(AppError::Unauthorized("Not authorized to delete this tweet".to_string()));
    }

    tweet_model.delete(db.as_ref()).await?;

    Ok(HttpResponse::NoContent().finish())
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
