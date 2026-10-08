FROM rust:latest AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/uangku_backend-cli /app/uangku-backend
COPY --from=builder /app/config /app/config

ENV LOCO_ENV=production
EXPOSE 3000

CMD ["/app/uangku-backend", "start"]