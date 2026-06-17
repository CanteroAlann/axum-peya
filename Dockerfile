FROM rust:1.79-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/web-app /usr/local/bin/web-app
EXPOSE 3000
ENTRYPOINT ["web-app"]
