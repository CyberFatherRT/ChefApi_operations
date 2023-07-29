FROM rust AS planner

WORKDIR /app

RUN rustup default nightly
RUN cargo install cargo-chef
COPY .. .
RUN cargo chef prepare --recipe-path recipe.json


FROM rust AS cacher

WORKDIR /app

RUN rustup default nightly
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


FROM rust AS builder

WORKDIR /app

COPY .. .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN rustup default nightly
RUN cargo build --release
RUN strip -s ./target/release/cyber_knight_api


FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/cyber_knight_api .

CMD ["./cyber_knight_api"]
