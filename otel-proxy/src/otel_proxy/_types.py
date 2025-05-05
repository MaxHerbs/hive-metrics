from pydantic import BaseModel


class DevicePayload(BaseModel):
    id: str
    location: str
    model: str
    deployment_type: str

    metrics: dict
