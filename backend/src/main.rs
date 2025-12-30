mod entities;
mod error;
mod graphql;
mod handlers;
mod models;
mod store;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web};
use graphql::create_schema;
use store::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // データベース接続プールを初期化
    let db = match init_db().await {
        Ok(pool) => {
            println!("Database initialized successfully");
            pool
        }
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };

    // GraphQLスキーマを作成
    let schema = create_schema(db.clone());

    HttpServer::new(move || {
        // CORS設定: Next.jsフロントエンド（localhost:3000）からのアクセスを許可
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::JsonConfig::default().limit(4096))
            // GraphQLエンドポイント
            .route("/graphql", web::post().to(handlers::graphql_handler))
            .route("/graphiql", web::get().to(handlers::graphiql_handler))
            // REST APIエンドポイント（後方互換性のため残す）
            .route("/api/register", web::post().to(handlers::register))
            .route("/api/login", web::post().to(handlers::login))
            .route("/api/logout", web::post().to(handlers::logout))
            .route("/api/tweets", web::post().to(handlers::create_tweet))
            .route("/api/tweets/{id}", web::get().to(handlers::get_tweet))
            .route("/api/tweets/{id}", web::delete().to(handlers::delete_tweet))
            .route("/api/timeline", web::get().to(handlers::get_timeline))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
