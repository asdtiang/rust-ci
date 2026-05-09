FROM rust:1.87-slim-bookworm AS builder

WORKDIR /usr/src/rust_ci
COPY . .
RUN cargo build --release

WORKDIR /app

COPY --from=builder /usr/src/rust_ci/target/release/rust_ci /app/rust_ci
COPY --from=builder /usr/src/rust_ci/templates /app/templates

EXPOSE 3000
CMD ["/app/rust_ci"]
