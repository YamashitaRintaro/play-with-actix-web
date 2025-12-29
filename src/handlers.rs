use crate::error::AppError;
use crate::models::*;
use crate::store::Db;
use crate::utils::{authenticate, create_jwt, hash_password, verify_password};
use actix_web::{HttpRequest, HttpResponse, web};
use chrono::Utc;
use uuid::Uuid;

type Result<T> = std::result::Result<T, AppError>;

/// ユーザー登録
pub async fn register(db: web::Data<Db>, req: web::Json<RegisterRequest>) -> Result<HttpResponse> {
    // メールアドレスの重複チェック
    let existing = sqlx::query("SELECT id FROM users WHERE email = ?")
        .bind(&req.email)
        .fetch_optional(db.as_ref())
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }

    let password_hash = hash_password(&req.password)?;
    let user_id = Uuid::new_v4();
    let created_at = Utc::now();

    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(user_id.to_string())
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .bind(created_at.to_rfc3339())
    .execute(db.as_ref())
    .await?;

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
    let row = sqlx::query("SELECT id, username, email, password_hash FROM users WHERE email = ?")
        .bind(&req.email)
        .fetch_optional(db.as_ref())
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    let user = User::from_row(&row)?;

    if !verify_password(&req.password, &user.password_hash)? {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    let token = create_jwt(user.id)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        },
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

    sqlx::query("INSERT INTO tweets (id, user_id, content, created_at) VALUES (?, ?, ?, ?)")
        .bind(tweet_id.to_string())
        .bind(user_id.to_string())
        .bind(&req.content)
        .bind(created_at.to_rfc3339())
        .execute(db.as_ref())
        .await?;

    Ok(HttpResponse::Created().json(TweetResponse {
        id: tweet_id,
        user_id,
        content: req.content.clone(),
        created_at,
    }))
}

/// ツイート取得
pub async fn get_tweet(db: web::Data<Db>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let row = sqlx::query("SELECT id, user_id, content, created_at FROM tweets WHERE id = ?")
        .bind(path.to_string())
        .fetch_optional(db.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Tweet not found".to_string()))?;

    let tweet = Tweet::from_row(&row)?;

    Ok(HttpResponse::Ok().json(TweetResponse::from(tweet)))
}

/// タイムライン取得
pub async fn get_timeline(req_http: HttpRequest, db: web::Data<Db>) -> Result<HttpResponse> {
    let user_id = authenticate(&req_http)?;

    let rows = sqlx::query(
        "SELECT id, user_id, content, created_at FROM tweets WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(user_id.to_string())
    .fetch_all(db.as_ref())
    .await?;

    let timeline: Vec<TweetResponse> = rows
        .iter()
        .filter_map(|row| Tweet::from_row(row).ok())
        .map(TweetResponse::from)
        .collect();

    Ok(HttpResponse::Ok().json(timeline))
}
