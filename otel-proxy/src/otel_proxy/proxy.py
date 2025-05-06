import logging
from contextlib import asynccontextmanager

import httpx
from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

from otel_proxy._types import DevicePayload, Settings

from ._metrics_factory import build_otlp

logger = logging.getLogger("uvicorn.error")

settings = Settings()


@asynccontextmanager
async def lifespan(app: FastAPI):
    client = httpx.AsyncClient()
    app.state.http_client = client
    yield
    await client.aclose()


app = FastAPI(lifespan=lifespan)


@app.get("/")
def root():
    return {"message": "Hello world!"}


@app.post("/metrics")
async def report_metrics(request: Request, device_metrics: DevicePayload):
    payload = build_otlp(device_metrics)
    client: httpx.AsyncClient = request.app.state.http_client
    logger.debug(f"Posting to {settings.otel_url}, with headers {settings.headers}")
    response = await client.post(
        settings.otel_url, headers=settings.headers, json=payload
    )
    return JSONResponse(
        status_code=response.status_code,
        content=response.json()
        if response.headers.get("Content-Type") == "application/json"
        else {"response": response.text},
    )
