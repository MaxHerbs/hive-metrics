use crate::AppState;
use std::{collections::HashMap, sync::Arc};

use axum::{extract::State, http::StatusCode, Json};
use opentelemetry::KeyValue;
use serde::Deserialize;

pub async fn log_metric(
    State(state): State<Arc<AppState>>,
    // headers: HeaderMap,
    Json(body): Json<MetricBody>,
) -> StatusCode {
    println!("{:#?}", body);

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
            println!("Unsupported metric called: {}", metric_name);
        }
    });

    StatusCode::ACCEPTED
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricBody {
    id: String,
    location: String,
    metrics: HashMap<String, f64>,
}
