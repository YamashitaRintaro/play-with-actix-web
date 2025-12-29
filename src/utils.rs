use actix_web::HttpRequest;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWTのペイロード（クレーム）を表す構造体
/// user_id: ユーザーID（文字列形式）
/// exp: 有効期限（Unixタイムスタンプ）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    // DEFAULT_COSTは10で、ハッシュ化の強度を表す
    // 値が大きいほどセキュアだが、処理時間も長くなる
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn create_jwt(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    // 有効期限を24時間後に設定
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        user_id: user_id.to_string(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    // JWTトークンを生成
    // Header::default() - デフォルトのヘッダー（HS256アルゴリズム）
    // claims - ペイロード
    // EncodingKey - エンコード用のキー
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

fn verify_jwt(token: &str) -> Result<Uuid, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    // JWTトークンをデコードして検証
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256), // HS256アルゴリズムで検証
    )?;

    // クレームからユーザーIDを取得してUUIDに変換
    Uuid::parse_str(&token_data.claims.user_id)
        .map_err(|_| jsonwebtoken::errors::ErrorKind::InvalidToken.into())
}

#[derive(Debug)]
pub enum AuthError {
    MissingHeader,
    InvalidFormat,
}

fn bearer_token(req: &HttpRequest) -> Result<&str, AuthError> {
    let header = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(AuthError::MissingHeader)?;

    header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidFormat)
}

pub fn authenticate(req: &HttpRequest) -> Result<Uuid, AuthError> {
    let token = bearer_token(req)?;
    verify_jwt(token).map_err(|_| AuthError::InvalidFormat)
}
