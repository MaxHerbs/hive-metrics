from pydantic import BaseModel
from pydantic_settings import BaseSettings


class DevicePayload(BaseModel):
    id: str
    location: str
    model: str
    deployment_type: str

    metrics: dict


class Settings(BaseSettings):
    otel_url: str
    headers: dict = {"Content-Type": "application/json"}
