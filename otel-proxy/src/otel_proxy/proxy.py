import logging
import os

import httpx
from fastapi import FastAPI
from fastapi.responses import JSONResponse

from otel_proxy._types import DevicePayload

from ._metrics_factory import build_otlp

logger = logging.getLogger("uvicorn.error")
OTEL_URL = os.environ["OTEL_URL"]
HEADERS = {"Content-Type": "application/json"}

app = FastAPI()


@app.get("/")
def root():
    return {"message": "Hello world!asdasd"}


@app.post("/metrics")
async def report_metrics(device_metrics: DevicePayload):
    payload = build_otlp(device_metrics)

    async with httpx.AsyncClient() as client:
        logger.debug(f"Posting to {OTEL_URL}, with headers {HEADERS}")
        response = await client.post(OTEL_URL, headers=HEADERS, json=payload)
        return JSONResponse(
            status_code=response.status_code,
            content=response.json()
            if response.headers.get("Content-Type") == "application/json"
            else {"response": response.text},
        )
