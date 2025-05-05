def formatter(temperature):
    payload = {
        "name": "temperature",
        "unit": "Celcius",
        "description": "Ambient temperature",
        "gauge": {
            "dataPoints": [{"asDouble": temperature, "timeUnixNano": "$TIMESTAMP"}]
        },
    }
    return payload
