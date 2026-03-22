FROM rust:latest AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Build real application
COPY . .
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends bash openssl libpq5 libsqlite3-0 ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/* \
    && groupadd -r appuser && useradd -r -g appuser -d /app appuser

WORKDIR /app
COPY --from=builder /app/target/release/axum-starter ./api
COPY --from=builder /app/public ./public
COPY --from=builder /app/run.sh ./run.sh

RUN mkdir -p /app/data/logs /app/public/uploads \
    && chown -R appuser:appuser /app
RUN chmod +x ./run.sh

ENV RUN_BINARY_PATH=/app/api

USER appuser

EXPOSE 3099
VOLUME ["/app/data", "/app/public"]
ENTRYPOINT ["./run.sh"]
CMD ["start"]
