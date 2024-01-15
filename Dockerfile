FROM paritytech/ci-linux:61d4fd50-20230713 AS chef

# Force reinstall to recent nightly stable build
RUN rustup toolchain list | xargs rustup toolchain uninstall
RUN rustup install nightly
RUN rustup install stable
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN command -v wasm-gc || cargo +nightly install --git https://github.com/alexcrichton/wasm-gc --force

# Continue installing chef like in other images.
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
RUN cargo test