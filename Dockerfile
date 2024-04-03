ARG APP_NAME=sos24-presentation

FROM rust:1-bullseye AS builder
ARG APP_NAME

WORKDIR /app
COPY . /app

RUN cargo build --release --bin ${APP_NAME} --verbose

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS release
ARG APP_NAME

LABEL maintainer="sohosai"
WORKDIR /app
COPY --from=builder /app/target/release/${APP_NAME} /usr/local/bin
ENTRYPOINT ["/usr/local/bin/sos24-presentation"]
