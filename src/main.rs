mod handlers;
mod models;
mod store;
mod utils;

use actix_web::{App, HttpServer, web};
use std::sync::{Arc, Mutex};
use store::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::new();
    let db = Arc::new(Mutex::new(app_state));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::JsonConfig::default().limit(4096))
            .route("/api/register", web::post().to(handlers::register))
            .route("/api/login", web::post().to(handlers::login))
            .route("/api/tweets", web::post().to(handlers::create_tweet))
            .route("/api/tweets/{id}", web::get().to(handlers::get_tweet))
            .route("/api/timeline", web::get().to(handlers::get_timeline))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
