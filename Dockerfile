FROM rust:1-bullseye AS builder
ARG APP-NAME
ENV APP-NAME=sos24-presentation

WORKDIR /app
COPY . /app

RUN cargo build --release --bin ${APP-NAME} --verbose

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS release
ENV APP-NAME=${APP-NAME}
LABEL maintainer="sohosai"
WORKDIR /app
COPY --from=builder /app/target/release/${APP-NAME} /usr/local/bin
ENTRYPOINT ["/usr/local/bin/${APP-NAME}"]
