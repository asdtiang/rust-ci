FROM docker.1ms.run/rust:1.87-slim-bookworm

WORKDIR /app
COPY . /app

RUN mkdir -p /usr/local/cargo && \
    echo '[source.crates-io]\nreplace-with = "ustc"\n[source.ustc]\nregistry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' > /usr/local/cargo/config.toml && \
    cargo build --release && \
    mv /app/target/release/rust_ci /rust_ci && \
    rm -rf /app/target /app/src /app/Cargo.toml /app/Cargo.lock /usr/local/cargo/registry

EXPOSE 3000
CMD ["/rust_ci"]
