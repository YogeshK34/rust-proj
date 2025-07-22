mod handlers;
mod models;
mod db;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use db::get_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let db_pool = get_db_pool().await.expect("Failed to create DB pool");

    println!("🚀 Server running at http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/users", web::post().to(handlers::create_user))
            .route("/get-users", web::get().to(handlers::get_users))
    })
    .bind(addr)?
    .run()
    .await
}
