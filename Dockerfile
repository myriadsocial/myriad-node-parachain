FROM paritytech/ci-linux:42a94a57-20220730@sha256:9666da1a0ad5f4d127e3cbd7fcf2aae0165a33ecff4fb0efd9baf03f24093483 AS chef
RUN cargo install cargo-chef --version 0.1.31
WORKDIR /app

FROM chef AS planner
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./runtime /app/runtime
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

# Rebuild the source code only when needed
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./node /app/node
COPY ./pallets /app/pallets
COPY ./runtime /app/runtime
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
RUN cargo build --release
RUN cargo test