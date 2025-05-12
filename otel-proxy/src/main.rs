mod routes;
mod telemetry;
use std::{collections::HashMap, sync::Arc};
use telemetry::{init_metrics, MetricFactory};

use routes::log_metric;

use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::{self, Next},
    response::Response,
    routing::post,
    Router,
};
use clap::Parser;

use opentelemetry::metrics::{Gauge, MeterProvider};

/// Otel-Proxy entry point
#[derive(Parser, Debug)]
#[command(name = "Otel Proxy", version = env!("CARGO_PKG_VERSION"), about = "Proxy server for the otel collector")]
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

    /// URL of otel-collector
    #[arg(long, env = "OTEL_URL")]
    otel_url: String,

    #[arg(long, env = "METRICS_CONFIG")]
    metrics_config: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Serve(args) => {
            println!("Running on port: {}", args.port);

            let meter_provider = init_metrics(&args);
            let meter = meter_provider.meter("otel-proxy");
            let metric_map = MetricFactory::create(&args, &meter);

            let shared_state = Arc::new(AppState { metric_map });

            let app = Router::new()
                .route("/metrics", post(log_metric))
                .route_layer(middleware::from_fn(my_middleware))
                .with_state(shared_state);

            let path = format!("0.0.0.0:{}", &args.port);
            let listener = tokio::net::TcpListener::bind(path).await.unwrap();
            println!("listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, app).await.unwrap();
        }
    }
}

struct AppState {
    metric_map: HashMap<String, Gauge<f64>>,
}

async fn my_middleware(headers: HeaderMap, request: Request, next: Next) -> Response {
    println!("Headers:");
    println!("{:?}", headers);

    next.run(request).await
}
