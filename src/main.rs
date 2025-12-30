mod entities;
mod error;
mod handlers;
mod models;
mod store;
mod templates;
mod utils;

use actix_files::Files;
use actix_web::{App, HttpServer, web};
use store::init_db;
use templates::init_tera;

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

    // Teraテンプレートエンジンの初期化
    let tera = init_tera();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::JsonConfig::default().limit(4096))
            // HTMLページ
            .route("/", web::get().to(handlers::index_page))
            .route("/login", web::get().to(handlers::login_page))
            .route("/register", web::get().to(handlers::register_page))
            // フォーム送信用のエンドポイント
            .route("/login", web::post().to(handlers::login_form))
            .route("/register", web::post().to(handlers::register_form))
            .route("/tweets", web::post().to(handlers::create_tweet_form))
            // APIエンドポイント（JSON）
            .route("/api/register", web::post().to(handlers::register))
            .route("/api/login", web::post().to(handlers::login))
            .route("/api/tweets", web::post().to(handlers::create_tweet))
            .route("/api/tweets/{id}", web::get().to(handlers::get_tweet))
            .route("/api/timeline", web::get().to(handlers::get_timeline))
            // 静的ファイル配信
            .service(Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
