FROM rust:slim-buster as builder
LABEL author="godfather"

RUN USER=root cargo new --bin CyberKnightApi

WORKDIR /CyberKnightApi

RUN rustup toolchain install nightly
RUN rustup default nightly

COPY . .

RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /CyberKnightApi/target/release/CyberKnightApi .

EXPOSE 8081

CMD ["./CyberKnightApi"]
