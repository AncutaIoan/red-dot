mod repository;
mod models;
mod controller;
mod error;

use std::error::Error;
use actix_identity::IdentityMiddleware;
use actix_web::{middleware::Logger, App, HttpServer, web::Data};
use dotenv::dotenv;
use sqlx::PgPool;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_governor::governor::middleware::NoOpMiddleware;
use crate::repository::app_state::AppState;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let app_state = create_app_state().await?;
    let governor_config = create_ip_governor();

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_config))
            .service(controller::incident_controller::add_incident)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}

async fn create_app_state() -> Result<Data<AppState>, Box<dyn Error>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(Data::new(AppState::new(pool)))
}

fn create_ip_governor() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware> {
    GovernorConfigBuilder::default()
        .seconds_per_request(5)
        .burst_size(10)
        .finish()
        .unwrap()
}
