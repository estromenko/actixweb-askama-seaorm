mod app_data;
mod config;
mod handlers;
mod models;
mod templates;

use actix_web::{middleware, web, App, HttpServer};

use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let conf = config::Config::from_env();
    let db: DatabaseConnection = Database::connect(conf.db_dsn).await.unwrap();

    let app_data = web::Data::new(app_data::AppData { db });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .service(handlers::index)
    })
    .bind(conf.address)?
    .run()
    .await
}
