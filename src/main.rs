use crate::config::Config;
use clap::Parser;
use std::fs;

use axum::routing::get;
use axum::Router;
use metrics::{initialize_registry, metrics_handler, Metrics};
use prometheus_client::registry::Registry;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod metrics;
mod tsdb;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "promguard-config.yaml")]
    config: String,
}

fn load_config(filename: &str) -> Config {
    let config_content =
        fs::read_to_string(filename).expect("Failed to read the configuration file");

    serde_yaml::from_str(&config_content).expect("Failed to parse the configuration file")
}

#[derive()]
pub struct AppState {
    pub metrics: Metrics,
    pub registry: Registry,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = load_config(&args.config);
    info!("Loaded configuration: {:?}", config);

    let state = Arc::new(Mutex::new(
        initialize_registry(Registry::with_prefix("promguard"), config).unwrap(),
    ));
    let router = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state);
    let port = 8080;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
