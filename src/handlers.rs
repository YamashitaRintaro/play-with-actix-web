use crate::entities::{tweet, user};
use crate::error::AppError;
use crate::models::*;
use crate::store::Db;
use crate::utils::{authenticate, create_jwt, hash_password, verify_password};
use actix_web::{HttpRequest, HttpResponse, http::header, web};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
};
use tera::Tera;
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
        ..Default::default()
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
        ..Default::default()
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
    let (user_id, token) =
        register_user(db.as_ref(), &req.username, &req.email, &req.password).await?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            username: req.username.clone(),
            email: req.email.clone(),
        },
    }))
}

/// フォームからのユーザー登録
pub async fn register_form(
    db: web::Data<Db>,
    tera: web::Data<Tera>,
    form: web::Form<RegisterRequest>,
) -> actix_web::Result<HttpResponse> {
    match register_user(db.as_ref(), &form.username, &form.email, &form.password).await {
        Ok((_, token)) => {
            // Cookieを設定してリダイレクト
            Ok(HttpResponse::SeeOther()
                .cookie(
                    actix_web::cookie::Cookie::build("token", token)
                        .path("/")
                        .max_age(actix_web::cookie::time::Duration::days(1))
                        .finish(),
                )
                .append_header((header::LOCATION, "/"))
                .finish())
        }
        Err(e) => {
            let mut context = tera::Context::new();
            context.insert("error", &e.to_string());
            crate::templates::render_template(&tera, "register.html", context)
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

/// フォームからのログイン
pub async fn login_form(
    db: web::Data<Db>,
    tera: web::Data<Tera>,
    form: web::Form<LoginRequest>,
) -> actix_web::Result<HttpResponse> {
    match login_user(db.as_ref(), &form.email, &form.password).await {
        Ok((_, token)) => {
            // Cookieを設定してリダイレクト
            Ok(HttpResponse::SeeOther()
                .cookie(
                    actix_web::cookie::Cookie::build("token", token)
                        .path("/")
                        .max_age(actix_web::cookie::time::Duration::days(1))
                        .finish(),
                )
                .append_header((header::LOCATION, "/"))
                .finish())
        }
        Err(e) => {
            let mut context = tera::Context::new();
            context.insert("error", &e.to_string());
            crate::templates::render_template(&tera, "login.html", context)
        }
    }
}

/// 認証ヘルパー（BearerトークンまたはCookie）
fn get_user_id_from_request(req: &HttpRequest) -> actix_web::Result<Uuid> {
    if let Ok(id) = authenticate(req) {
        Ok(id)
    } else if let Some(cookie) = req.cookie("token") {
        crate::utils::verify_jwt(cookie.value())
            .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Not logged in"))
    }
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

/// フォームからのツイート投稿
pub async fn create_tweet_form(
    req_http: HttpRequest,
    db: web::Data<Db>,
    form: web::Form<CreateTweetRequest>,
) -> actix_web::Result<HttpResponse> {
    let user_id = get_user_id_from_request(&req_http)?;

    match create_tweet_internal(db.as_ref(), user_id, &form.content).await {
        Ok(_) => {
            // リダイレクト
            Ok(HttpResponse::SeeOther()
                .append_header((header::LOCATION, "/"))
                .finish())
        }
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
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

// HTMLレンダリング用のハンドラー

/// トップページ（タイムライン）
pub async fn index_page(
    req_http: HttpRequest,
    db: web::Data<Db>,
    tera: web::Data<Tera>,
) -> actix_web::Result<HttpResponse> {
    let mut context = tera::Context::new();

    // 認証チェック（BearerトークンまたはCookie）
    let user_id = if let Ok(id) = authenticate(&req_http) {
        Some(id)
    } else if let Some(cookie) = req_http.cookie("token") {
        crate::utils::verify_jwt(cookie.value()).ok()
    } else {
        None
    };

    if let Some(user_id) = user_id {
        if let Ok(user) = user::Entity::find_by_id(user_id).one(db.as_ref()).await {
            if let Some(user) = user {
                if let Ok(tweets) = user
                    .find_related(tweet::Entity)
                    .order_by_desc(tweet::Column::CreatedAt)
                    .all(db.as_ref())
                    .await
                {
                    let tweets_data: Vec<serde_json::Value> = tweets
                        .into_iter()
                        .map(|t| {
                            serde_json::json!({
                                "content": t.content,
                                "created_at": t.created_at,
                            })
                        })
                        .collect();
                    context.insert("tweets", &tweets_data);
                }
            }
        }
    }

    crate::templates::render_template(&tera, "index.html", context)
}

/// ログインページ
pub async fn login_page(tera: web::Data<Tera>) -> actix_web::Result<HttpResponse> {
    let context = tera::Context::new();
    crate::templates::render_template(&tera, "login.html", context)
}

/// 登録ページ
pub async fn register_page(tera: web::Data<Tera>) -> actix_web::Result<HttpResponse> {
    let context = tera::Context::new();
    crate::templates::render_template(&tera, "register.html", context)
}
