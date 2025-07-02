from prometheus_client import start_http_server, Gauge
import random
import time

temperature_gauge = Gauge(
    "temperature_celsius",
    "Temperature reading from device",
    ["exported_job", "id", "instance", "job", "location"],
)


def generate_metrics():
    while True:
        temperature = random.uniform(18.0, 24.0)
        temperature_gauge.labels(
            exported_job="otel-proxy",
            id="test",
            instance="hive-opentelemetry-collector:9090",
            job="otel-collector",
            location="outside",
        ).set(temperature)

        time.sleep(2)


if __name__ == "__main__":
    start_http_server(9000)
    generate_metrics()
