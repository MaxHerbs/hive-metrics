use crate::AppState;
use std::{collections::HashMap, sync::Arc};

use axum::{extract::State, http::StatusCode, Json};
use opentelemetry::KeyValue;
use serde::{Deserialize, Serialize};

pub async fn log_metric(
    State(state): State<Arc<AppState>>,
    // headers: HeaderMap,
    Json(body): Json<MetricBody>,
) -> (StatusCode, Json<MetricsResponse>) {
    println!("{:#?}", body);

    let mut unsupported_metrics: Vec<String> = Vec::new();
    body.metrics.into_iter().for_each(|(metric_name, val)| {
        if let Some(metric_fn) = state.metric_map.get(&metric_name) {
            println!("Found metric for {}", metric_name);
            metric_fn.record(
                val,
                &[
                    KeyValue::new("id", body.id.clone()),
                    KeyValue::new("location", body.location.clone()),
                ],
            );
        } else {
            println!("Unsupported metric called: {}", &metric_name);
            unsupported_metrics.push(metric_name);
        }
    });

    if !unsupported_metrics.is_empty() {
        let error = format!("Unsupported metrics:{}", unsupported_metrics.join("\n"));
        return (
            StatusCode::BAD_REQUEST,
            Json(MetricsResponse {
                message: Some(error),
            }),
        );
    }
    (
        StatusCode::ACCEPTED,
        Json(MetricsResponse { message: None }),
    )
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricBody {
    id: String,
    location: String,
    metrics: HashMap<String, f64>,
}

#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    message: Option<String>,
}
