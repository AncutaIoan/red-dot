mod repository;
mod models;
mod controller;
mod error;

use actix_identity::IdentityMiddleware;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use actix_web::web::Data;
use crate::repository::app_state::AppState;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;
    let app_state = Data::new(AppState::new(pool));

    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(app_state.clone())
            .wrap(IdentityMiddleware::default()) // Session-based middleware
            .wrap(logger)
            .service(controller::incident_controller::add_incident)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
