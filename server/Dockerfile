FROM rust:1.96.0-alpine3.23 AS builder

WORKDIR /app
ENV SQLX_OFFLINE=true

COPY . .

RUN cargo build

FROM alpine:3.23

WORKDIR /app

COPY ./config /etc/magnet

COPY --from=builder /app/target/debug/server /app/magnet

ENTRYPOINT ["/app/magnet"]