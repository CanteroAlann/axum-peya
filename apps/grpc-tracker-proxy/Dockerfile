# ./apps/grpc-proxy/Dockerfile
FROM rust:latest AS builder
RUN apt-get update && apt-get install -y protobuf-compiler musl-dev
WORKDIR /usr/src/app
COPY . .

# Compilamos únicamente la aplicación del proxy dentro del workspace
RUN cargo build --release --package grpc-tracker-proxy

# --- ETAPA DE PRODUCCIÓN ---
# Cambiamos alpine:latest por debian:bookworm-slim
FROM debian:bookworm-slim

# Instalamos certificados SSL esenciales y librerías básicas si hicieran falta en el futuro
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/grpc-tracker-proxy /usr/local/bin/grpc-tracker-proxy
EXPOSE 5050
CMD ["grpc-tracker-proxy"]