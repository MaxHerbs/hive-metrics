FROM docker.io/library/rust:1.86.0-bookworm AS build

WORKDIR /app

COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:eccec5274132c1be0ce5d2c8e6fe41033e64af5e987ccee9007826e4c012069d AS runtime

COPY --from=build /app/target/release/gateway /gateway

ENTRYPOINT ["/gateway"]
