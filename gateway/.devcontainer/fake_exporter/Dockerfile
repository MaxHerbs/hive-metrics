FROM python:3.11-slim

WORKDIR /app
COPY exporter.py .
RUN pip install --no-cache-dir prometheus_client

ENTRYPOINT [ "python" ]
CMD  ["exporter.py"]
