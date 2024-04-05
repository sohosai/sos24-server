ARG APP_NAME=sos24-presentation

FROM rust:1-bullseye AS builder
ARG APP_NAME

WORKDIR /app
COPY . /app

# cacheされたディレクトリはイメージに焼き付けられないため、最終生成物はキャッシュ外にコピーする
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release --bin ${APP_NAME} --verbose \
    && mkdir -p /tmp/release && cp -R /app/target/release/* /tmp/release

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS release
ARG APP_NAME

LABEL maintainer="sohosai"
WORKDIR /app
COPY --from=builder /tmp/release/${APP_NAME} /usr/local/bin
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
ENTRYPOINT ["/usr/local/bin/sos24-presentation"]
