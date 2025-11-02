# syntax=docker/dockerfile:1.7
FROM rust:1.88-slim

ENV CARGO_INCREMENTAL=1

RUN apt-get update -qq \
    && apt-get install -y --no-install-recommends \
        libssl-dev pkg-config clang build-essential ca-certificates libpq-dev gdb \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src                   ./src/
COPY benches               ./benches/
COPY .rust-toolchain.toml  ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch \
    && cargo install diesel_cli --no-default-features --features postgres \
    && rustup show >/dev/null \
    && rustup component add clippy rustfmt

COPY . .

RUN cargo install --locked cargo-watch
