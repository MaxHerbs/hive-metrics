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

    /// Config string for reported guages
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
            let router = build_router(shared_state);

            let path = format!("0.0.0.0:{}", &args.port);
            let listener = tokio::net::TcpListener::bind(path).await.unwrap();
            println!("listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, router).await.unwrap();
        }
    }
}

fn build_router(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/metrics", post(log_metric))
        .route_layer(middleware::from_fn(my_middleware))
        .with_state(shared_state)
}

struct AppState {
    metric_map: HashMap<String, Gauge<f64>>,
}

async fn my_middleware(headers: HeaderMap, request: Request, next: Next) -> Response {
    println!("Headers:");
    println!("{:?}", headers);

    next.run(request).await
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use tokio::task;

    use super::*;

    #[tokio::test]
    async fn end_to_end_proxy_test() {
        let metrics_config = r#"
        [
            {
                "name": "cpu_usage",
                "unit": "percent",
                "description": "CPU usage of the system"
            }
        ]
        "#;

        let args = ServeArgs {
            port: 8888,
            otel_url: "http://localhost:4318".into(),
            metrics_config: metrics_config.into(),
        };

        let meter_provider = init_metrics(&args);
        let meter = meter_provider.meter("test-meter");

        let metric_map = MetricFactory::create(&args, &meter);
        assert!(metric_map.contains_key("cpu_usage"));
        assert_eq!(metric_map.len(), 1);

        let socket = format!("127.0.0.1:{}", args.port);
        let addr: SocketAddr = socket.parse().unwrap();

        let shared_state = std::sync::Arc::new(AppState { metric_map });

        let router = build_router(shared_state);
        let server = task::spawn(async move {
            axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), router)
                .await
                .unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let res = reqwest::Client::new()
            .post("http://localhost:8888/metrics")
            .json(&serde_json::json!({
                "id": "test",
                "location": "test-location",
                "metrics": {
                    "cpu_usage": 12.2
                }
            }))
            .send()
            .await
            .unwrap();

        assert!(res.status().is_success());
        server.abort();
    }
}
