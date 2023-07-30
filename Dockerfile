FROM rust

WORKDIR /app

COPY . .
RUN cargo build -p backend --release
EXPOSE 8081
ENTRYPOINT ["./target/release/backend"]