use replex::config::Config;
use salvo::prelude::*;
use std::env;
use std::time::Duration;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use replex::cache::CACHE_MANAGER;
use replex::router::main_router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let host = env::var("REPLEX_HOST").expect("REPLEX_HOST must be set!");
    let version = env!("CARGO_PKG_VERSION");
    let config = Config::load();
    let _init_cache = CACHE_MANAGER.clone();

    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish(),
    )
    .expect("setting default subscriber failed");

    tracing::info!("Replex version {}", version);
    tracing::info!("Host: {}", host);

    Server::new(
        TcpListener::new(format!("0.0.0.0:{}", config.port.unwrap_or(80)))
            .bind()
            .await,
    )
    .conn_idle_timeout(Duration::from_secs(60 * 101))
    .serve(main_router())
    .await;
}
