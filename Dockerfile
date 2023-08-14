FROM rust:1.70 AS builder
WORKDIR /app
COPY Cargo.lock Cargo.toml .
COPY ./src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/target cargo build --release && cp ./target/release/website_backend .


FROM debian:bullseye-slim
RUN apt-get update \
    && apt-get install -y openssl ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR /app
COPY --from=builder /app/website_backend /app

ENV APP_PORT=80
ENV APP_HOST=0.0.0.0
ENV ALLOWED_ORIGIN=
ENV WEBHOOK_URL=
EXPOSE 80

CMD ["./website_backend"]