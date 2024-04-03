FROM rust:1-bullseye AS builder

WORKDIR /app
COPY . /app

RUN cargo build --release --bin sos24_presentation --verbose

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS release
LABEL maintainer="sohosai"
WORKDIR /app
COPY --from=builder /app/target/release/sos24_presentation /usr/local/bin
ENTRYPOINT ["/usr/local/bin/sos24_presentation"]
