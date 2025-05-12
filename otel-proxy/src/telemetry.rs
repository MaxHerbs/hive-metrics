use std::{collections::HashMap, time::Duration};

use opentelemetry::{
    metrics::{Gauge, Meter},
    KeyValue,
};
use opentelemetry_otlp::{MetricExporter, WithExportConfig};
use opentelemetry_sdk::{
    metrics::{PeriodicReader, SdkMeterProvider},
    Resource,
};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use serde::Deserialize;

use crate::ServeArgs;

pub fn init_metrics(args: &ServeArgs) -> SdkMeterProvider {
    let otel_resources = Resource::builder()
        .with_attributes([
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ])
        .build();

    let exporter = MetricExporter::builder()
        .with_http()
        .with_endpoint(&args.otel_url)
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter)
        .with_interval(Duration::from_secs(5))
        .build();

    SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(otel_resources)
        .build()
}

pub struct MetricFactory {}

impl MetricFactory {
    pub fn create(args: &ServeArgs, meter: &Meter) -> HashMap<String, Gauge<f64>> {
        let input_metrics: Vec<InputMetric> = serde_json::from_str(&args.metrics_config).unwrap();
        let metrics: HashMap<String, Gauge<f64>> = input_metrics
            .into_iter()
            .map(|metric| {
                let guage = meter
                    .f64_gauge(metric.name.clone())
                    .with_description(metric.description.unwrap_or_default())
                    .with_unit(metric.unit)
                    .build();

                (metric.name, guage)
            })
            .collect();
        metrics
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct InputMetric {
    name: String,
    unit: String,
    description: Option<String>,
}
