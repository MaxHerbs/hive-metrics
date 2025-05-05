from otel_proxy._metrics_factory import get_supported_metrics


def test_supported_metrics():
    modules = get_supported_metrics()
    assert list(modules.keys()) == ["temperature"]
