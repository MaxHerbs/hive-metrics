import json
import logging
import time
from importlib import import_module
from pathlib import Path

from ._types import DevicePayload

logger = logging.getLogger("uvicorn.error")


modules_path = Path(__file__).parent / "metrics"
logger.debug(f"Modules path: {modules_path}")
available_modules = [
    module
    for module in modules_path.glob("*.py")
    if not modules_path.stem.startswith("_")
]

module_map = {}
for module in available_modules:
    module_name = f"otel_proxy.metrics.{module.stem}"
    imported_module = import_module(module_name)

    if fn := getattr(imported_module, "formatter", None):
        module_map[module.stem] = fn


def get_supported_metrics() -> dict:
    return module_map


def build_otlp(device: DevicePayload) -> str:
    payload = get_payload_base(device)
    metrics = []
    logger.debug(f"Available Modules {module_map}")

    for metric, val in device.metrics.items():
        logger.debug(f"Metric: {metric}, val: {val}")
        if fn := module_map.get(metric, None):
            logger.debug(f"Found function for {metric}")
            metrics.append(fn(val))
    logger.debug(metrics)

    payload_body = str(payload)
    payload_body = payload_body.replace("'$METRICS'", str(metrics))
    payload_body = payload_body.replace("$TIMESTAMP", str(int(time.time() * 1e9)))
    payload_body = payload_body.replace("'", '"')
    logger.error("\n\n")
    logger.error(payload_body)
    logger.error("\n\n")
    return json.loads(payload_body)


def get_payload_base(device: DevicePayload):
    return {
        "resourceMetrics": [
            {
                "resource": {
                    "attributes": [
                        {
                            "key": "device.id",
                            "value": {"stringValue": f"{device.id}"},
                        },
                        {
                            "key": "location",
                            "value": {"stringValue": f"{device.location}"},
                        },
                        {
                            "key": "deployment.type",
                            "value": {"stringValue": f"{device.deployment_type}"},
                        },
                    ]
                },
                "scopeMetrics": [
                    {
                        "scope": {
                            "name": "sensor.firmware",
                            "version": f"{device.model}",
                        },
                        "metrics": "$METRICS",
                    }
                ],
            }
        ]
    }
