FROM rust:1.87

# musl-tools 提供 musl-gcc，用于静态链接，兼容 CentOS 7 (glibc 2.17) 及更旧的 Linux
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . /app

RUN rustup target add x86_64-unknown-linux-musl

# --features sqlx/bundled 将 SQLite 静态打包进二进制，无需依赖系统 libsqlite3
RUN cargo build --release --target x86_64-unknown-linux-musl --features sqlx/bundled

RUN mv /app/target/x86_64-unknown-linux-musl/release/rust_ci /rust_ci && \
    rm -rf /app/target /app/src /app/Cargo.toml /app/Cargo.lock /usr/local/cargo/registry

EXPOSE 3000
CMD ["/rust_ci"]
