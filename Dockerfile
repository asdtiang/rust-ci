FROM rust:1.87-slim-bookworm

WORKDIR /app
COPY . /app
RUN cargo build --release && \
    mv /app/target/release/rust_ci /rust_ci && \
    rm -rf /app/target /app/src /app/Cargo.toml /app/Cargo.lock

EXPOSE 3000
CMD ["/rust_ci"]
