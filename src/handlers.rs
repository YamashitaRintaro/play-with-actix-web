use crate::models::*;
use crate::store::Db;
use crate::utils::{authenticate, create_jwt, hash_password, verify_password};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use chrono::Utc;
use uuid::Uuid;

// ユーザー登録
pub async fn register(db: web::Data<Db>, req: web::Json<RegisterRequest>) -> impl Responder {
    let app_state = db.lock().unwrap();
    let mut users = app_state.users.lock().unwrap();

    // ユーザー名とメールアドレスの重複チェック
    if users.iter().any(|u| u.email == req.email) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Email already exists"
        }));
    }

    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to hash password"
            }));
        }
    };

    let user = User {
        id: Uuid::new_v4(),
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash,
        created_at: Utc::now(),
    };

    let user_id = user.id;
    users.push(user.clone());

    let token = match create_jwt(user_id) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create token"
            }));
        }
    };

    HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        },
    })
}

// ログイン
pub async fn login(db: web::Data<Db>, req: web::Json<LoginRequest>) -> impl Responder {
    let app_state = db.lock().unwrap();
    let users = app_state.users.lock().unwrap();

    let user = match users.iter().find(|u| u.email == req.email) {
        Some(u) => u,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid email or password"
            }));
        }
    };

    match verify_password(&req.password, &user.password_hash) {
        Ok(true) => {
            let token = match create_jwt(user.id) {
                Ok(t) => t,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to create token"
                    }));
                }
            };

            HttpResponse::Ok().json(AuthResponse {
                token,
                user: UserResponse {
                    id: user.id,
                    username: user.username.clone(),
                    email: user.email.clone(),
                },
            })
        }
        _ => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid email or password"
        })),
    }
}

// ツイート投稿
pub async fn create_tweet(
    req_http: HttpRequest,
    db: web::Data<Db>,
    req: web::Json<CreateTweetRequest>,
) -> impl Responder {
    let user_id = match authenticate(&req_http) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized"
            }));
        }
    };

    if req.content.is_empty() || req.content.len() > 280 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Tweet content must be between 1 and 280 characters"
        }));
    }

    let tweet = Tweet {
        id: Uuid::new_v4(),
        user_id,
        content: req.content.clone(),
        created_at: Utc::now(),
    };

    let app_state = db.lock().unwrap();
    let mut tweets = app_state.tweets.lock().unwrap();
    tweets.push(tweet.clone());

    HttpResponse::Created().json(TweetResponse {
        id: tweet.id,
        user_id: tweet.user_id,
        content: tweet.content,
        created_at: tweet.created_at,
    })
}

// ツイート取得
pub async fn get_tweet(db: web::Data<Db>, path: web::Path<Uuid>) -> impl Responder {
    let app_state = db.lock().unwrap();
    let tweets = app_state.tweets.lock().unwrap();

    match tweets.iter().find(|t| t.id == *path) {
        Some(tweet) => HttpResponse::Ok().json(TweetResponse {
            id: tweet.id,
            user_id: tweet.user_id,
            content: tweet.content.clone(),
            created_at: tweet.created_at,
        }),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Tweet not found"
        })),
    }
}

// タイムライン取得
pub async fn get_timeline(req_http: HttpRequest, db: web::Data<Db>) -> impl Responder {
    let user_id = match authenticate(&req_http) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized"
            }));
        }
    };
    let app_state = db.lock().unwrap();
    let tweets = app_state.tweets.lock().unwrap();
    let timeline: Vec<TweetResponse> = tweets
        .iter()
        .filter(|t| t.user_id == user_id)
        .map(|t| TweetResponse {
            id: t.id,
            user_id: t.user_id,
            content: t.content.clone(),
            created_at: t.created_at,
        })
        .collect();
    HttpResponse::Ok().json(timeline)
}
