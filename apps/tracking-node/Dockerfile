FROM rust:latest AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release --package web-app

# --- ETAPA DE PRODUCCIÓN ---
FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/web-app /usr/local/bin/web-app

# axum port
EXPOSE 3000
# grpc port
EXPOSE 50051

ENTRYPOINT ["web-app"]
