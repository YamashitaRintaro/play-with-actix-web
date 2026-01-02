use crate::error::AppError;
use crate::graphql::AppSchema;
use crate::models::*;
use crate::store::Db;
use crate::utils::{authenticate, create_jwt, hash_password, verify_jwt, verify_password};
use actix_web::{HttpRequest, HttpResponse, web};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use chrono::Utc;
use uuid::Uuid;

type Result<T> = std::result::Result<T, AppError>;

// GraphQLハンドラー

/// Authorizationヘッダーからユーザーを認証し、リクエストにユーザーIDを追加
fn authenticate_request(
    req: &HttpRequest,
    mut request: async_graphql::Request,
) -> async_graphql::Request {
    match req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(verify_jwt)
    {
        Some(Ok(user_id)) => {
            request = request.data(user_id);
        }
        Some(Err(e)) => {
            eprintln!("JWT verification failed: {}", e);
        }
        None => {
            // Authorization ヘッダーがない場合は認証不要なリクエストとして続行
        }
    }
    request
}

/// GraphQLエンドポイント (POST)
pub async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let request = authenticate_request(&req, gql_req.into_inner());
    schema.execute(request).await.into()
}

/// GraphQLエンドポイント (GET) - クエリパラメータからGraphQLリクエストを処理
pub async fn graphql_handler_get(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    query: web::Query<GraphQLQueryParams>,
) -> GraphQLResponse {
    let mut request = async_graphql::Request::new(&query.query);

    if let Some(ref op_name) = query.operation_name {
        request = request.operation_name(op_name);
    }

    if let Some(ref vars) = query.variables {
        match serde_json::from_str(vars) {
            Ok(variables) => request = request.variables(variables),
            Err(e) => {
                return async_graphql::Response::from_errors(vec![
                    async_graphql::ServerError::new(
                        format!("Failed to parse GraphQL variables: {}", e),
                        None,
                    ),
                ])
                .into();
            }
        }
    }

    let request = authenticate_request(&req, request);
    schema.execute(request).await.into()
}

#[derive(serde::Deserialize)]
pub struct GraphQLQueryParams {
    query: String,
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    variables: Option<String>,
}

/// GraphQL Playgroundエンドポイント
pub async fn graphiql_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn register_user(
    db: &Db,
    username: &str,
    email: &str,
    password: &str,
) -> Result<(User, String)> {
    // メールアドレスの重複チェック
    let existing: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }

    let password_hash = hash_password(password)?;
    let user_id = Uuid::new_v4();
    let created_at = Utc::now().to_rfc3339();

    // ユーザーを挿入（Uuidは自動でTEXTに変換される）
    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(username)
    .bind(email)
    .bind(&password_hash)
    .bind(&created_at)
    .execute(db)
    .await?;

    let user = User {
        id: user_id,
        username: username.to_string(),
        email: email.to_string(),
        password_hash,
        created_at,
    };

    let token = create_jwt(user_id)?;

    Ok((user, token))
}

async fn login_user(db: &Db, email: &str, password: &str) -> Result<(User, String)> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    if !verify_password(password, &user.password_hash)? {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // user.id は既に Uuid 型なので parse 不要
    let token = create_jwt(user.id)?;

    Ok((user, token))
}

async fn create_tweet_internal(db: &Db, user_id: Uuid, content: &str) -> Result<TweetResponse> {
    if content.is_empty() || content.len() > 280 {
        return Err(AppError::BadRequest(
            "Tweet content must be between 1 and 280 characters".to_string(),
        ));
    }

    let tweet_id = Uuid::new_v4();
    let created_at = Utc::now();

    sqlx::query("INSERT INTO tweets (id, user_id, content, created_at) VALUES (?, ?, ?, ?)")
        .bind(tweet_id)
        .bind(user_id)
        .bind(content)
        .bind(created_at.to_rfc3339())
        .execute(db)
        .await?;

    Ok(TweetResponse {
        id: tweet_id,
        user_id,
        content: content.to_string(),
        created_at,
    })
}

pub async fn register(db: web::Data<Db>, req: web::Json<RegisterRequest>) -> Result<HttpResponse> {
    let (user, token) =
        register_user(db.as_ref(), &req.username, &req.email, &req.password).await?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse::from(user),
    }))
}

pub async fn login(db: web::Data<Db>, req: web::Json<LoginRequest>) -> Result<HttpResponse> {
    let (user, token) = login_user(db.as_ref(), &req.email, &req.password).await?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse::from(user),
    }))
}

pub async fn logout() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Logged out successfully" })))
}

pub async fn create_tweet(
    req_http: HttpRequest,
    db: web::Data<Db>,
    req: web::Json<CreateTweetRequest>,
) -> Result<HttpResponse> {
    let user_id = authenticate(&req_http)?;
    let tweet = create_tweet_internal(db.as_ref(), user_id, &req.content).await?;

    Ok(HttpResponse::Created().json(tweet))
}

pub async fn get_tweet(db: web::Data<Db>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let tweet: Tweet = sqlx::query_as("SELECT * FROM tweets WHERE id = ?")
        .bind(*path)
        .fetch_optional(db.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Tweet not found".to_string()))?;

    Ok(HttpResponse::Ok().json(TweetResponse::from(tweet)))
}

pub async fn delete_tweet(
    req_http: HttpRequest,
    db: web::Data<Db>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = authenticate(&req_http)?;

    let tweet: Tweet = sqlx::query_as("SELECT * FROM tweets WHERE id = ?")
        .bind(*path)
        .fetch_optional(db.as_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Tweet not found".to_string()))?;

    // 所有者のみ削除可能（tweet.user_id は既に Uuid 型）
    if tweet.user_id != user_id {
        return Err(AppError::Unauthorized(
            "Not authorized to delete this tweet".to_string(),
        ));
    }

    sqlx::query("DELETE FROM tweets WHERE id = ?")
        .bind(*path)
        .execute(db.as_ref())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_timeline(req_http: HttpRequest, db: web::Data<Db>) -> Result<HttpResponse> {
    let user_id = authenticate(&req_http)?;

    let tweets: Vec<Tweet> =
        sqlx::query_as("SELECT * FROM tweets WHERE user_id = ? ORDER BY created_at DESC")
            .bind(user_id)
            .fetch_all(db.as_ref())
            .await?;

    let timeline: Vec<TweetResponse> = tweets.into_iter().map(TweetResponse::from).collect();

    Ok(HttpResponse::Ok().json(timeline))
}
