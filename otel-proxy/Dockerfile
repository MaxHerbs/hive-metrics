FROM docker.io/library/rust:1.86.0-bookworm AS build

ENV OPENSSL_STATIC=1
ENV OPENSSL_NO_VENDOR=0

RUN rustup component add rustfmt clippy
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update && apt-get install -y \
    musl-tools \
    perl \
    pkg-config \
    cmake \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src


RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch AS runtime

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/otel-proxy /otel-proxy

ENTRYPOINT ["/otel-proxy"]
CMD ["serve"]
