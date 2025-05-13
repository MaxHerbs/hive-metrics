FROM docker.io/library/rust:1.86.0-bookworm AS build
RUN rustup component add rustfmt clippy

WORKDIR /app

RUN cargo init
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY src ./

RUN cargo build --release