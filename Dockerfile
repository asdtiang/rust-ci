FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/rust_ci
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/rust_ci/target/release/rust_ci /app/rust_ci
COPY --from=builder /usr/src/rust_ci/templates /app/templates

EXPOSE 3000
CMD ["/app/rust_ci"]
