FROM rust:1.87

WORKDIR /app
COPY . /app

RUN /app/Cargo.toml
RUN cargo build --release
RUN mv /app/target/release/rust_ci /rust_ci && \
    rm -rf /app/target /app/src /app/Cargo.toml /app/Cargo.lock /usr/local/cargo/registry

EXPOSE 3000
CMD ["/rust_ci"]
