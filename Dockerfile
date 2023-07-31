FROM rust:latest AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/website_backend ./target/release/website_backend
ENV APP_PORT=80
ENV APP_HOST=0.0.0.0
ENV ALLOWED_ORIGIN=
ENV WEBHOOK_URL=
EXPOSE 80
CMD ["/target/release/website_backend"]