pub mod crypto;

use std::sync::Arc;
use chrono::Duration;
use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use sqlx::Postgres;
use sqlx::postgres::PgPool;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;
use crate::config::crypto::CryptoService;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String
}

impl Config {

    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        config.try_deserialize()
            .context("Loading configuration from environment.")
    }

    #[instrument(skip(self))]
    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool.");
        PgPool::connect(&self.database_url)
            .await
            .context("Creating database connection pool.")
    }

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone())
        }
    }
}
