FROM rust:1-bullseye AS builder
ARG APP_NAME
ENV APP_NAME=sos24-presentation

WORKDIR /app
COPY . /app

RUN cargo build --release --bin ${APP_NAME} --verbose

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS release
ENV APP-NAME=${APP_NAME}
LABEL maintainer="sohosai"
WORKDIR /app
COPY --from=builder /app/target/release/${APP_NAME} /usr/local/bin
ENTRYPOINT ["/usr/local/bin/${APP_NAME}"]
