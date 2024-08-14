#[macro_use]
extern crate validator_derive;
mod config;
mod handlers;
mod models;

use actix_web::{App, HttpServer, middleware::Logger};
use color_eyre::Result;
use tracing::info;
use crate::config::Config;
use crate::handlers::app_config;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_env()
        .expect("Server configuration");

    info!("Starting server at http://{}:{}", config.host, config.port);

    let pool = config.db_pool().await
        .expect("Database configuration");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(pool.clone())
            .configure(app_config)
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .workers(2)
        .run()
        .await?;

    Ok(())
}
