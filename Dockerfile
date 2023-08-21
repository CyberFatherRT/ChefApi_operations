FROM rust:latest AS builder

RUN update-ca-certificates

ENV USER=chef
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /chef_api

COPY . .
RUN cargo build --release
RUN strip -s /cyber_knight_api/target/release/chef_api

FROM gcr.io/distroless/cc

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /cyber_knight_api

COPY --from=builder /chef_api/target/release/chef_api ./

USER knight:knight

EXPOSE 8081

CMD ["/cyber_knight_api/cyber_knight_api"]
