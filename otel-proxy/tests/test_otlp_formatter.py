from otel_proxy._metrics_factory import build_otlp
from otel_proxy._types import DevicePayload


def test_metrics_factory():
    payload = DevicePayload(
        id="testid",
        location="kitchen",
        model="model1",
        deployment_type="standard",
        metrics={"temperature": 22.5},
    )
    output = build_otlp(payload)
    response = """{"resourceMetrics": [{"resource": {"attributes": [{"key": "device.id", "value": {"stringValue": "testid"}}, {"key": "location", "value": {"stringValue": "kitchen"}}, {"key": "deployment.type", "value": {"stringValue": "standard"}}]}, "scopeMetrics": [{"scope": {"name": "sensor.firmware", "version": "model1"}, "metrics": [{"name": "temperature", "unit": "Celcius", "description": "Ambient temperature", "gauge": {"dataPoints": [{"asDouble": 22.5, "timeUnixNano": "1746562391990329344"}]}}]}]}]}"""  # noqa: E501
    assert output, response
