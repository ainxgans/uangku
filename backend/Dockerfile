FROM rust:1.90-slim-bullseye AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/uangku_backend-cli /app/uangku-backend
COPY --from=builder /app/config /app/config

ENV LOCO_ENV=production
EXPOSE 3000

CMD ["/app/uangku-backend", "start"]