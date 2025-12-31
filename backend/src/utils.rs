use crate::error::AppError;
use actix_web::HttpRequest;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWTのペイロード（クレーム）を表す構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
}

/// パスワードをハッシュ化する
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|_| AppError::Internal("Failed to hash password".to_string()))
}

/// パスワードを検証する
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|_| AppError::Internal("Failed to verify password".to_string()))
}

/// JWTシークレットキーを取得する
fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string())
}

/// JWTトークンを生成する
pub fn create_jwt(user_id: Uuid) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        user_id,
        exp: expiration,
    };

    let secret = get_jwt_secret();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| AppError::Internal("Failed to create token".to_string()))
}

/// JWTトークンを検証してユーザーIDを取得する
pub fn verify_jwt(token: &str) -> Result<Uuid, AppError> {
    let secret = get_jwt_secret();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    Ok(token_data.claims.user_id)
}

/// AuthorizationヘッダーからBearerトークンを抽出する
fn extract_bearer_token(req: &HttpRequest) -> Result<&str, AppError> {
    let header = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization format".to_string()))
}

/// リクエストからユーザーIDを認証して取得する
pub fn authenticate(req: &HttpRequest) -> Result<Uuid, AppError> {
    let token = extract_bearer_token(req)?;
    verify_jwt(token)
}
