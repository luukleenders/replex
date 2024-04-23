use salvo::prelude::*;
use std::env;
use std::time::Duration;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use replex::cache::CACHE_MANAGER;
use replex::config::Config;
use replex::router::main_router;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let _init_cache = CACHE_MANAGER.clone();
    let config = Config::load();

    if config.host.is_none() {
        tracing::error!("REPLEX_HOST is required. Exiting");
        return;
    }

    // set default log level
    // if let Err(_i) = env::var("RUST_LOG") {
    //     env::set_var("RUST_LOG", "info");
    // }

    let version = env!("CARGO_PKG_VERSION");
    tracing::info!("Replex version {}", version);

    let router = main_router();

    // Bind and serve the application
    let acceptor =
        TcpListener::new(format!("0.0.0.0:{}", config.port.unwrap_or(80)))
            .bind()
            .await;

    Server::new(acceptor)
        .conn_idle_timeout(Duration::from_secs(60 * 101))
        .serve(router)
        .await;
}
