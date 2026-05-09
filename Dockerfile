FROM rust:1.87-slim-bookworm AS builder

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/rust_ci /app/rust_ci
COPY --from=builder /app/templates /app/templates

EXPOSE 3000
CMD ["/app/rust_ci"]
