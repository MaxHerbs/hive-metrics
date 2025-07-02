use std::sync::Arc;

use axum::{
    Router,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::{Next, from_fn},
    response::{IntoResponse, Response},
    routing::get,
};
use clap::Parser;
mod database;

use sqlx::{Pool, Postgres};
use tracing::*;
use url::Url;

use crate::{authz::query_range_handler, database::DatabaseConfig};

mod authz;

#[derive(Parser, Debug)]
#[command(name = "Gateway", version = env!("CARGO_PKG_VERSION"), about = "Data Gateway for Hive Information")]
enum Cli {
    /// Starts a webserver
    Serve(ServeArgs),
}

/// Subcommands supported by the CLI
#[derive(Debug, Parser)]
struct ServeArgs {
    /// Port number to bind the application to
    #[arg(long, default_value_t = 8080, env = "PORT")]
    port: u16,

    /// URL of prometheus
    #[arg(long, env = "PROMETHEUS_URL")]
    prometheus_url: String,

    /// Database configuration
    #[command(flatten)]
    database_config: DatabaseConfig,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli {
        Cli::Serve(args) => {
            info!("{:?}", args);
            info!("Listening on port: {}", args.port);

            let db_pool = args
                .database_config
                .new_client()
                .await
                .expect("Failed to initiate SQL pool");

            let shared_state = Arc::new(AppState {
                db_pool,
                prometheus_endpoint: Url::parse(&args.prometheus_url).unwrap(),
            });

            let router = build_router(shared_state);

            let path = format!("0.0.0.0:{}", &args.port);
            let listener = tokio::net::TcpListener::bind(path).await.unwrap();
            axum::serve(listener, router).await.unwrap();
        }
    }
}

fn build_router(shared_state: Arc<AppState>) -> Router {
    let query = Router::new()
        .route("/query_range", get(query_range_handler))
        .with_state(shared_state);

    // let health_router = Router::new().route("/healthz", get(health_check));

    Router::new()
        .merge(query)
        // .merge(health_router)
        .layer(from_fn(incoming_requests))
}

struct AppState {
    db_pool: Pool<Postgres>,
    prometheus_endpoint: Url,
}

async fn incoming_requests(_headers: HeaderMap, request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    info!("Incoming request: {} {}", method, uri);
    let response = next.run(request).await;

    if !response.status().is_success() {
        error!(
            "Request failed: {} {} with code '{}'",
            method,
            uri,
            response.status()
        );
    }

    response
}

pub fn healtz() -> impl IntoResponse {
    StatusCode::OK
}
