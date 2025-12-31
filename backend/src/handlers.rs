use crate::entities::{tweet, user};
use crate::error::AppError;
use crate::graphql::AppSchema;
use crate::models::*;
use crate::store::Db;
use crate::utils::{authenticate, create_jwt, hash_password, verify_jwt, verify_password};
use actix_web::{HttpRequest, HttpResponse, web};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

type Result<T> = std::result::Result<T, AppError>;

// GraphQLハンドラー

/// Authorizationヘッダーからユーザーを認証し、リクエストにユーザーIDを追加
fn authenticate_request(
    req: &HttpRequest,
    mut request: async_graphql::Request,
) -> async_graphql::Request {
    let auth_result = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(verify_jwt);

    match auth_result {
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
                let error_response =
                    async_graphql::Response::from_errors(vec![async_graphql::ServerError::new(
                        format!("Failed to parse GraphQL variables: {}", e),
                        None,
                    )]);
                return error_response.into();
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

// ビジネスロジック層

/// ユーザー登録のビジネスロジック
async fn register_user(
    db: &Db,
    username: &str,
    email: &str,
    password: &str,
) -> Result<(user::Model, String)> {
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

    let user_model = new_user.insert(db).await?;

    let token = create_jwt(user_id)?;

    Ok((user_model, token))
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
    let (user_model, token) =
        register_user(db.as_ref(), &req.username, &req.email, &req.password).await?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserResponse::from(user_model),
    }))
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
        return Err(AppError::Unauthorized(
            "Not authorized to delete this tweet".to_string(),
        ));
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
