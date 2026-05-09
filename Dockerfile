FROM rust:1.87

WORKDIR /app
COPY . /app

RUN mkdir -p /usr/local/cargo && \
    echo '[source.crates-io]\nreplace-with = "rsproxy"\n[source.rsproxy]\nregistry = "sparse+https://rsproxy.cn/crates.io-index/"' > /usr/local/cargo/config.toml && \
    cargo build --release && \
    mv /app/target/release/rust_ci /rust_ci && \
    rm -rf /app/target /app/src /app/Cargo.toml /app/Cargo.lock /usr/local/cargo/registry

EXPOSE 3000
CMD ["/rust_ci"]
