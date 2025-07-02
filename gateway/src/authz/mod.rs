use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use tracing::debug;

mod user_auth;
use crate::{
    AppState,
    authz::user_auth::{get_or_create_user, user_owns_device},
};

#[derive(Debug, Deserialize)]
pub struct QueryRangeParams {
    device_id: String,
    start: String,
    end: String,
    step: String,
}

pub async fn query_range_handler(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<QueryRangeParams>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let prometheus_url = &app_state.prometheus_endpoint;
    let user_email = match headers.get("x-auth-request-user") {
        Some(value) => match value.to_str() {
            Ok(email) => email,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid user header").into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, "Missing auth header").into_response(),
    };

    let pool = &app_state.db_pool;

    let user_id = get_or_create_user(pool, user_email).await.unwrap();
    let authorized = user_owns_device(pool, user_id, &params.device_id)
        .await
        .unwrap();

    if !authorized {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    let metric_query = format!(r#"temperature_celsius{{id="{}"}}"#, params.device_id);
    debug!("Query {}", metric_query);

    let client = reqwest::Client::new();
    let url = format!(
        "{}api/v1/query_range?query={}&start={}&end={}&step={}",
        prometheus_url, metric_query, params.start, params.end, params.step
    );

    let res = client.get(&url).send().await;
    debug!("Response {:?}", res);

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json = response.text().await.unwrap_or_else(|_| "{}".to_string());
                (StatusCode::OK, json).into_response()
            } else {
                (
                    StatusCode::BAD_GATEWAY,
                    "Prometheus query failed".to_string(),
                )
                    .into_response()
            }
        }
        Err(_) => (
            StatusCode::BAD_GATEWAY,
            "Failed to contact Prometheus".to_string(),
        )
            .into_response(),
    }
}
